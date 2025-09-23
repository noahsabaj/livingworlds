//! House motto generation system - Gateway Module
//!
//! This is a pure gateway module following Living Worlds architecture.
//! All implementation lives in focused submodules.

// Private submodules - implementation details hidden from external code
mod api;
mod compound;
mod data;
mod generator;
mod selection;
mod types;

// Public re-exports - carefully controlled API surface

// Main generation function - maintains compatibility with original API
pub use generator::generate_motto;

// Advanced generation API for sophisticated use cases

// Configuration types for customization



// Data access for advanced scenarios (validation, analysis, debugging)

// Convenience API functions
