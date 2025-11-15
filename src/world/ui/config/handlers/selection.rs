//! Selection handling systems
//!
//! This module handles all selection button interactions using a generic approach.

#![allow(elided_lifetimes_in_paths)]

use super::super::components::*;
use super::super::types::*;
use crate::ui::colors;
use bevy::prelude::*;
use bevy_ui_builders::button::SelectionChanged;

// Special handler for preset selection (includes apply_preset logic)
pub fn handle_preset_selection(
    mut selection_events: EventReader<SelectionChanged>,
    preset_buttons: Query<&PresetButton>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    // bevy-ui-builders v0.2.1 handles all selection state and visual updates automatically
    // We just need to update our settings and apply the preset when selection changes
    for event in selection_events.read() {
        if event.selected {
            // Find which preset button was selected
            if let Ok(preset_button) = preset_buttons.get(event.entity) {
                settings.preset = preset_button.0;
                settings.apply_preset(); // Apply preset settings
                debug!("Selected preset: {:?}", preset_button.0);
            }
        }
    }
}

// Size selection handler using bevy-ui-builders v0.2.1 selection system
pub fn handle_size_selection(
    mut selection_events: EventReader<SelectionChanged>,
    size_buttons: Query<&SizeButton>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    // bevy-ui-builders v0.2.1 handles all selection state and visual updates automatically
    // We just need to update our settings when selection changes
    for event in selection_events.read() {
        if event.selected {
            // Find which size button was selected
            if let Ok(size_button) = size_buttons.get(event.entity) {
                settings.world_size = size_button.0;
                debug!("Selected world_size: {:?}", size_button.0);
            }
        }
    }
}

// Advanced setting handlers using bevy-ui-builders v0.2.1 selection system
pub fn handle_climate_selection(
    mut selection_events: EventReader<SelectionChanged>,
    climate_buttons: Query<&ClimateButton>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    for event in selection_events.read() {
        if event.selected {
            if let Ok(climate_button) = climate_buttons.get(event.entity) {
                settings.climate_type = climate_button.0.clone();
                debug!("Selected climate type: {:?}", climate_button.0);
            }
        }
    }
}

pub fn handle_island_selection(
    mut selection_events: EventReader<SelectionChanged>,
    island_buttons: Query<&IslandButton>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    for event in selection_events.read() {
        if event.selected {
            if let Ok(island_button) = island_buttons.get(event.entity) {
                settings.island_frequency = island_button.0.clone();
                debug!("Selected island frequency: {:?}", island_button.0);
            }
        }
    }
}

pub fn handle_aggression_selection(
    mut selection_events: EventReader<SelectionChanged>,
    aggression_buttons: Query<&AggressionButton>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    for event in selection_events.read() {
        if event.selected {
            if let Ok(aggression_button) = aggression_buttons.get(event.entity) {
                settings.aggression_level = aggression_button.0.clone();
                debug!("Selected aggression level: {:?}", aggression_button.0);
            }
        }
    }
}

pub fn handle_resource_selection(
    mut selection_events: EventReader<SelectionChanged>,
    resource_buttons: Query<&ResourceButton>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    for event in selection_events.read() {
        if event.selected {
            if let Ok(resource_button) = resource_buttons.get(event.entity) {
                settings.resource_abundance = resource_button.0.clone();
                debug!("Selected resource abundance: {:?}", resource_button.0);
            }
        }
    }
}

pub fn handle_calendar_selection(
    mut selection_events: EventReader<SelectionChanged>,
    calendar_buttons: Query<&CalendarButton>,
    mut settings: ResMut<WorldGenerationSettings>,
    calendar_registry: Res<crate::simulation::CalendarRegistry>,
    mut preview_query: Query<&mut Text, With<CalendarPreviewName>>,
    mut periods_query: Query<&mut Text, (With<CalendarPreviewPeriods>, Without<CalendarPreviewName>)>,
) {
    // bevy-ui-builders v0.2.1 handles all selection state and visual updates automatically
    // We just need to update our settings and preview panel when selection changes
    for event in selection_events.read() {
        if event.selected {
            // Find which calendar button was selected
            if let Ok(calendar_button) = calendar_buttons.get(event.entity) {
                let calendar_id = &calendar_button.0;
                settings.calendar_id = calendar_id.clone();

                // Update preview panel
                if let Some(calendar) = calendar_registry.get_calendar(calendar_id) {
                    if let Ok(mut text) = preview_query.single_mut() {
                        **text = format!("{} - {} days/year", calendar.name, calendar.days_per_year());
                    }

                    if let Ok(mut text) = periods_query.single_mut() {
                        let period_names: Vec<String> = calendar
                            .periods
                            .iter()
                            .take(6)
                            .map(|p| p.name.clone())
                            .collect();
                        let period_display = if calendar.periods.len() > 6 {
                            format!("{}, ... ({} total)", period_names.join(", "), calendar.periods.len())
                        } else {
                            period_names.join(", ")
                        };
                        **text = format!("Periods: {}", period_display);
                    }
                }

                debug!("Selected calendar: {}", calendar_id);
            }
        }
    }
}
