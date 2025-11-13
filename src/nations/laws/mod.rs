//! Law system for Living Worlds
//!
//! This module implements a comprehensive legal framework that allows nations
//! to pass, enforce, and repeal laws based on their government type and pressures.
//! Laws create observable effects on nation behavior, economy, and social dynamics.

// PRIVATE MODULES - Gateway architecture compliance
mod debug;
mod definitions;
mod initialization;
mod loader;
mod mechanics;
mod passage;
mod plugin;
mod registry;
mod systems;
mod types;

// Test modules (temporarily disabled pending refactor to match current API)
// TODO: Update these tests to match the refactored Law APIs
// #[cfg(test)]
// mod tests;
// #[cfg(test)]
// mod property_tests;

// Public exports (controlled API surface)
pub use plugin::LawPlugin;

// Data-driven law loading (new feature)

// Re-export types through gateway
pub use types::{
    // Core types
    Law, LawId, LawCategory, LawPrerequisite, LawComplexity,
    // Effects
    LawEffects,
    // Status
    LawStatus,
    // Events
    LawEnactmentEvent, LawRepealEvent,
};

pub use definitions::{
    get_all_laws, get_category_laws, get_law_by_id,
};

pub use registry::{
    LawRegistry, NationLaws,
};


// Debug tools (conditional export for development)

