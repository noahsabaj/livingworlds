//! Nation information display panel
//!
//! Shows detailed information about the currently selected nation including
//! ruler, House, statistics, and controlled provinces.

use crate::nations::{House, Nation, NationId};
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
            BorderColor(UI_BORDER_COLOR),
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
                    BorderColor(colors::BORDER),
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
        });
}

/// Update nation info panel with selected nation data
pub fn update_nation_info_panel(
    selected_nation: Res<SelectedNation>,
    nations_query: Query<&Nation>,
    houses_query: Query<&House>,
    governance_query: Query<&crate::nations::Governance>,
    ownership_cache: Res<crate::nations::ProvinceOwnershipCache>,
    mut panel_visibility: Query<&mut Visibility, With<NationInfoPanel>>,
    mut name_text: Query<
        &mut Text,
        (
            With<NationNameText>,
            Without<HouseText>,
            Without<RulerText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<StabilityText>,
            Without<MilitaryText>,
            Without<GovernmentText>,
            Without<LegitimacyText>,
        ),
    >,
    mut House_text: Query<
        &mut Text,
        (
            With<HouseText>,
            Without<NationNameText>,
            Without<RulerText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<StabilityText>,
            Without<MilitaryText>,
            Without<GovernmentText>,
            Without<LegitimacyText>,
        ),
    >,
    mut ruler_text: Query<
        &mut Text,
        (
            With<RulerText>,
            Without<NationNameText>,
            Without<HouseText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<StabilityText>,
            Without<MilitaryText>,
            Without<GovernmentText>,
            Without<LegitimacyText>,
        ),
    >,
    mut government_text: Query<
        &mut Text,
        (
            With<GovernmentText>,
            Without<NationNameText>,
            Without<HouseText>,
            Without<RulerText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<StabilityText>,
            Without<MilitaryText>,
            Without<LegitimacyText>,
        ),
    >,
    mut province_text: Query<
        &mut Text,
        (
            With<ProvinceCountText>,
            Without<NationNameText>,
            Without<HouseText>,
            Without<RulerText>,
            Without<TreasuryText>,
            Without<StabilityText>,
            Without<MilitaryText>,
            Without<GovernmentText>,
            Without<LegitimacyText>,
        ),
    >,
    mut treasury_text: Query<
        &mut Text,
        (
            With<TreasuryText>,
            Without<NationNameText>,
            Without<HouseText>,
            Without<RulerText>,
            Without<ProvinceCountText>,
            Without<StabilityText>,
            Without<MilitaryText>,
            Without<GovernmentText>,
            Without<LegitimacyText>,
        ),
    >,
    mut stability_text: Query<
        &mut Text,
        (
            With<StabilityText>,
            Without<NationNameText>,
            Without<HouseText>,
            Without<RulerText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<MilitaryText>,
            Without<GovernmentText>,
            Without<LegitimacyText>,
        ),
    >,
    mut military_text: Query<
        &mut Text,
        (
            With<MilitaryText>,
            Without<NationNameText>,
            Without<HouseText>,
            Without<RulerText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<StabilityText>,
            Without<GovernmentText>,
            Without<LegitimacyText>,
        ),
    >,
    mut legitimacy_text: Query<
        &mut Text,
        (
            With<LegitimacyText>,
            Without<NationNameText>,
            Without<HouseText>,
            Without<RulerText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<StabilityText>,
            Without<MilitaryText>,
            Without<GovernmentText>,
        ),
    >,
) {
    // Show/hide panel based on selection
    if let Ok(mut visibility) = panel_visibility.single_mut() {
        if selected_nation.entity.is_some() {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
            return;
        }
    }

    // Get selected nation data
    let Some(entity) = selected_nation.entity else {
        return;
    };
    let Ok(nation) = nations_query.get(entity) else {
        return;
    };

    // Update nation name
    if let Ok(mut text) = name_text.single_mut() {
        text.0 = nation.name.clone();
    }

    // Find and update House/ruler info
    for house in houses_query.iter() {
        if house.nation_id == nation.id {
            if let Ok(mut text) = House_text.single_mut() {
                text.0 = format!("House: {}", house.name);
            }
            if let Ok(mut text) = ruler_text.single_mut() {
                text.0 = format!("Ruler: {} {}", house.ruler.title, house.ruler.name);
            }
            break;
        }
    }

    // Update statistics
    let province_count = ownership_cache.count_nation_provinces(nation.id);

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

    // Update government type if governance component exists
    if let Ok(governance) = governance_query.get(entity) {
        if let Ok(mut text) = government_text.single_mut() {
            text.0 = format!("Government: {}", governance.government_type.structure_name());
        }

        // Calculate and display legitimacy with rich breakdown
        let calculated_legitimacy = governance
            .legitimacy_factors
            .calculate_legitimacy(governance.government_type);

        if let Ok(mut text) = legitimacy_text.single_mut() {
            // Build a rich display showing legitimacy sources
            let mut legitimacy_display = format!("Legitimacy: {:.0}%", calculated_legitimacy * 100.0);

            // Add color indicator based on legitimacy level
            let color_indicator = match calculated_legitimacy {
                x if x >= 0.8 => " ✓", // Strong legitimacy
                x if x >= 0.6 => " ⚬", // Moderate legitimacy
                x if x >= 0.4 => " ⚠", // Weak legitimacy
                _ => " ✗",              // Critical legitimacy
            };
            legitimacy_display.push_str(color_indicator);

            // Show primary legitimacy source based on government type
            use crate::nations::GovernmentCategory;
            let primary_source = match governance.government_type.category() {
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
                    // For other types, show prosperity or efficiency
                    format!(" ({}% efficiency)",
                        (governance.legitimacy_factors.administrative_efficiency * 100.0) as u32)
                }
            };

            legitimacy_display.push_str(&primary_source);
            text.0 = legitimacy_display;
        }
    }
}

/// Plugin for nation information UI
pub struct NationInfoPlugin;

impl Plugin for NationInfoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedNation>()
            .add_systems(OnEnter(GameState::InGame), spawn_nation_info_panel)
            .add_systems(
                Update,
                (
                    super::nation_selection::handle_nation_selection,
                    update_nation_info_panel,
                    super::nation_selection::highlight_selected_nation_territory,
                )
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
