//! GPU Compute nodes for the render graph
//!
//! This module implements the compute nodes that execute our GPU shaders
//! within Bevy's render graph system, following the Game of Life pattern.

use bevy::{
    prelude::*,
    render::{
        render_graph::{Node, NodeRunError, RenderGraphContext},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
    },
};

use super::{
    buffers::ComputeBindGroups,
    resources::{ComputeBufferHandles, ErosionComputeSettings, NoiseComputeSettings, GpuGenerationRequest},
};

/// Resource holding compute pipeline handles
#[derive(Resource)]
pub struct ComputePipelines {
    pub noise_layout: BindGroupLayout,
    pub erosion_layout: BindGroupLayout,
    pub noise_pipeline: CachedComputePipelineId,
    pub erosion_init_pipeline: CachedComputePipelineId,
    pub erosion_hydraulic_pipeline: CachedComputePipelineId,
    pub erosion_thermal_pipeline: CachedComputePipelineId,
    pub erosion_smooth_pipeline: CachedComputePipelineId,
}

/// State machine for noise generation pipeline
#[derive(Debug, Clone, Copy)]
enum NoiseComputeState {
    Loading,
    Ready,
    Generating,
    Complete,
}

/// Compute node for noise generation with state machine
pub struct NoiseComputeNode {
    state: NoiseComputeState,
}

impl Default for NoiseComputeNode {
    fn default() -> Self {
        Self {
            state: NoiseComputeState::Loading,
        }
    }
}

impl Node for NoiseComputeNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<ComputePipelines>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // State machine for pipeline progression
        match self.state {
            NoiseComputeState::Loading => {
                // Check if pipeline has loaded
                match pipeline_cache.get_compute_pipeline_state(pipeline.noise_pipeline) {
                    CachedPipelineState::Ok(_) => {
                        self.state = NoiseComputeState::Ready;
                        info!("Noise compute pipeline loaded successfully");
                    }
                    CachedPipelineState::Err(err) => {
                        warn!("Noise compute shader not ready: {:?}", err);
                    }
                    _ => {}
                }
            }
            NoiseComputeState::Ready => {
                // ONLY start generation if explicitly requested
                if let Some(request) = world.get_resource::<GpuGenerationRequest>() {
                    if request.requested && !request.completed {
                        // Also check that buffers exist
                        if let Some(buffers) = world.get_resource::<ComputeBufferHandles>() {
                            if buffers.positions_buffer.is_some() && buffers.elevations_buffer.is_some() {
                                self.state = NoiseComputeState::Generating;
                                info!("Starting GPU noise generation");
                            }
                        }
                    }
                }
            }
            NoiseComputeState::Generating => {
                // Check if generation has been completed via the request resource
                if let Some(request) = world.get_resource::<GpuGenerationRequest>() {
                    if request.completed {
                        self.state = NoiseComputeState::Complete;
                        info!("GPU noise generation complete");
                    }
                }
            }
            NoiseComputeState::Complete => {
                // Noise generation is done, could trigger erosion or readback here
            }
        }
    }

    fn run(
        &self,
        _graph: &mut RenderGraphContext<'_>,
        render_context: &mut RenderContext<'_>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        // Only run if we're in the generating state
        if !matches!(self.state, NoiseComputeState::Generating) {
            return Ok(());
        }

        // Check if already dispatched (dispatch_count > 0 means we've run)
        if let Some(request) = world.get_resource::<GpuGenerationRequest>() {
            if request.dispatch_count > 0 {
                return Ok(());
            }
        }

        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<ComputePipelines>();

        // Get bind groups if they exist
        let Some(bind_groups) = world.get_resource::<ComputeBindGroups>() else {
            return Ok(());
        };

        // Get the actual pipeline
        let Some(compute_pipeline) = pipeline_cache.get_compute_pipeline(pipeline.noise_pipeline)
        else {
            return Ok(());
        };

        // Get settings for dispatch size
        let settings = world.resource::<NoiseComputeSettings>();

        let mut pass =
            render_context
                .command_encoder()
                .begin_compute_pass(&ComputePassDescriptor {
                    label: Some("noise_generation_pass"),
                    timestamp_writes: None,
                });

        pass.set_pipeline(compute_pipeline);

        // Set bind group if it exists
        if let Some(ref bind_group) = bind_groups.noise_bind_group {
            pass.set_bind_group(0, bind_group, &[]);

            // Dispatch workgroups based on province count
            // This will be updated when we integrate with actual province data
            let num_provinces = 3_000_000u32; // Will come from actual data
            let workgroup_size = settings.workgroup_size;
            let num_workgroups = (num_provinces + workgroup_size - 1) / workgroup_size;

            pass.dispatch_workgroups(num_workgroups, 1, 1);

            info!(
                "Dispatched {} workgroups for {} provinces (one-time execution)",
                num_workgroups, num_provinces
            );
        }

        Ok(())
    }
}

