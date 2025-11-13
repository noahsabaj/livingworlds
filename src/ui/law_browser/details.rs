//! Law details panel UI

use bevy::prelude::*;
use crate::nations::{LawRegistry, LawEffects, LawPrerequisite};
use crate::ui::styles::{colors, dimensions};
use super::types::*;

/// Spawn the law details panel
pub fn spawn_law_details_panel(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(40.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(dimensions::SPACING_LARGE)),
            row_gap: Val::Px(dimensions::SPACING_MEDIUM),
            border: UiRect::left(Val::Px(dimensions::BORDER_WIDTH)),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        BackgroundColor(colors::BACKGROUND_MEDIUM),
        BorderColor::all(colors::BORDER),
        LawDetailsPanel,
    ))
    .with_children(|panel| {
        // Law name
        panel.spawn((
            Text::new("Select a law to view details"),
            TextFont {
                font_size: dimensions::FONT_SIZE_LARGE,
                ..default()
            },
            TextColor(colors::TEXT_PRIMARY),
            LawNameText,
        ));

        // Law description
        panel.spawn((
            Text::new(""),
            TextFont {
                font_size: dimensions::FONT_SIZE_NORMAL,
                ..default()
            },
            TextColor(colors::TEXT_SECONDARY),
            LawDescriptionText,
        ));

        // Separator
        panel.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(1.0),
                margin: UiRect::vertical(Val::Px(dimensions::SPACING_SMALL)),
                ..default()
            },
            BackgroundColor(colors::BORDER),
        ));

        // Effects section
        panel.spawn((
            Text::new("EFFECTS"),
            TextFont {
                font_size: dimensions::FONT_SIZE_SMALL,
                ..default()
            },
            TextColor(colors::TEXT_TERTIARY),
        ));

        panel.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::SPACING_TINY),
                padding: UiRect::left(Val::Px(dimensions::SPACING_MEDIUM)),
                ..default()
            },
            LawEffectsContainer,
        ));

        // Prerequisites section
        panel.spawn((
            Text::new("PREREQUISITES"),
            TextFont {
                font_size: dimensions::FONT_SIZE_SMALL,
                ..default()
            },
            TextColor(colors::TEXT_TERTIARY),
        ));

        panel.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::SPACING_TINY),
                padding: UiRect::left(Val::Px(dimensions::SPACING_MEDIUM)),
                ..default()
            },
            LawPrerequisitesContainer,
        ));

        // Conflicts section
        panel.spawn((
            Text::new("CONFLICTS WITH"),
            TextFont {
                font_size: dimensions::FONT_SIZE_SMALL,
                ..default()
            },
            TextColor(colors::TEXT_TERTIARY),
        ));

        panel.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::SPACING_TINY),
                padding: UiRect::left(Val::Px(dimensions::SPACING_MEDIUM)),
                ..default()
            },
            LawConflictsContainer,
        ));
    });
}

/// Update law details when a law is selected
pub fn update_law_details(
    selected_law: Res<SelectedLawId>,
    registry: Res<LawRegistry>,
    mut name_query: Query<&mut Text, (With<LawNameText>, Without<LawDescriptionText>)>,
    mut desc_query: Query<&mut Text, (With<LawDescriptionText>, Without<LawNameText>)>,
    mut commands: Commands,
    effects_container: Query<Entity, With<LawEffectsContainer>>,
    prereq_container: Query<Entity, With<LawPrerequisitesContainer>>,
    conflicts_container: Query<Entity, With<LawConflictsContainer>>,
) {
    if !selected_law.is_changed() {
        return;
    }

    if let Some(law_id) = selected_law.0 {
        if let Some(law) = registry.get_law(law_id) {
            // Update name
            for mut text in &mut name_query {
                text.0 = law.name.clone();
            }

            // Update description
            for mut text in &mut desc_query {
                text.0 = law.description.clone();
            }

            // Clear and rebuild effects container
            if let Ok(container) = effects_container.single() {
                commands.entity(container).despawn();
                commands.entity(container).with_children(|parent| {
                    spawn_effect_items(parent, &law.effects);
                });
            }

            // Clear and rebuild prerequisites container
            if let Ok(container) = prereq_container.single() {
                commands.entity(container).despawn();
                commands.entity(container).with_children(|parent| {
                    for prereq in &law.prerequisites {
                        spawn_prerequisite_item(parent, prereq);
                    }
                });
            }

            // Clear and rebuild conflicts container
            if let Ok(container) = conflicts_container.single() {
                commands.entity(container).despawn();
                commands.entity(container).with_children(|parent| {
                    for &conflict_id in &law.conflicts_with {
                        if let Some(conflict_law) = registry.get_law(conflict_id) {
                            parent.spawn((
                                Text::new(format!("• {}", conflict_law.name)),
                                TextFont {
                                    font_size: dimensions::FONT_SIZE_SMALL,
                                    ..default()
                                },
                                TextColor(colors::TEXT_SECONDARY),
                            ));
                        }
                    }
                });
            }
        }
    } else {
        // Clear details when no law selected
        for mut text in &mut name_query {
            text.0 = "Select a law to view details".to_string();
        }
        for mut text in &mut desc_query {
            text.0.clear();
        }
    }
}

