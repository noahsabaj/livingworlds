//! Interaction handling systems
//!
//! This module handles hover effects, toggles, and other UI interactions.

use super::super::components::*;
use super::super::types::WorldGenerationSettings;
use crate::ui::define_marker_interactions;
use bevy::prelude::*;

pub fn handle_preset_hover(
    interactions: Query<
        (&Interaction, &PresetDescription),
        (Changed<Interaction>, With<PresetButton>),
    >,
    mut description_text: Query<&mut Text, With<PresetDescriptionText>>,
) {
    for (interaction, preset_desc) in &interactions {
        if *interaction == Interaction::Hovered {
            if let Ok(mut text) = description_text.single_mut() {
                text.0 = preset_desc.0.clone();
            }
        }
    }
}

// Marker interaction automation - reduces 23 lines to 14 lines
define_marker_interactions! {
    AdvancedToggle => handle_advanced_toggle(
        mut advanced_panel: Query<&mut Node, With<AdvancedPanel>>
    ) {
        if let Ok(mut panel_style) = advanced_panel.single_mut() {
            let is_showing = panel_style.display == Display::Flex;
            panel_style.display = if is_showing {
                Display::None
            } else {
                Display::Flex
            };

            debug!(
                "Advanced settings toggled: {}",
                if is_showing { "hidden" } else { "shown" }
            );
        } else {
            warn!("Could not find AdvancedPanel entity for toggling");
        }
    }
}

pub fn handle_slider_interactions(
    _interactions: Query<(&Interaction, &Node, &Children), With<Button>>,
    _continent_sliders: Query<&mut Node, (With<ContinentSlider>, Without<Button>)>,
    _ocean_sliders: Query<
        &mut Node,
        (With<OceanSlider>, Without<Button>, Without<ContinentSlider>),
    >,
    _river_sliders: Query<
        &mut Node,
        (
            With<RiverSlider>,
            Without<Button>,
            Without<ContinentSlider>,
            Without<OceanSlider>,
        ),
    >,
    _settings: ResMut<WorldGenerationSettings>,
    _windows: Query<&Window>,
) {
    // This function remains for potential future custom slider behavior
    // Currently sliders use our standard SliderBuilder system
}
