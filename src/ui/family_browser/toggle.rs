//! Toggle button for family browser

use bevy::prelude::*;
use super::types::FamilyBrowserPanel;

/// Marker for the toggle family browser button
#[derive(Component)]
pub struct ToggleFamilyBrowserButton;

/// Spawn toggle button in HUD
pub fn spawn_toggle_button(parent: &mut crate::ui::ChildBuilder) {
    use crate::ui::{ButtonBuilder, ButtonSize};

    parent
        .spawn(Node {
            margin: UiRect::all(Val::Px(4.0)),
            ..default()
        })
        .with_children(|parent| {
            ButtonBuilder::new("Noble Houses")
                .size(ButtonSize::Small)
                .with_marker(ToggleFamilyBrowserButton)
                .build(parent);
        });
}

/// Handle toggle button clicks
pub fn handle_toggle_button(
    buttons: Query<&Interaction, (Changed<Interaction>, With<ToggleFamilyBrowserButton>)>,
    mut panel: Query<&mut Visibility, With<FamilyBrowserPanel>>,
) {
    for interaction in &buttons {
        if *interaction == Interaction::Pressed {
            if let Ok(mut visibility) = panel.single_mut() {
                *visibility = match *visibility {
                    Visibility::Hidden => Visibility::Visible,
                    _ => Visibility::Hidden,
                };
            }
        }
    }
}

/// Handle keyboard shortcut to toggle browser
pub fn handle_keyboard_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    mut panel: Query<&mut Visibility, With<FamilyBrowserPanel>>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        if let Ok(mut visibility) = panel.single_mut() {
            *visibility = match *visibility {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }
}
