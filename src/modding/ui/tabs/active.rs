//! Active modset tab UI
//!
//! This module displays the currently active mods with their load order,
//! allowing users to manage and reorder their active mod configuration.

use crate::modding::manager::ModManager;
use crate::modding::ui::types::{ModListItem, ModToggle};
use crate::ui::{
    colors, ButtonBuilder, ButtonSize, ButtonStyle, LabelBuilder, LabelStyle, PanelBuilder,
    PanelStyle, ScrollViewBuilder, ScrollbarVisibility,
};
use bevy::prelude::*;

/// Spawns the active modset tab content
pub fn spawn_active_modset_tab(parent: &mut ChildSpawnerCommands, mod_manager: &ModManager) {
    // Active modset list container
    ScrollViewBuilder::new()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .scrollbar_visibility(ScrollbarVisibility::AutoHide { timeout_secs: 2.0 })
        .build_with_children(parent, |scroll_content| {
            spawn_modset_header(scroll_content);

            let active_mods: Vec<_> = mod_manager
                .available_mods
                .iter()
                .filter(|m| m.enabled)
                .enumerate()
                .collect();

            if active_mods.is_empty() {
                spawn_empty_message(scroll_content);
            } else {
                for (index, loaded_mod) in active_mods {
                    spawn_mod_list_item(scroll_content, index, loaded_mod);
                }
            }
        });
}

/// Spawns the modset header
fn spawn_modset_header(modset: &mut ChildSpawnerCommands) {
    LabelBuilder::new("Active Modset")
        .style(LabelStyle::Title)
        .margin(UiRect::bottom(Val::Px(20.0)))
        .build(modset);
}

/// Spawns a single mod in the active list
fn spawn_mod_list_item(
    modset: &mut ChildSpawnerCommands,
    index: usize,
    loaded_mod: &crate::modding::types::LoadedMod,
) {
    let panel = PanelBuilder::new()
        .style(PanelStyle::Card)
        .width(Val::Percent(100.0))
        .padding(UiRect::all(Val::Px(10.0)))
        .margin(UiRect::bottom(Val::Px(5.0)))
        .border_color(colors::BORDER_DEFAULT)
        .flex_direction(FlexDirection::Row)
        .align_items(AlignItems::Center)
        .column_gap(Val::Px(15.0))
        .build_with_children(modset, |item| {
            spawn_drag_handle(item);
            spawn_load_order_number(item, index);
            spawn_mod_name(item, &loaded_mod.manifest.name);
            spawn_mod_version(item, &loaded_mod.manifest.version);
            spawn_disable_button(item, &loaded_mod.manifest.id);
        });
        
    // Add marker manually since PanelBuilder doesn't support it directly
    modset.commands().entity(panel).insert(ModListItem {
        mod_id: loaded_mod.manifest.id.clone(),
        load_order: index,
    });
}

/// Spawns the drag handle for reordering
fn spawn_drag_handle(item: &mut ChildSpawnerCommands) {
    LabelBuilder::new("≡")
        .style(LabelStyle::Body)
        //.text_color(colors::TEXT_TERTIARY) // Not supported, use default style
        .build(item);
}

/// Spawns the load order number
fn spawn_load_order_number(item: &mut ChildSpawnerCommands, index: usize) {
    LabelBuilder::new(format!("#{}", index + 1))
        .style(LabelStyle::Body)
        //.text_color(colors::TEXT_SECONDARY) // Not supported
        .build(item);
}

/// Spawns the mod name
fn spawn_mod_name(item: &mut ChildSpawnerCommands, name: &str) {
    // Wrap label in a Node for flex_grow
    item.spawn(Node {
        flex_grow: 1.0,
        ..default()
    }).with_children(|parent| {
        LabelBuilder::new(name)
            .style(LabelStyle::Body)
            .build(parent);
    });
}

/// Spawns the mod version
fn spawn_mod_version(item: &mut ChildSpawnerCommands, version: &str) {
    LabelBuilder::new(format!("v{}", version))
        .style(LabelStyle::Caption)
        .build(item);
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
    // Wrap label in a Node for align_self
    modset.spawn(Node {
        margin: UiRect::top(Val::Px(50.0)),
        align_self: AlignSelf::Center,
        ..default()
    }).with_children(|parent| {
        LabelBuilder::new("No mods currently active")
            .style(LabelStyle::Body)
            //.text_color(colors::TEXT_TERTIARY)
            .build(parent);
    });
}

// TODO: Future functionality
// - Drag and drop reordering of load order
// - Conflict detection between mods
// - Save/load modset presets
// - Export modset for sharing