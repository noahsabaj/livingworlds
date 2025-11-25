//! Nation information display panel
//!
//! Shows detailed information about the currently selected nation including
//! ruler, House, statistics, and controlled provinces.

use crate::nations::{House, Nation, get_structure_name, NationId};
use crate::states::GameState;
use crate::ui::*;
use crate::ui::styles::colors;
use bevy::prelude::*;

/// Resource tracking the currently selected nation
#[derive(Resource, Default)]
pub struct SelectedNation {
    pub entity: Option<Entity>,
    pub nation_id: Option<NationId>,
}

/// Message fired when nation selection changes
#[derive(Message, Debug, Clone)]
pub struct NationSelectionChanged {
    pub previous: Option<Entity>,
    pub current: Option<Entity>,
}

/// Marker for the nation info panel root
#[derive(Component)]
pub struct NationInfoPanel;

/// Marker for nation name text
#[derive(Component)]
pub struct NationNameText;

/// Marker for ruler text
#[derive(Component)]
pub struct RulerText;

/// Marker for House text
#[derive(Component)]
pub struct HouseText;

/// Marker for province count text
#[derive(Component)]
pub struct ProvinceCountText;

/// Marker for treasury text
#[derive(Component)]
pub struct TreasuryText;

/// Marker for stability text
#[derive(Component)]
pub struct StabilityText;

/// Marker for military strength text
#[derive(Component)]
pub struct MilitaryText;

/// Marker for government type text
#[derive(Component)]
pub struct GovernmentText;

/// Marker for legitimacy text
#[derive(Component)]
pub struct LegitimacyText;

/// Marker for view laws button
#[derive(Component)]
pub struct ViewLawsButton;

/// Marker for view family tree button
#[derive(Component)]
pub struct ViewFamilyTreeButton;

