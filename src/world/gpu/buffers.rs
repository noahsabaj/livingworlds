//! GPU Buffer management for compute shaders
//!
//! This module handles the creation and management of GPU buffers
//! for province data, noise parameters, and erosion simulation,
//! following Bevy's proper bind group patterns.

use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
    },
};
use bytemuck::{Pod, Zeroable};

use super::{
    node::ComputePipelines,
    resources::{ComputeBufferHandles, ErosionComputeSettings, NoiseComputeSettings},
};

/// GPU-compatible province data structure
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct GpuProvinceData {
    pub position: [f32; 2],
    pub elevation: f32,
    pub padding: f32,
}

/// GPU-compatible noise parameters - must match WGSL struct
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct GpuNoiseParams {
    pub seed: u32,
    pub octaves: u32,
    pub frequency: f32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub amplitude: f32,
    pub scale: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub _padding: [f32; 3],
}

/// GPU-compatible erosion parameters - must match WGSL struct
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct GpuErosionParams {
    pub width: u32,
    pub height: u32,
    pub cell_size: f32,
    pub seed: u32,
    pub initial_water: f32,
    pub evaporation_rate: f32,
    pub sediment_capacity: f32,
    pub min_slope: f32,
    pub erosion_rate: f32,
    pub deposition_rate: f32,
    pub gravity: f32,
    pub inertia: f32,
    pub max_lifetime: u32,
    pub thermal_angle_threshold: f32,
    pub thermal_rate: f32,
    pub _padding: f32,
}

/// Resource containing computed bind groups for GPU operations
#[derive(Resource)]
pub struct ComputeBindGroups {
    pub noise_bind_group: Option<BindGroup>,
    pub erosion_bind_group: Option<BindGroup>,
}

impl Default for ComputeBindGroups {
    fn default() -> Self {
        Self {
            noise_bind_group: None,
            erosion_bind_group: None,
        }
    }
}

/// System to prepare compute bind groups (runs every frame in PrepareBindGroups)
pub fn prepare_compute_bind_groups(
    mut commands: Commands<'_, '_>,
    pipeline: Res<'_, ComputePipelines>,
    buffer_handles: Res<'_, ComputeBufferHandles>,
    noise_settings: Res<'_, NoiseComputeSettings>,
    erosion_settings: Res<'_, ErosionComputeSettings>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
) {
    let mut bind_groups = ComputeBindGroups::default();

    // Prepare noise generation bind group
    if let (Some(positions_buffer), Some(elevations_buffer)) = (
        buffer_handles.positions_buffer.as_ref(),
        buffer_handles.elevations_buffer.as_ref(),
    ) {
        // Create uniform buffer for noise parameters
        let params = GpuNoiseParams {
            seed: noise_settings.seed,
            octaves: noise_settings.octaves,
            frequency: noise_settings.frequency,
            persistence: noise_settings.persistence,
            lacunarity: noise_settings.lacunarity,
            amplitude: noise_settings.amplitude,
            scale: noise_settings.scale,
            offset_x: noise_settings.offset_x,
            offset_y: noise_settings.offset_y,
            _padding: [0.0; 3],
        };

        // Create uniform buffer manually using bytemuck
        let params_bytes = bytemuck::bytes_of(&params);
        let uniform_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("noise_params_uniform"),
            contents: params_bytes,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // Create bind group with all three bindings
        let bind_group = render_device.create_bind_group(
            Some("noise_compute_bind_group"),
            &pipeline.noise_layout,
            &BindGroupEntries::sequential((
                positions_buffer.as_entire_buffer_binding(),
                elevations_buffer.as_entire_buffer_binding(),
                uniform_buffer.as_entire_buffer_binding(),
            )),
        );

        bind_groups.noise_bind_group = Some(bind_group);
        debug!("Created noise compute bind group");
    }

    // Prepare erosion simulation bind group
    if let (Some(heightmap_buffer), Some(droplet_starts_buffer)) = (
        buffer_handles.heightmap_buffer.as_ref(),
        buffer_handles.droplet_starts_buffer.as_ref(),
    ) {
        // Create uniform buffer for erosion parameters
        let params = GpuErosionParams {
            width: 2000, // Will come from actual dimensions
            height: 1500,
            cell_size: 1.0,
            seed: erosion_settings.seed,
            initial_water: erosion_settings.initial_water,
            evaporation_rate: erosion_settings.evaporation_rate,
            sediment_capacity: erosion_settings.sediment_capacity,
            min_slope: erosion_settings.min_slope,
            erosion_rate: erosion_settings.erosion_rate,
            deposition_rate: erosion_settings.deposition_rate,
            gravity: erosion_settings.gravity,
            inertia: erosion_settings.inertia,
            max_lifetime: erosion_settings.max_lifetime,
            thermal_angle_threshold: erosion_settings.thermal_angle_threshold,
            thermal_rate: erosion_settings.thermal_rate,
            _padding: 0.0,
        };

        // Create uniform buffer manually using bytemuck
        let params_bytes = bytemuck::bytes_of(&params);
        let uniform_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("erosion_params_uniform"),
            contents: params_bytes,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // Use elevations_buffer if available, otherwise fallback to heightmap
        let elevation_buffer = buffer_handles
            .elevations_buffer
            .as_ref()
            .unwrap_or(heightmap_buffer);

        // Create bind group with all five bindings (including elevation buffers)
        let bind_group = render_device.create_bind_group(
            Some("erosion_compute_bind_group"),
            &pipeline.erosion_layout,
            &BindGroupEntries::sequential((
                heightmap_buffer.as_entire_buffer_binding(),
                droplet_starts_buffer.as_entire_buffer_binding(),
                uniform_buffer.as_entire_buffer_binding(),
                // Use elevations_buffer for both input and output (they're the same in our case)
                elevation_buffer.as_entire_buffer_binding(),
                elevation_buffer.as_entire_buffer_binding(),
            )),
        );

        bind_groups.erosion_bind_group = Some(bind_group);
        debug!("Created erosion compute bind group");
    }

    // Insert bind groups resource for use by compute nodes
    commands.insert_resource(bind_groups);
}

