//! CLI Configuration Building for Living Worlds
//!
//! This module handles building application configuration from
//! command-line arguments.

use super::args::Args;
use crate::{AppConfig, DiagnosticsConfig};

/// FPS counter update interval in seconds
const DEFAULT_FPS_UPDATE_INTERVAL_SECS: f32 = 1.0;

/// Build application configuration from command line arguments
///
/// Constructs application configuration with CLI-driven overrides.
/// FPS display is enabled when `--show-fps` or `--debug` flags are set.
pub fn build_app_config(args: &Args) -> AppConfig {
    AppConfig {
        window: Default::default(),
        diagnostics: DiagnosticsConfig {
            show_fps: args.show_fps || args.debug,
            fps_interval: DEFAULT_FPS_UPDATE_INTERVAL_SECS,
            ..Default::default()
        },
        ..Default::default()
    }
}
