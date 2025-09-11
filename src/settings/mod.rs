//! Settings module for Living Worlds
//!
//! This module provides a comprehensive settings system with UI, persistence,
//! and real-time configuration updates.

// Submodules
pub mod types;
pub mod components;
pub mod persistence;
pub mod settings_ui;
pub mod handlers;
pub mod resolution;
pub mod navigation;

// Re-exports for convenience
pub use types::*;
pub use components::*;
pub use persistence::{load_settings, save_settings};
pub use settings_ui::spawn_settings_menu;

use bevy::prelude::*;
use bevy_pkv::PkvStore;

/// Plugin that manages all settings-related functionality
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources - Note: GameSettings is loaded from disk in load_settings
            .init_resource::<TempGameSettings>()
            .init_resource::<SettingsDirtyState>()
            .init_resource::<ResolutionConfirmation>()
            .init_resource::<FocusedElement>()
            
            // Events
            .add_event::<SettingsChanged>()
            .add_event::<RequestResolutionConfirm>()
            
            // Load settings on startup
            .add_systems(Startup, persistence::load_settings)
            
            // Systems - only run when settings menu is open
            .add_systems(Update, (
                handlers::handle_tab_buttons,
                handlers::handle_cycle_buttons,
                handlers::handle_toggle_buttons,
                handlers::handle_slider_interactions,
                handlers::handle_apply_cancel_buttons,
                handlers::handle_preset_buttons,
                handlers::handle_reset_button,
                handlers::handle_unsaved_changes_dialog,
                handlers::track_dirty_state,
                handlers::update_apply_exit_button_hover,
                handlers::update_ui_on_settings_change,
                resolution::handle_resolution_confirm_request,
                resolution::update_resolution_countdown,
                resolution::handle_resolution_confirm_buttons,
                navigation::handle_keyboard_navigation,
            ))
            
            // Apply settings when changed
            .add_systems(Update, handlers::apply_settings_changes);
    }
}

/// Helper function to despawn the settings menu
pub fn despawn_settings_menu(
    mut commands: Commands,
    query: Query<Entity, With<SettingsMenuRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}