/// System to initialize GPU buffers when province data is available
pub fn init_compute_buffers(
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    mut buffer_handles: ResMut<'_, ComputeBufferHandles>,
) {
    // Initialize buffers only once
    if buffer_handles.positions_buffer.is_some() {
        return;
    }

    info!("Initializing GPU compute buffers");

    // Create position buffer for provinces
    // This will be filled with actual province positions during world generation
    let num_provinces = 3_000_000;
    let positions_size = (num_provinces * std::mem::size_of::<[f32; 2]>()) as u64;

    let positions_buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("province_positions_buffer"),
        size: positions_size,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Create elevation output buffer (vec4<f32> for WebGPU alignment)
    let elevations_size = (num_provinces * std::mem::size_of::<[f32; 4]>()) as u64;

    let elevations_buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("province_elevations_buffer"),
        size: elevations_size,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Create heightmap buffer for erosion (atomic integers)
    let width = 2000;
    let height = 1500;
    let heightmap_size = ((width * height * std::mem::size_of::<i32>()) as u64);

    let heightmap_buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("heightmap_atomic_buffer"),
        size: heightmap_size,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    // Create droplet starts buffer
    let num_droplets = 500_000;
    let droplet_size = (num_droplets * std::mem::size_of::<[f32; 2]>()) as u64;

    let droplet_buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("droplet_starts_buffer"),
        size: droplet_size,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Create parameter uniform buffers
    let noise_params_buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("noise_params_buffer"),
        size: std::mem::size_of::<GpuNoiseParams>() as u64,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let erosion_params_buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("erosion_params_buffer"),
        size: std::mem::size_of::<GpuErosionParams>() as u64,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Store all buffer handles
    buffer_handles.positions_buffer = Some(positions_buffer);
    buffer_handles.elevations_buffer = Some(elevations_buffer);
    buffer_handles.heightmap_buffer = Some(heightmap_buffer);
    buffer_handles.droplet_starts_buffer = Some(droplet_buffer);
    buffer_handles.noise_params_buffer = Some(noise_params_buffer);
    buffer_handles.erosion_params_buffer = Some(erosion_params_buffer);

    info!(
        "GPU buffers initialized: {} provinces, {}x{} heightmap, {} droplets",
        num_provinces, width, height, num_droplets
    );
}

/// System to upload province positions to GPU when world generation starts
pub fn upload_province_positions(
    positions: Vec<Vec2>,
    render_device: &RenderDevice,
    render_queue: &RenderQueue,
    buffer_handles: &mut ComputeBufferHandles,
) {
    if let Some(ref buffer) = buffer_handles.positions_buffer {
        // Convert Vec2 positions to [f32; 2] array
        let gpu_positions: Vec<[f32; 2]> = positions.iter().map(|pos| [pos.x, pos.y]).collect();

        // Write to GPU buffer
        render_queue.write_buffer(buffer, 0, bytemuck::cast_slice(&gpu_positions));

        info!("Uploaded {} province positions to GPU", positions.len());
    }
}

