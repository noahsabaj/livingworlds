//! Map mode display and switcher component

use super::super::{ChildBuilder, ButtonBuilder, ButtonStyle, LabelBuilder, LabelStyle};
use crate::resources::MapMode;
use bevy::prelude::*;

/// Marker component for the map mode display
#[derive(Component, Reflect)]
pub struct MapModeDisplay;

/// Marker component for the map mode button
#[derive(Component)]
pub struct MapModeButton;

/// Marker component for the dropdown container
#[derive(Component)]
pub struct MapModeDropdown;

/// Marker component for individual dropdown items
#[derive(Component)]
pub struct MapModeDropdownItem {
    pub mode: MapMode,
}

/// Resource to track dropdown state
#[derive(Resource, Default)]
pub struct MapModeDropdownState {
    pub is_open: bool,
    pub clicked_this_frame: bool,
}

/// Get all available map modes in display order
fn get_all_map_modes() -> Vec<MapMode> {
    vec![
        MapMode::Political,
        MapMode::Terrain,
        MapMode::Climate,
        MapMode::Population,
        MapMode::Agriculture,
        MapMode::Infrastructure,
        MapMode::Minerals,
    ]
}

/// Spawn the map mode display UI element
pub fn spawn_map_mode_display(parent: &mut ChildBuilder) {
    // Main container for the entire map mode display
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End,
            row_gap: Val::Px(4.0),
            ..default()
        },
        MapModeDisplay,
    )).with_children(|main_container| {
        // Label showing "Map Mode:"
        LabelBuilder::new("Map Mode:")
            .style(LabelStyle::Caption)
            .font_size(14.0)
            .color(Color::srgba(0.8, 0.8, 0.8, 1.0))
            .build(main_container);

        // Container for button and dropdown (allows overflow for dropdown)
        main_container.spawn((
            Node {
                position_type: PositionType::Relative,
                overflow: Overflow::visible(),  // Allow dropdown to overflow container
                ..default()
            },
        )).with_children(|button_container| {
            // Main button showing current mode
            let button_entity = ButtonBuilder::new("Political Map")
                .style(ButtonStyle::Secondary)
                .width(Val::Px(160.0))
                .build(button_container);

            // Add marker to identify this button
            button_container.commands().entity(button_entity).insert(MapModeButton);

            // Dropdown container (initially hidden)
            spawn_dropdown_menu(button_container);
        });
    });
}

/// Spawn the dropdown menu (initially hidden)
fn spawn_dropdown_menu(parent: &mut ChildBuilder) {
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(100.0), // Position at bottom of parent container
            margin: UiRect::top(Val::Px(4.0)), // Small gap from button
            right: Val::Px(0.0),
            width: Val::Px(160.0),
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(1.0)),
            padding: UiRect::all(Val::Px(2.0)),
            display: Display::None, // Initially hidden
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
        BorderColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
        ZIndex(1000),
        MapModeDropdown,
    )).with_children(|dropdown| {
        // Create dropdown items for each map mode
        for mode in get_all_map_modes() {
            let item_entity = ButtonBuilder::new(mode.display_name())
                .style(ButtonStyle::Ghost)
                .width(Val::Px(154.0))
                .build(dropdown);

            dropdown.commands().entity(item_entity).insert(MapModeDropdownItem { mode });
        }
    });
}

/// Update the map mode display text
pub fn update_map_mode_display(
    current_map_mode: Res<MapMode>,
    button_query: Query<&Children, With<MapModeButton>>,
    mut text_query: Query<&mut Text>,
) {
    if current_map_mode.is_changed() {
        // Find the map mode button
        if let Ok(children) = button_query.single() {
            // Find the Text component in the button's children
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    **text = current_map_mode.display_name().to_string();
                    break; // Found and updated the text, we're done
                }
            }
        }
    }
}

