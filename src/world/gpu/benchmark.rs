//! GPU Performance Benchmarking System
//!
//! This module provides comprehensive benchmarking to measure and validate
//! the performance gains achieved by GPU acceleration over CPU computation.

use super::{
    validate_gpu_cpu_elevation_generation, GpuComputeStatus, GpuGenerationConfig,
    GpuPerformanceMetrics, ValidationConfig, ValidationResult,
};
use crate::resources::MapDimensions;
use bevy::prelude::*;
use std::time::{Duration, Instant};

/// Benchmark configuration for measuring GPU performance
#[derive(Resource, Debug, Clone)]
pub struct BenchmarkConfig {
    pub enabled: bool,
    pub iterations: u32,         // Number of benchmark runs
    pub warm_up_iterations: u32, // Warm-up runs before measurement
    pub test_sizes: Vec<u32>,    // Different province counts to test
    pub detailed_logging: bool,  // Whether to log detailed timing data
    pub export_results: bool,    // Whether to save results to file
    pub comparison_mode: bool,   // Run both GPU and CPU for comparison
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            enabled: false,        // Don't benchmark by default
            iterations: 5,         // 5 runs for averaging
            warm_up_iterations: 2, // 2 warm-up runs
            test_sizes: vec![
                10_000,    // Small world
                100_000,   // Medium world
                1_000_000, // Large world
                3_000_000, // Maximum world
            ],
            detailed_logging: true,
            export_results: true,
            comparison_mode: true, // Compare GPU vs CPU
        }
    }
}

/// Individual benchmark run result
#[derive(Debug, Clone, serde::Serialize)]
pub struct BenchmarkRun {
    pub province_count: u32,
    pub gpu_time: Duration,
    pub cpu_time: Duration,
    pub gpu_success: bool,
    pub cpu_success: bool,
    pub memory_usage_mb: f32,
    pub validation_passed: bool,
}

/// Complete benchmark suite results
#[derive(Debug, Clone, serde::Serialize)]
pub struct BenchmarkResults {
    pub runs: Vec<BenchmarkRun>,
    pub total_duration: Duration,
    pub average_speedup: f32,
    pub peak_speedup: f32,
    pub efficiency_rating: f32, // Performance score 0-100
    pub memory_overhead_mb: f32,
    pub gpu_reliability: f32, // Success rate 0-1
}

/// Resource for tracking benchmark state
#[derive(Resource, Debug, Default)]
pub struct BenchmarkState {
    pub running: bool,
    pub current_test: Option<String>,
    pub completed_runs: u32,
    pub total_runs: u32,
    pub last_results: Option<BenchmarkResults>,
}

