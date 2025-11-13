//! Toolbar builder system for creating consistent toolbars
//!
//! ToolbarBuilder provides a flexible API for creating horizontal and vertical
//! toolbars with buttons, separators, and custom controls.

#![allow(dead_code)] // Preserve UI utility functions for future use

use crate::ui::{ChildBuilder, colors, dimensions};
use crate::ui::{ButtonBuilder, ButtonSize, ButtonStyle};
use bevy::prelude::*;

/// Marker component for toolbars
#[derive(Component)]
pub struct Toolbar {
    pub orientation: ToolbarOrientation,
    pub style: ToolbarStyle,
}

/// Toolbar orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarOrientation {
    Horizontal,
    Vertical,
}

/// Toolbar style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarStyle {
    Default,
    Compact,
    Floating,
    Embedded,
}

impl ToolbarStyle {
    pub fn background_color(&self) -> Color {
        match self {
            ToolbarStyle::Default => colors::BACKGROUND_MEDIUM,
            ToolbarStyle::Compact => colors::BACKGROUND_LIGHT,
            ToolbarStyle::Floating => colors::BACKGROUND_DARK.with_alpha(0.95),
            ToolbarStyle::Embedded => Color::NONE,
        }
    }

    pub fn padding(&self) -> UiRect {
        match self {
            ToolbarStyle::Default => UiRect::all(Val::Px(dimensions::PADDING_MEDIUM)),
            ToolbarStyle::Compact => UiRect::all(Val::Px(dimensions::PADDING_SMALL)),
            ToolbarStyle::Floating => UiRect::all(Val::Px(dimensions::PADDING_LARGE)),
            ToolbarStyle::Embedded => UiRect::ZERO,
        }
    }
}

/// Builder for creating toolbars
pub struct ToolbarBuilder {
    orientation: ToolbarOrientation,
    style: ToolbarStyle,
    width: Val,
    height: Val,
    gap: Val,
    items: Vec<ToolbarItem>,
}

/// Represents an item in the toolbar
enum ToolbarItem {
    Button {
        text: String,
        style: ButtonStyle,
        size: ButtonSize,
        marker: Option<Box<dyn std::any::Any + Send + Sync>>,
    },
    Separator,
    Spacer,
    Custom {
        builder_fn: Box<dyn FnOnce(&mut ChildBuilder)>,
    },
}

impl ToolbarBuilder {
    pub fn new() -> Self {
        Self {
            orientation: ToolbarOrientation::Horizontal,
            style: ToolbarStyle::Default,
            width: Val::Auto,
            height: Val::Auto,
            gap: Val::Px(dimensions::MARGIN_SMALL),
            items: Vec::new(),
        }
    }

