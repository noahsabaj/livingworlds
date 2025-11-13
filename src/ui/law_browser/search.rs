//! Law search and filtering UI

use bevy::prelude::*;
use crate::ui::styles::{colors, dimensions};
use super::types::*;

/// Spawn the search bar for law filtering
pub fn spawn_search_bar(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(50.0),
            padding: UiRect::all(Val::Px(dimensions::SPACING_MEDIUM)),
            border: UiRect::bottom(Val::Px(dimensions::BORDER_WIDTH)),
            ..default()
        },
        BackgroundColor(colors::BACKGROUND_DARK),
        BorderColor::all(colors::BORDER),
    ))
    .with_children(|search_container| {
        // Search input placeholder (would integrate with text input system)
        search_container.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(35.0),
                padding: UiRect::horizontal(Val::Px(dimensions::SPACING_MEDIUM)),
                border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::SURFACE),
            BorderColor::all(colors::BORDER),
            LawSearchInput,
        ))
        .with_children(|input| {
            input.spawn((
                Text::new("Search laws..."),
                TextFont {
                    font_size: dimensions::FONT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(colors::TEXT_TERTIARY),
            ));
        });
    });
}

/// Filter laws based on search text
pub fn filter_laws_by_search(
    search_text: &str,
    law_name: &str,
    law_description: &str,
) -> bool {
    if search_text.is_empty() {
        return true;
    }

    let search_lower = search_text.to_lowercase();
    law_name.to_lowercase().contains(&search_lower)
        || law_description.to_lowercase().contains(&search_lower)
}