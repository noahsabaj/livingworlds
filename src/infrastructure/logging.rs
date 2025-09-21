//! Logging Configuration for Living Worlds
//!
//! This module handles logging system initialization and configuration.
//! It provides intelligent log level management based on debug mode and
//! sets appropriate filters for different components.

/// Logging configuration manager for Living Worlds
pub struct LoggingConfig;

impl LoggingConfig {
    /// Initialize the logging system with appropriate levels
    ///
    /// This configures the RUST_LOG environment variable that Bevy uses
    /// for its logging infrastructure. The configuration varies based on
    /// whether debug mode is enabled.
    ///
    /// # Arguments
    /// * `debug_mode` - Enable verbose debug logging if true
    ///
    /// # Debug Mode Levels
    /// - `debug_mode = true`: Sets debug level for Living Worlds and Bevy
    /// - `debug_mode = false`: Sets info level with warnings for noisy components
    ///
    /// # Example
    /// ```rust
    /// use living_worlds::infrastructure::LoggingConfig;
    ///
    /// // Enable debug logging
    /// LoggingConfig::initialize(true);
    ///
    /// // Use production logging levels
    /// LoggingConfig::initialize(false);
    /// ```
    pub fn initialize(debug_mode: bool) {
        // Bevy will use these environment variables for its own logging
        let log_level = if debug_mode {
            "debug,living_worlds=debug"
        } else {
            // Production levels: info for core, warn for noisy GPU components
            "info,living_worlds=info,wgpu=warn,naga=warn"
        };

        // SAFETY: Setting environment variables during startup is safe
        // as no other threads are accessing these variables yet
        unsafe {
            std::env::set_var("RUST_LOG", log_level);
        }
    }
}
