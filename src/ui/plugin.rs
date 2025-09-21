//! Main UI plugin implementation

use super::{
    buttons, components::ProgressBarPlugin, dialogs, hud, loading, nation_info, overlay_display,
    performance_dashboard, sliders, text_inputs, tile_info,
};
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

/// The main UI plugin orchestrator
define_plugin!(UIPlugin {
    plugins: [
        buttons::ButtonPlugin,
        dialogs::DialogPlugin,
        text_inputs::TextInputPlugin,
        loading::LoadingIndicatorPlugin,
        sliders::SliderPlugin,
        ProgressBarPlugin,
        hud::HudPlugin,
        overlay_display::OverlayDisplayPlugin,
        tile_info::TileInfoPlugin,
        nation_info::NationInfoPlugin,
        performance_dashboard::PerformanceDashboardPlugin
    ]
});
