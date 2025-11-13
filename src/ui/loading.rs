//! Loading indicators and spinners for UI
//!
//! This module provides animated loading indicators that can be used
//! throughout the application for async operations, loading screens,
//! and progress indication.

#![allow(dead_code)] // Preserve UI utility functions for future use

use super::{
    animation::{Animation, AnimationTarget, AnimationRepeatMode, EasingFunction},
    colors, dimensions, ChildBuilder,
};
use bevy::prelude::*;
use std::time::Duration;

/// Component for loading indicators
#[derive(Component)]
pub struct LoadingIndicator {
    pub style: LoadingStyle,
    pub size: LoadingSize,
    pub animated: bool,
}

// Animation components are now handled by the animation system
// No manual timer/speed tracking needed!

/// Style of loading indicator
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadingStyle {
    Spinner, // Rotating symbol
    Dots,    // Animated dots ...
    Pulse,   // Pulsing circle
    Bar,     // Indeterminate progress bar
}

/// Size of loading indicator
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadingSize {
    Small,  // 16px
    Medium, // 32px
    Large,  // 48px
    XLarge, // 64px
}

impl LoadingSize {
    pub fn font_size(&self) -> f32 {
        match self {
            LoadingSize::Small => 16.0,
            LoadingSize::Medium => 32.0,
            LoadingSize::Large => 48.0,
            LoadingSize::XLarge => 64.0,
        }
    }

    pub fn container_size(&self) -> f32 {
        match self {
            LoadingSize::Small => 20.0,
            LoadingSize::Medium => 40.0,
            LoadingSize::Large => 60.0,
            LoadingSize::XLarge => 80.0,
        }
    }
}

/// Builder for creating loading indicators
pub struct LoadingIndicatorBuilder {
    style: LoadingStyle,
    size: LoadingSize,
    color: Color,
    animated: bool,
    label: Option<String>,
    label_position: LabelPosition,
}

#[derive(Debug, Clone, Copy)]
pub enum LabelPosition {
    Below,
    Right,
    Above,
    Left,
}

impl LoadingIndicatorBuilder {
    pub fn new() -> Self {
        Self {
            style: LoadingStyle::Spinner,
            size: LoadingSize::Medium,
            color: colors::PRIMARY,
            animated: true,
            label: None,
            label_position: LabelPosition::Below,
        }
    }

    pub fn style(mut self, style: LoadingStyle) -> Self {
        self.style = style;
        self
    }

