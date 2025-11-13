//! Universal Dropdown Component System
//!
//! A reusable dropdown component that can be used throughout the UI,
//! eliminating the need for custom dropdown implementations.
//!
//! # Features
//! - Type-safe value selection
//! - Search/filter support
//! - Keyboard navigation
//! - Custom item rendering
//! - Multi-select support
//!
//! # Usage
//! ```rust,no_run
//! commands.spawn(
//!     DropdownBuilder::new()
//!         .items(vec!["Option 1", "Option 2", "Option 3"])
//!         .selected(0)
//!         .on_change(|value| info!("Selected: {}", value))
//!         .build()
//! );
//! ```no_run

// GATEWAY ARCHITECTURE - Pure exports only

mod types;
mod components;
mod systems;
mod builder;
mod plugin;

// Core types
pub use types::{
    DropdownItem, DropdownValue,
};

// Components
pub use components::Dropdown;

// Builder API
pub use builder::DropdownBuilder;

// Systems (for advanced users)

// Plugin
pub use plugin::DropdownPlugin;