/// Spawn effect display items
fn spawn_effect_items(parent: &mut ChildSpawnerCommands, effects: &LawEffects) {
    macro_rules! spawn_effect {
        ($field:expr_2021, $name:expr_2021, $format:expr_2021) => {
            if $field.abs() > 0.001 {
                let sign = if $field > 0.0 { "+" } else { "" };
                let color = if $field > 0.0 { colors::SUCCESS } else { colors::DANGER };
                parent.spawn((
                    Text::new(format!("• {} {}{:.0}{}", $name, sign, $field * 100.0, $format)),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_SMALL,
                        ..default()
                    },
                    TextColor(color),
                ));
            }
        };
    }

    spawn_effect!(effects.tax_efficiency_modifier, "Tax Efficiency", "%");
    spawn_effect!(effects.industrial_output_modifier, "Industrial Output", "%");
    spawn_effect!(effects.agricultural_output_modifier, "Agricultural Output", "%");
    spawn_effect!(effects.trade_income_modifier, "Trade Income", "%");
    spawn_effect!(effects.stability_change, "Stability", "%");
    spawn_effect!(effects.legitimacy_change, "Legitimacy", "%");
    spawn_effect!(effects.happiness_modifier, "Happiness", "%");
    spawn_effect!(effects.revolt_risk_change, "Revolt Risk", "%");
    spawn_effect!(effects.corruption_change, "Corruption", "%");
    spawn_effect!(effects.technology_rate_modifier, "Technology Rate", "%");
    spawn_effect!(effects.army_morale_modifier, "Military Strength", "%");
    spawn_effect!(effects.maintenance_cost_modifier, "Maintenance Cost", "%");
    spawn_effect!(effects.administrative_efficiency_modifier, "Admin Efficiency", "%");
    spawn_effect!(effects.diplomatic_reputation_change, "Diplomatic Reputation", "%");
    spawn_effect!(effects.population_growth_modifier, "Population Growth", "%");
    spawn_effect!(effects.cultural_conversion_modifier, "Cultural Conversion", "%");
    spawn_effect!(effects.reform_resistance_change, "Reform Resistance", "%");
}

/// Spawn prerequisite display item
fn spawn_prerequisite_item(parent: &mut ChildSpawnerCommands, prereq: &LawPrerequisite) {
    let text = match prereq {
        LawPrerequisite::GovernmentCategory(cat) => format!("• {:?} government", cat),
        LawPrerequisite::RequiresLaw(_) => "• Requires another law".to_string(),
        LawPrerequisite::TechnologyLevel(level) => format!("• Technology level {}", level),
        LawPrerequisite::MinimumStability(stab) => format!("• Minimum {:.0}% stability", stab * 100.0),
        LawPrerequisite::MinimumLegitimacy(leg) => format!("• Minimum {:.0}% legitimacy", leg * 100.0),
        LawPrerequisite::YearReached(year) => format!("• Year {} or later", year),
        LawPrerequisite::MinimumProvinces(count) => format!("• At least {} provinces", count),
        LawPrerequisite::Custom(desc) => format!("• {}", desc),
        LawPrerequisite::RequiresLaw(_) => "• Requires specific law".to_string(),
    };

    parent.spawn((
        Text::new(text),
        TextFont {
            font_size: dimensions::FONT_SIZE_SMALL,
            ..default()
        },
        TextColor(colors::TEXT_SECONDARY),
    ));
}