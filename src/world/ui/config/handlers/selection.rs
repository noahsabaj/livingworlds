//! Selection handling systems
//!
//! This module handles all selection button interactions using a generic approach.

#![allow(elided_lifetimes_in_paths)]

use super::super::components::*;
use super::super::types::*;
use crate::ui::colors;
use bevy::prelude::*;

// Generic macro for creating selection handlers
macro_rules! create_selection_handler {
    ($handler_name:ident, $button_type:ty, $field:ident, $value_type:ty) => {
        pub fn $handler_name(
            interactions: Query<(&Interaction, &$button_type), Changed<Interaction>>,
            mut all_buttons: Query<(
                Entity,
                &$button_type,
                &mut BackgroundColor,
                &mut BorderColor,
            )>,
            mut text_query: Query<&mut TextColor>,
            children_query: Query<&Children>,
            mut settings: ResMut<WorldGenerationSettings>,
        ) {
            for (interaction, button) in &interactions {
                match *interaction {
                    Interaction::Pressed => {
                        settings.$field = button.0;
                        debug!("Selected {}: {:?}", stringify!($field), button.0);

                        for (entity, btn, mut bg_color, mut border_color) in &mut all_buttons {
                            if btn.0 == button.0 {
                                // Selected button
                                *bg_color = BackgroundColor(colors::PRIMARY);
                                *border_color = BorderColor::all(colors::PRIMARY);

                                if let Ok(children) = children_query.get(entity) {
                                    for child in children.iter() {
                                        if let Ok(mut text_color) = text_query.get_mut(child) {
                                            *text_color = TextColor(Color::WHITE);
                                        }
                                    }
                                }
                            } else {
                                // Unselected buttons
                                *bg_color = BackgroundColor(colors::BACKGROUND_LIGHT);
                                *border_color = BorderColor::all(colors::BORDER_DEFAULT);

                                if let Ok(children) = children_query.get(entity) {
                                    for child in children.iter() {
                                        if let Ok(mut text_color) = text_query.get_mut(child) {
                                            *text_color = TextColor(colors::TEXT_PRIMARY);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Interaction::Hovered => {
                        // Apply hover effect to non-selected buttons
                        for (_entity, btn, mut bg_color, mut border_color) in &mut all_buttons {
                            if btn.0 == button.0 && btn.0 != settings.$field {
                                *bg_color = BackgroundColor(colors::BACKGROUND_LIGHT.lighter(0.15));
                                *border_color = BorderColor::all(colors::PRIMARY.with_alpha(0.5));
                            }
                        }
                    }
                    Interaction::None => {
                        // Reset non-selected button to default state
                        for (_entity, btn, mut bg_color, mut border_color) in &mut all_buttons {
                            if btn.0 == button.0 && btn.0 != settings.$field {
                                *bg_color = BackgroundColor(colors::BACKGROUND_LIGHT);
                                *border_color = BorderColor::all(colors::BORDER_DEFAULT);
                            }
                        }
                    }
                }
            }
        }
    };
}

// Special handler for preset selection (includes apply_preset logic)
pub fn handle_preset_selection(
    interactions: Query<(&Interaction, &PresetButton), Changed<Interaction>>,
    mut all_preset_buttons: Query<(
        Entity,
        &PresetButton,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
    mut text_query: Query<&mut TextColor>,
    children_query: Query<&Children>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    for (interaction, preset_button) in &interactions {
        match *interaction {
            Interaction::Pressed => {
                settings.preset = preset_button.0;
                settings.apply_preset(); // Apply preset settings
                debug!("Selected preset: {:?}", preset_button.0);

                for (entity, button, mut bg_color, mut border_color) in &mut all_preset_buttons {
                    if button.0 == preset_button.0 {
                        *bg_color = BackgroundColor(colors::PRIMARY);
                        *border_color = BorderColor::all(colors::PRIMARY);

                        if let Ok(children) = children_query.get(entity) {
                            for child in children.iter() {
                                if let Ok(mut text_color) = text_query.get_mut(child) {
                                    *text_color = TextColor(Color::WHITE);
                                }
                            }
                        }
                    } else {
                        *bg_color = BackgroundColor(colors::BACKGROUND_LIGHT);
                        *border_color = BorderColor::all(colors::BORDER_DEFAULT);

                        if let Ok(children) = children_query.get(entity) {
                            for child in children.iter() {
                                if let Ok(mut text_color) = text_query.get_mut(child) {
                                    *text_color = TextColor(colors::TEXT_PRIMARY);
                                }
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                for (_entity, button, mut bg_color, mut border_color) in &mut all_preset_buttons {
                    if button.0 == preset_button.0 && button.0 != settings.preset {
                        *bg_color = BackgroundColor(colors::BACKGROUND_LIGHT.lighter(0.15));
                        *border_color = BorderColor::all(colors::PRIMARY.with_alpha(0.5));
                    }
                }
            }
            Interaction::None => {
                for (_entity, button, mut bg_color, mut border_color) in &mut all_preset_buttons {
                    if button.0 == preset_button.0 && button.0 != settings.preset {
                        *bg_color = BackgroundColor(colors::BACKGROUND_LIGHT);
                        *border_color = BorderColor::all(colors::BORDER_DEFAULT);
                    }
                }
            }
        }
    }
}

// Generate handlers using the macro
create_selection_handler!(handle_size_selection, SizeButton, world_size, WorldSize);

// Advanced setting handlers using ParamSet to avoid query conflicts
pub fn handle_climate_selection(
    mut param_set: ParamSet<(
        Query<(&Interaction, &ClimateButton, Entity), Changed<Interaction>>,
        Query<(&ClimateButton, &mut BackgroundColor)>,
    )>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    let mut pressed_climate = None;
    for (interaction, climate_button, entity) in param_set.p0().iter() {
        if *interaction == Interaction::Pressed {
            pressed_climate = Some((climate_button.0.clone(), entity));
            break;
        }
    }

    if let Some((climate_type, pressed_entity)) = pressed_climate {
        settings.climate_type = climate_type.clone();

        for (_, mut bg_color) in param_set.p1().iter_mut() {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
        }

        if let Ok((_, mut bg_color)) = param_set.p1().get_mut(pressed_entity) {
            *bg_color = BackgroundColor(colors::PRIMARY);
        }

        debug!("Selected climate type: {:?}", climate_type);
    }
}

pub fn handle_island_selection(
    mut param_set: ParamSet<(
        Query<(&Interaction, &IslandButton, Entity), Changed<Interaction>>,
        Query<(&IslandButton, &mut BackgroundColor)>,
    )>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    let mut pressed_island = None;
    for (interaction, island_button, entity) in param_set.p0().iter() {
        if *interaction == Interaction::Pressed {
            pressed_island = Some((island_button.0.clone(), entity));
            break;
        }
    }

    if let Some((island_freq, pressed_entity)) = pressed_island {
        settings.island_frequency = island_freq.clone();

        for (_, mut bg_color) in param_set.p1().iter_mut() {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
        }

        if let Ok((_, mut bg_color)) = param_set.p1().get_mut(pressed_entity) {
            *bg_color = BackgroundColor(colors::PRIMARY);
        }

        debug!("Selected island frequency: {:?}", island_freq);
    }
}

pub fn handle_aggression_selection(
    mut param_set: ParamSet<(
        Query<(&Interaction, &AggressionButton, Entity), Changed<Interaction>>,
        Query<(&AggressionButton, &mut BackgroundColor)>,
    )>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    let mut pressed_aggression = None;
    for (interaction, aggression_button, entity) in param_set.p0().iter() {
        if *interaction == Interaction::Pressed {
            pressed_aggression = Some((aggression_button.0.clone(), entity));
            break;
        }
    }

    if let Some((aggression_level, pressed_entity)) = pressed_aggression {
        settings.aggression_level = aggression_level.clone();

        for (_, mut bg_color) in param_set.p1().iter_mut() {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
        }

        if let Ok((_, mut bg_color)) = param_set.p1().get_mut(pressed_entity) {
            *bg_color = BackgroundColor(colors::PRIMARY);
        }

        debug!("Selected aggression level: {:?}", aggression_level);
    }
}

pub fn handle_resource_selection(
    mut param_set: ParamSet<(
        Query<(&Interaction, &ResourceButton, Entity), Changed<Interaction>>,
        Query<(&ResourceButton, &mut BackgroundColor)>,
    )>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    let mut pressed_resource = None;
    for (interaction, resource_button, entity) in param_set.p0().iter() {
        if *interaction == Interaction::Pressed {
            pressed_resource = Some((resource_button.0.clone(), entity));
            break;
        }
    }

    if let Some((resource_abundance, pressed_entity)) = pressed_resource {
        settings.resource_abundance = resource_abundance.clone();

        for (_, mut bg_color) in param_set.p1().iter_mut() {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
        }

        if let Ok((_, mut bg_color)) = param_set.p1().get_mut(pressed_entity) {
            *bg_color = BackgroundColor(colors::PRIMARY);
        }

        debug!("Selected resource abundance: {:?}", resource_abundance);
    }
}
