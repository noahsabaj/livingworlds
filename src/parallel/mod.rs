//! Parallel processing infrastructure for Living Worlds
//!
//! This module provides a single source of truth for all parallel operations,
//! preventing common bugs like O(n²) complexity and ensuring consistent patterns
//! across the codebase.
//!
//! # Architecture
//!
//! Following the gateway pattern, this module acts as the sole entry point for
//! parallel processing operations. All Rayon usage should go through this module
//! rather than direct imports.
//!
//! # Key Features
//!
//! - **O(n²) Prevention**: Compile-time and runtime checks prevent quadratic complexity
//! - **Consistent Patterns**: All parallel operations follow the same builder pattern
//! - **Safety First**: Panic isolation, progress tracking, and performance monitoring
//! - **Smart Chunking**: Automatic optimal chunk size calculation based on data and CPU
//!
//! # Usage
//!
//! ```rust
//! use crate::parallel::ParallelOperation;
//!
//! let results = ParallelOperation::new("Province processing")
//!     .data(provinces)
//!     .validate_no_linear_search()
//!     .map(|province| process_province(province))
//!     .execute()?;
//! ```

// Private module declarations - implementation details hidden
mod builder;
mod chunk_size;
mod operations;
mod patterns;
mod plugin;
mod resources;
mod safety;
mod systems;

// Public exports - controlled API surface following gateway pattern
pub use builder::{ParallelOperation, ParallelOperationBuilder};
pub use chunk_size::ChunkStrategy;
pub use operations::{
    parallel_chunks, parallel_enumerate, parallel_filter_map, parallel_fold,
    parallel_map, parallel_mutate, parallel_query_iter, parallel_with_lookup,
    parallel_zip, parallel_zip_mutate,
};
pub use patterns::{ParallelPattern, WithLookupMap};
pub use plugin::ParallelPlugin;
pub use resources::ParallelOperationStats;
pub use safety::{ParallelSafetyError, SafetyValidator};

// Re-export common types for convenience
pub use rayon::current_num_threads;