/// Run comprehensive GPU performance benchmark
pub fn run_gpu_benchmark(
    config: &BenchmarkConfig,
    gpu_status: &GpuComputeStatus,
    validation_config: &ValidationConfig,
) -> BenchmarkResults {
    info!(
        "Starting GPU performance benchmark with {} test sizes",
        config.test_sizes.len()
    );

    let benchmark_start = Instant::now();
    let mut all_runs = Vec::new();

    for &province_count in &config.test_sizes {
        info!("Benchmarking {} provinces", province_count);

        // Calculate appropriate dimensions for province count
        let dimensions = calculate_dimensions_for_province_count(province_count);

        // Run warm-up iterations
        for i in 0..config.warm_up_iterations {
            debug!(
                "Warm-up iteration {} for {} provinces",
                i + 1,
                province_count
            );
            let _ = run_single_benchmark(
                dimensions,
                12345, // Fixed seed for reproducibility
                gpu_status,
                validation_config,
                config.comparison_mode,
            );
        }

        // Run actual benchmark iterations
        let mut run_times_gpu = Vec::new();
        let mut run_times_cpu = Vec::new();
        let mut validation_results = Vec::new();

        for i in 0..config.iterations {
            debug!(
                "Benchmark iteration {} for {} provinces",
                i + 1,
                province_count
            );

            let result = run_single_benchmark(
                dimensions,
                12345 + i, // Vary seed slightly for each run
                gpu_status,
                validation_config,
                config.comparison_mode,
            );

            run_times_gpu.push(result.gpu_time);
            run_times_cpu.push(result.cpu_time);
            validation_results.push(result.validation_passed);

            if config.detailed_logging {
                info!(
                    "  Run {}: GPU {:.2}s, CPU {:.2}s, Speedup {:.1}x, Valid: {}",
                    i + 1,
                    result.gpu_time.as_secs_f32(),
                    result.cpu_time.as_secs_f32(),
                    if result.gpu_time.as_secs_f32() > 0.0 {
                        result.cpu_time.as_secs_f32() / result.gpu_time.as_secs_f32()
                    } else {
                        1.0
                    },
                    result.validation_passed
                );
            }

            all_runs.push(result);
        }

        // Calculate averages for this province count
        let avg_gpu_time = average_duration(&run_times_gpu);
        let avg_cpu_time = average_duration(&run_times_cpu);
        let validation_success_rate = validation_results.iter().filter(|&&v| v).count() as f32
            / validation_results.len() as f32;

        info!(
            "Results for {} provinces: GPU {:.2}s, CPU {:.2}s, Speedup {:.1}x, Validation {:.0}%",
            province_count,
            avg_gpu_time.as_secs_f32(),
            avg_cpu_time.as_secs_f32(),
            if avg_gpu_time.as_secs_f32() > 0.0 {
                avg_cpu_time.as_secs_f32() / avg_gpu_time.as_secs_f32()
            } else {
                1.0
            },
            validation_success_rate * 100.0
        );
    }

    let total_duration = benchmark_start.elapsed();

    // Analyze results
    let analysis = analyze_benchmark_results(&all_runs);

    info!(
        "Benchmark completed in {:.2}s: Avg speedup {:.1}x, Peak speedup {:.1}x, Efficiency {:.0}%",
        total_duration.as_secs_f32(),
        analysis.average_speedup,
        analysis.peak_speedup,
        analysis.efficiency_rating
    );

    let results = BenchmarkResults {
        runs: all_runs,
        total_duration,
        average_speedup: analysis.average_speedup,
        peak_speedup: analysis.peak_speedup,
        efficiency_rating: analysis.efficiency_rating,
        memory_overhead_mb: analysis.memory_overhead_mb,
        gpu_reliability: analysis.gpu_reliability,
    };

    if config.export_results {
        export_benchmark_results(&results);
    }

    results
}

/// Run a single benchmark iteration
fn run_single_benchmark(
    dimensions: MapDimensions,
    seed: u32,
    gpu_status: &GpuComputeStatus,
    validation_config: &ValidationConfig,
    comparison_mode: bool,
) -> BenchmarkRun {
    let province_count = dimensions.provinces_per_row * dimensions.provinces_per_col;

    // Create test continent seeds
    let continent_seeds = vec![
        (Vec2::new(0.0, 0.0), 0.8, 200.0),
        (Vec2::new(500.0, 300.0), 0.6, 150.0),
        (Vec2::new(-300.0, -200.0), 0.7, 180.0),
    ];

    // Run validation which includes both GPU and CPU timing
    let validation_result = if comparison_mode {
        validate_gpu_cpu_elevation_generation(
            dimensions,
            seed,
            continent_seeds,
            validation_config,
            gpu_status,
            &GpuGenerationConfig::default(),
        )
    } else {
        // If not in comparison mode, create minimal result
        ValidationResult {
            passed: true,
            total_tested: 0,
            mismatched_count: 0,
            max_difference: 0.0,
            avg_difference: 0.0,
            gpu_time: Duration::from_secs(0),
            cpu_time: Duration::from_secs(0),
            speedup_factor: 1.0,
            sample_mismatches: Vec::new(),
        }
    };

    // Estimate memory usage (rough calculation)
    let memory_usage_mb = estimate_memory_usage(province_count);

    BenchmarkRun {
        province_count,
        gpu_time: validation_result.gpu_time,
        cpu_time: validation_result.cpu_time,
        gpu_success: gpu_status.compute_supported,
        cpu_success: true, // CPU should always work
        memory_usage_mb,
        validation_passed: validation_result.passed,
    }
}

