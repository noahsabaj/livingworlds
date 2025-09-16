//! Version information and build metadata
//!
//! This module provides the single source of truth for all version-related
//! information in Living Worlds. The version number comes from Cargo.toml
//! at compile time, while the development stage can be updated here.

/// The game version from Cargo.toml (automatically updated)
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The game name from Cargo.toml
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Development stage indicator
pub const STAGE: &str = "World Generation Complete";

/// Full version string for display
pub fn version_string() -> String {
    format!("v{} - {}", VERSION, STAGE)
}

/// Short version string (just the number)
pub fn version_number() -> String {
    format!("v{}", VERSION)
}

/// Build information for debugging
pub fn build_info() -> String {
    format!(
        "{} v{}\nStage: {}\nProfile: {}",
        NAME,
        VERSION,
        STAGE,
        if cfg!(debug_assertions) { "debug" } else { "release" }
    )
}