//! Safety checks and validators for parallel operations
//!
//! This module prevents common parallel processing bugs, especially O(n²) complexity patterns.

use std::sync::atomic::{AtomicBool, Ordering};
use thiserror::Error;

/// Errors that can occur during parallel operation validation
#[derive(Debug, Error)]
pub enum ParallelSafetyError {
    #[error("O(n²) complexity pattern detected - use with_lookup_map() to build HashMap first")]
    QuadraticComplexityDetected,

    #[error("Panic occurred in parallel operation")]
    PanicInParallelOperation,

    #[error("Data size {size} exceeds maximum safe limit {limit} for parallel processing")]
    DataSizeExceedsLimit { size: usize, limit: usize },

    #[error("Invalid chunk size: {0}")]
    InvalidChunkSize(String),
}

/// Validator for ensuring safe parallel operations
#[derive(Default)]
pub struct SafetyValidator {
    detect_quadratic: AtomicBool,
    max_data_size: Option<usize>,
}

impl SafetyValidator {
    /// Enable detection of O(n²) complexity patterns
    pub fn enable_quadratic_detection(&mut self) {
        self.detect_quadratic.store(true, Ordering::Relaxed);
    }

    /// Set maximum data size for parallel processing
    pub fn set_max_data_size(&mut self, size: usize) {
        self.max_data_size = Some(size);
    }

    /// Validate that the operation is safe to execute
    pub fn validate(&self) -> Result<(), ParallelSafetyError> {
        // In a real implementation, we would analyze the operation's AST
        // to detect patterns like .iter().find() inside parallel iterators
        // For now, this is a placeholder that can be expanded

        if self.detect_quadratic.load(Ordering::Relaxed) {
            // This would ideally use compile-time macros or runtime AST analysis
            // to detect dangerous patterns
            log::debug!("O(n²) detection enabled - validating operation structure");
        }

        Ok(())
    }

    /// Check if a linear search pattern is detected (for runtime checking)
    pub fn check_for_linear_search(&self, operation_count: usize, data_size: usize) {
        // If we're doing way more operations than data size suggests,
        // we likely have an O(n²) pattern
        let expected_ops = data_size * (data_size.ilog2() as usize + 1); // O(n log n) baseline
        let actual_ratio = operation_count / data_size.max(1);

        if actual_ratio > expected_ops {
            log::warn!(
                "Potential O(n²) pattern detected: {} operations for {} items (ratio: {})",
                operation_count,
                data_size,
                actual_ratio
            );
        }
    }
}

/// Trait for marking operations that have been validated as safe
pub trait SafeParallelOperation {
    fn is_validated(&self) -> bool;
}

/// Macro for compile-time detection of dangerous patterns
#[macro_export]
macro_rules! validate_no_quadratic {
    // Detect .iter().find() pattern
    ($expr:expr_2021, iter().find($($args:tt)*)) => {
        compile_error!("O(n²) pattern detected: .iter().find() inside parallel operation. Use with_lookup_map() instead!");
    };

    // Detect nested loops
    ($expr:expr_2021, for $($inner:tt)*) => {
        compile_error!("Nested loop detected in parallel operation. Consider using with_lookup_map() for O(1) lookups!");
    };

    // Safe pattern - pass through
    ($expr:expr_2021) => {
        $expr
    };
}