    pub fn size(mut self, size: LoadingSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Disable animation
    pub fn static_indicator(mut self) -> Self {
        self.animated = false;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set label position
    pub fn label_position(mut self, position: LabelPosition) -> Self {
        self.label_position = position;
        self
    }

    pub fn build(self, parent: &mut ChildBuilder) -> Entity {
        let _container_size = self.size.container_size();
        let flex_direction = match self.label_position {
            LabelPosition::Below => FlexDirection::Column,
            LabelPosition::Above => FlexDirection::ColumnReverse,
            LabelPosition::Right => FlexDirection::Row,
            LabelPosition::Left => FlexDirection::RowReverse,
        };

        let mut container = parent.spawn((
            Node {
                flex_direction,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(8.0),
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ));

        let container_id = container.id();

        container.with_children(|parent| {
            match self.style {
                LoadingStyle::Spinner => {
                    spawn_spinner(parent, self.size, self.color, self.animated);
                }
                LoadingStyle::Dots => {
                    spawn_dots(parent, self.size, self.color, self.animated);
                }
                LoadingStyle::Pulse => {
                    spawn_pulse(parent, self.size, self.color, self.animated);
                }
                LoadingStyle::Bar => {
                    spawn_indeterminate_bar(parent, self.size, self.color, self.animated);
                }
            }

            // Add label if specified
            if let Some(label_text) = self.label {
                parent.spawn((
                    Text::new(label_text),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_NORMAL,
                        ..default()
                    },
                    TextColor(colors::TEXT_SECONDARY),
                ));
            }
        });

        container_id
    }
}

/// Spawn a rotating spinner
fn spawn_spinner(
    parent: &mut ChildBuilder,
    size: LoadingSize,
    color: Color,
    animated: bool,
) -> Entity {
    let mut entity_commands = parent.spawn((
        Text::new("◈"), // Unicode diamond symbol
        TextFont {
            font_size: size.font_size(),
            ..default()
        },
        TextColor(color),
        Node {
            width: Val::Px(size.container_size()),
            height: Val::Px(size.container_size()),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        LoadingIndicator {
            style: LoadingStyle::Spinner,
            size,
            animated,
        },
    ));

    if animated {
        // Use our animation system for rotation
        entity_commands.insert(
            Animation::new(
                AnimationTarget::Rotation {
                    from: Quat::IDENTITY,
                    to: Quat::from_rotation_z(std::f32::consts::TAU)
                },
                Duration::from_secs(1)
            )
            .with_easing(EasingFunction::Linear)
            .with_repeat(AnimationRepeatMode::Loop),
        );
    }

    entity_commands.id()
}

/// Spawn animated dots
fn spawn_dots(
    parent: &mut ChildBuilder,
    size: LoadingSize,
    color: Color,
    animated: bool,
) -> Entity {
    let mut entity_commands = parent.spawn((
        Text::new("•"),
        TextFont {
            font_size: size.font_size(),
            ..default()
        },
        TextColor(color),
        Node {
            width: Val::Px(size.container_size()),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        LoadingIndicator {
            style: LoadingStyle::Dots,
            size,
            animated,
        },
    ));

    if animated {
        // For text changes, we'll use AnimationSequence (to be added later)
        // For now, use a scale animation to indicate loading
        entity_commands.insert(
            Animation::new(AnimationTarget::Opacity { from: 0.3, to: 1.0 }, Duration::from_millis(500))
                .with_easing(EasingFunction::Linear)
                .with_repeat(AnimationRepeatMode::PingPong),
        );
    }

    entity_commands.id()
}

/// Spawn a pulsing indicator
fn spawn_pulse(
    parent: &mut ChildBuilder,
    size: LoadingSize,
    color: Color,
    animated: bool,
) -> Entity {
    let mut entity_commands = parent.spawn((
        Text::new("●"), // Unicode circle
        TextFont {
            font_size: size.font_size(),
            ..default()
        },
        TextColor(color),
        Node {
            width: Val::Px(size.container_size()),
            height: Val::Px(size.container_size()),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        LoadingIndicator {
            style: LoadingStyle::Pulse,
            size,
            animated,
        },
    ));

    if animated {
        // Use our animation system for scale pulsing
        entity_commands.insert(
            Animation::new(
                AnimationTarget::Scale {
                    from: Vec3::splat(0.8),
                    to: Vec3::splat(1.2)
                },
                Duration::from_millis(500)
            )
            .with_easing(EasingFunction::EaseInOut)
            .with_repeat(AnimationRepeatMode::PingPong),
        );
    }

    entity_commands.id()
}

/// Spawn an indeterminate progress bar
fn spawn_indeterminate_bar(
    parent: &mut ChildBuilder,
    size: LoadingSize,
    color: Color,
    animated: bool,
) -> Entity {
    let height = match size {
        LoadingSize::Small => 4.0,
        LoadingSize::Medium => 6.0,
        LoadingSize::Large => 8.0,
        LoadingSize::XLarge => 10.0,
    };

    parent
        .spawn((
            Node {
                width: Val::Px(size.container_size() * 3.0),
                height: Val::Px(height),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_DARKER),
            BorderColor::all(colors::BORDER_DEFAULT),
            LoadingIndicator {
                style: LoadingStyle::Bar,
                size,
                animated,
            },
        ))
        .with_children(|bar| {
            // Animated fill that moves back and forth
            bar.spawn((
                Node {
                    width: Val::Percent(30.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(color),
            ));
        })
        .id()
}

// Spinner animation now handled by Animation component - no manual system needed!

// Pulse animation now handled by Animation component - no manual system needed!

// Dots animation can be handled with AnimationSequence for complex text changes

use bevy_plugin_builder::define_plugin;

/// Plugin that manages loading indicator animations
/// Note: Animation systems are now handled by the Animation plugin
define_plugin!(LoadingIndicatorPlugin {
    // Empty plugin - exists for API compatibility
    // Animations are handled by attaching Animation components directly
});

/// Quick spinner creation
pub fn loading_spinner() -> LoadingIndicatorBuilder {
    LoadingIndicatorBuilder::new().style(LoadingStyle::Spinner)
}

/// Quick dots creation
pub fn loading_dots() -> LoadingIndicatorBuilder {
    LoadingIndicatorBuilder::new().style(LoadingStyle::Dots)
}

/// Quick pulse creation
pub fn loading_pulse() -> LoadingIndicatorBuilder {
    LoadingIndicatorBuilder::new().style(LoadingStyle::Pulse)
}
