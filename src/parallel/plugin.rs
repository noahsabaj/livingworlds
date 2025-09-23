//! Bevy plugin for parallel processing integration
//!
//! This plugin ensures proper coordination between Bevy's ECS parallelism
//! and our custom parallel operations.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;


// Plugin for managing parallel operations in Living Worlds
///
// This plugin coordinates with Bevy's scheduler to ensure safe parallel execution
// and provides resources for monitoring parallel operation performance.
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

// Statistics for monitoring parallel operations
#[derive(Resource, Default, Debug)]
pub struct ParallelOperationStats {
    pub total_operations: u64,
    pub total_items_processed: u64,
    pub average_chunk_size: usize,
    pub quadratic_patterns_prevented: u32,
}

impl ParallelOperationStats {
    pub fn record_operation(&mut self, items: usize, chunk_size: usize) {
        self.total_operations += 1;
        self.total_items_processed += items as u64;

        // Update rolling average for chunk size
        let current_avg = self.average_chunk_size as f64;
        let new_avg = (current_avg * (self.total_operations - 1) as f64 + chunk_size as f64)
            / self.total_operations as f64;
        self.average_chunk_size = new_avg as usize;
    }

    pub fn record_quadratic_prevention(&mut self) {
        self.quadratic_patterns_prevented += 1;
        log::info!(
            "Prevented O(n²) pattern! Total prevented: {}",
            self.quadratic_patterns_prevented
        );
    }
}

// Initialize the parallel processing system
fn initialize_parallel_system(mut commands: Commands) {
    // Thread pool is already initialized in main.rs via ThreadPoolManager
    // This is where we'd set up any additional parallel processing infrastructure

    log::info!("Parallel processing system initialized");
    log::info!(
        "Using {} threads for parallel operations",
        rayon::current_num_threads()
    );

    // Initialize stats tracking
    commands.insert_resource(ParallelOperationStats::default());
}

// Log parallel processing configuration at startup
fn log_parallel_configuration() {
    let threads = rayon::current_num_threads();
    let total_cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    info!(
        "Parallel Configuration:
        - Worker threads: {}
        - Total CPU cores: {}
        - Thread utilization: {}%
        - Chunk strategy: Automatic
        - O(n²) prevention: Enabled",
        threads,
        total_cores,
        (threads * 100) / total_cores
    );
}

// Monitor parallel operations and log statistics
fn monitor_parallel_operations(stats: Res<ParallelOperationStats>) {
    if stats.total_operations > 0 && stats.total_operations % 100 == 0 {
        log::debug!(
            "Parallel stats: {} operations, {} items processed, avg chunk size: {}, O(n²) prevented: {}",
            stats.total_operations,
            stats.total_items_processed,
            stats.average_chunk_size,
            stats.quadratic_patterns_prevented
        );
    }
}