//! Family browser UI rendering

use bevy::prelude::*;
use crate::ui::*;
use super::types::*;

/// Spawn the family browser panel
pub fn spawn_family_browser_panel(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(100.0),
                width: Val::Px(400.0),
                height: Val::Percent(80.0),
                padding: UiRect::all(Val::Px(20.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(UI_BACKGROUND_COLOR),
            BorderColor::all(UI_BORDER_COLOR),
            FamilyBrowserPanel,
            Visibility::Hidden, // Start hidden
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("NOBLE HOUSES"),
                TextFont {
                    font_size: TEXT_SIZE_TITLE,
                    ..default()
                },
                TextColor(TEXT_COLOR_HEADER),
            ));

            // Separator
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(UI_BORDER_COLOR),
            ));

            // Filters section
            spawn_filters_section(parent);

            // Separator
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(UI_BORDER_COLOR),
            ));

            // Scrollable house list
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    HouseListContainer,
                ))
                .with_children(|_parent| {
                    // Houses will be spawned dynamically
                });
        });
}

/// Spawn filters section
fn spawn_filters_section(parent: &mut ChildBuilder) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|parent| {
            // Filter row 1: Nation and Tier dropdowns
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|parent| {
                    // Nation filter (placeholder for now)
                    parent.spawn((
                        Text::new("All Nations"),
                        TextFont {
                            font_size: TEXT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(TEXT_COLOR_SECONDARY),
                        Node {
                            flex_grow: 1.0,
                            ..default()
                        },
                    ));

                    // Tier filter (placeholder for now)
                    parent.spawn((
                        Text::new("All Tiers"),
                        TextFont {
                            font_size: TEXT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(TEXT_COLOR_SECONDARY),
                        Node {
                            flex_grow: 1.0,
                            ..default()
                        },
                    ));
                });

            // Filter row 2: Extinct checkbox
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    // Checkbox placeholder
                    parent.spawn((
                        Node {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(UI_BACKGROUND_COLOR),
                        BorderColor::all(UI_BORDER_COLOR),
                    ));

                    parent.spawn((
                        Text::new("Show Extinct"),
                        TextFont {
                            font_size: TEXT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(TEXT_COLOR_PRIMARY),
                    ));
                });

            // Search input (placeholder)
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(32.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(UI_BACKGROUND_COLOR),
                BorderColor::all(UI_BORDER_COLOR),
                Text::new("Search..."),
                TextFont {
                    font_size: TEXT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(TEXT_COLOR_SECONDARY),
            ));
        });
}

/// Update the house list based on current filters
pub fn update_house_list(
    mut commands: Commands,
    cache: Res<HousePrestigeCache>,
    filters: Res<FamilyBrowserFilters>,
    list_container: Query<Entity, With<HouseListContainer>>,
    existing_entries: Query<Entity, With<HouseEntry>>,
) {
    // Only update if cache or filters changed
    if !cache.is_changed() && !filters.is_changed() {
        return;
    }

    let Ok(container) = list_container.single() else {
        return;
    };

    // Despawn existing entries
    for entry in &existing_entries {
        commands.entity(entry).despawn();
    }

    // Get filtered houses
    let houses = cache.filtered(&filters);

    // Spawn new entries
    commands.entity(container).with_children(|parent| {
        for (rank, house) in houses.iter().enumerate() {
            spawn_house_entry(parent, house, rank + 1);
        }

        // Empty state
        if houses.is_empty() {
            parent.spawn((
                Text::new("No houses found"),
                TextFont {
                    font_size: TEXT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(TEXT_COLOR_SECONDARY),
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
            ));
        }
    });
}

/// Spawn a single house entry
fn spawn_house_entry(parent: &mut ChildBuilder, house: &HousePrestige, rank: usize) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(12.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(UI_BACKGROUND_COLOR),
            BorderColor::all(UI_BORDER_COLOR),
            HouseEntry {
                house_entity: house.house_entity,
            },
        ))
        .with_children(|parent| {
            // Header row: rank, name, stars
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    // Rank and name
                    parent.spawn((
                        Text::new(format!("{}. {}", rank, house.house_name)),
                        TextFont {
                            font_size: TEXT_SIZE_LARGE,
                            ..default()
                        },
                        TextColor(TEXT_COLOR_PRIMARY),
                    ));

                    // Stars
                    let stars = "*".repeat(house.star_count() as usize);
                    parent.spawn((
                        Text::new(stars),
                        TextFont {
                            font_size: TEXT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.84, 0.0)), // Gold
                    ));
                });

            // Info row: status and years
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|parent| {
                    let status = if house.is_extinct {
                        "Extinct".to_string()
                    } else if house.is_ruling {
                        format!("Ruling {}", house.nation_name.as_deref().unwrap_or("Unknown"))
                    } else {
                        "Not Ruling".to_string()
                    };

                    parent.spawn((
                        Text::new(format!("{} - {} years", status, house.years_in_power)),
                        TextFont {
                            font_size: TEXT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(TEXT_COLOR_SECONDARY),
                    ));
                });

            // View Tree button
            ButtonBuilder::new("View Tree")
                .size(ButtonSize::Small)
                .with_marker(ViewTreeButton {
                    house_entity: house.house_entity,
                })
                .build(parent);
        });
}

/// Handle View Tree button clicks
pub fn handle_view_tree_button(
    buttons: Query<(&Interaction, &ViewTreeButton), Changed<Interaction>>,
    mut open_events: MessageWriter<OpenFamilyTreeEvent>,
) {
    for (interaction, button) in &buttons {
        if *interaction == Interaction::Pressed {
            open_events.write(OpenFamilyTreeEvent {
                house_entity: button.house_entity,
            });
        }
    }
}
