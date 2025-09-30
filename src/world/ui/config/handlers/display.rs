//! Display update systems
//!
//! This module handles updating display text for various UI elements.

use super::super::components::*;
use super::super::types::WorldGenerationSettings;
use crate::resources::WorldSize;
use bevy::prelude::*;
use crate::ui::TextBuffer;

pub fn update_seed_display(
    settings: Res<WorldGenerationSettings>,
    mut seed_text: Query<&mut TextBuffer, With<SeedInput>>,
    mut time_estimate: Query<&mut Text, (With<GenerationTimeEstimate>, Without<SeedInput>)>,
) {
    if settings.is_changed() {
        for mut text_buffer in &mut seed_text {
            text_buffer.content = settings.seed.to_string();
        }

        if let Ok(mut estimate_text) = time_estimate.single_mut() {
            let time_range = match settings.world_size {
                WorldSize::Small => "~1-2 seconds",
                WorldSize::Medium => "~2-4 seconds",
                WorldSize::Large => "~3-5 seconds",
            };
            estimate_text.0 = format!("Estimated generation time: {}", time_range);
        }
    }
}

pub fn update_slider_displays(
    settings: Res<WorldGenerationSettings>,
    mut continent_text: Query<&mut Text, With<ContinentValueText>>,
    mut ocean_text: Query<&mut Text, (With<OceanValueText>, Without<ContinentValueText>)>,
    mut river_text: Query<
        &mut Text,
        (
            With<RiverValueText>,
            Without<OceanValueText>,
            Without<ContinentValueText>,
        ),
    >,
    mut nations_text: Query<
        &mut Text,
        (
            With<StartingNationsValueText>,
            Without<RiverValueText>,
            Without<OceanValueText>,
            Without<ContinentValueText>,
        ),
    >,
    mut tech_text: Query<
        &mut Text,
        (
            With<TechSpeedValueText>,
            Without<StartingNationsValueText>,
            Without<RiverValueText>,
            Without<OceanValueText>,
            Without<ContinentValueText>,
        ),
    >,
) {
    for mut text in &mut continent_text {
        text.0 = settings.continent_count.to_string();
    }

    for mut text in &mut ocean_text {
        text.0 = format!("{}%", (settings.ocean_coverage * 100.0) as u32);
    }

    for mut text in &mut river_text {
        text.0 = format!("{:.1}x", settings.river_density);
    }

    for mut text in &mut nations_text {
        text.0 = settings.starting_nations.to_string();
    }

    for mut text in &mut tech_text {
        text.0 = format!("{:.1}x", settings.tech_progression_speed);
    }
}
