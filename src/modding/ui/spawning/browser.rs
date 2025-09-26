//! Main mod browser UI spawning
//!
//! This module handles the spawning of the main mod browser window,
//! including the header, content area, and action bar.

use crate::modding::manager::ModManager;
use crate::modding::ui::spawning::search::spawn_search_bar;
use crate::modding::ui::state::ModBrowserState;
use crate::modding::ui::tabs::spawn_tab_content;
use crate::modding::ui::types::{
    CloseModBrowserButton, ConfirmModsetButton, ContentArea, ModBrowserRoot, ModBrowserTab,
    ModBrowserTabButton,
};
use crate::ui::{
    colors, helpers, layers, ButtonBuilder, ButtonSize, ButtonStyle, PanelBuilder, PanelStyle,
};
use bevy::prelude::*;

/// Spawns the complete mod browser UI
pub fn spawn_mod_browser(
    commands: &mut Commands,
    mod_manager: &ModManager,
    browser_state: &ModBrowserState,
) {
    // Create modal overlay that blocks clicks
    let overlay_entity = helpers::spawn_modal_overlay(
        commands,
        colors::OVERLAY_DARK,
        ZIndex(layers::MODAL_OVERLAY),
    );

    // Add our root marker and configure for column layout
    commands.entity(overlay_entity).insert((
        Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ModBrowserRoot,
    ));

    // Add content to overlay
    commands.entity(overlay_entity).with_children(|parent| {
        spawn_header(parent, browser_state);
        spawn_main_content(parent, browser_state, mod_manager);
        spawn_action_bar(parent);
    });
}

/// Spawns the header with tabs and search
fn spawn_header(parent: &mut ChildSpawnerCommands, browser_state: &ModBrowserState) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_DARK),
        ))
        .with_children(|header| {
            spawn_tab_buttons(header, browser_state);
            spawn_search_bar(header, &browser_state.search_query);
            spawn_user_info(header);
        });
}

/// Spawns the tab buttons
fn spawn_tab_buttons(header: &mut ChildSpawnerCommands, browser_state: &ModBrowserState) {
    PanelBuilder::new()
        .style(PanelStyle::Transparent)
        .flex_direction(FlexDirection::Row)
        .column_gap(Val::Px(20.0))
        .build_with_children(header, |tabs| {
            // Installed tab
            ButtonBuilder::new("Installed")
                .style(if browser_state.current_tab == ModBrowserTab::Installed {
                    ButtonStyle::Primary
                } else {
                    ButtonStyle::Secondary
                })
                .size(ButtonSize::Medium)
                .with_marker(ModBrowserTabButton {
                    tab: ModBrowserTab::Installed,
                })
                .build(tabs);

            // Workshop tab
            ButtonBuilder::new("Workshop")
                .style(if browser_state.current_tab == ModBrowserTab::Workshop {
                    ButtonStyle::Primary
                } else {
                    ButtonStyle::Secondary
                })
                .size(ButtonSize::Medium)
                .with_marker(ModBrowserTabButton {
                    tab: ModBrowserTab::Workshop,
                })
                .build(tabs);

            // Active Modset tab
            ButtonBuilder::new("Active Modset")
                .style(if browser_state.current_tab == ModBrowserTab::ActiveModset {
                    ButtonStyle::Primary
                } else {
                    ButtonStyle::Secondary
                })
                .size(ButtonSize::Medium)
                .with_marker(ModBrowserTabButton {
                    tab: ModBrowserTab::ActiveModset,
                })
                .build(tabs);
        });
}

/// Spawns the user info section
fn spawn_user_info(header: &mut ChildSpawnerCommands) {
    PanelBuilder::new()
        .style(PanelStyle::Transparent)
        .align_items(AlignItems::Center)
        .column_gap(Val::Px(10.0))
        .build_with_children(header, |info| {
            info.spawn((
                Text::new("Steam User"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
            ));
        });
}

/// Spawns the main content area with sidebar and tab content
fn spawn_main_content(
    parent: &mut ChildSpawnerCommands,
    browser_state: &ModBrowserState,
    mod_manager: &ModManager,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_LIGHT),
        ))
        .with_children(|content| {
            spawn_sidebar(content);
            spawn_content_area(content, browser_state, mod_manager);
        });
}

