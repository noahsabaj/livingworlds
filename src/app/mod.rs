//! Application Module - Bevy App Construction and Configuration
//!
//! This module provides application building functionality for Living Worlds,
//! following the gateway architecture pattern. It handles:
//! - Core Bevy application construction with all plugins
//! - Plugin dependency management and initialization ordering
//! - Steam integration and audio configuration
//! - Error handling for application building failures
//!
//! # Gateway Architecture
//! This module controls access to application building components through a clean API.
//! Internal implementation details are private, and external code can only
//! access functionality through the exported interfaces.
//!
//! # Usage
//! ```rust,no_run
//! use living_worlds::app::{build_app, build_app_with_config};
//! use living_worlds::config::AppConfig;
//!
//! // Build app with default configuration
//! let app = build_app()?;
//!
//! // Build app with custom configuration
//! let config = AppConfig::custom();
//! let app = build_app_with_config(config)?;
//! ```no_run

mod builder;
mod initialization;
mod plugins;

pub use builder::{build_app, build_app_with_config};
pub use builder::AppBuildError;
