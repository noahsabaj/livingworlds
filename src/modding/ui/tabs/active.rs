//! Active modset tab UI
//!
//! This module displays the currently active mods with their load order,
//! allowing users to manage and reorder their active mod configuration.

use crate::modding::manager::ModManager;
use crate::modding::ui::types::{ModListItem, ModToggle};
use crate::ui::{colors, ButtonBuilder, ButtonSize, ButtonStyle};
use bevy::prelude::*;

/// Spawns the active modset tab content
pub fn spawn_active_modset_tab(parent: &mut ChildSpawnerCommands, mod_manager: &ModManager) {
    // Active modset list container
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            ..default()
        })
        .with_children(|modset| {
            spawn_modset_header(modset);

            let active_mods: Vec<_> = mod_manager
                .available_mods
                .iter()
                .filter(|m| m.enabled)
                .enumerate()
                .collect();

            if active_mods.is_empty() {
                spawn_empty_message(modset);
            } else {
                for (index, loaded_mod) in active_mods {
                    spawn_mod_list_item(modset, index, loaded_mod);
                }
            }
        });
}

/// Spawns the modset header
fn spawn_modset_header(modset: &mut ChildSpawnerCommands) {
    modset.spawn((
        Text::new("Active Modset"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(colors::TEXT_PRIMARY),
        Node {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        },
    ));
}

/// Spawns a single mod in the active list
fn spawn_mod_list_item(
    modset: &mut ChildSpawnerCommands,
    index: usize,
    loaded_mod: &crate::modding::types::LoadedMod,
) {
    modset
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::bottom(Val::Px(5.0)),
                border: UiRect::all(Val::Px(1.0)),
                column_gap: Val::Px(15.0),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_MEDIUM),
            BorderColor::all(colors::BORDER_DEFAULT),
            ModListItem {
                mod_id: loaded_mod.manifest.id.clone(),
                load_order: index,
            },
        ))
        .with_children(|item| {
            spawn_drag_handle(item);
            spawn_load_order_number(item, index);
            spawn_mod_name(item, &loaded_mod.manifest.name);
            spawn_mod_version(item, &loaded_mod.manifest.version);
            spawn_disable_button(item, &loaded_mod.manifest.id);
        });
}

/// Spawns the drag handle for reordering
fn spawn_drag_handle(item: &mut ChildSpawnerCommands) {
    item.spawn((
        Text::new("≡"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(colors::TEXT_TERTIARY),
    ));
}

/// Spawns the load order number
fn spawn_load_order_number(item: &mut ChildSpawnerCommands, index: usize) {
    item.spawn((
        Text::new(format!("#{}", index + 1)),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(colors::TEXT_SECONDARY),
    ));
}

/// Spawns the mod name
fn spawn_mod_name(item: &mut ChildSpawnerCommands, name: &str) {
    item.spawn((
        Text::new(name),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(colors::TEXT_PRIMARY),
        Node {
            flex_grow: 1.0,
            ..default()
        },
    ));
}

/// Spawns the mod version
fn spawn_mod_version(item: &mut ChildSpawnerCommands, version: &str) {
    item.spawn((
        Text::new(format!("v{}", version)),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(colors::TEXT_TERTIARY),
    ));
}

/// Spawns the disable button for the mod
fn spawn_disable_button(item: &mut ChildSpawnerCommands, mod_id: &str) {
    ButtonBuilder::new("Disable")
        .style(ButtonStyle::Danger)
        .size(ButtonSize::Small)
        .with_marker(ModToggle {
            mod_id: mod_id.to_string(),
        })
        .build(item);
}

/// Spawns the empty modset message
fn spawn_empty_message(modset: &mut ChildSpawnerCommands) {
    modset.spawn((
        Text::new("No mods currently active"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(colors::TEXT_TERTIARY),
        Node {
            margin: UiRect::top(Val::Px(50.0)),
            align_self: AlignSelf::Center,
            ..default()
        },
    ));
}

// TODO: Future functionality
// - Drag and drop reordering of load order
// - Conflict detection between mods
// - Save/load modset presets
// - Export modset for sharing