//! Reusable button system for Living Worlds UI
//!
//! Provides a standardized button component with consistent styling,
//! hover effects, and behavior across the entire game interface.

#![allow(dead_code)] // Preserve UI utility functions for future use

use super::{ChildBuilder, styles::{colors, dimensions, helpers}};
use bevy::prelude::*;

/// Component for styled buttons with consistent behavior
#[derive(Component, Debug, Clone)]
pub struct StyledButton {
    pub style: ButtonStyle,
    pub size: ButtonSize,
    pub enabled: bool,
}

impl Default for StyledButton {
    fn default() -> Self {
        Self {
            style: ButtonStyle::Secondary,
            size: ButtonSize::Medium,
            enabled: true,
        }
    }
}

/// Button style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    Primary,   // Main actions (blue)
    Secondary, // Secondary actions (gray)
    Danger,    // Destructive actions (red)
    Success,   // Positive actions (green)
    Warning,   // Cautionary actions (yellow)
    Ghost,     // Transparent with border only
}

/// Button size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
    XLarge,
}

impl ButtonStyle {
    pub fn base_color(&self) -> Color {
        match self {
            ButtonStyle::Primary => colors::PRIMARY,
            ButtonStyle::Secondary => colors::SECONDARY,
            ButtonStyle::Danger => colors::DANGER,
            ButtonStyle::Success => colors::SUCCESS,
            ButtonStyle::Warning => colors::WARNING,
            ButtonStyle::Ghost => Color::NONE,
        }
    }

    pub fn hover_color(&self) -> Color {
        match self {
            ButtonStyle::Primary => colors::PRIMARY_HOVER,
            ButtonStyle::Secondary => colors::SECONDARY_HOVER,
            ButtonStyle::Danger => colors::DANGER_HOVER,
            ButtonStyle::Success => colors::SUCCESS_HOVER,
            ButtonStyle::Warning => colors::WARNING_HOVER,
            ButtonStyle::Ghost => colors::GHOST_HOVER,
        }
    }

    pub fn pressed_color(&self) -> Color {
        match self {
            ButtonStyle::Primary => colors::PRIMARY_PRESSED,
            ButtonStyle::Secondary => colors::SECONDARY_PRESSED,
            ButtonStyle::Danger => colors::DANGER_PRESSED,
            ButtonStyle::Success => colors::SUCCESS_PRESSED,
            ButtonStyle::Warning => colors::WARNING_PRESSED,
            ButtonStyle::Ghost => colors::GHOST_PRESSED,
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            ButtonStyle::Ghost => colors::TEXT_SECONDARY,
            _ => colors::TEXT_PRIMARY,
        }
    }

    pub fn border_color(&self) -> Color {
        match self {
            ButtonStyle::Primary => colors::PRIMARY.lighter(0.2),
            ButtonStyle::Secondary => colors::BORDER_DEFAULT,
            ButtonStyle::Danger => colors::DANGER.lighter(0.2),
            ButtonStyle::Success => colors::SUCCESS.lighter(0.2),
            ButtonStyle::Warning => colors::WARNING.lighter(0.2),
            ButtonStyle::Ghost => colors::BORDER_DEFAULT,
        }
    }
}

impl ButtonSize {
    pub fn width(&self) -> f32 {
        match self {
            ButtonSize::Small => dimensions::BUTTON_WIDTH_SMALL,
            ButtonSize::Medium => dimensions::BUTTON_WIDTH_MEDIUM,
            ButtonSize::Large => dimensions::BUTTON_WIDTH_LARGE,
            ButtonSize::XLarge => dimensions::BUTTON_WIDTH_XLARGE,
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            ButtonSize::Small => dimensions::BUTTON_HEIGHT_SMALL,
            ButtonSize::Medium => dimensions::BUTTON_HEIGHT,
            ButtonSize::Large => dimensions::BUTTON_HEIGHT_LARGE,
            ButtonSize::XLarge => dimensions::BUTTON_HEIGHT_LARGE,
        }
    }

    pub fn font_size(&self) -> f32 {
        match self {
            ButtonSize::Small => dimensions::FONT_SIZE_SMALL,
            ButtonSize::Medium => dimensions::FONT_SIZE_MEDIUM,
            ButtonSize::Large => dimensions::FONT_SIZE_LARGE,
            ButtonSize::XLarge => dimensions::FONT_SIZE_LARGE,
        }
    }
}

/// Builder for creating styled buttons
pub struct ButtonBuilder {
    text: String,
    style: ButtonStyle,
    size: ButtonSize,
    enabled: bool,
    margin: Option<UiRect>,
    marker: Option<Box<dyn FnOnce(&mut EntityCommands)>>,
    custom_width: Option<Val>,
}

