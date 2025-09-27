//! Main Settings Menu Spawner
//!
//! Core functionality for spawning the complete settings menu UI.
//! Orchestrates all components using the builder pattern.

use super::{spawn_apply_cancel_buttons, spawn_tab_buttons};
use crate::settings::{components::*, types::*};
use crate::states::{CurrentSettingsTab, SettingsTab};
use crate::ui::{colors, ChildBuilder};
use bevy::prelude::*;

/// Main function to spawn the settings menu UI
pub fn spawn_settings_menu(
    mut commands: Commands,
    settings: &GameSettings,
    temp_settings: &mut TempGameSettings,
    current_tab: &CurrentSettingsTab,
    dirty_state: &mut SettingsDirtyState,
) {
    debug!("Spawning settings menu");

    // Copy current settings to temp for editing
    temp_settings.0 = settings.clone();

    // Reset dirty state when opening menu
    dirty_state.is_dirty = false;

    // Root container - dark overlay that blocks clicks
    commands
        .spawn((
            Button, // Add Button to block all clicks behind settings
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::OVERLAY_DARK),
            SettingsMenuRoot,
            ZIndex(200), // Above other menus
        ))
        .with_children(|parent| {
            // Settings panel
            parent
                .spawn((
                    Node {
                        width: Val::Px(800.0),     // Width constraint for readability
                        max_height: Val::Vh(90.0), // Safety valve - never bigger than 90% viewport
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        // NO HEIGHT SPECIFIED! Content determines it naturally
                        ..default()
                    },
                    BackgroundColor(colors::SURFACE),
                    BorderColor(colors::BORDER_HOVER),
                    ZIndex(10), // Above click blocker
                ))
                .with_children(|panel| {
                    // Title
                    spawn_title(panel);

                    // Tab buttons
                    spawn_tab_buttons(panel, current_tab.0);

                    // Tab content area
                    spawn_content_area(panel, current_tab.0, &temp_settings.0);

                    // Apply/Cancel buttons
                    spawn_apply_cancel_buttons(panel);
                });
        });
}

/// Spawns the settings menu title
fn spawn_title(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|title| {
            title.spawn((
                Text::new("SETTINGS"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
            ));
        });
}

/// Spawns the tab content area with appropriate content based on current tab
fn spawn_content_area(
    parent: &mut ChildBuilder,
    current_tab: SettingsTab,
    temp_settings: &GameSettings,
) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(1.0)),
                overflow: Overflow::scroll_y(), // Handle overflow gracefully
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_MEDIUM),
            BorderColor(colors::BORDER_DEFAULT),
        ))
        .with_children(|content| match current_tab {
            SettingsTab::Graphics => {
                crate::settings::ui::content::spawn_graphicstabdeclarative_content(
                    content,
                    &temp_settings.graphics,
                )
            }
            SettingsTab::Audio => {
                crate::settings::ui::content::spawn_audiotabdeclarative_content(
                    content,
                    &temp_settings.audio,
                )
            }
            SettingsTab::Interface => {
                crate::settings::ui::content::spawn_interfacetabdeclarative_content(
                    content,
                    &temp_settings.interface,
                )
            }
            SettingsTab::Performance => {
                crate::settings::ui::content::spawn_performance_content(content)
            }
            SettingsTab::Controls => {
                crate::settings::ui::content::spawn_controlstabdeclarative_content(
                    content,
                    &temp_settings.controls,
                )
            }
        });
}
