//! Updates for nation laws panel
//!
//! Populates the panel with current law data from the selected nation.

use bevy::prelude::*;
use crate::nations::laws::{LawRegistry, NationLaws, LawEffects};
use crate::ui::styles::{colors, dimensions};
use crate::ui::SelectedNation;
use super::types::*;

/// Update the list of active laws
pub fn update_active_laws_list(
    selected_nation: Res<SelectedNation>,
    nation_laws_query: Query<&NationLaws>,
    registry: Res<LawRegistry>,
    mut commands: Commands,
    container_query: Query<Entity, With<ActiveLawsContainer>>,
    combined_effects_text: Query<Entity, With<CombinedEffectsText>>,
) {
    if !selected_nation.is_changed() {
        return;
    }

    let Ok(container) = container_query.get_single() else {
        return;
    };

    // Clear existing laws
    commands.entity(container).despawn_descendants();

    // Get nation's laws if selected
    let Some(nation_entity) = selected_nation.entity else {
        // No nation selected - show empty state
        commands.entity(container).with_children(|parent| {
            parent.spawn((
                Text::new("No nation selected"),
                TextFont {
                    font_size: typography::TEXT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::TEXT_TERTIARY),
            ));
        });
        return;
    };

    let Ok(nation_laws) = nation_laws_query.get(nation_entity) else {
        // Nation has no laws component
        commands.entity(container).with_children(|parent| {
            parent.spawn((
                Text::new("No laws enacted"),
                TextFont {
                    font_size: typography::TEXT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::TEXT_TERTIARY),
            ));
        });
        return;
    };

    // Update combined effects text
    if let Ok(effects_entity) = combined_effects_text.get_single() {
        // CRITICAL FIX: Store parent before despawning to avoid panic
        let parent_entity = effects_entity.parent();
        commands.entity(effects_entity).despawn();

        // Spawn new combined effects text
        let effects = &nation_laws.combined_effects;
        let effects_text = format_combined_effects(effects);

        if let Some(parent) = parent_entity {
            commands.entity(parent).with_children(|parent| {
                parent.spawn((
                    Text::new(effects_text),
                    TextFont {
                        font_size: typography::TEXT_SIZE_SMALL,
                        ..default()
                    },
                    TextColor(colors::TEXT_SECONDARY),
                    CombinedEffectsText,
                ));
            });
        }
    }

    // Spawn active law items
    if nation_laws.active_laws.is_empty() {
        commands.entity(container).with_children(|parent| {
            parent.spawn((
                Text::new("No laws enacted"),
                TextFont {
                    font_size: typography::TEXT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::TEXT_TERTIARY),
            ));
        });
    } else {
        commands.entity(container).with_children(|parent| {
            for &law_id in &nation_laws.active_laws {
                if let Some(law) = registry.get_law(law_id) {
                    spawn_active_law_item(parent, law_id, &law.name, &law.category.name());
                }
            }
        });
    }
}

/// Update the list of proposed laws
pub fn update_proposed_laws_list(
    selected_nation: Res<SelectedNation>,
    nation_laws_query: Query<&NationLaws>,
    registry: Res<LawRegistry>,
    mut commands: Commands,
    container_query: Query<Entity, With<ProposedLawsContainer>>,
) {
    if !selected_nation.is_changed() {
        return;
    }

    let Ok(container) = container_query.get_single() else {
        return;
    };

    // Clear existing proposals
    commands.entity(container).despawn_descendants();

    // Get nation's laws if selected
    let Some(nation_entity) = selected_nation.entity else {
        return;
    };

    let Ok(nation_laws) = nation_laws_query.get(nation_entity) else {
        return;
    };

    // Spawn proposed law items
    if nation_laws.proposed_laws.is_empty() {
        commands.entity(container).with_children(|parent| {
            parent.spawn((
                Text::new("No laws under debate"),
                TextFont {
                    font_size: typography::TEXT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::TEXT_TERTIARY),
            ));
        });
    } else {
        commands.entity(container).with_children(|parent| {
            for (index, proposal) in nation_laws.proposed_laws.iter().enumerate() {
                if let Some(law) = registry.get_law(proposal.law_id) {
                    spawn_proposed_law_item(
                        parent,
                        index,
                        &law.name,
                        proposal.support_percentage,
                        proposal.debate_duration
                    );
                }
            }
        });
    }
}

