//! HUD (Heads-Up Display) Module - Pure Gateway
//!
//! Orchestrates HUD elements including time display, speed indicators,
//! and control hints. This is a pure gateway module that only coordinates
//! submodules without containing any implementation logic.

use bevy::prelude::*;

// Submodules - all private, exposed through plugin
mod time_display;
mod speed_display;
mod control_hints;
mod setup;


/// Plugin that manages all HUD elements
pub struct HudPlugin;

/// Marker component for the HUD root entity
#[derive(Component)]
pub struct HudRoot;

// PLUGIN IMPLEMENTATION - Pure Orchestration

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        use crate::states::GameState;

        app
            // Register components
            .register_type::<time_display::GameTimeDisplay>()
            .register_type::<speed_display::GameSpeedDisplay>()
            .register_type::<control_hints::ControlHintsText>()

            // Systems from submodules
            .add_systems(OnEnter(GameState::InGame), setup::setup_hud)
            .add_systems(OnExit(GameState::InGame), setup::cleanup_hud)
            .add_systems(Update, (
                time_display::update_time_display,
                speed_display::update_speed_display,
                control_hints::update_control_hints,
            ).run_if(in_state(GameState::InGame)));
    }
}