/// State machine for erosion simulation pipeline
#[derive(Debug, Clone, Copy)]
enum ErosionComputeState {
    Loading,
    Ready,
    InitHeightmap,
    HydraulicErosion(u32), // Track batch number
    ThermalErosion,
    Smoothing,
    Complete,
}

/// Compute node for erosion simulation with state machine
pub struct ErosionComputeNode {
    state: ErosionComputeState,
}

impl Default for ErosionComputeNode {
    fn default() -> Self {
        Self {
            state: ErosionComputeState::Loading,
        }
    }
}

impl Node for ErosionComputeNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<ComputePipelines>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // State machine for erosion pipeline progression
        match self.state {
            ErosionComputeState::Loading => {
                // Check if all erosion pipelines have loaded
                let init_ready = matches!(
                    pipeline_cache.get_compute_pipeline_state(pipeline.erosion_init_pipeline),
                    CachedPipelineState::Ok(_)
                );
                let hydraulic_ready = matches!(
                    pipeline_cache.get_compute_pipeline_state(pipeline.erosion_hydraulic_pipeline),
                    CachedPipelineState::Ok(_)
                );
                let thermal_ready = matches!(
                    pipeline_cache.get_compute_pipeline_state(pipeline.erosion_thermal_pipeline),
                    CachedPipelineState::Ok(_)
                );
                let smooth_ready = matches!(
                    pipeline_cache.get_compute_pipeline_state(pipeline.erosion_smooth_pipeline),
                    CachedPipelineState::Ok(_)
                );

                if init_ready && hydraulic_ready && thermal_ready && smooth_ready {
                    self.state = ErosionComputeState::Ready;
                    info!("All erosion compute pipelines loaded successfully");
                }
            }
            ErosionComputeState::Ready => {
                // Check if we should start erosion (after noise is complete)
                if let Some(buffers) = world.get_resource::<ComputeBufferHandles>() {
                    if buffers.heightmap_buffer.is_some() && buffers.droplet_starts_buffer.is_some()
                    {
                        self.state = ErosionComputeState::InitHeightmap;
                        info!("Starting GPU erosion simulation");
                    }
                }
            }
            ErosionComputeState::InitHeightmap => {
                // After init, move to hydraulic erosion
                self.state = ErosionComputeState::HydraulicErosion(0);
            }
            ErosionComputeState::HydraulicErosion(batch) => {
                let settings = world.resource::<ErosionComputeSettings>();
                let total_batches = (settings.num_droplets + settings.droplets_per_batch - 1)
                    / settings.droplets_per_batch;

                if batch >= total_batches {
                    self.state = ErosionComputeState::ThermalErosion;
                } else {
                    self.state = ErosionComputeState::HydraulicErosion(batch + 1);
                }
            }
            ErosionComputeState::ThermalErosion => {
                self.state = ErosionComputeState::Smoothing;
            }
            ErosionComputeState::Smoothing => {
                self.state = ErosionComputeState::Complete;
                info!("GPU erosion simulation complete");
            }
            ErosionComputeState::Complete => {
                // Erosion is done, could trigger readback here
            }
        }
    }

    fn run(
        &self,
        _graph: &mut RenderGraphContext<'_>,
        render_context: &mut RenderContext<'_>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<ComputePipelines>();
        let settings = world.resource::<ErosionComputeSettings>();

        // Get bind groups if they exist
        let Some(bind_groups) = world.get_resource::<ComputeBindGroups>() else {
            return Ok(());
        };

        // Execute based on current state
        match self.state {
            ErosionComputeState::InitHeightmap => {
                if let Some(init_pipeline) =
                    pipeline_cache.get_compute_pipeline(pipeline.erosion_init_pipeline)
                {
                    if let Some(ref bind_group) = bind_groups.erosion_bind_group {
                        let mut pass = render_context.command_encoder().begin_compute_pass(
                            &ComputePassDescriptor {
                                label: Some("erosion_init_pass"),
                                timestamp_writes: None,
                            },
                        );

                        pass.set_pipeline(init_pipeline);
                        pass.set_bind_group(0, bind_group, &[]);

                        // Dispatch for heightmap initialization
                        let heightmap_size = 2000 * 1500; // Will come from actual dimensions
                        let workgroups = (heightmap_size + 255) / 256;
                        pass.dispatch_workgroups(workgroups, 1, 1);
                    }
                }
            }
            ErosionComputeState::HydraulicErosion(batch) => {
                if let Some(hydraulic_pipeline) =
                    pipeline_cache.get_compute_pipeline(pipeline.erosion_hydraulic_pipeline)
                {
                    if let Some(ref bind_group) = bind_groups.erosion_bind_group {
                        let mut pass = render_context.command_encoder().begin_compute_pass(
                            &ComputePassDescriptor {
                                label: Some(&format!("hydraulic_erosion_batch_{}", batch)),
                                timestamp_writes: None,
                            },
                        );

                        pass.set_pipeline(hydraulic_pipeline);
                        pass.set_bind_group(0, bind_group, &[]);

                        // Dispatch for this batch of droplets
                        let droplets_this_batch = settings
                            .droplets_per_batch
                            .min(settings.num_droplets - batch * settings.droplets_per_batch);
                        let workgroups = (droplets_this_batch + 63) / 64;
                        pass.dispatch_workgroups(workgroups, 1, 1);
                    }
                }
            }
            ErosionComputeState::ThermalErosion => {
                if let Some(thermal_pipeline) =
                    pipeline_cache.get_compute_pipeline(pipeline.erosion_thermal_pipeline)
                {
                    if let Some(ref bind_group) = bind_groups.erosion_bind_group {
                        let mut pass = render_context.command_encoder().begin_compute_pass(
                            &ComputePassDescriptor {
                                label: Some("thermal_erosion_pass"),
                                timestamp_writes: None,
                            },
                        );

                        pass.set_pipeline(thermal_pipeline);
                        pass.set_bind_group(0, bind_group, &[]);

                        // Dispatch for thermal erosion (2D grid)
                        let width_workgroups = (2000 + 7) / 8;
                        let height_workgroups = (1500 + 7) / 8;
                        pass.dispatch_workgroups(width_workgroups, height_workgroups, 1);
                    }
                }
            }
            ErosionComputeState::Smoothing => {
                if let Some(smooth_pipeline) =
                    pipeline_cache.get_compute_pipeline(pipeline.erosion_smooth_pipeline)
                {
                    if let Some(ref bind_group) = bind_groups.erosion_bind_group {
                        let mut pass = render_context.command_encoder().begin_compute_pass(
                            &ComputePassDescriptor {
                                label: Some("terrain_smoothing_pass"),
                                timestamp_writes: None,
                            },
                        );

                        pass.set_pipeline(smooth_pipeline);
                        pass.set_bind_group(0, bind_group, &[]);

                        // Dispatch for smoothing (2D grid)
                        let width_workgroups = (2000 + 7) / 8;
                        let height_workgroups = (1500 + 7) / 8;
                        pass.dispatch_workgroups(width_workgroups, height_workgroups, 1);
                    }
                }
            }
            _ => {} // Other states don't need GPU execution
        }

        Ok(())
    }
}

