//! CLI Configuration Building for Living Worlds
//!
//! This module handles building application configuration from
//! command-line arguments. It provides structured configuration
//! generation based on CLI inputs.

use super::args::Args;
use crate::{AppConfig, DiagnosticsConfig};

/// CLI configuration builder for Living Worlds
pub struct CLIConfig;

impl CLIConfig {
    /// Build application configuration from command line arguments
    ///
    /// This method takes parsed command-line arguments and constructs
    /// a complete application configuration with appropriate defaults
    /// and CLI-driven overrides.
    ///
    /// # Arguments
    /// * `args` - Parsed command-line arguments structure
    ///
    /// # Returns
    /// * `AppConfig` - Complete application configuration
    ///
    /// # Configuration Logic
    /// - Window settings use defaults (can be configured via args in future)
    /// - FPS display enabled if `--show-fps` or `--debug` flags are set
    /// - Audio permanently disabled to prevent ALSA underrun errors on Linux
    /// - Console output disabled in favor of UI-based diagnostics
    ///
    /// # Example
    /// ```ignore
    /// use living_worlds::cli::{Args, CLIConfig};
    ///
    /// let args = Args::parse();
    /// let config = CLIConfig::build_app_config(&args);
    /// ```
    pub fn build_app_config(args: &Args) -> AppConfig {
        AppConfig {
            window: Default::default(), // Use default window settings
            diagnostics: DiagnosticsConfig {
                show_fps: args.show_fps || args.debug,
                fps_interval: 1.0,
                use_console: false, // Always use UI display, not console
            },
            enable_audio: false, // Always disabled to prevent ALSA underrun errors
        }
    }
}
