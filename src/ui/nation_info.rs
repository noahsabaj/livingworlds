//! Nation information display panel
//!
//! Shows detailed information about the currently selected nation including
//! ruler, dynasty, statistics, and controlled provinces.

use crate::nations::{House, Nation, NationId};
use crate::states::GameState;
use crate::ui::*;
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

/// Marker for dynasty text
#[derive(Component)]
pub struct DynastyText;

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

            // Dynasty and ruler
            parent.spawn((
                Text::new("Dynasty: Unknown"),
                TextFont {
                    font_size: TEXT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(TEXT_COLOR_SECONDARY),
                DynastyText,
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
        });
}

/// Update nation info panel with selected nation data
pub fn update_nation_info_panel(
    selected_nation: Res<SelectedNation>,
    nations_query: Query<&Nation>,
    houses_query: Query<&House>,
    ownership_cache: Res<crate::nations::ProvinceOwnershipCache>,
    mut panel_visibility: Query<&mut Visibility, With<NationInfoPanel>>,
    mut name_text: Query<
        &mut Text,
        (
            With<NationNameText>,
            Without<DynastyText>,
            Without<RulerText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<StabilityText>,
            Without<MilitaryText>,
        ),
    >,
    mut dynasty_text: Query<
        &mut Text,
        (
            With<DynastyText>,
            Without<NationNameText>,
            Without<RulerText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<StabilityText>,
            Without<MilitaryText>,
        ),
    >,
    mut ruler_text: Query<
        &mut Text,
        (
            With<RulerText>,
            Without<NationNameText>,
            Without<DynastyText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<StabilityText>,
            Without<MilitaryText>,
        ),
    >,
    mut province_text: Query<
        &mut Text,
        (
            With<ProvinceCountText>,
            Without<NationNameText>,
            Without<DynastyText>,
            Without<RulerText>,
            Without<TreasuryText>,
            Without<StabilityText>,
            Without<MilitaryText>,
        ),
    >,
    mut treasury_text: Query<
        &mut Text,
        (
            With<TreasuryText>,
            Without<NationNameText>,
            Without<DynastyText>,
            Without<RulerText>,
            Without<ProvinceCountText>,
            Without<StabilityText>,
            Without<MilitaryText>,
        ),
    >,
    mut stability_text: Query<
        &mut Text,
        (
            With<StabilityText>,
            Without<NationNameText>,
            Without<DynastyText>,
            Without<RulerText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<MilitaryText>,
        ),
    >,
    mut military_text: Query<
        &mut Text,
        (
            With<MilitaryText>,
            Without<NationNameText>,
            Without<DynastyText>,
            Without<RulerText>,
            Without<ProvinceCountText>,
            Without<TreasuryText>,
            Without<StabilityText>,
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

    // Find and update dynasty/ruler info
    for house in houses_query.iter() {
        if house.nation_id == nation.id {
            if let Ok(mut text) = dynasty_text.single_mut() {
                text.0 = format!("Dynasty: {}", house.name);
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
