//! Overlay display plugin implementation
//!
//! This module contains the OverlayDisplayPlugin that manages the resource overlay
//! display system including the current overlay indicator and mineral legend.

use bevy::prelude::*;
use crate::states::GameState;

use super::{mineral_legend, overlay_text, setup};

/// Plugin that manages overlay display UI
pub struct OverlayDisplayPlugin;

impl Plugin for OverlayDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems from submodules
            .add_systems(OnEnter(GameState::InGame), setup::setup_overlay_display)
            .add_systems(OnExit(GameState::InGame), setup::cleanup_overlay_display)
            .add_systems(
                Update,
                (
                    overlay_text::update_overlay_display,
                    mineral_legend::update_mineral_legend_visibility
                        .run_if(resource_changed::<crate::resources::MapMode>),
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}