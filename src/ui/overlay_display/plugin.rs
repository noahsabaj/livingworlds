//! Overlay display plugin implementation - UI AUTOMATION PERFECTION!
//!
//! This module demonstrates ADVANCED conditional system automation!
//! 30 lines of manual registration → 15 lines declarative elegance!

use crate::states::GameState;
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

use super::{mineral_legend, overlay_text, setup};

/// Plugin that manages overlay display UI using ADVANCED AUTOMATION!
///
/// **AUTOMATION ACHIEVEMENT**: 30 lines manual → 15 lines declarative!
define_plugin!(OverlayDisplayPlugin {
    update: [
        (overlay_text::update_overlay_display,
         mineral_legend::update_mineral_legend_visibility
            .run_if(resource_changed::<crate::resources::MapMode>))
            .run_if(in_state(GameState::InGame))
    ],

    on_enter: {
        GameState::InGame => [setup::setup_overlay_display]
    },

    on_exit: {
        GameState::InGame => [setup::cleanup_overlay_display]
    }
});
