//! Common UI components for Living Worlds
//! 
//! Provides reusable UI components with consistent styling
//! that can be used throughout the game interface.

use bevy::prelude::*;
use super::styles::{colors, dimensions, helpers};

/// Component for panels/containers
#[derive(Component, Debug)]
pub struct Panel {
    pub style: PanelStyle,
}

/// Panel style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelStyle {
    Default,      // Standard panel
    Elevated,     // With shadow/depth
    Transparent,  // No background
    Dark,         // Dark background
    Light,        // Light background
    Bordered,     // With visible border
}

impl PanelStyle {
    /// Get the background color for this panel style
    pub fn background_color(&self) -> Color {
        match self {
            PanelStyle::Default => colors::BACKGROUND_MEDIUM,
            PanelStyle::Elevated => colors::BACKGROUND_LIGHT,
            PanelStyle::Transparent => Color::NONE,
            PanelStyle::Dark => colors::BACKGROUND_DARK,
            PanelStyle::Light => colors::BACKGROUND_LIGHT,
            PanelStyle::Bordered => colors::BACKGROUND_MEDIUM,
        }
    }
    
    /// Get the border color for this panel style
    pub fn border_color(&self) -> Color {
        match self {
            PanelStyle::Bordered => colors::BORDER_DEFAULT,
            _ => Color::NONE,
        }
    }
}

/// Component for text labels
#[derive(Component, Debug)]
pub struct Label {
    pub style: LabelStyle,
}

/// Label style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelStyle {
    Title,       // Large title text
    Heading,     // Section heading
    Body,        // Normal body text
    Caption,     // Small caption text
    Muted,       // De-emphasized text
    Error,       // Error message text
    Success,     // Success message text
}

impl LabelStyle {
    /// Get the font size for this label style
    pub fn font_size(&self) -> f32 {
        match self {
            LabelStyle::Title => dimensions::FONT_SIZE_TITLE,
            LabelStyle::Heading => dimensions::FONT_SIZE_LARGE,
            LabelStyle::Body => dimensions::FONT_SIZE_NORMAL,
            LabelStyle::Caption => dimensions::FONT_SIZE_SMALL,
            LabelStyle::Muted => dimensions::FONT_SIZE_SMALL,
            LabelStyle::Error => dimensions::FONT_SIZE_NORMAL,
            LabelStyle::Success => dimensions::FONT_SIZE_NORMAL,
        }
    }
    
    /// Get the text color for this label style
    pub fn text_color(&self) -> Color {
        match self {
            LabelStyle::Title => colors::TEXT_TITLE,
            LabelStyle::Heading => colors::TEXT_PRIMARY,
            LabelStyle::Body => colors::TEXT_SECONDARY,
            LabelStyle::Caption => colors::TEXT_MUTED,
            LabelStyle::Muted => colors::TEXT_MUTED,
            LabelStyle::Error => colors::DANGER,
            LabelStyle::Success => colors::SUCCESS,
        }
    }
}

/// Component for separators/dividers
#[derive(Component, Debug)]
pub struct Separator {
    pub orientation: Orientation,
    pub style: SeparatorStyle,
}

/// Orientation for separators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Separator style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeparatorStyle {
    Solid,
    Dashed,
    Dotted,
    Faint,
}

impl SeparatorStyle {
    /// Get the color for this separator style
    pub fn color(&self) -> Color {
        match self {
            SeparatorStyle::Solid => colors::BORDER_DEFAULT,
            SeparatorStyle::Dashed => colors::BORDER_DEFAULT,
            SeparatorStyle::Dotted => colors::BORDER_DEFAULT,
            SeparatorStyle::Faint => Color::srgba(1.0, 1.0, 1.0, 0.1),
        }
    }
    
    /// Get the thickness for this separator style
    pub fn thickness(&self) -> f32 {
        match self {
            SeparatorStyle::Solid => dimensions::BORDER_WIDTH_THIN,
            SeparatorStyle::Dashed => dimensions::BORDER_WIDTH_THIN,
            SeparatorStyle::Dotted => dimensions::BORDER_WIDTH_THIN,
            SeparatorStyle::Faint => dimensions::BORDER_WIDTH_THIN,
        }
    }
}

/// Component for progress bars
#[derive(Component, Debug)]
pub struct ProgressBar {
    pub value: f32,      // 0.0 to 1.0
    pub style: ProgressBarStyle,
}

/// Progress bar style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressBarStyle {
    Default,
    Success,
    Warning,
    Danger,
}

impl ProgressBarStyle {
    /// Get the fill color for this progress bar style
    pub fn fill_color(&self) -> Color {
        match self {
            ProgressBarStyle::Default => colors::PRIMARY,
            ProgressBarStyle::Success => colors::SUCCESS,
            ProgressBarStyle::Warning => colors::WARNING,
            ProgressBarStyle::Danger => colors::DANGER,
        }
    }
    
    /// Get the background color for this progress bar style
    pub fn background_color(&self) -> Color {
        colors::BACKGROUND_DARK
    }
}

/// Builder for creating panels
pub struct PanelBuilder<'a> {
    parent: &'a mut ChildSpawnerCommands<'a>,
    style: PanelStyle,
    width: Val,
    height: Val,
    padding: UiRect,
    margin: UiRect,
}

impl<'a> PanelBuilder<'a> {
    /// Create a new panel builder
    pub fn new(parent: &'a mut ChildSpawnerCommands<'a>) -> Self {
        Self {
            parent,
            style: PanelStyle::Default,
            width: Val::Auto,
            height: Val::Auto,
            padding: helpers::standard_padding(),
            margin: UiRect::ZERO,
        }
    }
    
