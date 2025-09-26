//! Nation laws panel module gateway
//!
//! Displays active and proposed laws for the currently selected nation.

// PRIVATE MODULES - Gateway architecture compliance
mod panel;
mod plugin;
mod types;
mod updates;

// ESSENTIAL EXPORTS
pub use plugin::NationLawsPanelPlugin;
pub use types::{NationLawsPanel, NationLawsPanelState};

// Internal re-exports for convenience
pub(crate) use panel::spawn_nation_laws_panel;
pub(crate) use updates::{update_active_laws_list, update_proposed_laws_list};