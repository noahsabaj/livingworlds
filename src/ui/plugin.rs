//! Main UI plugin implementation

use super::{
    animation, hud, law_browser, loading, nation_info, nation_laws_panel,
    overlay_display, performance_dashboard, shortcuts, tile_info,
};
use bevy_plugin_builder::define_plugin;
use bevy_ui_builders::UiBuilderPlugin;

// The main UI plugin orchestrator
define_plugin!(UIPlugin {
    events: [
        super::TextInputSubmitEvent
    ],

    plugins: [
        // Core UI systems
        UiBuilderPlugin,
        animation::AnimationPlugin,
        shortcuts::ShortcutPlugin,
        // Game-specific UI plugins
        loading::LoadingIndicatorPlugin,
        hud::HudPlugin,
        overlay_display::OverlayDisplayPlugin,
        tile_info::TileInfoPlugin,
        nation_info::NationInfoPlugin,
        performance_dashboard::PerformanceDashboardPlugin,
        law_browser::LawBrowserPlugin,
        nation_laws_panel::NationLawsPanelPlugin
    ]
});
