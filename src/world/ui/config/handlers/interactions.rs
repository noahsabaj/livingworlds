//! Interaction handling systems
//!
//! This module handles hover effects, toggles, and other UI interactions.

use bevy::prelude::*;
use super::super::components::*;
use super::super::types::WorldGenerationSettings;

pub fn handle_preset_hover(
    interactions: Query<(&Interaction, &PresetDescription), (Changed<Interaction>, With<PresetButton>)>,
    mut description_text: Query<&mut Text, With<PresetDescriptionText>>,
) {
    for (interaction, preset_desc) in &interactions {
        if *interaction == Interaction::Hovered {
            if let Ok(mut text) = description_text.get_single_mut() {
                text.0 = preset_desc.0.clone();
            }
        }
    }
}

pub fn handle_advanced_toggle(
    interactions: Query<&Interaction, (Changed<Interaction>, With<AdvancedToggle>)>,
    mut advanced_panel: Query<&mut Node, With<AdvancedPanel>>,
    mut toggle_text: Query<&mut Text, With<AdvancedToggleText>>,
    mut chevron_text: Query<&mut Text, (With<AdvancedToggleChevron>, Without<AdvancedToggleText>)>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            if let Ok(mut panel_style) = advanced_panel.get_single_mut() {
                let is_showing = panel_style.display == Display::Flex;
                panel_style.display = if is_showing {
                    Display::None
                } else {
                    Display::Flex
                };

                // Update toggle text
                for mut text in &mut toggle_text {
                    text.0 = if is_showing {
                        "Show Advanced Settings".to_string()
                    } else {
                        "Hide Advanced Settings".to_string()
                    };
                }

                // Update chevron
                for mut text in &mut chevron_text {
                    text.0 = if is_showing {
                        "▶".to_string()
                    } else {
                        "▼".to_string()
                    };
                }

                println!("Advanced settings toggled: {}", if is_showing { "hidden" } else { "shown" });
            }
        }
    }
}

pub fn handle_slider_interactions(
    _interactions: Query<(&Interaction, &Node, &Children), With<Button>>,
    _continent_sliders: Query<&mut Node, (With<ContinentSlider>, Without<Button>)>,
    _ocean_sliders: Query<&mut Node, (With<OceanSlider>, Without<Button>, Without<ContinentSlider>)>,
    _river_sliders: Query<&mut Node, (With<RiverSlider>, Without<Button>, Without<ContinentSlider>, Without<OceanSlider>)>,
    _settings: ResMut<WorldGenerationSettings>,
    _windows: Query<&Window>,
) {
    // This function remains for potential future custom slider behavior
    // Currently sliders use our standard SliderBuilder system
}