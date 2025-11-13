//! Nation laws panel module gateway
//!
//! Displays active and proposed laws for the currently selected nation.

// PRIVATE MODULES - Gateway architecture compliance
mod handlers;
mod panel;
mod plugin;
mod types;
mod updates;

// ESSENTIAL EXPORTS
pub use plugin::NationLawsPanelPlugin;
pub use types::NationLawsPanelState;

// Internal re-exports for convenience
