//! HUD plugin implementation

use crate::states::GameState;
use crate::ui::despawn_ui_entities;
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

use super::{control_hints, map_mode_display, setup, speed_display, time_display, HudRoot};

// Plugin that manages all HUD elements.
define_plugin!(HudPlugin {
    resources: [
        map_mode_display::MapModeDropdownState
    ],

    reflect: [
        time_display::GameTimeDisplay,
        speed_display::GameSpeedDisplay,
        control_hints::ControlHintsText,
        map_mode_display::MapModeDisplay
    ],

    update: [
        (time_display::update_time_display,
         speed_display::update_speed_display,
         control_hints::update_control_hints,
         // Map mode systems with explicit ordering to prevent race conditions
         map_mode_display::handle_map_mode_button,
         map_mode_display::handle_dropdown_item_clicks
            .before(map_mode_display::handle_dropdown_close),
         map_mode_display::handle_dropdown_close
            .before(map_mode_display::handle_map_mode_shortcut),
         map_mode_display::handle_map_mode_shortcut
            .before(map_mode_display::update_map_mode_display),
         map_mode_display::update_map_mode_display).run_if(in_state(GameState::InGame))
    ],

    on_enter: {
        GameState::InGame => [setup::setup_hud]
    },

    on_exit: {
        GameState::InGame => [despawn_ui_entities::<HudRoot>]
    }
});
