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
pub use generator::{
    GenerationMethod, GenerationStatistics, MottoGenerationResult, MottoGenerator, ValidationReport,
};

// Configuration types for customization
pub use types::{CompoundMottoConfig, MottoError, MottoRarity, MottoVariation, TraitCombination};

pub use selection::{MottoSelector, RarityProbabilities, SelectionConfig, SelectionStatistics};

pub use compound::{CompoundMottoGenerator, CompoundStatistics};

// Data access for advanced scenarios (validation, analysis, debugging)
pub use data::{MottoRegistry, MottoStatistics, TraitCultureStats};

// Convenience API functions
pub use api::{
    create_generator, create_generator_with_config, get_motto_data_statistics,
    validate_motto_system,
};
