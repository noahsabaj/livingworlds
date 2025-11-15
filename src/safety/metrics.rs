//! Parallel Safety Metrics System
//!
//! This module provides metrics collection and reporting for parallel operations
//! to track performance improvements, safety validations, and potential issues.

use bevy::prelude::*;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Duration;

/// Resource for tracking parallel operation safety metrics
#[derive(Resource, Default)]
pub struct ParallelSafetyMetrics {
    /// Number of parallel operations validated
    pub operations_validated: AtomicUsize,
    /// Number of data integrity checks passed
    pub integrity_checks_passed: AtomicUsize,
    /// Number of performance improvements detected
    pub performance_improvements: AtomicUsize,
    /// Total time saved through parallelization (microseconds)
    pub time_saved_us: AtomicU64,
    /// Number of potential race conditions detected
    pub race_conditions_detected: AtomicUsize,
}

impl ParallelSafetyMetrics {
    /// Record a successful parallel operation validation
    pub fn record_validation(&self, time_saved: Duration) {
        self.operations_validated.fetch_add(1, Ordering::Relaxed);
        self.time_saved_us
            .fetch_add(time_saved.as_micros() as u64, Ordering::Relaxed);
    }

    /// Record a successful data integrity check
    pub fn record_integrity_check(&self) {
        self.integrity_checks_passed.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a performance improvement
    pub fn record_performance_improvement(&self) {
        self.performance_improvements
            .fetch_add(1, Ordering::Relaxed);
    }

    /// Record a potential race condition
    pub fn record_race_condition(&self) {
        self.race_conditions_detected
            .fetch_add(1, Ordering::Relaxed);
        warn!("Potential race condition detected in parallel operation!");
    }

    /// Get summary statistics
    pub fn summary(&self) -> SafetySummary {
        SafetySummary {
            operations_validated: self.operations_validated.load(Ordering::Relaxed),
            integrity_checks_passed: self.integrity_checks_passed.load(Ordering::Relaxed),
            performance_improvements: self.performance_improvements.load(Ordering::Relaxed),
            time_saved_ms: self.time_saved_us.load(Ordering::Relaxed) as f64 / 1000.0,
            race_conditions_detected: self.race_conditions_detected.load(Ordering::Relaxed),
        }
    }
}

/// Summary of safety check results
#[derive(Debug, Clone)]
pub struct SafetySummary {
    pub operations_validated: usize,
    pub integrity_checks_passed: usize,
    pub performance_improvements: usize,
    pub time_saved_ms: f64,
    pub race_conditions_detected: usize,
}

/// System to log safety metrics periodically
pub fn log_safety_metrics(
    safety_metrics: Res<ParallelSafetyMetrics>,
    time: Res<Time>,
    mut last_log: Local<f64>,
) {
    if time.elapsed_secs_f64() - *last_log > 30.0 {
        let summary = safety_metrics.summary();
        info!(
            "Parallel Safety Summary: {} operations validated, {} integrity checks passed, {:.2}ms time saved, {} race conditions detected",
            summary.operations_validated,
            summary.integrity_checks_passed,
            summary.time_saved_ms,
            summary.race_conditions_detected
        );
        *last_log = time.elapsed_secs_f64();
    }
}
