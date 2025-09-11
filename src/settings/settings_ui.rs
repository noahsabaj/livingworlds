//! UI spawning and creation for the settings menu

use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use crate::states::{CurrentSettingsTab, SettingsTab};
use crate::ui::buttons::{ButtonBuilder, ButtonStyle, ButtonSize};
use super::types::*;
use super::components::*;

/// Main function to spawn the settings menu UI
pub fn spawn_settings_menu(
    mut commands: Commands,
    settings: Res<GameSettings>,
    mut temp_settings: ResMut<TempGameSettings>,
    current_tab: Res<CurrentSettingsTab>,
    mut dirty_state: ResMut<SettingsDirtyState>,
) {
    println!("Spawning settings menu");
    
    // Copy current settings to temp for editing
    temp_settings.0 = settings.clone();
    
    // Reset dirty state when opening menu
    dirty_state.is_dirty = false;
    
    // Root container - dark overlay that blocks clicks
    commands.spawn((
        Button,  // Add Button to block all clicks behind settings
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        SettingsMenuRoot,
        ZIndex(200), // Above other menus
    )).with_children(|parent| {
        
        // Settings panel
        parent.spawn((
            Node {
                width: Val::Px(800.0),
                height: Val::Px(600.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
            BorderColor(Color::srgb(0.4, 0.4, 0.45)),
            ZIndex(10),  // Above click blocker
        )).with_children(|panel| {
            // Title
            panel.spawn((
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            )).with_children(|title| {
                title.spawn((
                    Text::new("SETTINGS"),
                    TextFont {
                        font_size: 36.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            });
            
            // Tab buttons
            spawn_tab_buttons(panel, current_tab.0);
            
            // Tab content area
            panel.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.08, 0.08, 0.08)),
                BorderColor(Color::srgb(0.3, 0.3, 0.35)),
            )).with_children(|content| {
                // Spawn content based on current tab
                match current_tab.0 {
                    SettingsTab::Graphics => spawn_graphics_content(content, &temp_settings.0.graphics),
                    SettingsTab::Audio => spawn_audio_content(content, &temp_settings.0.audio),
                    SettingsTab::Interface => spawn_interface_content(content, &temp_settings.0.interface),
                    SettingsTab::Performance => spawn_performance_content(content),
                    SettingsTab::Controls => spawn_controls_content(content, &temp_settings.0.controls),
                }
            });
            
            // Apply/Cancel buttons
            spawn_apply_cancel_buttons(panel);
        });
    });
}

/// Spawns the tab buttons row
fn spawn_tab_buttons(parent: &mut ChildSpawnerCommands, current_tab: SettingsTab) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            margin: UiRect::bottom(Val::Px(20.0)),
            column_gap: Val::Px(10.0),
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).with_children(|tabs| {
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

/// Creates a single tab button
fn create_tab_button(parent: &mut ChildSpawnerCommands, text: &str, tab: SettingsTab, current_tab: SettingsTab) {
    let is_active = tab == current_tab;
    let style = if is_active {
        ButtonStyle::Primary
    } else {
        ButtonStyle::Secondary
    };
    
    ButtonBuilder::new(text)
        .style(style)
        .size(ButtonSize::Small)
        .with_marker(TabButton { tab })
        .build(parent);
}

/// Spawns the apply/cancel buttons
fn spawn_apply_cancel_buttons(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            margin: UiRect::top(Val::Px(20.0)),
            column_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).with_children(|buttons| {
        // Apply button
        ButtonBuilder::new("Apply")
            .style(ButtonStyle::Success)
            .size(ButtonSize::Medium)
            .enabled(false)  // Initially disabled until settings change
            .with_marker(ApplyButton)
            .build(buttons);
        
        // Exit button
        ButtonBuilder::new("Exit")
            .style(ButtonStyle::Danger)
            .size(ButtonSize::Medium)
            .with_marker(CancelButton)  // Keep component name for compatibility
            .build(buttons);
    });
}

/// Spawns graphics settings content with preset buttons
fn spawn_graphics_content(parent: &mut ChildSpawnerCommands, settings: &GraphicsSettings) {
    // Graphics Presets row
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(15.0),
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).with_children(|content| {
        // Preset buttons row
        spawn_graphics_presets(content, settings);
        
        // Individual settings
        create_cycle_row(content, "Window Mode", settings.window_mode.as_str(), SettingType::WindowMode);
        create_cycle_row(content, "Resolution", &settings.resolution.as_str(), SettingType::Resolution);
        create_toggle_row(content, "VSync", settings.vsync, SettingType::VSync);
        create_slider_row(content, "Render Scale", settings.render_scale, 0.5, 2.0, SettingType::RenderScale, true);
        create_cycle_row(content, "Shadow Quality", settings.shadow_quality.as_str(), SettingType::ShadowQuality);
    });
}

