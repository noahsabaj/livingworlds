//! Family tree viewer - Gateway module
//!
//! Visualizes family relationships in a hierarchical tree layout
//! with interactive features like bloodline highlighting and filtering.

// PRIVATE modules
mod layout;
mod plugin;
mod systems;
mod types;
mod ui;

// PUBLIC exports
pub use plugin::FamilyTreePlugin;
pub use types::{FamilyTreePanel, TreeRelationshipFilters};
