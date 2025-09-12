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

// ============================================================================
// PROGRESS BAR BUILDER
// ============================================================================

/// Marker for progress bar fill
#[derive(Component)]
pub struct ProgressBarFill;

/// Marker for progress bar background
#[derive(Component)]
pub struct ProgressBarTrack;

/// Marker for progress bar label
#[derive(Component)]
pub struct ProgressBarLabel;

/// Builder for creating progress bars
pub struct ProgressBarBuilder {
    value: f32,
    style: ProgressBarStyle,
    width: Val,
    height: Val,
    show_label: bool,
    label_format: fn(f32) -> String,
    animated: bool,
    border_radius: Val,
    margin: UiRect,
}

impl ProgressBarBuilder {
    /// Create a new progress bar builder with initial value (0.0 to 1.0)
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            style: ProgressBarStyle::Default,
            width: Val::Px(200.0),
            height: Val::Px(20.0),
            show_label: false,
            label_format: |v| format!("{}%", (v * 100.0) as i32),
            animated: true,
            border_radius: Val::Px(4.0),
            margin: UiRect::ZERO,
        }
    }
    
    /// Set the progress bar style
    pub fn style(mut self, style: ProgressBarStyle) -> Self {
        self.style = style;
        self
    }
    
    /// Set the width
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }
    
    /// Set the height
    pub fn height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }
    
    /// Show a label with the progress percentage
    pub fn with_label(mut self) -> Self {
        self.show_label = true;
        self
    }
    
    /// Set a custom label format function
    pub fn label_format(mut self, format: fn(f32) -> String) -> Self {
        self.label_format = format;
        self.show_label = true;
        self
    }
    
    /// Disable animation (progress changes instantly)
    pub fn no_animation(mut self) -> Self {
        self.animated = false;
        self
    }
    
    /// Set border radius
    pub fn border_radius(mut self, radius: Val) -> Self {
        self.border_radius = radius;
        self
    }
    
    /// Set margin
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }
    
    /// Build the progress bar
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        // Check if we can use the simple helper
        if !self.animated && self.border_radius == Val::Px(4.0) && self.margin == UiRect::ZERO {
            // Use helper for simple cases
            let label = if self.show_label {
                Some((self.label_format)(self.value))
            } else {
                None
            };
            spawn_progress_bar_full(
                parent,
                self.value,
                self.style,
                self.width,
                self.height,
                self.show_label,
                label,
            )
        } else {
            // Complex case with custom settings - build manually
            parent.spawn((
                Node {
                    width: self.width,
                    height: self.height,
                    position_type: PositionType::Relative,
                    margin: self.margin,
                    ..default()
                },
                BackgroundColor(self.style.background_color()),
                BorderRadius::all(self.border_radius),
                ProgressBar {
                    value: self.value,
                    style: self.style,
                },
                ProgressBarTrack,
            )).with_children(|track| {
                // Fill bar
                track.spawn((
                    Node {
                        width: Val::Percent(self.value * 100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    BackgroundColor(self.style.fill_color()),
                    BorderRadius::all(self.border_radius),
                    ProgressBarFill,
                ));
                
                // Optional label
                if self.show_label {
                    track.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    )).with_children(|label_container| {
                        label_container.spawn((
                            Text::new((self.label_format)(self.value)),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            ProgressBarLabel,
                        ));
                    });
                }
            }).id()
        }
    }
}

// ============================================================================
// PANEL BUILDER
// ============================================================================

