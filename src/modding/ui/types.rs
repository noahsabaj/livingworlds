//! Type definitions for the mod browser UI
//!
//! This module contains all component markers, events, and type definitions
//! used throughout the mod browser UI system.

use bevy::prelude::*;

// ============================================================================
// EVENTS
// ============================================================================

/// Event to open the mod browser
#[derive(Event)]
pub struct OpenModBrowserEvent;

/// Event to close the mod browser
#[derive(Event)]
pub struct CloseModBrowserEvent;

/// Event to apply mod changes and soft-reset
#[derive(Event)]
pub struct ApplyModChangesEvent;

/// Event to switch tabs in the mod browser
#[derive(Event)]
pub struct SwitchModTabEvent {
    pub tab: ModBrowserTab,
}

// ============================================================================
// COMPONENTS - UI ROOT MARKERS
// ============================================================================

/// Root component for the mod browser UI
#[derive(Component)]
pub struct ModBrowserRoot;

/// Component for the main content area
#[derive(Component)]
pub struct ContentArea;

/// Component for the search input field marker
#[derive(Component)]
pub struct SearchInputMarker;

// ============================================================================
// COMPONENTS - TABS
// ============================================================================

/// Component for mod browser tabs
#[derive(Component)]
pub struct ModBrowserTabButton {
    pub tab: ModBrowserTab,
}

/// Current active tab
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModBrowserTab {
    Installed,
    Workshop,
    ActiveModset,
}

// ============================================================================
// COMPONENTS - MOD CARDS & ITEMS
// ============================================================================

/// Component for a mod card in the grid
#[derive(Component)]
pub struct ModCard {
    pub mod_id: String,
    pub workshop_id: Option<u64>,
}

/// Component for mod toggle checkbox
#[derive(Component)]
pub struct ModToggle {
    pub mod_id: String,
}

/// Component for mod list items in Active Modset
#[derive(Component)]
pub struct ModListItem {
    pub mod_id: String,
    pub load_order: usize,
}

// ============================================================================
// COMPONENTS - FILTERS
// ============================================================================

/// Component for filter checkboxes
#[derive(Component)]
pub struct FilterCheckbox {
    pub filter_type: FilterType,
}

/// Types of filters available
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    ShowEnabled,
    ShowDisabled,
    ShowLocal,
    ShowWorkshop,
}

// ============================================================================
// COMPONENTS - BUTTONS
// ============================================================================

/// Component for the confirm modset button
#[derive(Component)]
pub struct ConfirmModsetButton;

/// Component for the close mod browser button
#[derive(Component)]
pub struct CloseModBrowserButton;