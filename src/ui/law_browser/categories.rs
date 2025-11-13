//! Law category tabs UI

use bevy::prelude::*;
use crate::nations::LawCategory;
use crate::ui::styles::colors;
use crate::ui::styles::dimensions;
// typography module removed - not needed
use super::types::*;

/// Spawn category tabs for law browser
pub fn spawn_category_tabs(parent: &mut ChildSpawnerCommands, selected_category: Option<LawCategory>) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(50.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(dimensions::SPACING_SMALL),
            padding: UiRect::all(Val::Px(dimensions::SPACING_SMALL)),
            border: UiRect::bottom(Val::Px(dimensions::BORDER_WIDTH)),
            ..default()
        },
        BorderColor::all(colors::BORDER),
        CategoryTabsContainer,
    ))
    .with_children(|tabs| {
        spawn_category_tab(tabs, LawCategory::Economic, selected_category);
        spawn_category_tab(tabs, LawCategory::Military, selected_category);
        spawn_category_tab(tabs, LawCategory::Social, selected_category);
        spawn_category_tab(tabs, LawCategory::Religious, selected_category);
        spawn_category_tab(tabs, LawCategory::Criminal, selected_category);
        spawn_category_tab(tabs, LawCategory::Property, selected_category);
        spawn_category_tab(tabs, LawCategory::Immigration, selected_category);
        spawn_category_tab(tabs, LawCategory::Environmental, selected_category);
        spawn_category_tab(tabs, LawCategory::Technology, selected_category);
        spawn_category_tab(tabs, LawCategory::Cultural, selected_category);
        spawn_category_tab(tabs, LawCategory::Administrative, selected_category);
        spawn_category_tab(tabs, LawCategory::Diplomatic, selected_category);
    });
}

/// Spawn a single category tab button
fn spawn_category_tab(
    parent: &mut ChildSpawnerCommands,
    category: LawCategory,
    selected_category: Option<LawCategory>,
) {
    let is_selected = selected_category == Some(category);

    parent.spawn((
        Button,
        Node {
            padding: UiRect::horizontal(Val::Px(dimensions::SPACING_MEDIUM)),
            min_width: Val::Px(80.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
            ..default()
        },
        BackgroundColor(if is_selected {
            colors::PRIMARY
        } else {
            colors::SURFACE
        }),
        BorderColor::all(if is_selected {
            colors::BORDER_ACTIVE
        } else {
            colors::BORDER
        }),
        CategoryTab { category },
    ))
    .with_children(|button| {
        button.spawn((
            Text::new(category.name()),
            TextFont {
                font_size: dimensions::FONT_SIZE_SMALL,
                ..default()
            },
            TextColor(if is_selected {
                colors::TEXT_PRIMARY
            } else {
                colors::TEXT_SECONDARY
            }),
        ));
    });
}

/// Update category tab selection visual state
pub fn update_category_tab_visuals(
    mut tabs: Query<(&CategoryTab, &mut BackgroundColor, &mut BorderColor, &Children)>,
    mut texts: Query<&mut TextColor>,
    selected: Res<SelectedLawCategory>,
) {
    if !selected.is_changed() {
        return;
    }

    for (tab, mut bg, mut border, children) in &mut tabs {
        let is_selected = selected.0 == Some(tab.category);

        *bg = BackgroundColor(if is_selected {
            colors::PRIMARY
        } else {
            colors::SURFACE
        });

        *border = BorderColor::all(if is_selected {
            colors::BORDER_ACTIVE
        } else {
            colors::BORDER
        });

        // Update text color
        for child in children.iter() {
            if let Ok(mut text_color) = texts.get_mut(child) {
                *text_color = TextColor(if is_selected {
                    colors::TEXT_PRIMARY
                } else {
                    colors::TEXT_SECONDARY
                });
            }
        }
    }
}

/// Handle category tab clicks
pub fn handle_category_tab_clicks(
    mut interaction_query: Query<(&Interaction, &CategoryTab), (Changed<Interaction>, With<Button>)>,
    mut selected_category: ResMut<SelectedLawCategory>,
) {
    for (interaction, tab) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            selected_category.0 = Some(tab.category);
        }
    }
}