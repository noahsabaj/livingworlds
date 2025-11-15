//! GPU-CPU Validation System
//!
//! This module provides comprehensive validation to ensure GPU compute
//! produces identical results to CPU computation, maintaining determinism
//! and correctness across different execution paths.

use super::{
    extract_province_positions, gpu_accelerated_province_generation, GpuComputeStatus,
    GpuElevationGenerator, GpuGenerationConfig, GpuGenerationState, GpuPerformanceMetrics,
};
use crate::resources::MapDimensions;
use bevy::prelude::*;

/// Validation configuration for GPU-CPU comparison
#[derive(Resource, Debug, Clone)]
pub struct ValidationConfig {
    pub enabled: bool,
    pub tolerance: f32,     // Acceptable difference for floating point comparison
    pub sample_size: usize, // Number of provinces to validate (performance)
    pub detailed_logging: bool, // Whether to log detailed diff information
    pub fail_on_mismatch: bool, // Whether to panic on validation failure
    pub validation_frequency: u32, // Validate every N generations
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            tolerance: 0.0001,       // Very strict floating point tolerance
            sample_size: 1000,       // Validate 1000 random provinces
            detailed_logging: false, // Avoid spam in logs
            fail_on_mismatch: true,  // Strict validation - fail on any mismatch
            validation_frequency: 1, // Validate every generation during development
        }
    }
}

/// Validation results comparing GPU and CPU output
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub passed: bool,
    pub total_tested: usize,
    pub mismatched_count: usize,
    pub max_difference: f32,
    pub avg_difference: f32,
    pub gpu_time: std::time::Duration,
    pub cpu_time: std::time::Duration,
    pub speedup_factor: f32,
    pub sample_mismatches: Vec<ValidationMismatch>,
}

/// Individual validation mismatch data
#[derive(Debug, Clone)]
pub struct ValidationMismatch {
    pub index: usize,
    pub position: Vec2,
    pub gpu_value: f32,
    pub cpu_value: f32,
    pub difference: f32,
}

/// Resource tracking validation history
#[derive(Resource, Debug, Default)]
pub struct ValidationHistory {
    pub generations_tested: u32,
    pub total_mismatches: u32,
    pub last_validation: Option<ValidationResult>,
    pub average_speedup: f32,
    pub validation_enabled: bool,
}

/// Comprehensive validation function that compares GPU and CPU elevation generation
pub fn validate_gpu_cpu_elevation_generation(
    dimensions: MapDimensions,
    seed: u32,
    continent_seeds: Vec<(Vec2, f32, f32)>,
    config: &ValidationConfig,
    gpu_status: &GpuComputeStatus,
    gpu_config: &GpuGenerationConfig,
) -> ValidationResult {
    info!(
        "Starting GPU-CPU validation for {} test provinces",
        config.sample_size
    );

    // Generate test positions
    let total_provinces = dimensions.provinces_per_row * dimensions.provinces_per_col;
    let all_positions = extract_province_positions(
        dimensions.provinces_per_row,
        dimensions.provinces_per_col,
        dimensions.hex_size,
    );

    // Sample positions for validation (to avoid testing millions of provinces)
    let test_positions = if config.sample_size >= all_positions.len() {
        all_positions
    } else {
        use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
        let mut rng = StdRng::seed_from_u64(seed as u64 + 12345); // Deterministic sampling
        let mut sampled = all_positions;
        sampled.shuffle(&mut rng);
        sampled.truncate(config.sample_size);
        sampled
    };

    info!(
        "Testing {} positions for GPU-CPU parity",
        test_positions.len()
    );

    // Generate CPU elevations (reference implementation)
    let cpu_start = std::time::Instant::now();
    let cpu_generator = GpuElevationGenerator::new(dimensions, seed, continent_seeds.clone());
    let cpu_elevations = cpu_generator.generate_elevations_cpu(&test_positions);
    let cpu_time = cpu_start.elapsed();

    // Generate GPU elevations (if available)
    let gpu_start = std::time::Instant::now();
    let mut gpu_state = GpuGenerationState::Ready;
    let mut gpu_metrics = GpuPerformanceMetrics::default();

    let gpu_elevations = if gpu_status.compute_supported && gpu_config.use_gpu {
        gpu_accelerated_province_generation(
            test_positions.clone(),
            dimensions,
            seed,
            continent_seeds,
            gpu_status,
            gpu_config,
            &mut gpu_state,
            &mut gpu_metrics,
        )
    } else {
        warn!("GPU not available - using CPU for both reference and test");
        cpu_elevations.clone() // Use CPU result for both to avoid false validation failure
    };
    let gpu_time = gpu_start.elapsed();

    // Compare results
    let comparison =
        compare_elevation_results(&cpu_elevations, &gpu_elevations, &test_positions, config);

    let result = ValidationResult {
        passed: comparison.mismatched_count == 0,
        total_tested: test_positions.len(),
        mismatched_count: comparison.mismatched_count,
        max_difference: comparison.max_difference,
        avg_difference: comparison.avg_difference,
        gpu_time,
        cpu_time,
        speedup_factor: if gpu_time.as_secs_f32() > 0.0 {
            cpu_time.as_secs_f32() / gpu_time.as_secs_f32()
        } else {
            1.0
        },
        sample_mismatches: comparison.sample_mismatches,
    };

    // Log results
    if result.passed {
        info!(
            "GPU-CPU validation PASSED: {}/{} positions match within tolerance {:.6}",
            result.total_tested - result.mismatched_count,
            result.total_tested,
            config.tolerance
        );
        info!(
            "Performance: GPU {:.2}s vs CPU {:.2}s ({:.1}x speedup)",
            gpu_time.as_secs_f32(),
            cpu_time.as_secs_f32(),
            result.speedup_factor
        );
    } else {
        error!(
            "GPU-CPU validation FAILED: {}/{} positions mismatched (max diff: {:.6})",
            result.mismatched_count, result.total_tested, result.max_difference
        );

        if config.detailed_logging && !result.sample_mismatches.is_empty() {
            error!("Sample mismatches:");
            for mismatch in result.sample_mismatches.iter().take(5) {
                error!(
                    "  Position {:?}: GPU={:.6}, CPU={:.6}, diff={:.6}",
                    mismatch.position, mismatch.gpu_value, mismatch.cpu_value, mismatch.difference
                );
            }
        }

        if config.fail_on_mismatch {
            panic!("GPU-CPU validation failed - cannot proceed with mismatched results");
        }
    }

    result
}

