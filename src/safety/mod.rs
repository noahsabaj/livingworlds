//! Parallel Safety Validation System
//!
//! This module provides comprehensive safety validation, performance benchmarking,
//! and race condition detection for parallel operations in Living Worlds.
//!
//! ## Gateway Architecture
//!
//! This module follows Living Worlds' gateway architecture pattern. All submodules
//! are private, and external access is controlled through this mod.rs file.
//!
//! ## Core Systems
//!
//! - **Metrics**: Track safety validations, performance improvements, and potential issues
//! - **Validation**: Ensure parallel operations produce correct results and are thread-safe
//! - **Detection**: Monitor for race conditions and concurrent access patterns
//! - **Benchmarking**: Compare performance between parallel and sequential operations
//! - **Plugin**: Bevy integration for automatic system registration
//!
//! ## Usage
//!
//! ```ignore
//! use crate::safety::{ParallelSafetyPlugin, validate_parallel_consistency};
//!
//! // Add to Bevy app
//! app.add_plugins(ParallelSafetyPlugin);
//!
//! // Validate parallel operations
//! let is_safe = validate_parallel_consistency(
//!     &data,
//!     |d| parallel_operation(d),
//!     |d| sequential_operation(d),
//!     "operation_name"
//! );
//! ```ignore

// Private submodules - all access controlled through this gateway
mod benchmarking;
mod detection;
mod metrics;
mod plugin;
mod validation;

// Public exports - controlled API surface
pub use plugin::ParallelSafetyPlugin;

// Metrics system exports

// Validation system exports

// Detection system exports

// Benchmarking system exports
