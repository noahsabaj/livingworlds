//! Family browser - Gateway module
//!
//! Provides a prestige-ranked list of all noble houses with filtering
//! and sorting capabilities.

// PRIVATE modules
mod plugin;
mod systems;
mod toggle;
mod types;
mod ui;

// PUBLIC exports
pub use plugin::FamilyBrowserPlugin;
pub use toggle::spawn_toggle_button;
pub use types::{
    FamilyBrowserFilters, FamilyBrowserPanel, OpenFamilyTreeEvent,
    CloseFamilyTreeEvent, SelectedHouseTree,
};
