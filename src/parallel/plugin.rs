//! Bevy plugin for parallel processing integration
//!
//! This plugin ensures proper coordination between Bevy's ECS parallelism
//! and our custom parallel operations.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::resources::ParallelOperationStats;
use super::systems::{initialize_parallel_system, log_parallel_configuration, monitor_parallel_operations};

/// Plugin for managing parallel operations in Living Worlds
///
/// This plugin coordinates with Bevy's scheduler to ensure safe parallel execution
/// and provides resources for monitoring parallel operation performance.
define_plugin!(ParallelPlugin {
    resources: [ParallelOperationStats],

    startup: [
        initialize_parallel_system,
        log_parallel_configuration
    ],

    update: [
        monitor_parallel_operations.run_if(resource_exists::<ParallelOperationStats>)
    ]
});