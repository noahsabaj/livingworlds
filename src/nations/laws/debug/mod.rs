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
    force_enact_law,
    force_repeal_law,
    trigger_law_proposal,
    LawDebugCommands,
};

pub use overlay::{
    LawDebugOverlay,
    LawDebugPlugin,
    toggle_law_debug_overlay,
};

pub use validation::{
    validate_law_consistency,
    validate_nation_laws,
    check_law_conflicts,
    LawValidationReport,
};