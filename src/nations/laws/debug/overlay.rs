//! Law debug overlay for visual debugging
//!
//! Displays law information as an overlay during development.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use crate::nations::{Nation, laws::NationLaws};
use crate::ui::{colors, dimensions};
use crate::ui::SelectedNation;

/// Marker for law debug overlay
#[derive(Component)]
pub struct LawDebugOverlay;

/// Resource tracking overlay state
#[derive(Resource, Default)]
pub struct LawDebugOverlayState {
    pub enabled: bool,
}

define_plugin!(LawDebugPlugin {
    resources: [
        LawDebugOverlayState
    ],

    update: [
        toggle_law_debug_overlay,
        update_law_debug_overlay
    ]
});

/// Toggle debug overlay with F3 key
pub fn toggle_law_debug_overlay(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<LawDebugOverlayState>,
    mut commands: Commands,
    overlay_query: Query<Entity, With<LawDebugOverlay>>,
) {
    if keys.just_pressed(KeyCode::F3) {
        state.enabled = !state.enabled;

        if state.enabled {
            spawn_debug_overlay(&mut commands);
            info!("Law debug overlay enabled");
        } else {
            // Remove overlay
            for entity in &overlay_query {
                commands.entity(entity).despawn_recursive();
            }
            info!("Law debug overlay disabled");
        }
    }
}

/// Spawn the debug overlay UI
fn spawn_debug_overlay(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                width: Val::Px(400.0),
                max_height: Val::Vh(50.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                border: UiRect::all(Val::Px(2.0)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            BorderColor(colors::WARNING),
            ZIndex(1000), // Always on top
            LawDebugOverlay,
        ))
        .with_children(|overlay| {
            // Title
            overlay.spawn((
                Text::new("LAW DEBUG OVERLAY (F3)"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(colors::WARNING),
                DebugTitle,
            ));

            // Nation info
            overlay.spawn((
                Text::new("No nation selected"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                DebugNationText,
            ));

            // Active laws count
            overlay.spawn((
                Text::new("Active laws: 0"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::SUCCESS),
                DebugActiveLawsText,
            ));

            // Proposed laws count
            overlay.spawn((
                Text::new("Proposed laws: 0"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::WARNING),
                DebugProposedLawsText,
            ));

            // Combined effects
            overlay.spawn((
                Text::new("Combined effects: None"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
                DebugEffectsText,
            ));

            // Debug commands help
            overlay.spawn((
                Text::new("Commands:\n  Shift+E: Force enact\n  Shift+R: Force repeal\n  Shift+P: Test proposal\n  L: Law browser"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::TEXT_TERTIARY),
            ));
        });
}

/// Update debug overlay with current law data
fn update_law_debug_overlay(
    selected_nation: Res<SelectedNation>,
    nations_query: Query<(&Nation, &NationLaws)>,
    mut nation_text: Query<&mut Text, (With<DebugNationText>, Without<DebugActiveLawsText>, Without<DebugProposedLawsText>, Without<DebugEffectsText>)>,
    mut active_text: Query<&mut Text, (With<DebugActiveLawsText>, Without<DebugNationText>, Without<DebugProposedLawsText>, Without<DebugEffectsText>)>,
    mut proposed_text: Query<&mut Text, (With<DebugProposedLawsText>, Without<DebugNationText>, Without<DebugActiveLawsText>, Without<DebugEffectsText>)>,
    mut effects_text: Query<&mut Text, (With<DebugEffectsText>, Without<DebugNationText>, Without<DebugActiveLawsText>, Without<DebugProposedLawsText>)>,
) {
    if let Some(nation_entity) = selected_nation.entity {
        if let Ok((nation, nation_laws)) = nations_query.get(nation_entity) {
            // Update nation name
            if let Ok(mut text) = nation_text.get_single_mut() {
                text.0 = format!("Nation: {}", nation.name);
            }

            // Update active laws count
            if let Ok(mut text) = active_text.get_single_mut() {
                text.0 = format!("Active laws: {} (cooldowns: {})",
                    nation_laws.active_laws.len(),
                    nation_laws.proposal_cooldowns.len()
                );
            }

            // Update proposed laws info
            if let Ok(mut text) = proposed_text.get_single_mut() {
                if nation_laws.proposed_laws.is_empty() {
                    text.0 = "Proposed laws: 0".to_string();
                } else {
                    let avg_support = nation_laws.proposed_laws.iter()
                        .map(|p| p.current_support)
                        .sum::<f32>() / nation_laws.proposed_laws.len() as f32;
                    text.0 = format!("Proposed laws: {} (avg support: {:.0}%)",
                        nation_laws.proposed_laws.len(),
                        avg_support * 100.0
                    );
                }
            }

            // Update combined effects summary
            if let Ok(mut text) = effects_text.get_single_mut() {
                let effects = &nation_laws.combined_effects;
                let mut effect_parts = Vec::new();

                if effects.tax_efficiency_modifier.abs() > 0.001 {
                    effect_parts.push(format!("Tax: {:+.0}%", effects.tax_efficiency_modifier * 100.0));
                }
                if effects.stability_change.abs() > 0.001 {
                    effect_parts.push(format!("Stab: {:+.0}%", effects.stability_change * 100.0));
                }
                if effects.army_morale_modifier.abs() > 0.001 {
                    effect_parts.push(format!("Mil: {:+.0}%", effects.army_morale_modifier * 100.0));
                }

                if effect_parts.is_empty() {
                    text.0 = "Combined effects: None".to_string();
                } else {
                    text.0 = format!("Effects: {}", effect_parts.join(", "));
                }
            }
        }
    } else {
        // Clear debug info when no nation selected
        if let Ok(mut text) = nation_text.get_single_mut() {
            text.0 = "No nation selected".to_string();
        }
        if let Ok(mut text) = active_text.get_single_mut() {
            text.0 = "Active laws: 0".to_string();
        }
        if let Ok(mut text) = proposed_text.get_single_mut() {
            text.0 = "Proposed laws: 0".to_string();
        }
        if let Ok(mut text) = effects_text.get_single_mut() {
            text.0 = "Combined effects: None".to_string();
        }
    }
}

// Marker components for debug text
#[derive(Component)]
struct DebugTitle;

#[derive(Component)]
struct DebugNationText;

#[derive(Component)]
struct DebugActiveLawsText;

#[derive(Component)]
struct DebugProposedLawsText;

#[derive(Component)]
struct DebugEffectsText;