/// Spawn the nation info panel UI
pub fn spawn_nation_info_panel(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                right: Val::Px(20.0),
                width: Val::Px(320.0),
                max_height: Val::Vh(70.0),
                padding: UiRect::all(Val::Px(20.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(UI_BACKGROUND_COLOR),
            BorderColor::all(UI_BORDER_COLOR),
            NationInfoPanel,
            Visibility::Hidden, // Start hidden, show when nation selected
        ))
        .with_children(|parent| {
            // Title section
            parent.spawn((
                Text::new("NATION OVERVIEW"),
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

            // Nation name
            parent.spawn((
                Text::new("No Nation Selected"),
                TextFont {
                    font_size: TEXT_SIZE_LARGE,
                    ..default()
                },
                TextColor(TEXT_COLOR_PRIMARY),
                NationNameText,
            ));

            // House and ruler
            parent.spawn((
                Text::new("House: Unknown"),
                TextFont {
                    font_size: TEXT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(TEXT_COLOR_SECONDARY),
                HouseText,
            ));

            parent.spawn((
                Text::new("Ruler: Unknown"),
                TextFont {
                    font_size: TEXT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(TEXT_COLOR_SECONDARY),
                RulerText,
            ));

            // Government type
            parent.spawn((
                Text::new("Government: Unknown"),
                TextFont {
                    font_size: TEXT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(TEXT_COLOR_SECONDARY),
                GovernmentText,
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

            // Statistics
            parent.spawn((
                Text::new("Statistics"),
                TextFont {
                    font_size: TEXT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(TEXT_COLOR_HEADER),
            ));

            // Province count
            parent.spawn((
                Text::new("Provinces: 0"),
                TextFont {
                    font_size: TEXT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(TEXT_COLOR_PRIMARY),
                ProvinceCountText,
            ));

            // Treasury
            parent.spawn((
                Text::new("Treasury: 0 gold"),
                TextFont {
                    font_size: TEXT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(TEXT_COLOR_PRIMARY),
                TreasuryText,
            ));

            // Stability
            parent.spawn((
                Text::new("Stability: 0%"),
                TextFont {
                    font_size: TEXT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(TEXT_COLOR_PRIMARY),
                StabilityText,
            ));

            // Military strength
            parent.spawn((
                Text::new("Military: 0 strength"),
                TextFont {
                    font_size: TEXT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(TEXT_COLOR_PRIMARY),
                MilitaryText,
            ));

            // Legitimacy
            parent.spawn((
                Text::new("Legitimacy: 0%"),
                TextFont {
                    font_size: TEXT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(TEXT_COLOR_PRIMARY),
                LegitimacyText,
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

            // View laws button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(colors::SURFACE),
                    BorderColor::all(colors::BORDER),
                    ViewLawsButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("View Nation Laws"),
                        TextFont {
                            font_size: TEXT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(TEXT_COLOR_PRIMARY),
                    ));
                });

            // View family tree button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(colors::SURFACE),
                    BorderColor::all(colors::BORDER),
                    ViewFamilyTreeButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("View Family Tree"),
                        TextFont {
                            font_size: TEXT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(TEXT_COLOR_PRIMARY),
                    ));
                });
        });
}

/// Watch for changes to SelectedNation and fire messages
pub fn watch_nation_selection(
    selected_nation: Res<SelectedNation>,
    mut messages: MessageWriter<NationSelectionChanged>,
    mut last_selection: Local<Option<Entity>>,
) {
    if selected_nation.is_changed() {
        let current = selected_nation.entity;
        if *last_selection != current {
            messages.write(NationSelectionChanged {
                previous: *last_selection,
                current,
            });
            *last_selection = current;
        }
    }
}

/// Update panel visibility based on selection
pub fn update_panel_visibility(
    mut messages: MessageReader<NationSelectionChanged>,
    mut panel_visibility: Query<&mut Visibility, With<NationInfoPanel>>,
) {
    for message in messages.read() {
        if let Ok(mut visibility) = panel_visibility.single_mut() {
            *visibility = if message.current.is_some() {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

/// Update nation name display
pub fn update_nation_basic_info(
    mut messages: MessageReader<NationSelectionChanged>,
    nations_query: Query<&Nation>,
    mut name_text: Query<&mut Text, With<NationNameText>>,
) {
    for message in messages.read() {
        let Ok(mut text) = name_text.single_mut() else {
            continue;
        };

        if let Some(entity) = message.current {
            if let Ok(nation) = nations_query.get(entity) {
                text.0 = nation.name.clone();
            } else {
                text.0 = "Invalid Nation Data".to_string();
                warn!("Selected nation entity {:?} missing Nation component", entity);
            }
        } else {
            text.0 = "No Nation Selected".to_string();
        }
    }
}

/// Update House and ruler information display
pub fn update_house_ruler_info(
    mut messages: MessageReader<NationSelectionChanged>,
    nations_query: Query<Option<&crate::relationships::RuledBy>>,
    houses_query: Query<&House>,
    mut house_text: Query<&mut Text, (With<HouseText>, Without<RulerText>)>,
    mut ruler_text: Query<&mut Text, With<RulerText>>,
) {
    for message in messages.read() {
        let Ok(mut house_text) = house_text.single_mut() else {
            continue;
        };
        let Ok(mut ruler_text) = ruler_text.single_mut() else {
            continue;
        };

        if let Some(entity) = message.current {
            if let Ok(ruled_by_opt) = nations_query.get(entity) {
                if let Some(ruled_by) = ruled_by_opt {
                    if let Some(ruler_entity) = ruled_by.current_ruler() {
                        if let Ok(house) = houses_query.get(ruler_entity) {
                            house_text.0 = format!("House: {}", house.name);
                            ruler_text.0 = format!("Ruler: {} {}", house.ruler.title, house.ruler.name);
                        } else {
                            house_text.0 = "House: Invalid Data".to_string();
                            ruler_text.0 = "Ruler: Invalid Data".to_string();
                        }
                    } else {
                        house_text.0 = "House: No Ruler".to_string();
                        ruler_text.0 = "Ruler: None".to_string();
                    }
                } else {
                    house_text.0 = "House: Unruled".to_string();
                    ruler_text.0 = "Ruler: None".to_string();
                }
            } else {
                house_text.0 = "House: Invalid Data".to_string();
                ruler_text.0 = "Ruler: Invalid Data".to_string();
            }
        } else {
            house_text.0 = "House: Unknown".to_string();
            ruler_text.0 = "Ruler: Unknown".to_string();
        }
    }
}

/// Update nation statistics display (provinces, treasury, stability, military)
pub fn update_nation_statistics(
    mut messages: MessageReader<NationSelectionChanged>,
    nations_query: Query<&Nation>,
    controls_query: Query<&crate::relationships::Controls>,
    mut province_text: Query<
        &mut Text,
        (
            With<ProvinceCountText>,
            Without<TreasuryText>,
            Without<StabilityText>,
            Without<MilitaryText>,
        ),
    >,
    mut treasury_text: Query<
        &mut Text,
        (
            With<TreasuryText>,
            Without<ProvinceCountText>,
            Without<StabilityText>,
            Without<MilitaryText>,
        ),
    >,
    mut stability_text: Query<
        &mut Text,
        (
            With<StabilityText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<MilitaryText>,
        ),
    >,
    mut military_text: Query<
        &mut Text,
        (
            With<MilitaryText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<StabilityText>,
        ),
    >,
) {
    for message in messages.read() {
        if let Some(entity) = message.current {
            if let Ok(nation) = nations_query.get(entity) {
                // Update all statistics
                let province_count = crate::nations::get_nation_province_count(&controls_query, entity);

                if let Ok(mut text) = province_text.single_mut() {
                    text.0 = format!("Provinces: {}", province_count);
                }
                if let Ok(mut text) = treasury_text.single_mut() {
                    text.0 = format!("Treasury: {:.0} gold", nation.treasury);
                }
                if let Ok(mut text) = stability_text.single_mut() {
                    text.0 = format!("Stability: {:.0}%", nation.stability * 100.0);
                }
                if let Ok(mut text) = military_text.single_mut() {
                    text.0 = format!("Military: {:.0} strength", nation.military_strength);
                }
            } else {
                // Invalid nation data
                if let Ok(mut text) = province_text.single_mut() {
                    text.0 = "Provinces: Invalid Data".to_string();
                }
                if let Ok(mut text) = treasury_text.single_mut() {
                    text.0 = "Treasury: Invalid Data".to_string();
                }
                if let Ok(mut text) = stability_text.single_mut() {
                    text.0 = "Stability: Invalid Data".to_string();
                }
                if let Ok(mut text) = military_text.single_mut() {
                    text.0 = "Military: Invalid Data".to_string();
                }
            }
        } else {
            // No nation selected
            if let Ok(mut text) = province_text.single_mut() {
                text.0 = "Provinces: 0".to_string();
            }
            if let Ok(mut text) = treasury_text.single_mut() {
                text.0 = "Treasury: 0 gold".to_string();
            }
            if let Ok(mut text) = stability_text.single_mut() {
                text.0 = "Stability: 0%".to_string();
            }
            if let Ok(mut text) = military_text.single_mut() {
                text.0 = "Military: 0 strength".to_string();
            }
        }
    }
}

/// Update government type display
pub fn update_government_display(
    mut messages: MessageReader<NationSelectionChanged>,
    governance_query: Query<&crate::nations::Governance>,
    mut government_text: Query<&mut Text, With<GovernmentText>>,
) {
    for message in messages.read() {
        let Ok(mut text) = government_text.single_mut() else {
            continue;
        };

        if let Some(entity) = message.current {
            if let Ok(governance) = governance_query.get(entity) {
                text.0 = format!("Government: {}", get_structure_name(&governance.government_type));
            } else {
                text.0 = "Government: Unknown".to_string();
            }
        } else {
            text.0 = "Government: Unknown".to_string();
        }
    }
}

/// Calculate and cache legitimacy in Governance component
/// This runs less frequently and updates the cached value
pub fn update_cached_legitimacy(
    mut governance_query: Query<&mut crate::nations::Governance, Changed<crate::nations::Governance>>,
) {
    for mut governance in governance_query.iter_mut() {
        // Recalculate legitimacy when governance changes
        let calculated = governance
            .legitimacy_factors
            .calculate_legitimacy(governance.government_type);

        // Update cached value
        governance.legitimacy = calculated;

        // Could also update trend here if needed
        // governance.legitimacy_trend = ...
    }
}

/// Format and display cached legitimacy value
pub fn update_legitimacy_display(
    mut messages: MessageReader<NationSelectionChanged>,
    governance_query: Query<&crate::nations::Governance>,
    mut legitimacy_text: Query<&mut Text, With<LegitimacyText>>,
) {
    for message in messages.read() {
        let Ok(mut text) = legitimacy_text.single_mut() else {
            continue;
        };

        if let Some(entity) = message.current {
            if let Ok(governance) = governance_query.get(entity) {
                // Use cached legitimacy value
                let legitimacy = governance.legitimacy;

                // Build rich display
                let mut display = format!("Legitimacy: {:.0}%", legitimacy * 100.0);

                // Add indicator (NO EMOJIS - using text symbols)
                let indicator = match legitimacy {
                    x if x >= 0.8 => " [High]",
                    x if x >= 0.6 => " [Mod]",
                    x if x >= 0.4 => " [Low]",
                    _ => " [Critical]",
                };
                display.push_str(indicator);

                // Show primary legitimacy source
                use crate::nations::GovernmentCategory;
                let source = match governance.government_type.category() {
                    GovernmentCategory::Democratic => {
                        if let Some(electoral) = &governance.legitimacy_factors.electoral_mandate {
                            format!(" ({}% vote)", (electoral.vote_percentage * 100.0) as u32)
                        } else {
                            String::new()
                        }
                    }
                    GovernmentCategory::Theocratic | GovernmentCategory::Monarchic => {
                        if let Some(divine) = &governance.legitimacy_factors.divine_approval {
                            format!(" ({}% clergy)", (divine.clergy_support * 100.0) as u32)
                        } else {
                            String::new()
                        }
                    }
                    GovernmentCategory::Socialist | GovernmentCategory::Anarchist => {
                        if let Some(revolutionary) = &governance.legitimacy_factors.revolutionary_fervor {
                            format!(" ({}% fervor)", (revolutionary.revolutionary_zeal * 100.0) as u32)
                        } else {
                            String::new()
                        }
                    }
                    _ => {
                        format!(" ({}% efficiency)",
                            (governance.legitimacy_factors.administrative_efficiency * 100.0) as u32)
                    }
                };

                display.push_str(&source);
                text.0 = display;
            } else {
                text.0 = "Legitimacy: Unknown".to_string();
            }
        } else {
            text.0 = "Legitimacy: 0%".to_string();
        }
    }
}

use bevy_plugin_builder::define_plugin;

/// Handle View Family Tree button click
pub fn handle_view_family_tree_button(
    buttons: Query<&Interaction, (Changed<Interaction>, With<ViewFamilyTreeButton>)>,
    selected_nation: Res<SelectedNation>,
    nations_query: Query<(&Nation, &crate::nations::NationId, Option<&crate::relationships::RuledBy>)>,
    houses_query: Query<(Entity, &House)>,
    mut open_events: MessageWriter<crate::ui::family_browser::OpenFamilyTreeEvent>,
) {
    for interaction in &buttons {
        if *interaction == Interaction::Pressed {
            // Get the selected nation's house
            if let Some(nation_entity) = selected_nation.entity {
                if let Ok((_, _, ruled_by)) = nations_query.get(nation_entity) {
                    // Find the house entity for this nation
                    if let Some(ruled_by) = ruled_by {
                        if let Some(house_entity) = ruled_by.current_ruler() {
                            open_events.write(crate::ui::family_browser::OpenFamilyTreeEvent {
                                house_entity,
                            });
                        }
                    }
                }
            }
        }
    }
}

/// Plugin for nation information UI
define_plugin!(NationInfoPlugin {
    resources: [SelectedNation],
    messages: [NationSelectionChanged],

    update: [
        // Handle user input and update SelectedNation resource
        super::nation_selection::handle_nation_selection.run_if(in_state(GameState::InGame)),

        // Watch for changes and emit messages
        watch_nation_selection.run_if(in_state(GameState::InGame)),

        // React to selection changes
        update_panel_visibility.run_if(in_state(GameState::InGame)),
        update_nation_basic_info.run_if(in_state(GameState::InGame)),
        update_house_ruler_info.run_if(in_state(GameState::InGame)),
        update_nation_statistics.run_if(in_state(GameState::InGame)),
        update_government_display.run_if(in_state(GameState::InGame)),
        update_legitimacy_display.run_if(in_state(GameState::InGame)),

        // Update cached legitimacy when governance changes (independent of selection)
        update_cached_legitimacy.run_if(in_state(GameState::InGame)),

        // Other UI interactions
        handle_view_family_tree_button.run_if(in_state(GameState::InGame)),
        super::nation_selection::highlight_selected_nation_territory.run_if(in_state(GameState::InGame))
    ],

    on_enter: {
        GameState::InGame => [spawn_nation_info_panel]
    }
});