impl ButtonBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: ButtonStyle::Secondary,
            size: ButtonSize::Medium,
            enabled: true,
            margin: None,
            marker: None,
            custom_width: None,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Set whether the button is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.custom_width = Some(width);
        self
    }

    pub fn height(mut self, height: Val) -> Self {
        // Note: height is currently derived from size, but we can add support if needed
        self
    }

    pub fn build_with_children<F>(self, parent: &mut ChildBuilder, children: F) -> Entity
    where
        F: FnOnce(&mut ChildBuilder),
    {
        let entity = self.build(parent);
        // Get the entity's commands and add children
        parent.commands().entity(entity).with_children(children);
        entity
    }

    pub fn with_marker<M: Component>(mut self, marker: M) -> Self {
        self.marker = Some(Box::new(move |entity: &mut EntityCommands| {
            entity.insert(marker);
        }));
        self
    }

    pub fn build(self, parent: &mut ChildBuilder) -> Entity {
        // If we have a custom marker, we need to handle it differently
        if let Some(marker_fn) = self.marker {
            // We have to build manually when using custom markers
            // because we can't access the entity commands after spawning
            let base_color = if self.enabled {
                self.style.base_color()
            } else {
                colors::DISABLED
            };

            let text_color = if self.enabled {
                self.style.text_color()
            } else {
                colors::TEXT_MUTED
            };

            let border_color = if self.enabled {
                self.style.border_color()
            } else {
                colors::DISABLED_HOVER
            };

            let mut entity_commands = parent.spawn((
                Button,
                StyledButton {
                    style: self.style,
                    size: self.size,
                    enabled: self.enabled,
                },
                Node {
                    width: self.custom_width.unwrap_or(Val::Px(self.size.width())),
                    height: Val::Px(self.size.height()),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: helpers::standard_border(),
                    margin: self.margin.unwrap_or_default(),
                    ..default()
                },
                BackgroundColor(base_color),
                BorderColor(border_color),
            ));

            // Add custom marker
            marker_fn(&mut entity_commands);

            let entity = entity_commands.id();

            // Add text child
            entity_commands.with_children(|button| {
                button.spawn((
                    Text::new(self.text),
                    TextFont {
                        font_size: self.size.font_size(),
                        ..default()
                    },
                    TextColor(text_color),
                ));
            });

            entity
        } else {
            // No custom marker, spawn button directly
            let (base_color, border_color, text_color) = if self.enabled {
                (
                    self.style.base_color(),
                    self.style.border_color(),
                    Color::WHITE,
                )
            } else {
                (
                    colors::BORDER_DEFAULT,
                    colors::DISABLED_HOVER,
                    colors::BORDER_ACTIVE,
                )
            };

            let entity = parent
                .spawn((
                    Button,
                    StyledButton {
                        style: self.style,
                        size: self.size,
                        enabled: self.enabled,
                    },
                    Node {
                        width: self.custom_width.unwrap_or(Val::Px(self.size.width())),
                        height: Val::Px(self.size.height()),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: helpers::standard_border(),
                        margin: self.margin.unwrap_or_default(),
                        ..default()
                    },
                    BackgroundColor(base_color),
                    BorderColor(border_color),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(self.text),
                        TextFont {
                            font_size: self.size.font_size(),
                            ..default()
                        },
                        TextColor(text_color),
                    ));
                })
                .id();

            entity
        }
    }

    /// Build button with spawner (for use in macro contexts)
    pub fn build_in(self, parent: &mut ChildBuilder) -> Entity {
        // Similar implementation but adapted for ChildSpawner
        let base_color = if self.enabled {
            self.style.base_color()
        } else {
            colors::DISABLED
        };

        let text_color = if self.enabled {
            self.style.text_color()
        } else {
            colors::TEXT_DISABLED
        };

        let width = self
            .custom_width
            .unwrap_or_else(|| Val::Px(self.size.width()));
        let height = Val::Px(self.size.height());

        let mut entity_commands = parent.spawn((
            Button,
            Node {
                width,
                height,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: self.margin.unwrap_or_default(),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(base_color),
            BorderColor(self.style.border_color()),
            StyledButton {
                style: self.style,
                size: self.size,
                enabled: self.enabled,
            },
        ));

        // Apply marker if provided
        if let Some(marker_fn) = self.marker {
            marker_fn(&mut entity_commands);
        }

        let entity = entity_commands
            .with_children(|button| {
                button.spawn((
                    Text::new(self.text),
                    TextFont {
                        font_size: self.size.font_size(),
                        ..default()
                    },
                    TextColor(text_color),
                ));
            })
            .id();

        entity
    }
}

/// Helper functions for creating common button types
pub mod presets {
    use super::*;

    pub fn primary_button(text: impl Into<String>) -> ButtonBuilder {
        ButtonBuilder::new(text).style(ButtonStyle::Primary)
    }

    pub fn secondary_button(text: impl Into<String>) -> ButtonBuilder {
        ButtonBuilder::new(text).style(ButtonStyle::Secondary)
    }

    pub fn danger_button(text: impl Into<String>) -> ButtonBuilder {
        ButtonBuilder::new(text).style(ButtonStyle::Danger)
    }

    pub fn success_button(text: impl Into<String>) -> ButtonBuilder {
        ButtonBuilder::new(text).style(ButtonStyle::Success)
    }

    pub fn warning_button(text: impl Into<String>) -> ButtonBuilder {
        ButtonBuilder::new(text).style(ButtonStyle::Warning)
    }

    pub fn ghost_button(text: impl Into<String>) -> ButtonBuilder {
        ButtonBuilder::new(text).style(ButtonStyle::Ghost)
    }
}

/// Universal hover system for styled buttons
pub fn styled_button_hover_system(
    mut interactions: Query<
        (&Interaction, &StyledButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button, mut bg_color) in &mut interactions {
        if !button.enabled {
            continue; // Skip disabled buttons
        }

        *bg_color = BackgroundColor(match interaction {
            Interaction::Hovered => button.style.hover_color(),
            Interaction::Pressed => button.style.pressed_color(),
            Interaction::None => button.style.base_color(),
        });
    }
}

/// Plugin for the button system
/// Button plugin using MINIMAL AUTOMATION!
///
/// **AUTOMATION ACHIEVEMENT**: 6 lines manual → 3 lines declarative!
use bevy_plugin_builder::define_plugin;

define_plugin!(ButtonPlugin {
    update: [styled_button_hover_system]
});
