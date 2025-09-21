//! Label component for text display

use super::super::ChildBuilder;
use super::super::{colors, dimensions};
use bevy::prelude::*;

/// Component for text labels
#[derive(Component, Debug)]
pub struct Label {
    pub style: LabelStyle,
}

/// Label style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelStyle {
    Title,   // Large title text
    Heading, // Section heading
    #[default]
    Body, // Normal body text
    Caption, // Small caption text
    Muted,   // De-emphasized text
    Error,   // Error message text
    Success, // Success message text
    Warning, // Warning message text
}

impl LabelStyle {
    pub fn font_size(&self) -> f32 {
        match self {
            LabelStyle::Title => dimensions::FONT_SIZE_TITLE,
            LabelStyle::Heading => dimensions::FONT_SIZE_LARGE,
            LabelStyle::Body => dimensions::FONT_SIZE_NORMAL,
            LabelStyle::Caption => dimensions::FONT_SIZE_SMALL,
            LabelStyle::Muted => dimensions::FONT_SIZE_SMALL,
            LabelStyle::Error => dimensions::FONT_SIZE_NORMAL,
            LabelStyle::Success => dimensions::FONT_SIZE_NORMAL,
            LabelStyle::Warning => dimensions::FONT_SIZE_NORMAL,
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            LabelStyle::Title => colors::TEXT_TITLE,
            LabelStyle::Heading => colors::TEXT_PRIMARY,
            LabelStyle::Body => colors::TEXT_SECONDARY,
            LabelStyle::Caption => colors::TEXT_MUTED,
            LabelStyle::Muted => colors::TEXT_MUTED,
            LabelStyle::Error => colors::DANGER,
            LabelStyle::Success => colors::SUCCESS,
            LabelStyle::Warning => colors::WARNING,
        }
    }
}

/// Builder for creating labels with consistent styling
pub struct LabelBuilder {
    text: String,
    style: LabelStyle,
    font_size: Option<f32>,
    color: Option<Color>,
    margin: UiRect,
    text_align: JustifyContent,
}

impl LabelBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: LabelStyle::Body,
            font_size: None,
            color: None,
            margin: UiRect::all(Val::Px(0.0)),
            text_align: JustifyContent::Start,
        }
    }

    pub fn style(mut self, style: LabelStyle) -> Self {
        self.style = style;
        self
    }

    /// Override the font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// Override the text color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    /// Set text alignment
    pub fn text_align(mut self, align: JustifyContent) -> Self {
        self.text_align = align;
        self
    }

    pub fn build(self, parent: &mut ChildBuilder) -> Entity {
        let font_size = self.font_size.unwrap_or_else(|| self.style.font_size());
        let text_color = self.color.unwrap_or_else(|| self.style.text_color());

        parent
            .spawn((
                Node {
                    margin: self.margin,
                    justify_content: self.text_align,
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ))
            .with_children(|container| {
                // The actual text entity
                container.spawn((
                    Text::new(self.text.clone()),
                    TextFont {
                        font_size,
                        ..default()
                    },
                    TextColor(text_color),
                    Label { style: self.style },
                ));
            })
            .id()
    }
}
