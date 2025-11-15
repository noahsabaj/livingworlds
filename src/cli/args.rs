//! Command Line Arguments Processing for Living Worlds
//!
//! This module handles parsing and validation of command-line arguments
//! for the Living Worlds game. It provides structured access to CLI
//! parameters and development options.

use crate::resources::WorldSize;
use clap::Parser;

/// Living Worlds - Command line arguments
///
/// The game is primarily menu-driven, but these arguments allow
/// for debugging and development workflows.
#[derive(Parser, Debug)]
#[command(
    name = "Living Worlds",
    about = "A procedural civilization observer simulator",
    version,
    author
)]
pub struct Args {
    #[arg(long, help = "Enable debug output and diagnostics")]
    pub debug: bool,

    #[arg(
        long,
        help = "Skip menu and generate world immediately (development only)"
    )]
    pub dev_quick_start: bool,

    #[arg(
        long,
        requires = "dev_quick_start",
        help = "Seed for development world generation"
    )]
    pub dev_seed: Option<u32>,

    #[arg(
        long,
        requires = "dev_quick_start",
        value_parser = parse_world_size,
        help = "World size: small, medium, or large"
    )]
    pub dev_size: Option<WorldSize>,

    #[arg(
        long,
        default_value = "0",
        help = "Number of worker threads (0 for auto)"
    )]
    pub threads: usize,

    #[arg(long, help = "Display FPS counter")]
    pub show_fps: bool,
}

/// Parse and validate world size from string
///
/// # Arguments
/// * `s` - String representation of world size
///
/// # Returns
/// * `Ok(WorldSize)` - Valid world size enum
/// * `Err(String)` - Error message for invalid input
///
/// # Valid Inputs
/// - "small", "s" → WorldSize::Small
/// - "medium", "m" → WorldSize::Medium
/// - "large", "l" → WorldSize::Large
fn parse_world_size(s: &str) -> Result<WorldSize, String> {
    match s.to_lowercase().as_str() {
        "small" | "s" => Ok(WorldSize::Small),
        "medium" | "m" => Ok(WorldSize::Medium),
        "large" | "l" => Ok(WorldSize::Large),
        _ => Err(format!(
            "Invalid world size '{s}'. Must be: small, medium, or large"
        )),
    }
}

