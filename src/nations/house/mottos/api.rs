//! Convenience API functions for motto generation
//!
//! This module provides convenience functions that wrap the core generator
//! functionality for common use cases, keeping all implementation logic
//! out of the gateway module.

use super::data::{MottoRegistry, MottoStatistics};
use super::generator::{MottoGenerator, ValidationReport};
use super::selection::SelectionConfig;
use super::types::{CompoundMottoConfig, MottoError};

/// Create a new motto generator with default settings
///
/// This is a convenience function for creating a generator instance when you need
/// to generate multiple mottos or want access to advanced features.
pub fn create_generator() -> MottoGenerator {
    MottoGenerator::new()
}

/// Create a motto generator with custom configuration
///
/// Allows full customization of the selection and compound generation behavior.
pub fn create_generator_with_config(
    selection_config: SelectionConfig,
    compound_config: CompoundMottoConfig,
) -> MottoGenerator {
    MottoGenerator::with_configs(selection_config, compound_config)
}

/// Validate the entire motto system
///
/// Performs comprehensive validation of all motto data and generation capabilities.
/// Returns a detailed report of any issues found.
pub fn validate_motto_system() -> Result<ValidationReport, MottoError> {
    let mut generator = MottoGenerator::new();
    generator.validate_system()
}

/// Get comprehensive statistics about the motto data
///
/// Useful for understanding coverage, balance, and system health.
pub fn get_motto_data_statistics() -> Result<MottoStatistics, MottoError> {
    let mut registry = MottoRegistry::new();
    registry.get_statistics()
}
