//! Progress bar component for showing progress and loading states

use super::super::ChildBuilder;
use super::super::{colors, dimensions};
use bevy::prelude::*;

/// Component for progress bars
#[derive(Component, Debug)]
pub struct ProgressBar {
    pub value: f32, // 0.0 to 1.0
    pub style: ProgressBarStyle,
    pub animated: bool,
}

/// Progress bar style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressBarStyle {
    #[default]
    Default, // Standard progress bar
    Thin,      // Thinner bar
    Thick,     // Thicker bar
    Segmented, // Segmented appearance
}

impl ProgressBarStyle {
    pub fn height(&self) -> f32 {
        match self {
            ProgressBarStyle::Thin => 4.0,
            ProgressBarStyle::Thick => 12.0,
            _ => 8.0,
        }
    }

    pub fn track_color(&self) -> Color {
        colors::BACKGROUND_DARK
    }

    pub fn fill_color(&self) -> Color {
        colors::PRIMARY
    }
}

/// Marker component for the fill portion of a progress bar
#[derive(Component)]
pub struct ProgressBarFill;

/// Marker component for the track/background of a progress bar
#[derive(Component)]
pub struct ProgressBarTrack;

/// Marker component for the progress bar label
#[derive(Component)]
pub struct ProgressBarLabel;

/// Builder for creating progress bars with consistent styling
pub struct ProgressBarBuilder {
    value: f32,
    style: ProgressBarStyle,
    width: Val,
    height: Option<f32>,
    margin: UiRect,
    track_color: Option<Color>,
    fill_color: Option<Color>,
    show_label: bool,
    custom_label: Option<String>,
    animated: bool,
}

impl ProgressBarBuilder {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            style: ProgressBarStyle::Default,
            width: Val::Percent(100.0),
            height: None,
            margin: UiRect::all(Val::Px(0.0)),
            track_color: None,
            fill_color: None,
            show_label: false,
            custom_label: None,
            animated: false,
        }
    }

    pub fn style(mut self, style: ProgressBarStyle) -> Self {
        self.style = style;
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Override the height
    pub fn height(mut self, height: Val) -> Self {
        if let Val::Px(px) = height {
            self.height = Some(px);
        }
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    /// Override the track color
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Override the fill color
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Show a percentage label
    pub fn with_label(mut self) -> Self {
        self.show_label = true;
        self
    }

    /// Show a custom label text instead of percentage
    pub fn with_label_text(mut self, text: impl Into<String>) -> Self {
        self.custom_label = Some(text.into());
        self.show_label = true;
        self
    }

    /// Enable animation
    pub fn animated(mut self) -> Self {
        self.animated = true;
        self
    }

    pub fn build(self, parent: &mut ChildBuilder) -> Entity {
        let height = Val::Px(self.height.unwrap_or_else(|| self.style.height()));
        let track_color = self.track_color.unwrap_or_else(|| self.style.track_color());
        let fill_color = self.fill_color.unwrap_or_else(|| self.style.fill_color());

        parent
            .spawn((
                Node {
                    width: self.width,
                    flex_direction: FlexDirection::Column,
                    margin: self.margin,
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ))
            .with_children(|container| {
                // Progress bar track (background)
                container
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height,
                            position_type: PositionType::Relative,
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        BackgroundColor(track_color),
                        BorderRadius::all(Val::Px(2.0)),
                        ProgressBarTrack,
                    ))
                    .with_children(|track| {
                        // Progress bar fill
                        track.spawn((
                            Node {
                                width: Val::Percent(self.value * 100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                top: Val::Px(0.0),
                                ..default()
                            },
                            BackgroundColor(fill_color),
                            BorderRadius::all(Val::Px(2.0)),
                            ProgressBarFill,
                        ));
                    });

                // Optional label
                if self.show_label {
                    let label_text = self
                        .custom_label
                        .unwrap_or_else(|| format!("{}%", (self.value * 100.0) as i32));

                    container
                        .spawn((
                            Node {
                                margin: UiRect::top(Val::Px(4.0)),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                        ))
                        .with_children(|label_container| {
                            label_container.spawn((
                                Text::new(label_text),
                                TextFont {
                                    font_size: dimensions::FONT_SIZE_SMALL,
                                    ..default()
                                },
                                TextColor(colors::TEXT_MUTED),
                                ProgressBarLabel,
                            ));
                        });
                }
            })
            .insert(ProgressBar {
                value: self.value,
                style: self.style,
                animated: self.animated,
            })
            .id()
    }
}

/// System to update progress bar fills when value changes
pub fn update_progress_bars(
    mut bars: Query<(Entity, &ProgressBar), Changed<ProgressBar>>,
    children_query: Query<&Children>,
    mut fills: Query<&mut Node, With<ProgressBarFill>>,
    mut labels: Query<&mut Text, With<ProgressBarLabel>>,
) {
    let bar_count = bars.iter().count();
    if bar_count > 0 {
        bevy::log::info!("Visual System: update_progress_bars found {} changed progress bars", bar_count);
    }

    for (entity, bar) in &mut bars {
        bevy::log::info!("Visual System: Updating progress bar visual to {:.1}%", bar.value * 100.0);

        // Use the recursive helper to find and update fills/labels
        find_and_update_fill(entity, bar.value, &children_query, &mut fills, &mut labels);
    }
}

/// Recursively find progress bar fill in children hierarchy
fn find_and_update_fill(
    entity: Entity,
    value: f32,
    children_query: &Query<&Children>,
    fills: &mut Query<&mut Node, With<ProgressBarFill>>,
    labels: &mut Query<&mut Text, With<ProgressBarLabel>>,
) {
    // Try to update this entity if it's a fill
    if let Ok(mut fill_node) = fills.get_mut(entity) {
        let new_width = Val::Percent(value * 100.0);
        if fill_node.width != new_width {
            bevy::log::info!("Force Update: Setting fill width from {:?} to {:.1}%",
                fill_node.width, value * 100.0);
            fill_node.width = new_width;
        }
    }

    // Try to update this entity if it's a label
    if let Ok(mut label_text) = labels.get_mut(entity) {
        let new_text = format!("{}%", (value * 100.0) as i32);
        if **label_text != new_text {
            **label_text = new_text;
        }
    }

    // Recursively check children
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            find_and_update_fill(child, value, children_query, fills, labels);
        }
    }
}

/// Force update all progress bars regardless of change detection (for debugging)
pub fn force_update_progress_bars(
    bars: Query<(Entity, &ProgressBar)>,
    children_query: Query<&Children>,
    mut fills: Query<&mut Node, With<ProgressBarFill>>,
    mut labels: Query<&mut Text, With<ProgressBarLabel>>,
) {
    for (entity, bar) in bars.iter() {
        // Recursively search for fill and label components in the hierarchy
        find_and_update_fill(entity, bar.value, &children_query, &mut fills, &mut labels);
    }
}

/// Plugin to add progress bar systems
pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_progress_bars, force_update_progress_bars));
    }
}