/// Builder for creating panels
pub struct PanelBuilder<'a> {
    parent: &'a mut ChildSpawnerCommands<'a>,
    style: PanelStyle,
    width: Val,
    height: Val,
    padding: UiRect,
    margin: UiRect,
    title: Option<String>,
    title_style: LabelStyle,
    show_separator: bool,
    separator_style: SeparatorStyle,
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
            title: None,
            title_style: LabelStyle::Heading,
            show_separator: false,
            separator_style: SeparatorStyle::Faint,
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
    
    /// Add a title to the panel using LabelBuilder
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self.show_separator = true;  // Usually want separator after title
        self
    }
    
    /// Set the title style
    pub fn title_style(mut self, style: LabelStyle) -> Self {
        self.title_style = style;
        self
    }
    
    /// Add a separator after the title
    pub fn with_separator(mut self) -> Self {
        self.show_separator = true;
        self
    }
    
    /// Set the separator style
    pub fn separator_style(mut self, style: SeparatorStyle) -> Self {
        self.separator_style = style;
        self.show_separator = true;
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
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(self.style.background_color()),
            BorderColor(self.style.border_color()),
        ))
        .with_children(|panel| {
            // Add title if specified
            if let Some(title) = self.title {
                panel.spawn((
                    Text::new(title),
                    TextFont {
                        font_size: self.title_style.font_size(),
                        ..default()
                    },
                    TextColor(self.title_style.text_color()),
                    Node {
                        margin: UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)),
                        ..default()
                    },
                ));
            }
            
            // Add separator if specified
            if self.show_separator {
                panel.spawn((
                    Node {
                        height: Val::Px(self.separator_style.thickness()),
                        width: Val::Percent(100.0),
                        margin: UiRect::vertical(Val::Px(dimensions::MARGIN_SMALL)),
                        ..default()
                    },
                    BackgroundColor(self.separator_style.color()),
                ));
            }
            
            // Add user-provided children
            children_fn(panel);
        })
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
        // If we have custom margin, build manually
        if self.margin != UiRect::ZERO {
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
        } else {
            // Use the helper for simple case
            spawn_label(self.parent, self.text, self.style)
        }
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
        // If we have the default margin, use the helper
        if self.margin == UiRect::vertical(Val::Px(dimensions::MARGIN_SMALL)) {
            spawn_separator(self.parent, self.orientation, self.style)
        } else {
            // Custom margin - build manually
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

// ============================================================================
// STATIC HELPER FUNCTIONS (ONE-LINE SPAWNING)
// ============================================================================

/// Static helper to spawn a label without using the builder pattern
/// This avoids lifetime issues in preset functions and other contexts
pub fn spawn_label(
    parent: &mut ChildSpawnerCommands,
    text: impl Into<String>,
    style: LabelStyle,
) -> Entity {
    let text = text.into();
    parent.spawn((
        Text::new(text),
        TextFont {
            font_size: style.font_size(),
            ..default()
        },
        TextColor(style.text_color()),
        Label { style },
    )).id()
}

/// Static helper to spawn a separator without using the builder pattern
pub fn spawn_separator(
    parent: &mut ChildSpawnerCommands,
    orientation: Orientation,
    style: SeparatorStyle,
) -> Entity {
    let (width, height) = match orientation {
        Orientation::Horizontal => (Val::Percent(100.0), Val::Px(style.thickness())),
        Orientation::Vertical => (Val::Px(style.thickness()), Val::Percent(100.0)),
    };
    
    parent.spawn((
        Node {
            width,
            height,
            margin: UiRect::all(Val::Px(dimensions::MARGIN_SMALL)),
            ..default()
        },
        BackgroundColor(style.color()),
        Separator { orientation, style },
    )).id()
}

/// Static helper to spawn a panel without using the builder pattern
pub fn spawn_panel(
    parent: &mut ChildSpawnerCommands,
    style: PanelStyle,
) -> Entity {
    spawn_panel_full(
        parent,
        style,
        Val::Auto,
        Val::Auto,
        UiRect::all(Val::Px(dimensions::PADDING_MEDIUM)),
    )
}

/// Static helper to spawn a panel with all options
pub fn spawn_panel_full(
    parent: &mut ChildSpawnerCommands,
    style: PanelStyle,
    width: Val,
    height: Val,
    padding: UiRect,
) -> Entity {
    let mut node = Node {
        width,
        height,
        padding,
        flex_direction: FlexDirection::Column,
        ..default()
    };
    
    // Add border if needed
    if style == PanelStyle::Bordered {
        node.border = helpers::standard_border();
    }
    
    parent.spawn((
        node,
        BackgroundColor(style.background_color()),
        BorderColor(style.border_color()),
        Panel { style },
    )).id()
}

/// Static helper to spawn a progress bar without using the builder pattern
pub fn spawn_progress_bar(
    parent: &mut ChildSpawnerCommands,
    value: f32,
    style: ProgressBarStyle,
) -> Entity {
    spawn_progress_bar_full(
        parent,
        value,
        style,
        Val::Px(200.0),
        Val::Px(20.0),
        false,
        None,
    )
}

/// Static helper to spawn a progress bar with all options
pub fn spawn_progress_bar_full(
    parent: &mut ChildSpawnerCommands,
    value: f32,
    style: ProgressBarStyle,
    width: Val,
    height: Val,
    show_text: bool,
    label: Option<String>,
) -> Entity {
    let clamped_value = value.clamp(0.0, 1.0);
    
    // Container
    let container = parent.spawn((
        Node {
            width,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).with_children(|container| {
        // Optional label
        if let Some(label_text) = label {
            container.spawn((
                Text::new(label_text),
                TextFont {
                    font_size: dimensions::FONT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
            ));
        }
        
        // Progress bar track
        container.spawn((
            Node {
                width: Val::Percent(100.0),
                height,
                border: helpers::standard_border(),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(style.background_color()),
            BorderColor(colors::BORDER_DEFAULT),
            ProgressBar {
                value: clamped_value,
                style,
            },
        )).with_children(|track| {
            // Progress fill
            track.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(clamped_value * 100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(style.fill_color()),
                ProgressBarFill,
            ));
            
            // Optional text overlay
            if show_text {
                track.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                )).with_children(|overlay| {
                    overlay.spawn((
                        Text::new(format!("{:.0}%", clamped_value * 100.0)),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_SMALL,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                });
            }
        });
    }).id();
    
    container
}