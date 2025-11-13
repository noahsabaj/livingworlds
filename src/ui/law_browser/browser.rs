//! Main law browser UI implementation

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

use crate::nations::{get_all_laws, LawRegistry, LawCategory};
use crate::states::GameState;
use crate::ui::{ShortcutEvent, ShortcutId};
use crate::ui::styles::{colors, dimensions};

use super::categories::{
    handle_category_tab_clicks, spawn_category_tabs, update_category_tab_visuals,
};
use super::details::{spawn_law_details_panel, update_law_details};
use super::search::{filter_laws_by_search, spawn_search_bar};
use super::types::*;

define_plugin!(LawBrowserPlugin {
    resources: [
        LawBrowserState,
        SelectedLawCategory,
        SelectedLawId
    ],

    startup: [
        setup_law_browser_resources
    ],

    update: [
        toggle_law_browser.run_if(in_state(GameState::InGame)),
        handle_category_tab_clicks.run_if(in_state(GameState::InGame)),
        handle_law_item_clicks.run_if(in_state(GameState::InGame)),
        handle_close_button.run_if(in_state(GameState::InGame)),
        update_category_tab_visuals.run_if(in_state(GameState::InGame)),
        update_law_details.run_if(in_state(GameState::InGame)),
        update_laws_list.run_if(in_state(GameState::InGame))
    ]
});

/// Initialize law browser resources
fn setup_law_browser_resources(mut commands: Commands) {
    commands.insert_resource(LawBrowserState::default());
    commands.insert_resource(SelectedLawCategory(Some(LawCategory::Economic)));
    commands.insert_resource(SelectedLawId::default());
}

/// Toggle law browser visibility using shortcuts registry
fn toggle_law_browser(
    mut shortcut_events: MessageReader<ShortcutEvent>,
    mut state: ResMut<LawBrowserState>,
    mut commands: Commands,
    query: Query<Entity, With<LawBrowserRoot>>,
) {
    for event in shortcut_events.read() {
        // Check for a law browser toggle shortcut (could be L key or any custom binding)
        if let ShortcutId::Custom(ref id) = event.shortcut_id {
            if id == "ToggleLawBrowser" {
                state.is_open = !state.is_open;

                if state.is_open {
                    if query.is_empty() {
                        spawn_law_browser(&mut commands);
                    }
                } else {
                    for entity in &query {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}

/// Spawn the complete law browser UI
pub fn spawn_law_browser(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                left: Val::Percent(10.0),
                top: Val::Percent(10.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_DARKER),
            BorderColor::all(colors::BORDER_ACTIVE),
            ZIndex(100),
            LawBrowserRoot,
        ))
        .with_children(|browser| {
            // Header with title and close button
            spawn_header(browser);

            // Search bar
            spawn_search_bar(browser);

            // Category tabs
            spawn_category_tabs(browser, Some(LawCategory::Economic));

            // Main content area
            browser
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    BackgroundColor(colors::BACKGROUND_DARK),
                ))
                .with_children(|content| {
                    // Laws list
                    spawn_laws_list(content);

                    // Law details panel
                    spawn_law_details_panel(content);
                });
        });
}

/// Spawn the header with title and close button
fn spawn_header(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                padding: UiRect::all(Val::Px(dimensions::SPACING_LARGE)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                border: UiRect::bottom(Val::Px(dimensions::BORDER_WIDTH)),
                ..default()
            },
            BackgroundColor(colors::SURFACE),
            BorderColor::all(colors::BORDER),
        ))
        .with_children(|header| {
            // Title
            header.spawn((
                Text::new("LAW CODEX"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_TITLE,
                    ..default()
                },
                TextColor(colors::TEXT_TITLE),
            ));

            // Close button
            header
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                        ..default()
                    },
                    BackgroundColor(colors::DANGER),
                    BorderColor::all(colors::BORDER),
                    LawBrowserCloseButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("X"),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                });
        });
}

/// Spawn the laws list container
fn spawn_laws_list(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(60.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(dimensions::SPACING_MEDIUM)),
            row_gap: Val::Px(dimensions::SPACING_SMALL),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        BackgroundColor(colors::BACKGROUND_DARK),
        LawsListContainer,
    ));
}

/// Update the laws list when category changes
fn update_laws_list(
    selected_category: Res<SelectedLawCategory>,
    registry: Res<LawRegistry>,
    mut commands: Commands,
    container_query: Query<Entity, With<LawsListContainer>>,
    state: Res<LawBrowserState>,
) {
    if !selected_category.is_changed() && !state.is_changed() {
        return;
    }

    if let Ok(container) = container_query.single() {
        // Clear existing laws
        commands.entity(container).despawn();

        // Get laws for selected category
        if let Some(category) = selected_category.0 {
            let laws = get_all_laws()
                .into_iter()
                .filter(|law| law.category == category)
                .filter(|law| filter_laws_by_search(&state.search_text, &law.name, &law.description));

            // Spawn law items
            commands.entity(container).with_children(|parent| {
                for law in laws {
                    spawn_law_item(parent, law.id, &law.name, &law.description);
                }
            });
        }
    }
}

/// Spawn a single law item in the list
fn spawn_law_item(parent: &mut ChildSpawnerCommands, law_id: crate::nations::LawId, name: &str, description: &str) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(60.0),
                padding: UiRect::all(Val::Px(dimensions::SPACING_MEDIUM)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::SPACING_TINY),
                border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                ..default()
            },
            BackgroundColor(colors::SURFACE),
            BorderColor::all(colors::BORDER),
            LawListItem { law_id },
        ))
        .with_children(|item| {
            // Law name
            item.spawn((
                Text::new(name),
                TextFont {
                    font_size: dimensions::FONT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
            ));

            // Law description preview
            item.spawn((
                Text::new(if description.len() > 80 {
                    format!("{}...", &description[..80])
                } else {
                    description.to_string()
                }),
                TextFont {
                    font_size: dimensions::FONT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
            ));
        });
}

/// Handle law item clicks
fn handle_law_item_clicks(
    mut interaction_query: Query<
        (&Interaction, &LawListItem, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut selected_law: ResMut<SelectedLawId>,
) {
    for (interaction, item, mut bg) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                selected_law.0 = Some(item.law_id);
                *bg = BackgroundColor(colors::PRIMARY);
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(colors::SURFACE_HOVER);
            }
            Interaction::None => {
                *bg = BackgroundColor(colors::SURFACE);
            }
        }
    }
}

/// Handle close button click
fn handle_close_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<LawBrowserCloseButton>)>,
    mut state: ResMut<LawBrowserState>,
    mut commands: Commands,
    browser_query: Query<Entity, With<LawBrowserRoot>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            state.is_open = false;
            for entity in &browser_query {
                commands.entity(entity).despawn();
            }
            break; // Only handle the first pressed button
        }
    }
}