/// Calculate dimensions for a target province count
fn calculate_dimensions_for_province_count(target_count: u32) -> MapDimensions {
    // Calculate approximate square dimensions
    let side_length = (target_count as f32).sqrt() as u32;
    let provinces_per_row = side_length;
    let provinces_per_col = (target_count + provinces_per_row - 1) / provinces_per_row;

    MapDimensions {
        provinces_per_row,
        provinces_per_col,
        hex_size: 1.0, // Standard hex size
        width_pixels: provinces_per_row as f32,
        height_pixels: provinces_per_col as f32,
        bounds: crate::world::MapBounds::default(),
    }
}

/// Calculate average of duration vector
fn average_duration(durations: &[Duration]) -> Duration {
    if durations.is_empty() {
        return Duration::from_secs(0);
    }

    let total_nanos: u128 = durations.iter().map(|d| d.as_nanos()).sum();
    Duration::from_nanos((total_nanos / durations.len() as u128) as u64)
}

/// Analyze benchmark results and calculate metrics
struct BenchmarkAnalysis {
    average_speedup: f32,
    peak_speedup: f32,
    efficiency_rating: f32,
    memory_overhead_mb: f32,
    gpu_reliability: f32,
}

fn analyze_benchmark_results(runs: &[BenchmarkRun]) -> BenchmarkAnalysis {
    if runs.is_empty() {
        return BenchmarkAnalysis {
            average_speedup: 1.0,
            peak_speedup: 1.0,
            efficiency_rating: 0.0,
            memory_overhead_mb: 0.0,
            gpu_reliability: 0.0,
        };
    }

    // Calculate speedups
    let speedups: Vec<f32> = runs
        .iter()
        .filter(|run| run.gpu_time.as_secs_f32() > 0.0)
        .map(|run| run.cpu_time.as_secs_f32() / run.gpu_time.as_secs_f32())
        .collect();

    let average_speedup = if speedups.is_empty() {
        1.0
    } else {
        speedups.iter().sum::<f32>() / speedups.len() as f32
    };

    let peak_speedup = speedups.iter().fold(1.0f32, |acc, &x| acc.max(x));

    // Calculate efficiency rating (0-100 based on expected performance)
    let efficiency_rating = ((average_speedup - 1.0) / 19.0 * 100.0).clamp(0.0, 100.0); // Assume 20x is perfect

    // Calculate memory usage
    let memory_overhead_mb = runs
        .iter()
        .map(|run| run.memory_usage_mb)
        .fold(0.0f32, |acc, x| acc.max(x));

    // Calculate GPU reliability
    let gpu_success_count = runs
        .iter()
        .filter(|run| run.gpu_success && run.validation_passed)
        .count();
    let gpu_reliability = gpu_success_count as f32 / runs.len() as f32;

    BenchmarkAnalysis {
        average_speedup,
        peak_speedup,
        efficiency_rating,
        memory_overhead_mb,
        gpu_reliability,
    }
}

/// Estimate memory usage for province count
fn estimate_memory_usage(province_count: u32) -> f32 {
    // Rough estimate: positions + elevations + GPU buffers
    let positions_mb = (province_count * 8) as f32 / 1_048_576.0; // Vec2 = 8 bytes
    let elevations_mb = (province_count * 4) as f32 / 1_048_576.0; // f32 = 4 bytes
    let gpu_overhead_mb = 50.0; // Estimated GPU buffer overhead

    positions_mb + elevations_mb + gpu_overhead_mb
}

/// Export benchmark results to file
fn export_benchmark_results(results: &BenchmarkResults) {
    use std::fs;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("gpu_benchmark_{}.json", timestamp);

    match serde_json::to_string_pretty(results) {
        Ok(json) => {
            if let Err(e) = fs::write(&filename, json) {
                error!("Failed to write benchmark results to {}: {}", filename, e);
            } else {
                info!("Benchmark results exported to {}", filename);
            }
        }
        Err(e) => {
            error!("Failed to serialize benchmark results: {}", e);
        }
    }
}

/// System to display benchmark progress
pub fn benchmark_progress_system(benchmark_state: Res<BenchmarkState>) {
    if benchmark_state.running {
        if let Some(ref current_test) = benchmark_state.current_test {
            debug!(
                "Benchmark progress: {} ({}/{})",
                current_test, benchmark_state.completed_runs, benchmark_state.total_runs
            );
        }
    }
}