    /// Set the panel style
    pub fn style(mut self, style: PanelStyle) -> Self {
        self.style = style;
        self
    }
    
    /// Set the panel width
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }
    
    /// Set the panel height
    pub fn height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }
    
    /// Set the panel padding
    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }
    
    /// Set the panel margin
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }
    
    /// Build the panel with children
    pub fn build_with_children<F>(self, children_fn: F) -> Entity
    where
        F: FnOnce(&mut ChildSpawnerCommands),
    {
        let border = if self.style == PanelStyle::Bordered {
            helpers::standard_border()
        } else {
            UiRect::ZERO
        };
        
        self.parent.spawn((
            Panel { style: self.style },
            Node {
                width: self.width,
                height: self.height,
                padding: self.padding,
                margin: self.margin,
                border,
                ..default()
            },
            BackgroundColor(self.style.background_color()),
            BorderColor(self.style.border_color()),
        ))
        .with_children(children_fn)
        .id()
    }
    
    /// Build the panel without children
    pub fn build(self) -> Entity {
        self.build_with_children(|_| {})
    }
}

/// Builder for creating labels
pub struct LabelBuilder<'a> {
    parent: &'a mut ChildSpawnerCommands<'a>,
    text: String,
    style: LabelStyle,
    margin: UiRect,
}

impl<'a> LabelBuilder<'a> {
    /// Create a new label builder
    pub fn new(parent: &'a mut ChildSpawnerCommands<'a>, text: impl Into<String>) -> Self {
        Self {
            parent,
            text: text.into(),
            style: LabelStyle::Body,
            margin: UiRect::ZERO,
        }
    }
    
    /// Set the label style
    pub fn style(mut self, style: LabelStyle) -> Self {
        self.style = style;
        self
    }
    
    /// Set the label margin
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }
    
    /// Build the label
    pub fn build(self) -> Entity {
        self.parent.spawn((
            Label { style: self.style },
            Node {
                margin: self.margin,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new(self.text),
                TextFont {
                    font_size: self.style.font_size(),
                    ..default()
                },
                TextColor(self.style.text_color()),
            ));
        })
        .id()
    }
}

/// Builder for creating separators
pub struct SeparatorBuilder<'a> {
    parent: &'a mut ChildSpawnerCommands<'a>,
    orientation: Orientation,
    style: SeparatorStyle,
    margin: UiRect,
}

impl<'a> SeparatorBuilder<'a> {
    /// Create a new separator builder
    pub fn new(parent: &'a mut ChildSpawnerCommands<'a>) -> Self {
        Self {
            parent,
            orientation: Orientation::Horizontal,
            style: SeparatorStyle::Solid,
            margin: UiRect::vertical(Val::Px(dimensions::MARGIN_SMALL)),
        }
    }
    
    /// Set the separator orientation
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }
    
    /// Set the separator style
    pub fn style(mut self, style: SeparatorStyle) -> Self {
        self.style = style;
        self
    }
    
    /// Set the separator margin
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }
    
    /// Build the separator
    pub fn build(self) -> Entity {
        let (width, height) = match self.orientation {
            Orientation::Horizontal => (Val::Percent(100.0), Val::Px(self.style.thickness())),
            Orientation::Vertical => (Val::Px(self.style.thickness()), Val::Percent(100.0)),
        };
        
        self.parent.spawn((
            Separator {
                orientation: self.orientation,
                style: self.style,
            },
            Node {
                width,
                height,
                margin: self.margin,
                ..default()
            },
            BackgroundColor(self.style.color()),
        ))
        .id()
    }
}

/// Helper functions for creating common components
pub mod presets {
    use super::*;
    
    /// Create a title label
    pub fn title_label<'a>(parent: &'a mut ChildSpawnerCommands<'a>, text: impl Into<String>) -> Entity {
        LabelBuilder::new(parent, text)
            .style(LabelStyle::Title)
            .margin(UiRect::bottom(Val::Px(dimensions::MARGIN_LARGE)))
            .build()
    }
    
    /// Create a heading label
    pub fn heading_label<'a>(parent: &'a mut ChildSpawnerCommands<'a>, text: impl Into<String>) -> Entity {
        LabelBuilder::new(parent, text)
            .style(LabelStyle::Heading)
            .margin(UiRect::bottom(Val::Px(dimensions::MARGIN_MEDIUM)))
            .build()
    }
    
    /// Create a body text label
    pub fn body_label<'a>(parent: &'a mut ChildSpawnerCommands<'a>, text: impl Into<String>) -> Entity {
        LabelBuilder::new(parent, text)
            .style(LabelStyle::Body)
            .build()
    }
    
    /// Create a horizontal separator
    pub fn horizontal_separator<'a>(parent: &'a mut ChildSpawnerCommands<'a>) -> Entity {
        SeparatorBuilder::new(parent)
            .orientation(Orientation::Horizontal)
            .style(SeparatorStyle::Solid)
            .build()
    }
    
    /// Create a faint horizontal separator
    pub fn faint_separator<'a>(parent: &'a mut ChildSpawnerCommands<'a>) -> Entity {
        SeparatorBuilder::new(parent)
            .orientation(Orientation::Horizontal)
            .style(SeparatorStyle::Faint)
            .build()
    }
    
    /// Create a dark panel
    pub fn dark_panel<'a>(parent: &'a mut ChildSpawnerCommands<'a>) -> PanelBuilder<'a> {
        PanelBuilder::new(parent).style(PanelStyle::Dark)
    }
    
    /// Create a bordered panel
    pub fn bordered_panel<'a>(parent: &'a mut ChildSpawnerCommands<'a>) -> PanelBuilder<'a> {
        PanelBuilder::new(parent).style(PanelStyle::Bordered)
    }
}