    pub fn orientation(mut self, orientation: ToolbarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set horizontal orientation (convenience method)
    pub fn horizontal(mut self) -> Self {
        self.orientation = ToolbarOrientation::Horizontal;
        self
    }

    /// Set vertical orientation (convenience method)
    pub fn vertical(mut self) -> Self {
        self.orientation = ToolbarOrientation::Vertical;
        self
    }

    pub fn style(mut self, style: ToolbarStyle) -> Self {
        self.style = style;
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }

    pub fn gap(mut self, gap: Val) -> Self {
        self.gap = gap;
        self
    }

    pub fn button(mut self, text: impl Into<String>) -> Self {
        self.items.push(ToolbarItem::Button {
            text: text.into(),
            style: ButtonStyle::Secondary,
            size: ButtonSize::Small,
            marker: None,
        });
        self
    }

    pub fn primary_button(mut self, text: impl Into<String>) -> Self {
        self.items.push(ToolbarItem::Button {
            text: text.into(),
            style: ButtonStyle::Primary,
            size: ButtonSize::Small,
            marker: None,
        });
        self
    }

    pub fn ghost_button(mut self, text: impl Into<String>) -> Self {
        self.items.push(ToolbarItem::Button {
            text: text.into(),
            style: ButtonStyle::Ghost,
            size: ButtonSize::Small,
            marker: None,
        });
        self
    }

    pub fn danger_button(mut self, text: impl Into<String>) -> Self {
        self.items.push(ToolbarItem::Button {
            text: text.into(),
            style: ButtonStyle::Danger,
            size: ButtonSize::Small,
            marker: None,
        });
        self
    }

    pub fn separator(mut self) -> Self {
        self.items.push(ToolbarItem::Separator);
        self
    }

    pub fn spacer(mut self) -> Self {
        self.items.push(ToolbarItem::Spacer);
        self
    }

    pub fn custom<F>(mut self, builder_fn: F) -> Self
    where
        F: FnOnce(&mut ChildBuilder) + 'static,
    {
        self.items.push(ToolbarItem::Custom {
            builder_fn: Box::new(builder_fn),
        });
        self
    }

    pub fn build(self, parent: &mut ChildBuilder) -> Entity {
        let (flex_direction, justify_content) = match self.orientation {
            ToolbarOrientation::Horizontal => (FlexDirection::Row, JustifyContent::Start),
            ToolbarOrientation::Vertical => (FlexDirection::Column, JustifyContent::Start),
        };

        let (column_gap, row_gap) = match self.orientation {
            ToolbarOrientation::Horizontal => (self.gap, Val::ZERO),
            ToolbarOrientation::Vertical => (Val::ZERO, self.gap),
        };

        parent
            .spawn((
                Toolbar {
                    orientation: self.orientation,
                    style: self.style,
                },
                Node {
                    width: self.width,
                    height: self.height,
                    flex_direction,
                    justify_content,
                    align_items: AlignItems::Center,
                    padding: self.style.padding(),
                    column_gap,
                    row_gap,
                    ..default()
                },
                BackgroundColor(self.style.background_color()),
            ))
            .with_children(|toolbar| {
                for item in self.items {
                    match item {
                        ToolbarItem::Button {
                            text, style, size, ..
                        } => {
                            ButtonBuilder::new(text)
                                .style(style)
                                .size(size)
                                .build(toolbar);
                        }

                        ToolbarItem::Separator => {
                            let (width, height) = match self.orientation {
                                ToolbarOrientation::Horizontal => (Val::Px(1.0), Val::Px(20.0)),
                                ToolbarOrientation::Vertical => (Val::Px(20.0), Val::Px(1.0)),
                            };

                            toolbar.spawn((
                                Node {
                                    width,
                                    height,
                                    margin: match self.orientation {
                                        ToolbarOrientation::Horizontal => {
                                            UiRect::horizontal(Val::Px(dimensions::MARGIN_SMALL))
                                        }
                                        ToolbarOrientation::Vertical => {
                                            UiRect::vertical(Val::Px(dimensions::MARGIN_SMALL))
                                        }
                                    },
                                    ..default()
                                },
                                BackgroundColor(colors::BORDER_DEFAULT),
                            ));
                        }

                        ToolbarItem::Spacer => {
                            toolbar.spawn((
                                Node {
                                    flex_grow: 1.0,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                            ));
                        }

                        ToolbarItem::Custom { builder_fn } => {
                            builder_fn(toolbar);
                        }
                    }
                }
            })
            .id()
    }
}

/// Convenience function to create a toolbar builder
pub fn toolbar() -> ToolbarBuilder {
    ToolbarBuilder::new()
}

mod presets {
    use super::*;

    pub fn editor_toolbar(parent: &mut ChildBuilder) -> Entity {
        // Manual construction due to lifetime constraints
        let style = ToolbarStyle::Default;
        parent
            .spawn((
                Toolbar {
                    orientation: ToolbarOrientation::Horizontal,
                    style,
                },
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    padding: style.padding(),
                    column_gap: Val::Px(dimensions::MARGIN_SMALL),
                    ..default()
                },
                BackgroundColor(style.background_color()),
            ))
            .with_children(|toolbar| {
                let spawn_separator = |parent: &mut ChildBuilder| {
                    parent.spawn((
                        Node {
                            width: Val::Px(1.0),
                            height: Val::Px(20.0),
                            margin: UiRect::horizontal(Val::Px(dimensions::MARGIN_SMALL)),
                            ..default()
                        },
                        BackgroundColor(colors::BORDER_DEFAULT),
                    ));
                };

                ButtonBuilder::new("New")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Small)
                    .build(toolbar);
                ButtonBuilder::new("Open")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Small)
                    .build(toolbar);
                ButtonBuilder::new("Save")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Small)
                    .build(toolbar);
                spawn_separator(toolbar);
                ButtonBuilder::new("Cut")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Small)
                    .build(toolbar);
                ButtonBuilder::new("Copy")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Small)
                    .build(toolbar);
                ButtonBuilder::new("Paste")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Small)
                    .build(toolbar);
                spawn_separator(toolbar);
                ButtonBuilder::new("Undo")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Small)
                    .build(toolbar);
                ButtonBuilder::new("Redo")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Small)
                    .build(toolbar);

                // Spacer
                toolbar.spawn((
                    Node {
                        flex_grow: 1.0,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                ));

                ButtonBuilder::new("Help")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Small)
                    .build(toolbar);
            })
            .id()
    }

    pub fn navigation_toolbar(parent: &mut ChildBuilder) -> Entity {
        let style = ToolbarStyle::Compact;
        parent
            .spawn((
                Toolbar {
                    orientation: ToolbarOrientation::Horizontal,
                    style,
                },
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    padding: style.padding(),
                    column_gap: Val::Px(dimensions::MARGIN_SMALL),
                    ..default()
                },
                BackgroundColor(style.background_color()),
            ))
            .with_children(|toolbar| {
                ButtonBuilder::new("←")
                    .style(ButtonStyle::Ghost)
                    .size(ButtonSize::Small)
                    .build(toolbar);
                ButtonBuilder::new("→")
                    .style(ButtonStyle::Ghost)
                    .size(ButtonSize::Small)
                    .build(toolbar);

                toolbar.spawn((
                    Node {
                        width: Val::Px(1.0),
                        height: Val::Px(20.0),
                        margin: UiRect::horizontal(Val::Px(dimensions::MARGIN_SMALL)),
                        ..default()
                    },
                    BackgroundColor(colors::BORDER_DEFAULT),
                ));
                ButtonBuilder::new("↑")
                    .style(ButtonStyle::Ghost)
                    .size(ButtonSize::Small)
                    .build(toolbar);
                ButtonBuilder::new("Home")
                    .style(ButtonStyle::Ghost)
                    .size(ButtonSize::Small)
                    .build(toolbar);

                toolbar.spawn((
                    Node {
                        flex_grow: 1.0,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                ));

                ButtonBuilder::new("⚙")
                    .style(ButtonStyle::Ghost)
                    .size(ButtonSize::Small)
                    .build(toolbar);
            })
            .id()
    }

    pub fn action_toolbar(parent: &mut ChildBuilder) -> Entity {
        let style = ToolbarStyle::Floating;
        parent
            .spawn((
                Toolbar {
                    orientation: ToolbarOrientation::Vertical,
                    style,
                },
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    padding: style.padding(),
                    row_gap: Val::Px(dimensions::MARGIN_SMALL),
                    ..default()
                },
                BackgroundColor(style.background_color()),
            ))
            .with_children(|toolbar| {
                ButtonBuilder::new("Play")
                    .style(ButtonStyle::Primary)
                    .size(ButtonSize::Medium)
                    .build(toolbar);
                ButtonBuilder::new("Pause")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Medium)
                    .build(toolbar);
                ButtonBuilder::new("Stop")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Medium)
                    .build(toolbar);

                toolbar.spawn((
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(1.0),
                        margin: UiRect::vertical(Val::Px(dimensions::MARGIN_SMALL)),
                        ..default()
                    },
                    BackgroundColor(colors::BORDER_DEFAULT),
                ));
                ButtonBuilder::new("Settings")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Medium)
                    .build(toolbar);

                toolbar.spawn((
                    Node {
                        flex_grow: 1.0,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                ));

                ButtonBuilder::new("Exit")
                    .style(ButtonStyle::Danger)
                    .size(ButtonSize::Medium)
                    .build(toolbar);
            })
            .id()
    }
}
