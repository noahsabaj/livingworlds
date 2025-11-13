//! Law system debug tools gateway
//!
//! Provides debug utilities for testing and validating the law system.

// PRIVATE MODULES - Gateway architecture compliance
mod commands;
mod overlay;
mod validation;

// ESSENTIAL EXPORTS
pub use commands::{
    spawn_test_laws,
    LawDebugCommands,
};

pub use overlay::{
    LawDebugPlugin,
    toggle_law_debug_overlay,
};

pub use validation::validate_law_consistency;