/// Spawns the left sidebar with filters
fn spawn_sidebar(content: &mut ChildSpawnerCommands) {
    content
        .spawn((
            Node {
                width: Val::Px(250.0),
                padding: UiRect::all(Val::Px(20.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_MEDIUM),
        ))
        .with_children(|sidebar| {
            spawn_filter_section(sidebar);
            spawn_stats_section(sidebar);
            spawn_sort_dropdown(sidebar);
        });
}

/// Spawns the filter section in the sidebar
fn spawn_filter_section(sidebar: &mut ChildSpawnerCommands) {
    // Filter header
    sidebar.spawn((
        Text::new("FILTER MODS"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(colors::TEXT_PRIMARY),
        Node {
            margin: UiRect::bottom(Val::Px(15.0)),
            ..default()
        },
    ));

    // Filter checkboxes would go here
    // For now, just placeholders
    sidebar.spawn((
        Text::new("[ ] Enabled\n[ ] Disabled\n[ ] Local\n[ ] Workshop"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(colors::TEXT_SECONDARY),
    ));
}

/// Spawns the stats section in the sidebar
fn spawn_stats_section(sidebar: &mut ChildSpawnerCommands) {
    // Stats header
    sidebar.spawn((
        Text::new("MOD STATS"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(colors::TEXT_PRIMARY),
        Node {
            margin: UiRect::bottom(Val::Px(15.0)),
            ..default()
        },
    ));

    // Stats display
    sidebar.spawn((
        Text::new("Total Installed: 5\nActive: 3\nDisabled: 2"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(colors::TEXT_SECONDARY),
    ));
}

/// Spawns the sort dropdown in the sidebar
fn spawn_sort_dropdown(sidebar: &mut ChildSpawnerCommands) {
    // Sort by label
    sidebar.spawn((
        Text::new("SORT BY"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(colors::TEXT_PRIMARY),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    ));

    // Dropdown placeholder
    sidebar
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(35.0),
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(1.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_LIGHT),
            BorderColor(colors::BORDER_DEFAULT),
        ))
        .with_children(|dropdown| {
            dropdown.spawn((
                Text::new("Name (A-Z)"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
            ));
        });
}

/// Spawns the main content area
fn spawn_content_area(
    content: &mut ChildSpawnerCommands,
    browser_state: &ModBrowserState,
    mod_manager: &ModManager,
) {
    content
        .spawn((
            Node {
                flex_grow: 1.0,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ContentArea,
        ))
        .with_children(|main| {
            // Show content based on current tab
            spawn_tab_content(
                main,
                browser_state.current_tab,
                mod_manager,
                &browser_state.search_query,
            );
        });
}

/// Spawns the bottom action bar
fn spawn_action_bar(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(70.0),
                padding: UiRect::all(Val::Px(15.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_DARK),
        ))
        .with_children(|bar| {
            spawn_left_buttons(bar);
            spawn_confirm_button(bar);
        });
}

/// Spawns the left side buttons in the action bar
fn spawn_left_buttons(bar: &mut ChildSpawnerCommands) {
    bar.spawn(Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(10.0),
        ..default()
    })
    .with_children(|left| {
        ButtonBuilder::new("Back to Main Menu")
            .style(ButtonStyle::Secondary)
            .size(ButtonSize::Medium)
            .with_marker(CloseModBrowserButton)
            .build(left);

        ButtonBuilder::new("Refresh")
            .style(ButtonStyle::Secondary)
            .size(ButtonSize::Medium)
            .build(left);
    });
}

/// Spawns the confirm modset button
fn spawn_confirm_button(bar: &mut ChildSpawnerCommands) {
    ButtonBuilder::new("CONFIRM MODSET")
        .style(ButtonStyle::Primary)
        .size(ButtonSize::Large)
        .with_marker(ConfirmModsetButton)
        .build(bar);
}