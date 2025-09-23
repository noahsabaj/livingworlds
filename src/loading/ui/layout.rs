//! Loading screen layout functions

use super::components::LoadingScreenRoot;
use super::sections::{spawn_bottom_section, spawn_details_panel, spawn_top_section};
use crate::loading::state::LoadingState;
use bevy::prelude::*;

/// Setup the loading screen UI using builders
pub fn setup_loading_screen(mut commands: Commands, loading_state: Res<LoadingState>) {
    // Root container with proper UI components to avoid B0004 warnings
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.02, 0.02, 0.03)),
            LoadingScreenRoot,
        ))
        .with_children(|root_panel| {
            // ===== TOP SECTION: Title and Operation =====
            spawn_top_section(root_panel, &loading_state);

            // ===== MIDDLE SECTION: Details Panel with Loading Indicator =====
            spawn_details_panel(root_panel, &loading_state);

            // ===== BOTTOM SECTION: Progress Bar and Tips =====
            spawn_bottom_section(root_panel, &loading_state);
        });
}
