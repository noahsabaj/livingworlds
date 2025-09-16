//! HUD plugin implementation
//!
//! This module contains the HudPlugin that orchestrates all HUD functionality
//! including time display, speed indicators, and control hints.

use bevy::prelude::*;
use crate::states::GameState;

use super::{control_hints, setup, speed_display, time_display};

/// Plugin that manages all HUD elements
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register components
            .register_type::<time_display::GameTimeDisplay>()
            .register_type::<speed_display::GameSpeedDisplay>()
            .register_type::<control_hints::ControlHintsText>()
            // Systems from submodules
            .add_systems(OnEnter(GameState::InGame), setup::setup_hud)
            .add_systems(OnExit(GameState::InGame), setup::cleanup_hud)
            .add_systems(
                Update,
                (
                    time_display::update_time_display,
                    speed_display::update_speed_display,
                    control_hints::update_control_hints,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}