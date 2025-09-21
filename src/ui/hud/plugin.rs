//! HUD plugin implementation

use crate::states::GameState;
use crate::ui::despawn_ui_entities;
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

use super::{control_hints, setup, speed_display, time_display, HudRoot};

/// Plugin that manages all HUD elements.
define_plugin!(HudPlugin {
    reflect: [
        time_display::GameTimeDisplay,
        speed_display::GameSpeedDisplay,
        control_hints::ControlHintsText
    ],

    update: [
        (time_display::update_time_display,
         speed_display::update_speed_display,
         control_hints::update_control_hints).run_if(in_state(GameState::InGame))
    ],

    on_enter: {
        GameState::InGame => [setup::setup_hud]
    },

    on_exit: {
        GameState::InGame => [despawn_ui_entities::<HudRoot>]
    }
});
