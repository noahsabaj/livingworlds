//! CLI Module - Command Line Interface Management
//!
//! This module provides command-line interface handling for Living Worlds,
//! following the gateway architecture pattern. It handles:
//! - Command-line argument parsing and validation
//! - Application configuration building from CLI inputs
//! - Development mode parameter processing
//! - Error handling for invalid command-line inputs
//!
//! # Gateway Architecture
//! This module controls access to CLI components through a clean API.
//! Internal implementation details are private, and external code can only
//! access functionality through the exported interfaces.
//!
//! # Usage
//! ```ignore
//! use living_worlds::cli::{Args, build_app_config};
//!
//! // Parse command-line arguments
//! let args = Args::parse();
//!
//! // Build application configuration from CLI inputs
//! let config = build_app_config(&args);
//! ```

// Private module declarations - implementation details hidden from external code
mod args;
mod config;

// Public exports - controlled API surface following gateway pattern
pub use args::Args;
pub use config::build_app_config;
