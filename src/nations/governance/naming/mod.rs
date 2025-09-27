//! Gateway module for governance-aware nation naming
//!
//! This module provides dynamic nation name generation based on government type,
//! ensuring names match their political structure.

// Private submodules (gateway architecture)
mod builder;
mod development;
mod formatter;
mod generator;
mod selection;
mod utils;
mod validation;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod builder_test;

// Public exports (controlled API surface)
pub use builder::build_nation_name;
pub use development::DevelopmentLevel;
pub use generator::{generate_governance_aware_name, get_ruler_title, get_structure_name};
pub use selection::suggest_government_for_culture;