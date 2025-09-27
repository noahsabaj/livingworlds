//! Parallel processing resources

use bevy::prelude::*;

/// Statistics for monitoring parallel operations
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