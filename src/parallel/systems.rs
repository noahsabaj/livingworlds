//! Parallel processing systems

use bevy::prelude::*;
use super::resources::ParallelOperationStats;

/// Initialize the parallel processing system
pub fn initialize_parallel_system(mut commands: Commands) {
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

/// Log parallel processing configuration at startup
pub fn log_parallel_configuration() {
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

/// Monitor parallel operations and log statistics
pub fn monitor_parallel_operations(stats: Res<ParallelOperationStats>) {
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