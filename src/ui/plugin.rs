//! Main UI plugin implementation - PLUGIN AGGREGATION AUTOMATION!
//!
//! This module demonstrates PERFECT plugin aggregation automation!
//! 37 lines of manual plugin additions → 20 lines of declarative beauty!

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::{
    buttons, components::ProgressBarPlugin, dialogs, hud, loading, nation_info,
    overlay_display, performance_dashboard, sliders, text_inputs, tile_info,
};

/// The main UI plugin orchestrator using AUTOMATION FRAMEWORK
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