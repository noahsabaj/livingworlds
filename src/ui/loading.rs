//! Loading indicators and spinners for UI
//!
//! This module provides animated loading indicators that can be used
//! throughout the application for async operations, loading screens,
//! and progress indication.

use bevy::prelude::*;
use super::{colors, dimensions};
use crate::math::fast_sin;


/// Component for loading indicators
#[derive(Component)]
pub struct LoadingIndicator {
    pub style: LoadingStyle,
    pub size: LoadingSize,
    pub animated: bool,
}

/// Component for rotating spinners
#[derive(Component)]
pub struct LoadingSpinner {
    pub speed: f32,  // Rotations per second
}

/// Component for pulsing indicators
#[derive(Component)]
pub struct LoadingPulse {
    pub speed: f32,  // Pulses per second
    pub intensity: f32,  // 0.0 to 1.0
}

/// Component for animated dots (...)
#[derive(Component)]
pub struct LoadingDots {
    pub max_dots: usize,
    pub current_dots: usize,
    pub timer: Timer,
}


/// Style of loading indicator
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadingStyle {
    Spinner,     // Rotating symbol
    Dots,        // Animated dots ...
    Pulse,       // Pulsing circle
    Bar,         // Indeterminate progress bar
}

/// Size of loading indicator
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadingSize {
    Small,   // 16px
    Medium,  // 32px
    Large,   // 48px
    XLarge,  // 64px
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
pub struct LoadingIndicatorBuilder<'a> {
    parent: &'a mut ChildSpawnerCommands<'a>,
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

impl<'a> LoadingIndicatorBuilder<'a> {
    pub fn new(parent: &'a mut ChildSpawnerCommands<'a>) -> Self {
        Self {
            parent,
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

    pub fn build(self) -> Entity {
        let container_size = self.size.container_size();
        let flex_direction = match self.label_position {
            LabelPosition::Below => FlexDirection::Column,
            LabelPosition::Above => FlexDirection::ColumnReverse,
            LabelPosition::Right => FlexDirection::Row,
            LabelPosition::Left => FlexDirection::RowReverse,
        };

        let container = self.parent.spawn((
            Node {
                flex_direction,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(8.0),
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        )).id();

        self.parent.entity(container).with_children(|parent| {
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

        container
    }
}


/// Spawn a rotating spinner
fn spawn_spinner(
    parent: &mut ChildSpawnerCommands,
    size: LoadingSize,
    color: Color,
    animated: bool,
) -> Entity {
    let entity = parent.spawn((
        Text::new("◈"),  // Unicode diamond symbol
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
    )).id();

    if animated {
        parent.entity(entity).insert(LoadingSpinner { speed: 1.0 });
    }

    entity
}

/// Spawn animated dots
fn spawn_dots(
    parent: &mut ChildSpawnerCommands,
    size: LoadingSize,
    color: Color,
    animated: bool,
) -> Entity {
    let entity = parent.spawn((
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
    )).id();

    if animated {
        parent.entity(entity).insert(LoadingDots {
            max_dots: 3,
            current_dots: 1,
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        });
    }

    entity
}

/// Spawn a pulsing indicator
fn spawn_pulse(
    parent: &mut ChildSpawnerCommands,
    size: LoadingSize,
    color: Color,
    animated: bool,
) -> Entity {
    let entity = parent.spawn((
        Text::new("●"),  // Unicode circle
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
    )).id();

    if animated {
        parent.entity(entity).insert(LoadingPulse {
            speed: 2.0,
            intensity: 0.5,
        });
    }

    entity
}

/// Spawn an indeterminate progress bar
fn spawn_indeterminate_bar(
    parent: &mut ChildSpawnerCommands,
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

    parent.spawn((
        Node {
            width: Val::Px(size.container_size() * 3.0),
            height: Val::Px(height),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(colors::BACKGROUND_DARKER),
        BorderColor(colors::BORDER_DEFAULT),
        LoadingIndicator {
            style: LoadingStyle::Bar,
            size,
            animated,
        },
    )).with_children(|bar| {
        // Animated fill that moves back and forth
        bar.spawn((
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(color),
        ));
    }).id()
}


/// System to animate rotating spinners
pub fn animate_loading_spinners(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &LoadingSpinner)>,
) {
    for (mut transform, spinner) in &mut query {
        let rotation = time.elapsed_secs() * spinner.speed * std::f32::consts::TAU;
        transform.rotation = Quat::from_rotation_z(rotation);
    }
}

/// System to animate pulsing indicators
pub fn animate_loading_pulses(
    time: Res<Time>,
    mut query: Query<(&mut TextColor, &LoadingPulse)>,
) {
    for (mut text_color, pulse) in &mut query {
        let alpha = fast_sin(time.elapsed_secs() * pulse.speed * std::f32::consts::TAU);
        let intensity = pulse.intensity;

        // Pulse between full opacity and reduced opacity
        let current_alpha = 1.0 - (alpha * 0.5 + 0.5) * intensity;
        text_color.0.set_alpha(current_alpha);
    }
}

/// System to animate dots
pub fn animate_loading_dots(
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut LoadingDots)>,
) {
    for (mut text, mut dots) in &mut query {
        dots.timer.tick(time.delta());

        if dots.timer.finished() {
            dots.current_dots = (dots.current_dots % dots.max_dots) + 1;

            let dot_string = "•".repeat(dots.current_dots);
            text.0 = dot_string;
        }
    }
}


/// Plugin that manages loading indicator animations
pub struct LoadingIndicatorPlugin;

impl Plugin for LoadingIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                animate_loading_spinners,
                animate_loading_pulses,
                animate_loading_dots,
            ));
    }
}


/// Quick spinner creation
pub fn loading_spinner<'a>(parent: &'a mut ChildSpawnerCommands<'a>) -> LoadingIndicatorBuilder<'a> {
    LoadingIndicatorBuilder::new(parent).style(LoadingStyle::Spinner)
}

/// Quick dots creation
pub fn loading_dots<'a>(parent: &'a mut ChildSpawnerCommands<'a>) -> LoadingIndicatorBuilder<'a> {
    LoadingIndicatorBuilder::new(parent).style(LoadingStyle::Dots)
}

/// Quick pulse creation
pub fn loading_pulse<'a>(parent: &'a mut ChildSpawnerCommands<'a>) -> LoadingIndicatorBuilder<'a> {
    LoadingIndicatorBuilder::new(parent).style(LoadingStyle::Pulse)
}