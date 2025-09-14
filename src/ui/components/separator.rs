//! Separator component for visual dividers

use bevy::prelude::*;
use super::super::{colors, dimensions};
use super::types::Orientation;

/// Component for separators/dividers
#[derive(Component, Debug)]
pub struct Separator {
    pub orientation: Orientation,
    pub style: SeparatorStyle,
}

/// Separator style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorStyle {
    #[default]
    Solid,       // Solid line
    Dashed,      // Dashed line (simulated with width)
    Dotted,      // Dotted line (simulated with width)
    Thick,       // Thicker line
    Thin,        // Thinner line
    Invisible,   // Spacing only, no visual
}

impl SeparatorStyle {
    pub fn color(&self) -> Color {
        match self {
            SeparatorStyle::Invisible => Color::NONE,
            _ => colors::BORDER_DEFAULT,
        }
    }

    pub fn thickness(&self) -> f32 {
        match self {
            SeparatorStyle::Thick => 3.0,
            SeparatorStyle::Thin => 0.5,
            SeparatorStyle::Invisible => 0.0,
            _ => 1.0,
        }
    }
}

/// Builder for creating separators with consistent styling
pub struct SeparatorBuilder {
    orientation: Orientation,
    style: SeparatorStyle,
    color: Option<Color>,
    thickness: Option<f32>,
    margin: UiRect,
    length: Val,
}

impl SeparatorBuilder {
    pub fn new() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            style: SeparatorStyle::Solid,
            color: None,
            thickness: None,
            margin: UiRect::vertical(Val::Px(dimensions::SEPARATOR_MARGIN)),
            length: Val::Percent(100.0),
        }
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn style(mut self, style: SeparatorStyle) -> Self {
        self.style = style;
        self
    }

    /// Override the color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Override the thickness
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = Some(thickness);
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    pub fn length(mut self, length: Val) -> Self {
        self.length = length;
        self
    }

    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let color = self.color.unwrap_or_else(|| self.style.color());
        let thickness = self.thickness.unwrap_or_else(|| self.style.thickness());

        let (width, height) = match self.orientation {
            Orientation::Horizontal => (self.length, Val::Px(thickness)),
            Orientation::Vertical => (Val::Px(thickness), self.length),
        };

        parent.spawn((
            Node {
                width,
                height,
                margin: self.margin,
                ..default()
            },
            BackgroundColor(color),
            Separator {
                orientation: self.orientation,
                style: self.style,
            },
        )).id()
    }
}