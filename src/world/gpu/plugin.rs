//! GPU compute plugin for noise generation and erosion simulation

use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResourcePlugin, render_graph::RenderGraph, Render, RenderApp,
        RenderSet,
    },
};
use bevy_plugin_builder::define_plugin;

use super::{
    benchmark::{benchmark_progress_system, BenchmarkConfig, BenchmarkState},
    buffers::{
        init_compute_buffers, prepare_compute_bind_groups, process_gpu_readbacks,
        request_elevation_readback, ComputeBindGroups, GpuElevationData, GpuReadbackManager,
    },
    capabilities::check_gpu_compute_support,
    coordinator::{
        coordinate_gpu_generation, handle_gpu_failures, monitor_gpu_timeouts, GpuGenerationConfig,
        GpuGenerationState, GpuPerformanceMetrics,
    },
    node::{init_compute_pipelines, NoiseComputeNode},
    resources::{ComputeBufferHandles, ErosionComputeSettings, NoiseComputeSettings},
    types::{ComputeLabel, GpuComputeStatus},
    validation::{periodic_validation_system, ValidationConfig, ValidationHistory},
};

/// GPU compute plugin for noise generation and erosion simulation
define_plugin!(NoiseComputePlugin {
    resources: [
        NoiseComputeSettings,
        ErosionComputeSettings,
        GpuComputeStatus,
        GpuGenerationState,
        GpuGenerationConfig,
        GpuPerformanceMetrics,
        ValidationConfig,
        ValidationHistory,
        BenchmarkConfig,
        BenchmarkState,
        GpuElevationData
    ],

    plugins: [
        ExtractResourcePlugin::<NoiseComputeSettings>::default(),
        ExtractResourcePlugin::<ErosionComputeSettings>::default(),
        ExtractResourcePlugin::<GpuElevationData>::default()
    ],

    update: [(
        coordinate_gpu_generation,
        handle_gpu_failures,
        monitor_gpu_timeouts,
        periodic_validation_system,
        benchmark_progress_system
    )
        .chain()],

    custom_init: |app: &mut App| {
        // Configure GPU sub-app
        let render_app = app.sub_app_mut(RenderApp);

        render_app
            // Initialize GPU resources
            .init_resource::<ComputeBufferHandles>()
            .init_resource::<ComputeBindGroups>()
            .init_resource::<GpuReadbackManager>()
            // Render systems with proper scheduling
            .add_systems(
                Render,
                (
                    init_compute_pipelines,
                    init_compute_buffers,
                    prepare_compute_bind_groups,
                )
                    .chain()
                    .in_set(RenderSet::PrepareResources),
            )
            .add_systems(Render, (process_gpu_readbacks,).in_set(RenderSet::Cleanup));

        // Configure render graph with compute nodes
        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(ComputeLabel::NoiseGeneration, NoiseComputeNode::default());
        render_graph.add_node_edge(
            ComputeLabel::NoiseGeneration,
            bevy::render::graph::CameraDriverLabel,
        );
    },

    // Initialize GPU compute support checking
    custom_finish: |app: &mut App| {
        let render_app = app.sub_app(RenderApp);
        let render_device = render_app
            .world()
            .resource::<bevy::render::renderer::RenderDevice>();

        let gpu_status = check_gpu_compute_support(render_device);
        app.insert_resource(gpu_status.clone());

        if !gpu_status.compute_supported {
            warn!(
                "GPU compute not fully supported: {:?}. Falling back to CPU generation.",
                gpu_status.fallback_reason
            );
        } else {
            info!(
                "GPU compute initialized: max workgroup size = {}, max buffer = {:.2} GB",
                gpu_status.max_workgroup_size,
                gpu_status.max_buffer_size as f64 / 1_073_741_824.0
            );
        }
    }
});
