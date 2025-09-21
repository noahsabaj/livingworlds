//! Parallel Performance Benchmarking System
//!
//! This module provides benchmarking capabilities to measure and compare
//! performance between parallel and sequential operations.

use rayon::prelude::*;
use std::time::{Duration, Instant};

/// Performance benchmark for parallel vs sequential operations
pub fn benchmark_parallel_performance<T, F, G>(
    data: &[T],
    parallel_op: F,
    sequential_op: G,
    operation_name: &str,
    iterations: usize,
) -> PerformanceBenchmark
where
    T: Clone + Send + Sync + std::fmt::Debug,
    F: Fn(&[T]) + Send + Sync,
    G: Fn(&[T]),
{
    let mut parallel_times = Vec::with_capacity(iterations);
    let mut sequential_times = Vec::with_capacity(iterations);

    // Warm up
    parallel_op(data);
    sequential_op(data);

    // Benchmark parallel operation
    for _ in 0..iterations {
        let start = Instant::now();
        parallel_op(data);
        parallel_times.push(start.elapsed());
    }

    // Benchmark sequential operation
    for _ in 0..iterations {
        let start = Instant::now();
        sequential_op(data);
        sequential_times.push(start.elapsed());
    }

    let avg_parallel = parallel_times.iter().sum::<Duration>().as_secs_f64() / iterations as f64;
    let avg_sequential =
        sequential_times.iter().sum::<Duration>().as_secs_f64() / iterations as f64;

    let speedup = avg_sequential / avg_parallel;
    let efficiency = speedup / rayon::current_num_threads() as f64;

    PerformanceBenchmark {
        operation_name: operation_name.to_string(),
        parallel_time_ms: avg_parallel * 1000.0,
        sequential_time_ms: avg_sequential * 1000.0,
        speedup,
        efficiency,
        data_size: data.len(),
        thread_count: rayon::current_num_threads(),
    }
}

/// Performance benchmark results
#[derive(Debug, Clone)]
pub struct PerformanceBenchmark {
    pub operation_name: String,
    pub parallel_time_ms: f64,
    pub sequential_time_ms: f64,
    pub speedup: f64,
    pub efficiency: f64,
    pub data_size: usize,
    pub thread_count: usize,
}

impl PerformanceBenchmark {
    /// Check if the parallel version shows good performance
    pub fn is_performance_good(&self) -> bool {
        self.speedup > 1.5 && self.efficiency > 0.3
    }

    /// Get performance rating
    pub fn performance_rating(&self) -> &'static str {
        if self.speedup > 4.0 {
            "Excellent"
        } else if self.speedup > 2.0 {
            "Good"
        } else if self.speedup > 1.2 {
            "Fair"
        } else {
            "Poor"
        }
    }
}
