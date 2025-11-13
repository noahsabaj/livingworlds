//! Setting Builder - Declarative Settings Tab Generation System
//!
//! This module provides the `define_setting_tab!` macro that eliminates boilerplate
//! from settings UI creation and event handling. Instead of manually implementing
//! UI spawning and event handlers, you declare your settings structure and the
//! macro handles all generation using existing UI builders.
//!
//! ## Revolutionary Features
//!
//! - **Zero Boilerplate**: Eliminates repetitive UI spawning and event handling code
//! - **Type Safety**: Compile-time validation of all setting declarations
//! - **Leverages Existing**: Uses proven UI builders (SliderBuilder, ButtonBuilder, etc.)
//! - **Consistent Styling**: Automatic application of standardized UI patterns
//! - **Error Prevention**: Can't forget to register event handlers or UI components
//!
//! ## Example Usage
//!
//! ```ignore
//! use crate::settings::setting_builder::define_setting_tab;
//! use crate::settings::types::GraphicsSettings;
//!
//! define_setting_tab!(GraphicsTab {
//!     settings_type: GraphicsSettings,
//!
//!     sections: [
//!         Section("Display") {
//!             cycle: "Window Mode" => window_mode,
//!             cycle: "Resolution" => resolution,
//!             toggle: "VSync" => vsync,
//!             slider: "Render Scale" => render_scale (0.5..2.0, percentage)
//!         },
//!
//!         Section("Quality") {
//!             cycle: "Shadow Quality" => shadow_quality,
//!             presets: [Low, Medium, High, Ultra]
//!         }
//!     ]
//! });
//! ```ignore
//!
//! This replaces 80+ lines of manual UI spawning and event handling with
//! 15 lines of declarative configuration.

// Private implementation modules
mod generation;
mod macros;
mod registration;
mod validation;

// Public API exports (gateway pattern)
pub use macros::define_setting_tab;

// Re-export commonly used types for convenience

// Make sure the macro is available at crate root
