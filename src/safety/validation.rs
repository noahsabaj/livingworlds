//! Parallel Operation Validation System
//!
//! This module provides validation functions to ensure parallel operations
//! produce correct results and maintain thread safety.

use bevy::prelude::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Validate that parallel and sequential operations produce identical results
pub fn validate_parallel_consistency<T, F, G>(
    data: &[T],
    parallel_op: F,
    sequential_op: G,
    operation_name: &str,
) -> bool
where
    T: Clone + Send + Sync + std::fmt::Debug + PartialEq,
    F: Fn(&[T]) -> Vec<T> + Send + Sync,
    G: Fn(&[T]) -> Vec<T>,
{
    if data.is_empty() {
        return true;
    }

    let start_parallel = Instant::now();
    let parallel_result = parallel_op(data);
    let parallel_time = start_parallel.elapsed();

    let start_sequential = Instant::now();
    let sequential_result = sequential_op(data);
    let sequential_time = start_sequential.elapsed();

    // Check if results are identical (for operations where order matters)
    let results_match = parallel_result.len() == sequential_result.len()
        && parallel_result
            .iter()
            .zip(sequential_result.iter())
            .all(|(a, b)| a == b);

    if results_match {
        info!(
            "✅ Parallel consistency validated for {}: {} items processed",
            operation_name,
            data.len()
        );

        if parallel_time < sequential_time {
            let speedup = sequential_time.as_secs_f64() / parallel_time.as_secs_f64();
            info!(
                "🚀 Performance improvement: {:.2}x speedup ({:.2}ms vs {:.2}ms)",
                speedup,
                sequential_time.as_secs_f64() * 1000.0,
                parallel_time.as_secs_f64() * 1000.0
            );
        }
    } else {
        error!(
            "❌ Parallel consistency FAILED for {}: results differ! Parallel: {} items, Sequential: {} items",
            operation_name,
            parallel_result.len(),
            sequential_result.len()
        );
    }

    results_match
}

/// Validate thread safety of parallel operations
pub fn validate_thread_safety<T, F>(data: Arc<Vec<T>>, operation: F, operation_name: &str) -> bool
where
    T: Send + Sync + Clone + std::fmt::Debug + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    F: Fn(&T) -> bool + Send + Sync + Copy + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    let num_threads = rayon::current_num_threads();
    let success_count = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));

    // Run the operation in parallel multiple times to stress test
    (0..num_threads * 4).into_par_iter().for_each(|_| {
        for item in data.iter() {
            match std::panic::catch_unwind(|| operation(item)) {
                Ok(true) => {
                    success_count.fetch_add(1, Ordering::Relaxed);
                }
                Ok(false) => {
                    error_count.fetch_add(1, Ordering::Relaxed);
                }
                Err(_) => {
                    error_count.fetch_add(1, Ordering::Relaxed);
                    error!("Panic occurred in parallel operation: {}", operation_name);
                }
            }
        }
    });

    let successes = success_count.load(Ordering::Relaxed);
    let errors = error_count.load(Ordering::Relaxed);
    let total = successes + errors;

    if errors == 0 {
        info!(
            "✅ Thread safety validated for {}: {}/{} operations successful",
            operation_name, successes, total
        );
        true
    } else {
        error!(
            "❌ Thread safety FAILED for {}: {}/{} operations failed",
            operation_name, errors, total
        );
        false
    }
}