/// Spawn an active law item
fn spawn_active_law_item(
    parent: &mut ChildSpawnerCommands,
    law_id: crate::nations::laws::LawId,
    name: &str,
    category: &str,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(dimensions::SPACING_SMALL)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                ..default()
            },
            BackgroundColor(colors::SURFACE),
            BorderColor(colors::BORDER),
            ActiveLawItem { law_id },
        ))
        .with_children(|item| {
            // Law info
            item.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(dimensions::SPACING_TINY),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ))
            .with_children(|info| {
                // Law name
                info.spawn((
                    Text::new(name),
                    TextFont {
                        font_size: typography::TEXT_SIZE_BODY,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                ));

                // Category
                info.spawn((
                    Text::new(format!("[{}]", category)),
                    TextFont {
                        font_size: typography::TEXT_SIZE_SMALL,
                        ..default()
                    },
                    TextColor(colors::TEXT_TERTIARY),
                ));
            });

            // Repeal button
            item.spawn((
                Button,
                Node {
                    padding: UiRect::horizontal(Val::Px(dimensions::SPACING_SMALL)),
                    padding: UiRect::vertical(Val::Px(dimensions::SPACING_TINY)),
                    border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                    ..default()
                },
                BackgroundColor(colors::DANGER),
                BorderColor(colors::BORDER),
                RepealLawButton { law_id },
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("Repeal"),
                    TextFont {
                        font_size: typography::TEXT_SIZE_SMALL,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                ));
            });
        });
}

/// Spawn a proposed law item
fn spawn_proposed_law_item(
    parent: &mut ChildSpawnerCommands,
    index: usize,
    name: &str,
    support: f32,
    debate_duration: f32,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(dimensions::SPACING_SMALL)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::SPACING_SMALL),
                border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                ..default()
            },
            BackgroundColor(colors::SURFACE_DARK),
            BorderColor(colors::BORDER),
            ProposedLawItem { index },
        ))
        .with_children(|item| {
            // Law name
            item.spawn((
                Text::new(name),
                TextFont {
                    font_size: typography::TEXT_SIZE_BODY,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
            ));

            // Support percentage with progress bar visual
            item.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(20.0),
                    border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_DARKER),
                BorderColor(colors::BORDER),
            ))
            .with_children(|progress| {
                // Progress fill
                progress.spawn((
                    Node {
                        width: Val::Percent(support * 100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(if support >= 0.5 {
                        colors::SUCCESS
                    } else {
                        colors::WARNING
                    }),
                ));
            });

            // Support text
            item.spawn((
                Text::new(format!("Support: {:.0}% | Debate: {:.0} days",
                    support * 100.0, debate_duration)),
                TextFont {
                    font_size: typography::TEXT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
            ));

            // Action buttons
            item.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(dimensions::SPACING_SMALL),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ))
            .with_children(|actions| {
                // Support button
                actions.spawn((
                    Button,
                    Node {
                        flex_grow: 1.0,
                        padding: UiRect::all(Val::Px(dimensions::SPACING_SMALL)),
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                        ..default()
                    },
                    BackgroundColor(colors::SUCCESS),
                    BorderColor(colors::BORDER),
                    SupportLawButton { proposal_index: index },
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Support"),
                        TextFont {
                            font_size: typography::TEXT_SIZE_SMALL,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                });

                // Oppose button
                actions.spawn((
                    Button,
                    Node {
                        flex_grow: 1.0,
                        padding: UiRect::all(Val::Px(dimensions::SPACING_SMALL)),
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                        ..default()
                    },
                    BackgroundColor(colors::DANGER),
                    BorderColor(colors::BORDER),
                    OpposeLawButton { proposal_index: index },
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Oppose"),
                        TextFont {
                            font_size: typography::TEXT_SIZE_SMALL,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                });
            });
        });
}

/// Format combined effects into readable text
fn format_combined_effects(effects: &LawEffects) -> String {
    let mut parts = Vec::new();

    // Helper to add effect if non-zero
    let mut add_effect = |value: f32, name: &str, suffix: &str| {
        if value.abs() > 0.001 {
            let sign = if value > 0.0 { "+" } else { "" };
            parts.push(format!("{}{}{:.0}{}", name, sign, value * 100.0, suffix));
        }
    };

    add_effect(effects.tax_efficiency_modifier, "Tax: ", "%");
    add_effect(effects.stability_change, "Stability: ", "%");
    add_effect(effects.happiness_modifier, "Happiness: ", "%");
    add_effect(effects.industrial_output_modifier, "Industry: ", "%");
    add_effect(effects.military_strength_modifier, "Military: ", "%");
    add_effect(effects.technology_rate_modifier, "Tech: ", "%");

    if parts.is_empty() {
        "No combined effects".to_string()
    } else {
        parts.join(" | ")
    }
}