/// Handle map mode button clicks (now toggles dropdown)
pub fn handle_map_mode_button(
    mut interactions: Query<&Interaction, (Changed<Interaction>, With<MapModeButton>)>,
    mut dropdown_state: ResMut<MapModeDropdownState>,
    mut dropdown_query: Query<&mut Node, With<MapModeDropdown>>,
) {
    for interaction in &mut interactions {
        if *interaction == Interaction::Pressed {
            dropdown_state.is_open = !dropdown_state.is_open;
            dropdown_state.clicked_this_frame = true; // Mark that we handled a click

            // Toggle dropdown visibility
            if let Ok(mut dropdown_node) = dropdown_query.single_mut() {
                dropdown_node.display = if dropdown_state.is_open {
                    Display::Flex
                } else {
                    Display::None
                };
            }

            debug!("Dropdown toggled: {}", if dropdown_state.is_open { "opened" } else { "closed" });
        }
    }
}

/// Handle dropdown item clicks
pub fn handle_dropdown_item_clicks(
    mut interactions: Query<(&Interaction, &MapModeDropdownItem), (Changed<Interaction>, With<MapModeDropdownItem>)>,
    mut current_map_mode: ResMut<MapMode>,
    mut dropdown_state: ResMut<MapModeDropdownState>,
    mut dropdown_query: Query<&mut Node, With<MapModeDropdown>>,
) {
    for (interaction, item) in &mut interactions {
        if *interaction == Interaction::Pressed {
            // Change map mode
            *current_map_mode = item.mode;
            dropdown_state.clicked_this_frame = true; // Mark that we handled a click

            // Close dropdown
            dropdown_state.is_open = false;
            if let Ok(mut dropdown_node) = dropdown_query.single_mut() {
                dropdown_node.display = Display::None;
            }

            debug!("Map mode selected from dropdown: {:?}", item.mode);
        }
    }
}

/// Handle closing dropdown when clicking outside or pressing Escape
pub fn handle_dropdown_close(
    mut shortcut_events: EventReader<crate::ui::shortcuts::ShortcutEvent>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut dropdown_state: ResMut<MapModeDropdownState>,
    mut dropdown_query: Query<&mut Node, With<MapModeDropdown>>,
) {
    use crate::ui::shortcuts::ShortcutId;

    // Close on Escape key
    for event in shortcut_events.read() {
        if dropdown_state.is_open && (event.shortcut_id == ShortcutId::Escape || event.shortcut_id == ShortcutId::OpenMainMenu) {
            dropdown_state.is_open = false;
            if let Ok(mut dropdown_node) = dropdown_query.single_mut() {
                dropdown_node.display = Display::None;
            }
            return;
        }
    }

    // Close on mouse click outside dropdown
    // If dropdown is open and we detect a mouse click but clicked_this_frame is false,
    // that means the click was outside the dropdown elements
    if dropdown_state.is_open && mouse.just_pressed(MouseButton::Left) {
        if !dropdown_state.clicked_this_frame {
            // Click was outside - close dropdown
            dropdown_state.is_open = false;
            if let Ok(mut dropdown_node) = dropdown_query.single_mut() {
                dropdown_node.display = Display::None;
            }
        }
    }

    // Reset the click flag for next frame
    dropdown_state.clicked_this_frame = false;
}

/// Handle keyboard shortcut for quick Political ↔ Terrain switching (Tab key)
pub fn handle_map_mode_shortcut(
    mut shortcut_events: EventReader<crate::ui::shortcuts::ShortcutEvent>,
    mut current_map_mode: ResMut<MapMode>,
    mut dropdown_state: ResMut<MapModeDropdownState>,
) {
    use crate::ui::shortcuts::ShortcutId;

    for event in shortcut_events.read() {
        if event.shortcut_id == ShortcutId::MapModeToggle {
            // Quick toggle between Political and Terrain (most common switch)
            *current_map_mode = match *current_map_mode {
                MapMode::Political => MapMode::Terrain,
                _ => MapMode::Political,
            };

            // Close dropdown when using keyboard shortcut to avoid UI desync
            dropdown_state.is_open = false;
            dropdown_state.clicked_this_frame = false;

            debug!("Map mode quick-switched to: {:?}", *current_map_mode);
        }
    }
}