/// Spawns the graphics preset buttons
fn spawn_graphics_presets(parent: &mut ChildSpawnerCommands, settings: &GraphicsSettings) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(10.0),
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).with_children(|presets| {
        // Label
        presets.spawn((
            Text::new("Quality Presets:"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
            Node {
                margin: UiRect::right(Val::Px(15.0)),
                ..default()
            },
        ));
        
        // Preset buttons
        for preset in [GraphicsPreset::Low, GraphicsPreset::Medium, GraphicsPreset::High, GraphicsPreset::Ultra] {
            let is_active = settings.current_preset() == Some(preset);
            let preset_text = match preset {
                GraphicsPreset::Low => "Low",
                GraphicsPreset::Medium => "Medium",
                GraphicsPreset::High => "High",
                GraphicsPreset::Ultra => "Ultra",
            };
            
            let mut entity_commands = presets.spawn((
                Button,
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(35.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(if is_active {
                    Color::srgb(0.15, 0.3, 0.15)
                } else {
                    Color::srgb(0.15, 0.15, 0.18)
                }),
                BorderColor(if is_active {
                    Color::srgb(0.3, 0.5, 0.3)
                } else {
                    Color::srgb(0.3, 0.3, 0.35)
                }),
                PresetButton { preset },
                Focusable { order: preset as u32 },
            ));
            
            entity_commands.with_children(|btn| {
                btn.spawn((
                    Text::new(preset_text),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            });
        }
        
        // Show "Custom" indicator if no preset matches
        if settings.current_preset().is_none() {
            presets.spawn((
                Text::new("(Custom)"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.7, 0.3)),
                Node {
                    margin: UiRect::left(Val::Px(10.0)),
                    ..default()
                },
            ));
        }
    });
}

/// Spawns audio settings content
fn spawn_audio_content(parent: &mut ChildSpawnerCommands, settings: &AudioSettings) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(15.0),
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).with_children(|content| {
        create_slider_row(content, "Master Volume", settings.master_volume, 0.0, 1.0, SettingType::MasterVolume, true);
        create_slider_row(content, "Music Volume", settings.music_volume, 0.0, 1.0, SettingType::MusicVolume, true);
        create_slider_row(content, "SFX Volume", settings.sfx_volume, 0.0, 1.0, SettingType::SFXVolume, true);
    });
}

/// Spawns interface settings content
fn spawn_interface_content(parent: &mut ChildSpawnerCommands, settings: &InterfaceSettings) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(15.0),
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).with_children(|content| {
        create_slider_row(content, "UI Scale", settings.ui_scale, 0.75, 1.5, SettingType::UIScale, true);
        create_toggle_row(content, "Show FPS", settings.show_fps, SettingType::ShowFPS);
        create_toggle_row(content, "Show Tooltips", settings.show_tooltips, SettingType::ShowTooltips);
    });
}

/// Spawns performance settings content
fn spawn_performance_content(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Text::new("Performance settings coming soon"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.6, 0.6, 0.6)),
    ));
}

/// Spawns controls settings content
fn spawn_controls_content(parent: &mut ChildSpawnerCommands, settings: &ControlSettings) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(15.0),
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).with_children(|content| {
        create_slider_row(content, "Camera Speed", settings.camera_speed, 0.5, 2.0, SettingType::CameraSpeed, false);
        create_slider_row(content, "Zoom Speed", settings.zoom_speed, 0.5, 2.0, SettingType::ZoomSpeed, false);
        create_toggle_row(content, "Invert Zoom", settings.invert_zoom, SettingType::InvertZoom);
    });
}

// ============================================================================
// WIDGET CREATION HELPERS
// ============================================================================

/// Creates a row with a cycle button for switching between options
fn create_cycle_row(parent: &mut ChildSpawnerCommands, label: &str, current: &str, setting_type: SettingType) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(15.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).with_children(|row| {
        // Label
        row.spawn((
            Text::new(label),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
        ));
        
        // Cycle button
        ButtonBuilder::new(format!("< {} >", current))
            .style(ButtonStyle::Secondary)
            .size(ButtonSize::Medium)
            .with_marker(CycleButton { setting_type })
            .build(row);
    });
}

/// Creates a row with a toggle checkbox
fn create_toggle_row(parent: &mut ChildSpawnerCommands, label: &str, enabled: bool, setting_type: SettingType) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(15.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).with_children(|row| {
        // Label
        row.spawn((
            Text::new(label),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
        ));
        
        // Toggle checkbox
        let checkbox_text = if enabled { "✓" } else { "" };
        let style = if enabled { ButtonStyle::Success } else { ButtonStyle::Secondary };
        
        let mut entity_commands = row.spawn((
            Button,
            Node {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(if enabled {
                Color::srgb(0.2, 0.4, 0.2)
            } else {
                Color::srgb(0.15, 0.15, 0.18)
            }),
            BorderColor(Color::srgb(0.3, 0.3, 0.35)),
            ToggleButton { setting_type, enabled },
        ));
        
        if enabled {
            entity_commands.with_children(|btn| {
                btn.spawn((
                    Text::new("✓"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            });
        }
    });
}

/// Creates a row with a slider for numeric values
fn create_slider_row(parent: &mut ChildSpawnerCommands, label: &str, value: f32, min: f32, max: f32, setting_type: SettingType, as_percentage: bool) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(15.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).with_children(|row| {
        // Label
        row.spawn((
            Text::new(label),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
        ));
        
        // Slider container
        row.spawn((
            Node {
                width: Val::Px(250.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        )).with_children(|container| {
            // Slider track
            container.spawn((
                Button,  // Required for Interaction to work
                Node {
                    width: Val::Px(180.0),
                    height: Val::Px(6.0),
                    position_type: PositionType::Relative,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
                Interaction::default(),
                RelativeCursorPosition::default(),  // Track cursor position relative to this element
                Slider { setting_type, value, min, max },
            )).with_children(|track| {
                // Slider handle
                let normalized = (value - min) / (max - min);
                track.spawn((
                    Node {
                        width: Val::Px(16.0),
                        height: Val::Px(16.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(normalized * 164.0), // 180 - 16 for handle width
                        top: Val::Px(-5.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.5, 0.6)),
                    BorderColor(Color::srgb(0.7, 0.7, 0.8)),
                    SliderHandle,
                ));
            });
            
            // Value display
            let display_text = if as_percentage {
                format!("{:.0}%", value * 100.0)
            } else {
                format!("{:.1}s", value)
            };
            
            container.spawn((
                Text::new(display_text),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                SliderValueText { setting_type },
            ));
        });
    });
}