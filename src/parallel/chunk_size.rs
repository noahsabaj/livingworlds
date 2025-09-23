//! Intelligent chunk size calculation for optimal parallel performance
//!
//! This module provides strategies for determining the best chunk size
//! based on data size, CPU count, and cache efficiency.

use rayon::current_num_threads;

/// Strategy for determining chunk sizes in parallel operations
#[derive(Debug, Clone, Copy)]
pub enum ChunkStrategy {
    /// Automatically determine optimal chunk size
    Auto,

    /// Fixed chunk size
    Fixed(usize),

    /// Chunk size based on percentage of data
    Percentage(f32),

    /// One chunk per thread
    PerThread,

    /// Optimize for cache line efficiency
    CacheOptimized,
}

impl ChunkStrategy {
    /// Calculate the actual chunk size for given data
    pub fn calculate_chunk_size(&self, data_size: usize) -> usize {
        match self {
            ChunkStrategy::Auto => self.calculate_auto_chunk_size(data_size),
            ChunkStrategy::Fixed(size) => *size,
            ChunkStrategy::Percentage(pct) => ((data_size as f32 * pct) as usize).max(1),
            ChunkStrategy::PerThread => {
                let threads = current_num_threads();
                (data_size / threads).max(1)
            }
            ChunkStrategy::CacheOptimized => self.calculate_cache_optimized_size(data_size),
        }
    }

    /// Calculate automatic chunk size based on heuristics
    fn calculate_auto_chunk_size(&self, data_size: usize) -> usize {
        let threads = current_num_threads();

        // Base calculation: divide work evenly among threads
        let base_chunk = data_size / threads;

        // Apply heuristics based on data size
        let chunk_size = if data_size < 1000 {
            // Small data: process sequentially or in large chunks
            data_size
        } else if data_size < 10_000 {
            // Medium data: moderate chunks
            base_chunk.max(100)
        } else if data_size < 100_000 {
            // Large data: ensure chunks aren't too small
            base_chunk.max(1000)
        } else {
            // Very large data (like 3M provinces): optimize for cache
            // Cap at 50k for cache efficiency (as used in overlay/cache.rs)
            base_chunk.min(50_000).max(1000)
        };

        chunk_size.max(1)
    }

    /// Calculate cache-optimized chunk size
    fn calculate_cache_optimized_size(&self, data_size: usize) -> usize {
        // Typical L3 cache is 8-32MB, L2 is 256KB-1MB
        // Assuming each item is ~100-200 bytes, optimize for L2 cache
        const CACHE_LINE_SIZE: usize = 64;
        const L2_CACHE_SIZE: usize = 256 * 1024; // 256KB
        const ESTIMATED_ITEM_SIZE: usize = 128; // Rough estimate

        let items_per_cache = L2_CACHE_SIZE / ESTIMATED_ITEM_SIZE;
        let threads = current_num_threads();

        // Ensure each thread gets cache-friendly chunks
        let optimal_chunk = items_per_cache / 2; // Leave room for other data

        // Balance between cache efficiency and parallelism
        let min_chunks_needed = threads * 2; // At least 2 chunks per thread for load balancing
        let max_chunk_size = data_size / min_chunks_needed;

        optimal_chunk.min(max_chunk_size).max(CACHE_LINE_SIZE)
    }

    /// Get a human-readable description of the strategy
    pub fn description(&self) -> String {
        match self {
            ChunkStrategy::Auto => "Automatic chunk sizing based on data size and CPU count".to_string(),
            ChunkStrategy::Fixed(size) => format!("Fixed chunk size of {} items", size),
            ChunkStrategy::Percentage(pct) => format!("{}% of data per chunk", pct * 100.0),
            ChunkStrategy::PerThread => "One chunk per available thread".to_string(),
            ChunkStrategy::CacheOptimized => "Optimized for CPU cache efficiency".to_string(),
        }
    }
}

/// Calculate optimal number of chunks for a given data size
pub fn calculate_optimal_chunks(data_size: usize) -> usize {
    let threads = current_num_threads();

    // Rule of thumb: 2-4 chunks per thread for good load balancing
    // But not too many to avoid overhead
    let target_chunks = threads * 3;

    // Ensure chunks aren't too small
    let min_chunk_size = 100;
    let max_chunks = data_size / min_chunk_size;

    target_chunks.min(max_chunks).max(1)
}