/// System to initialize compute pipelines at render startup
pub fn init_compute_pipelines(
    mut commands: Commands<'_, '_>,
    render_device: Res<RenderDevice>,
    asset_server: Res<AssetServer>,
    pipeline_cache: Res<PipelineCache>,
    existing_pipelines: Option<Res<ComputePipelines>>,
) {
    // Check if pipelines are already initialized
    if existing_pipelines.is_some() {
        return;
    }

    use bevy::render::render_resource::binding_types::{
        storage_buffer, storage_buffer_read_only,
    };

    // Create bind group layout for noise generation
    let noise_layout = render_device.create_bind_group_layout(
        "noise_compute_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                storage_buffer_read_only::<[f32; 2]>(false), // positions
                storage_buffer::<[f32; 4]>(false),           // elevations (vec4 for alignment)
                BindGroupLayoutEntry {
                    binding: u32::MAX,  // Set to MAX to avoid warning
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(std::num::NonZeroU64::new(std::mem::size_of::<super::buffers::GpuNoiseParams>() as u64)
                            .expect("GpuNoiseParams size should be non-zero")),
                    },
                    count: None,
                }, // params
            ),
        ),
    );

    // Create bind group layout for erosion
    let erosion_layout = render_device.create_bind_group_layout(
        "erosion_compute_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                storage_buffer::<i32>(false),                // binding 0: heightmap (atomic)
                storage_buffer_read_only::<[f32; 2]>(false), // binding 1: droplet starts
                BindGroupLayoutEntry {                       // binding 2: params
                    binding: u32::MAX,  // Set to MAX to avoid warning
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(std::num::NonZeroU64::new(std::mem::size_of::<super::buffers::GpuErosionParams>() as u64)
                            .expect("GpuErosionParams size should be non-zero")),
                    },
                    count: None,
                },
                storage_buffer_read_only::<[f32; 4]>(false), // binding 3: input_elevations (vec4)
                storage_buffer::<[f32; 4]>(false),           // binding 4: output_elevations (vec4)
            ),
        ),
    );

    // Load shaders
    let noise_shader = asset_server.load("shaders/noise_compute.wgsl");
    let erosion_shader = asset_server.load("shaders/erosion_compute.wgsl");

    // Queue noise pipeline
    let noise_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("noise_generation_pipeline".into()),
        layout: vec![noise_layout.clone()],
        shader: noise_shader,
        shader_defs: vec![],
        entry_point: Some("main".into()),
        push_constant_ranges: vec![],
        zero_initialize_workgroup_memory: true,
    });

    // Queue erosion pipelines (different entry points)
    let erosion_init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("erosion_init_pipeline".into()),
        layout: vec![erosion_layout.clone()],
        shader: erosion_shader.clone(),
        shader_defs: vec![],
        entry_point: Some("init_heightmap".into()),
        push_constant_ranges: vec![],
        zero_initialize_workgroup_memory: true,
    });

    let erosion_hydraulic_pipeline =
        pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("erosion_hydraulic_pipeline".into()),
            layout: vec![erosion_layout.clone()],
            shader: erosion_shader.clone(),
            shader_defs: vec![],
            entry_point: Some("hydraulic_erosion".into()),
            push_constant_ranges: vec![],
            zero_initialize_workgroup_memory: true,
        });

    let erosion_thermal_pipeline =
        pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("erosion_thermal_pipeline".into()),
            layout: vec![erosion_layout.clone()],
            shader: erosion_shader.clone(),
            shader_defs: vec![],
            entry_point: Some("thermal_erosion".into()),
            push_constant_ranges: vec![],
            zero_initialize_workgroup_memory: true,
        });

    let erosion_smooth_pipeline =
        pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("erosion_smooth_pipeline".into()),
            layout: vec![erosion_layout.clone()],
            shader: erosion_shader,
            shader_defs: vec![],
            entry_point: Some("smooth_terrain".into()),
            push_constant_ranges: vec![],
            zero_initialize_workgroup_memory: true,
        });

    // Store pipeline handles
    commands.insert_resource(ComputePipelines {
        noise_layout,
        erosion_layout,
        noise_pipeline,
        erosion_init_pipeline,
        erosion_hydraulic_pipeline,
        erosion_thermal_pipeline,
        erosion_smooth_pipeline,
    });

    info!("Compute pipelines queued for compilation");
}