/// Shared elevation data that can be extracted between app worlds
#[derive(Resource, Default, Clone, ExtractResource)]
pub struct GpuElevationData {
    pub elevations: Option<Vec<f32>>,
    pub ready: bool,
}

/// Async GPU readback system for getting computed data back to CPU
/// This resource lives only in the Render app
#[derive(Resource, Default)]
pub struct GpuReadbackManager {
    pub elevation_requests: Vec<PendingReadback>,
    pub completed_elevations: Option<Vec<f32>>,
}

/// Represents a pending GPU readback operation
pub struct PendingReadback {
    pub staging_buffer: Buffer,
    pub size: u64,
    pub request_id: u32,
}

/// System to request elevation data readback from GPU
pub fn request_elevation_readback(
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    buffer_handles: Res<'_, ComputeBufferHandles>,
    mut readback_manager: ResMut<'_, GpuReadbackManager>,
    mut commands: Commands<'_, '_>,
) {
    if let Some(ref gpu_buffer) = buffer_handles.elevations_buffer {
        let num_provinces = 3_000_000; // Will come from actual province count
        let buffer_size = (num_provinces * std::mem::size_of::<f32>()) as u64;

        // Create staging buffer for CPU readback
        let staging_buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("elevation_readback_staging"),
            size: buffer_size,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Create command encoder to copy GPU data to staging buffer
        let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("elevation_readback_encoder"),
        });

        // Copy from GPU storage buffer to CPU-readable staging buffer
        encoder.copy_buffer_to_buffer(gpu_buffer, 0, &staging_buffer, 0, buffer_size);

        // Submit copy command
        render_queue.submit(std::iter::once(encoder.finish()));

        // Store pending readback request
        let request = PendingReadback {
            staging_buffer,
            size: buffer_size,
            request_id: readback_manager.elevation_requests.len() as u32,
        };

        readback_manager.elevation_requests.push(request);
        info!(
            "Requested GPU elevation readback for {} provinces",
            num_provinces
        );
    }
}

/// System to poll for completed GPU readbacks and process results
pub fn process_gpu_readbacks(
    mut readback_manager: ResMut<'_, GpuReadbackManager>,
    mut elevation_data: ResMut<'_, GpuElevationData>,
) {
    // Process pending readback requests
    let mut completed_indices = Vec::new();
    let mut completed_elevation_data = None;

    for (index, request) in readback_manager.elevation_requests.iter().enumerate() {
        let buffer_slice = request.staging_buffer.slice(..);

        // Try to map the buffer asynchronously
        let (sender, receiver) = std::sync::mpsc::channel();

        buffer_slice.map_async(MapMode::Read, move |result| {
            let _ = sender.send(result);
        });

        // Check if mapping completed (non-blocking)
        if let Ok(Ok(())) = receiver.try_recv() {
            // Buffer is ready - read the data (vec4<f32> format)
            let mapped_range = buffer_slice.get_mapped_range();
            // Cast to vec4 format and extract only the .x component (first element)
            let vec4_data: &[[f32; 4]] = bytemuck::cast_slice(&mapped_range);
            let elevation_data: Vec<f32> = vec4_data.iter().map(|v| v[0]).collect();

            // Store elevation data to assign later
            let data_len = elevation_data.len();
            completed_elevation_data = Some(elevation_data);
            completed_indices.push(index);

            info!(
                "GPU elevation readback completed: {} elevations",
                data_len
            );

            // Unmap the buffer
            drop(mapped_range);
            request.staging_buffer.unmap();
        }
    }

    // Assign completed elevation data after the loop to avoid borrowing conflicts
    if let Some(elevations) = completed_elevation_data {
        readback_manager.completed_elevations = Some(elevations.clone());
        // Update the shared resource for the main app
        elevation_data.elevations = Some(elevations);
        elevation_data.ready = true;
    }

    // Remove completed requests (reverse order to maintain indices)
    for &index in completed_indices.iter().rev() {
        readback_manager.elevation_requests.remove(index);
    }
}

/// Get completed elevation data if available
pub fn get_completed_elevations(readback_manager: &GpuReadbackManager) -> Option<Vec<f32>> {
    readback_manager.completed_elevations.clone()
}
