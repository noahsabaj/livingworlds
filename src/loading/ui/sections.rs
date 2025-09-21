//! UI section builders for different parts of the loading screen

use super::components::{CancelGenerationButton, LoadingProgressBar, LoadingStatusText};
use crate::loading::state::{LoadingOperation, LoadingState};
use crate::ui::{
    colors, dimensions, get_random_tip, ButtonBuilder, ButtonStyle, LabelBuilder, LabelStyle,
    PanelBuilder, PanelStyle, ProgressBarBuilder,
};
use bevy::prelude::*;

/// Spawn the top section with title and operation subtitle
pub fn spawn_top_section(parent: &mut ChildSpawnerCommands, loading_state: &LoadingState) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|top| {
            // Main title using fixed LabelBuilder
            LabelBuilder::new("LIVING WORLDS")
                .style(LabelStyle::Title)
                .margin(UiRect::bottom(Val::Px(20.0)))
                .build(top);

            // Operation subtitle
            let subtitle = match loading_state.operation {
                LoadingOperation::GeneratingWorld => "Generating New World",
                LoadingOperation::LoadingSave => "Loading Saved Game",
                LoadingOperation::ApplyingMods => "Applying Mod Changes",
                LoadingOperation::None => "Loading...",
            };

            // Operation subtitle using fixed LabelBuilder
            LabelBuilder::new(subtitle)
                .style(LabelStyle::Heading)
                .build(top);
        });
}

/// Spawn the details panel with loading indicator
pub fn spawn_details_panel(parent: &mut ChildSpawnerCommands, loading_state: &LoadingState) {
    // Create panel with consistent styling using PanelBuilder
    PanelBuilder::new()
        .style(PanelStyle::Card)
        .width(Val::Px(600.0))
        .padding(UiRect::all(Val::Px(30.0)))
        .border(UiRect::all(Val::Px(2.0)))
        .flex_direction(FlexDirection::Column)
        .background_color(colors::SURFACE)
        .border_color(colors::BORDER)
        .build_with_children(parent, |panel| {
            // Loading indicator using LabelBuilder for consistency
            LabelBuilder::new("Loading...")
                .font_size(dimensions::FONT_SIZE_LARGE)
                .color(colors::PRIMARY)
                .margin(UiRect::bottom(Val::Px(20.0)))
                .build(panel);

            // Details based on operation
            spawn_operation_details(panel, loading_state);
        });
}

/// Spawn operation-specific details
fn spawn_operation_details(parent: &mut ChildSpawnerCommands, loading_state: &LoadingState) {
    match &loading_state.operation {
        LoadingOperation::GeneratingWorld => {
            if let Some(seed) = loading_state.details.world_seed {
                LabelBuilder::new(&format!("World Seed: {}", seed))
                    .font_size(dimensions::FONT_SIZE_NORMAL)
                    .color(colors::TEXT_PRIMARY)
                    .margin(UiRect::bottom(Val::Px(10.0)))
                    .build(parent);
            }

            if let Some(size) = &loading_state.details.world_size {
                LabelBuilder::new(&format!("World Size: {}", size))
                    .font_size(dimensions::FONT_SIZE_NORMAL)
                    .color(colors::TEXT_PRIMARY)
                    .build(parent);
            }
        }
        LoadingOperation::ApplyingMods => {
            LabelBuilder::new("Reloading game systems with new mod configuration")
                .font_size(dimensions::FONT_SIZE_SMALL)
                .color(colors::TEXT_MUTED)
                .build(parent);
        }
        LoadingOperation::LoadingSave => {
            if let Some(name) = &loading_state.details.save_name {
                LabelBuilder::new(&format!("Save: {}", name))
                    .font_size(dimensions::FONT_SIZE_NORMAL)
                    .color(colors::TEXT_PRIMARY)
                    .margin(UiRect::bottom(Val::Px(10.0)))
                    .build(parent);
            }

            if let Some(days) = loading_state.details.game_days {
                LabelBuilder::new(&format!("World Age: {:.0} days", days))
                    .font_size(dimensions::FONT_SIZE_NORMAL)
                    .color(colors::TEXT_PRIMARY)
                    .build(parent);
            }
        }
        _ => {}
    }
}

/// Spawn the bottom section with progress bar and tips
pub fn spawn_bottom_section(parent: &mut ChildSpawnerCommands, loading_state: &LoadingState) {
    parent
        .spawn(Node {
            width: Val::Percent(60.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|bottom| {
            // Status text using LabelBuilder for consistency
            LabelBuilder::new(&loading_state.current_step)
                .font_size(dimensions::FONT_SIZE_MEDIUM)
                .color(colors::TEXT_PRIMARY)
                .margin(UiRect::bottom(Val::Px(15.0)))
                .build(bottom);

            // Progress bar using ProgressBarBuilder with custom label
            let progress_entity = ProgressBarBuilder::new(loading_state.progress)
                .width(Val::Percent(100.0))
                .height(Val::Px(30.0))
                .with_label_text(&loading_state.current_step)
                .animated()
                .margin(UiRect::bottom(Val::Px(20.0)))
                .build(bottom);

            // Mark the progress bar for updates
            bottom
                .commands()
                .entity(progress_entity)
                .insert(LoadingProgressBar);

            // Loading tip using our new tips system
            LabelBuilder::new(get_random_tip())
                .style(LabelStyle::Caption)
                .margin(UiRect::top(Val::Px(20.0)))
                .build(bottom);

            // Cancel button - only show during world generation
            if loading_state.operation == LoadingOperation::GeneratingWorld {
                ButtonBuilder::new("Cancel Generation")
                    .style(ButtonStyle::Danger)
                    .margin(UiRect::top(Val::Px(30.0)))
                    .with_marker(CancelGenerationButton)
                    .build(bottom);
            }
        });
}
