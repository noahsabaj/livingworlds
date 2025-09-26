//! Installed mods tab UI
//!
//! This module handles the rendering of the installed mods grid view,
//! including mod cards with metadata and enable/disable toggles.

use crate::modding::manager::ModManager;
use crate::modding::ui::types::{ModCard, ModToggle};
use crate::ui::{colors, ButtonBuilder, ButtonSize, ButtonStyle};
use bevy::prelude::*;

/// Spawns the installed mods tab content
pub fn spawn_installed_tab(
    parent: &mut ChildSpawnerCommands,
    mod_manager: &ModManager,
    search_query: &str,
) {
    // Grid for mod cards
    parent
        .spawn(Node {
            display: Display::Grid,
            width: Val::Percent(100.0),
            grid_template_columns: vec![GridTrack::fr(1.0); 3],
            row_gap: Val::Px(20.0),
            column_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|grid| {
            // Filter mods based on search query
            let filtered_mods: Vec<_> = mod_manager
                .available_mods
                .iter()
                .filter(|m| filter_mod(m, search_query))
                .collect();

            // Generate mod cards for each filtered mod
            for loaded_mod in filtered_mods {
                spawn_mod_card(grid, loaded_mod);
            }
        });
}

/// Filters a mod based on the search query
fn filter_mod(loaded_mod: &&crate::modding::types::LoadedMod, search_query: &str) -> bool {
    if search_query.is_empty() {
        return true;
    }

    let query_lower = search_query.to_lowercase();
    let manifest = &loaded_mod.manifest;

    manifest.name.to_lowercase().contains(&query_lower)
        || manifest.description.to_lowercase().contains(&query_lower)
        || manifest.author.to_lowercase().contains(&query_lower)
        || manifest.id.to_lowercase().contains(&query_lower)
}

/// Spawns a single mod card in the grid
fn spawn_mod_card(grid: &mut ChildSpawnerCommands, loaded_mod: &crate::modding::types::LoadedMod) {
    // Mod card container
    grid.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(colors::BACKGROUND_MEDIUM),
        BorderColor(colors::BORDER_DEFAULT),
        ModCard {
            mod_id: loaded_mod.manifest.id.clone(),
            workshop_id: None,
        },
    ))
    .with_children(|card| {
        spawn_mod_thumbnail(card);
        spawn_mod_info(card, loaded_mod);
        spawn_mod_toggle(card, loaded_mod);
    });
}

/// Spawns the mod thumbnail placeholder
fn spawn_mod_thumbnail(card: &mut ChildSpawnerCommands) {
    card.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(120.0),
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(colors::BACKGROUND_DARK),
    ))
    .with_children(|thumb| {
        thumb.spawn((
            Text::new("MOD ICON"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(colors::TEXT_TERTIARY),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                ..default()
            },
        ));
    });
}

/// Spawns mod information texts (name, author, version, description)
fn spawn_mod_info(card: &mut ChildSpawnerCommands, loaded_mod: &crate::modding::types::LoadedMod) {
    let manifest = &loaded_mod.manifest;

    // Mod name
    card.spawn((
        Text::new(&manifest.name),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(colors::TEXT_PRIMARY),
        Node {
            margin: UiRect::bottom(Val::Px(5.0)),
            ..default()
        },
    ));

    // Author
    card.spawn((
        Text::new(format!("by {}", manifest.author)),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(colors::TEXT_SECONDARY),
        Node {
            margin: UiRect::bottom(Val::Px(5.0)),
            ..default()
        },
    ));

    // Version and compatibility
    card.spawn((
        Text::new(format!(
            "v{} | Game v{}",
            manifest.version, manifest.compatible_game_version
        )),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(colors::TEXT_TERTIARY),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    ));

    // Description
    card.spawn((
        Text::new(&manifest.description),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(colors::TEXT_SECONDARY),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    ));
}

/// Spawns the enable/disable toggle for a mod
fn spawn_mod_toggle(card: &mut ChildSpawnerCommands, loaded_mod: &crate::modding::types::LoadedMod) {
    card.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(10.0),
        ..default()
    })
    .with_children(|toggle_row| {
        // Checkbox button
        ButtonBuilder::new(if loaded_mod.enabled { "[X]" } else { "[ ]" })
            .style(ButtonStyle::Secondary)
            .size(ButtonSize::Small)
            .with_marker(ModToggle {
                mod_id: loaded_mod.manifest.id.clone(),
            })
            .build(toggle_row);

        // Label
        toggle_row.spawn((
            Text::new("Enabled"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::TEXT_PRIMARY),
        ));
    });
}