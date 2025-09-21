//! Tab Button Spawner
//!
//! Handles creation of the tab navigation buttons at the top of the settings menu.

use crate::settings::components::TabButton;
use crate::states::SettingsTab;
use crate::ui::{ButtonBuilder, ButtonSize, ButtonStyle, ChildBuilder};
use bevy::prelude::*;

/// Spawns the tab buttons row
pub fn spawn_tab_buttons(parent: &mut ChildBuilder, current_tab: SettingsTab) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                margin: UiRect::bottom(Val::Px(20.0)),
                column_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|tabs| {
            // Graphics tab button
            create_tab_button(tabs, "Graphics", SettingsTab::Graphics, current_tab);
            // Audio tab button
            create_tab_button(tabs, "Audio", SettingsTab::Audio, current_tab);
            // Interface tab button
            create_tab_button(tabs, "Interface", SettingsTab::Interface, current_tab);
            // Controls tab button
            create_tab_button(tabs, "Controls", SettingsTab::Controls, current_tab);
        });
}

/// Creates a single tab button using the ButtonBuilder (eating our own dog food)
fn create_tab_button(
    parent: &mut ChildBuilder,
    text: &str,
    tab: SettingsTab,
    current_tab: SettingsTab,
) {
    let is_active = tab == current_tab;
    let style = if is_active {
        ButtonStyle::Primary
    } else {
        ButtonStyle::Secondary
    };

    ButtonBuilder::new(text)
        .style(style)
        .size(ButtonSize::Small)
        .with_marker(TabButton { tab, enabled: true })
        .build(parent);
}
