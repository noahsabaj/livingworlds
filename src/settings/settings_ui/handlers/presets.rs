//! Graphics Preset Handlers
//!
//! Focused handlers for graphics preset buttons and reset functionality.

use crate::settings::{components::*, types::*};
use crate::ui::styles::colors;
use bevy::ecs::system::ParamSet;
use bevy::prelude::*;

/// Handle graphics preset button clicks
pub fn handle_preset_buttons(
    mut preset_queries: ParamSet<(
        Query<(&Interaction, &PresetButton, Entity), (Changed<Interaction>, With<Button>)>,
        Query<
            (
                Entity,
                &PresetButton,
                &mut BackgroundColor,
                &mut BorderColor,
            ),
            With<Button>,
        >,
    )>,
    mut temp_settings: ResMut<TempGameSettings>,
    mut dirty_state: ResMut<SettingsDirtyState>,
    mut slider_queries: Query<(&mut Slider, &Children)>,
    mut text_query: Query<&mut Text, With<SliderValueText>>,
) {
    // First, collect information about which buttons were interacted with
    let mut pressed_preset = None;
    // Pre-allocate with reasonable capacity for UI buttons (typically 5-20 presets)
    let mut hover_interactions = Vec::with_capacity(16);
    let mut none_interactions = Vec::with_capacity(16);

    for (interaction, preset_button, entity) in preset_queries.p0().iter() {
        match *interaction {
            Interaction::Pressed => {
                pressed_preset = Some(preset_button.preset);
            }
            Interaction::Hovered => {
                hover_interactions.push((entity, preset_button.preset));
            }
            Interaction::None => {
                none_interactions.push((entity, preset_button.preset));
            }
        }
    }

    // If a preset was pressed, apply it
    if let Some(pressed) = pressed_preset {
        info!("Applying graphics preset: {:?}", pressed);
        temp_settings.0.graphics.apply_preset(pressed);
        dirty_state.is_dirty = true;

        for (mut slider, children) in &mut slider_queries {
            match slider.setting_type {
                SettingType::RenderScale => {
                    slider.value = temp_settings.0.graphics.render_scale;
                }
                _ => {}
            }

            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    if slider.setting_type == SettingType::RenderScale {
                        text.0 = format!("{:.0}%", slider.value * 100.0);
                    }
                }
            }
        }

        for (_entity, button_preset, mut bg_color, mut border_color) in
            preset_queries.p1().iter_mut()
        {
            let is_selected = button_preset.preset == pressed;
            if is_selected {
                // This is the newly selected preset - make it green
                *bg_color = BackgroundColor(colors::SURFACE_SELECTED);
                *border_color = BorderColor(colors::BORDER_SELECTED);
            } else {
                // This is not selected - make it gray
                *bg_color = BackgroundColor(colors::SECONDARY);
                *border_color = BorderColor(colors::BORDER_DEFAULT);
            }
        }
    }

    for (entity, preset) in hover_interactions {
        let is_selected = temp_settings.0.graphics.current_preset() == Some(preset);
        if !is_selected {
            // Only change hover color if not currently selected
            if let Ok((_, _, mut bg_color, mut border_color)) = preset_queries.p1().get_mut(entity)
            {
                *bg_color = BackgroundColor(colors::SURFACE_HOVER);
                *border_color = BorderColor(colors::BORDER_HOVER);
            }
        }
    }

    for (entity, preset) in none_interactions {
        let is_selected = temp_settings.0.graphics.current_preset() == Some(preset);
        if let Ok((_, _, mut bg_color, mut border_color)) = preset_queries.p1().get_mut(entity) {
            if is_selected {
                *bg_color = BackgroundColor(colors::SURFACE_SELECTED); // Green for selected
                *border_color = BorderColor(colors::BORDER_SELECTED);
            } else {
                *bg_color = BackgroundColor(colors::SECONDARY);
                *border_color = BorderColor(colors::BORDER_DEFAULT);
            }
        }
    }
}

/// Handle reset to defaults button
pub fn handle_reset_button(
    mut interactions: Query<(&Interaction, &ResetButton), (Changed<Interaction>, With<Button>)>,
    mut temp_settings: ResMut<TempGameSettings>,
    mut dirty_state: ResMut<SettingsDirtyState>,
    settings: Res<GameSettings>,
) {
    for (interaction, _reset_button) in &mut interactions {
        if *interaction == Interaction::Pressed {
            info!("Resetting settings to defaults");

            // Reset temp settings to defaults
            temp_settings.0 = GameSettings::default();

            // Mark as dirty if different from current settings
            dirty_state.is_dirty = temp_settings.0 != *settings;

            info!(
                "Settings reset to defaults - dirty state: {}",
                dirty_state.is_dirty
            );
        }
    }
}