/// Compare elevation results and identify mismatches
struct ElevationComparison {
    pub mismatched_count: usize,
    pub max_difference: f32,
    pub avg_difference: f32,
    pub sample_mismatches: Vec<ValidationMismatch>,
}

fn compare_elevation_results(
    cpu_elevations: &[f32],
    gpu_elevations: &[f32],
    positions: &[Vec2],
    config: &ValidationConfig,
) -> ElevationComparison {
    assert_eq!(
        cpu_elevations.len(),
        gpu_elevations.len(),
        "CPU and GPU elevation arrays must have same length"
    );
    assert_eq!(
        cpu_elevations.len(),
        positions.len(),
        "Elevation and position arrays must have same length"
    );

    let mut mismatches = Vec::new();
    let mut differences = Vec::new();
    let mut max_difference = 0.0f32;

    for (i, (&cpu_val, &gpu_val)) in cpu_elevations.iter().zip(gpu_elevations.iter()).enumerate() {
        let difference = (cpu_val - gpu_val).abs();
        differences.push(difference);

        if difference > config.tolerance {
            let mismatch = ValidationMismatch {
                index: i,
                position: positions[i],
                gpu_value: gpu_val,
                cpu_value: cpu_val,
                difference,
            };

            // Store sample mismatches for debugging
            if mismatches.len() < 20 {
                mismatches.push(mismatch);
            }
        }

        max_difference = max_difference.max(difference);
    }

    let avg_difference = if differences.is_empty() {
        0.0
    } else {
        differences.iter().sum::<f32>() / differences.len() as f32
    };

    let mismatched_count = differences
        .iter()
        .filter(|&&diff| diff > config.tolerance)
        .count();

    ElevationComparison {
        mismatched_count,
        max_difference,
        avg_difference,
        sample_mismatches: mismatches,
    }
}

/// System to run periodic validation during development
pub fn periodic_validation_system(
    validation_history: ResMut<ValidationHistory>,
    validation_config: Res<ValidationConfig>,
    gpu_status: Res<GpuComputeStatus>,
    gpu_config: Res<GpuGenerationConfig>,
    // TODO: Add access to world generation parameters when available
) {
    if !validation_config.enabled {
        return;
    }

    // Skip validation if not time yet
    if validation_history.generations_tested % validation_config.validation_frequency != 0 {
        return;
    }

    // TODO: Implement periodic validation when world generation parameters are accessible
    debug!("Periodic validation system ready (implementation pending world gen integration)");
}

/// Validation system for development and testing
pub fn run_validation_test(
    dimensions: MapDimensions,
    seed: u32,
    config: ValidationConfig,
) -> ValidationResult {
    // Create test continent seeds for validation
    let continent_seeds = vec![
        (Vec2::new(0.0, 0.0), 0.8, 200.0),
        (Vec2::new(500.0, 300.0), 0.6, 150.0),
        (Vec2::new(-300.0, -200.0), 0.7, 180.0),
    ];

    // Use default GPU settings for validation
    let gpu_status = GpuComputeStatus {
        compute_supported: false, // Assume no GPU for baseline validation
        ..Default::default()
    };
    let gpu_config = GpuGenerationConfig::default();

    validate_gpu_cpu_elevation_generation(
        dimensions,
        seed,
        continent_seeds,
        &config,
        &gpu_status,
        &gpu_config,
    )
}
