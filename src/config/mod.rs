//! Configuration Module - Application Configuration Management
//!
//! This module provides configuration types and management for Living Worlds,
//! following the gateway architecture pattern. It handles:
//! - Application-wide configuration settings
//! - Window configuration and display settings
//! - Diagnostics and performance monitoring configuration
//! - Default value management and validation
//!
//! # Gateway Architecture
//! This module controls access to configuration components through a clean API.
//! Internal implementation details are private, and external code can only
//! access functionality through the exported interfaces.
//!
//! # Usage
//! ```rust,no_run
//! use living_worlds::config::{AppConfig, WindowConfig, DiagnosticsConfig};
//!
//! // Create configuration with defaults
//! let config = AppConfig::default();
//!
//! // Customize window settings
//! let window_config = WindowConfig {
//!     width: 1920.0,
//!     height: 1080.0,
//!     title: "Living Worlds".to_string(),
//!     resizable: true,
//! };
//! ```no_run

// Private module declarations - implementation details hidden from external code
mod app;
mod diagnostics;
mod window;

// Public exports - controlled API surface following gateway pattern
pub use app::AppConfig;
pub use diagnostics::DiagnosticsConfig;
pub use window::WindowConfig;
