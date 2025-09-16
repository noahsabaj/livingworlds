//! Main UI plugin implementation
//!
//! This module contains the UIPlugin that orchestrates all UI functionality
//! and integrates it with the Bevy app.

use bevy::prelude::*;

use super::buttons;
use super::components::ProgressBarPlugin;
use super::dialogs;
use super::hud;
use super::loading;
use super::overlay_display;
use super::sliders;
use super::text_inputs;
use super::tile_info;

/// The main UI plugin that orchestrates all UI functionality
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(buttons::ButtonPlugin)
            .add_plugins(dialogs::DialogPlugin)
            .add_plugins(text_inputs::TextInputPlugin)
            .add_plugins(loading::LoadingIndicatorPlugin)
            .add_plugins(sliders::SliderPlugin)
            .add_plugins(ProgressBarPlugin)
            .add_plugins(hud::HudPlugin)
            .add_plugins(overlay_display::OverlayDisplayPlugin)
            .add_plugins(tile_info::TileInfoPlugin);
    }
}