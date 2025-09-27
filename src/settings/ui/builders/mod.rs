//! Settings Builders Subsystem - Gateway
//!
//! Gateway for settings-specific UI builders. These builders leverage the existing
//! UI builder system to create consistent settings UI components.

// PRIVATE MODULES - Implementation hidden
mod preset_grid;
mod section;
mod setting_row;

// CONTROLLED EXPORTS - Builder structs for settings UI
pub use preset_grid::PresetGridBuilder;
pub use section::SettingSectionBuilder;
pub use setting_row::SettingRowBuilder;
