//! Toolbar builder system for creating consistent toolbars
//! 
//! ToolbarBuilder provides a flexible API for creating horizontal and vertical
//! toolbars with buttons, separators, and custom controls.

use bevy::prelude::*;
use super::styles::{colors, dimensions};
use super::buttons::{ButtonBuilder, ButtonStyle, ButtonSize};

// ============================================================================
// COMPONENTS
// ============================================================================

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
    /// Get the background color for this toolbar style
    pub fn background_color(&self) -> Color {
        match self {
            ToolbarStyle::Default => colors::BACKGROUND_MEDIUM,
            ToolbarStyle::Compact => colors::BACKGROUND_LIGHT,
            ToolbarStyle::Floating => colors::BACKGROUND_DARK.with_alpha(0.95),
            ToolbarStyle::Embedded => Color::NONE,
        }
    }
    
    /// Get the padding for this toolbar style
    pub fn padding(&self) -> UiRect {
        match self {
            ToolbarStyle::Default => UiRect::all(Val::Px(dimensions::PADDING_MEDIUM)),
            ToolbarStyle::Compact => UiRect::all(Val::Px(dimensions::PADDING_SMALL)),
            ToolbarStyle::Floating => UiRect::all(Val::Px(dimensions::PADDING_LARGE)),
            ToolbarStyle::Embedded => UiRect::ZERO,
        }
    }
}

// ============================================================================
// TOOLBAR BUILDER
// ============================================================================

/// Builder for creating toolbars
pub struct ToolbarBuilder<'a> {
    parent: &'a mut ChildSpawnerCommands<'a>,
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
        builder_fn: Box<dyn FnOnce(&mut ChildSpawnerCommands)>,
    },
}

impl<'a> ToolbarBuilder<'a> {
    /// Create a new toolbar builder
    pub fn new(parent: &'a mut ChildSpawnerCommands<'a>) -> Self {
        Self {
            parent,
            orientation: ToolbarOrientation::Horizontal,
            style: ToolbarStyle::Default,
            width: Val::Auto,
            height: Val::Auto,
            gap: Val::Px(dimensions::MARGIN_SMALL),
            items: Vec::new(),
        }
    }
    
    /// Set the toolbar orientation
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
    
    /// Set the toolbar style
    pub fn style(mut self, style: ToolbarStyle) -> Self {
        self.style = style;
        self
    }
    
    /// Set the toolbar width
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }
    
    /// Set the toolbar height
    pub fn height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }
    
    /// Set the gap between items
    pub fn gap(mut self, gap: Val) -> Self {
        self.gap = gap;
        self
    }
    
    /// Add a button to the toolbar
    pub fn button(mut self, text: impl Into<String>) -> Self {
        self.items.push(ToolbarItem::Button {
            text: text.into(),
            style: ButtonStyle::Secondary,
            size: ButtonSize::Small,
            marker: None,
        });
        self
    }
    
    /// Add a primary button to the toolbar
    pub fn primary_button(mut self, text: impl Into<String>) -> Self {
        self.items.push(ToolbarItem::Button {
            text: text.into(),
            style: ButtonStyle::Primary,
            size: ButtonSize::Small,
            marker: None,
        });
        self
    }
    
    /// Add a ghost button to the toolbar
    pub fn ghost_button(mut self, text: impl Into<String>) -> Self {
        self.items.push(ToolbarItem::Button {
            text: text.into(),
            style: ButtonStyle::Ghost,
            size: ButtonSize::Small,
            marker: None,
        });
        self
    }
    
    /// Add a danger button to the toolbar
    pub fn danger_button(mut self, text: impl Into<String>) -> Self {
        self.items.push(ToolbarItem::Button {
            text: text.into(),
            style: ButtonStyle::Danger,
            size: ButtonSize::Small,
            marker: None,
        });
        self
    }
    
    /// Add a separator to the toolbar
    pub fn separator(mut self) -> Self {
        self.items.push(ToolbarItem::Separator);
        self
    }
    
    /// Add a spacer that pushes remaining items to the other side
    pub fn spacer(mut self) -> Self {
        self.items.push(ToolbarItem::Spacer);
        self
    }
    
    /// Add a custom item using a closure
    pub fn custom<F>(mut self, builder_fn: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands) + 'static,
    {
        self.items.push(ToolbarItem::Custom {
            builder_fn: Box::new(builder_fn),
        });
        self
    }
    
    /// Build the toolbar
    pub fn build(self) -> Entity {
        let (flex_direction, justify_content) = match self.orientation {
            ToolbarOrientation::Horizontal => (FlexDirection::Row, JustifyContent::Start),
            ToolbarOrientation::Vertical => (FlexDirection::Column, JustifyContent::Start),
        };
        
        let (column_gap, row_gap) = match self.orientation {
            ToolbarOrientation::Horizontal => (self.gap, Val::ZERO),
            ToolbarOrientation::Vertical => (Val::ZERO, self.gap),
        };
        
        self.parent.spawn((
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
        )).with_children(|toolbar| {
            for item in self.items {
                match item {
                    ToolbarItem::Button { text, style, size, .. } => {
                        ButtonBuilder::new(text)
                            .style(style)
                            .size(size)
                            .build(toolbar);
                    },
                    
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
                                    ToolbarOrientation::Horizontal => UiRect::horizontal(Val::Px(dimensions::MARGIN_SMALL)),
                                    ToolbarOrientation::Vertical => UiRect::vertical(Val::Px(dimensions::MARGIN_SMALL)),
                                },
                                ..default()
                            },
                            BackgroundColor(colors::BORDER_DEFAULT),
                        ));
                    },
                    
                    ToolbarItem::Spacer => {
                        toolbar.spawn((
                            Node {
                                flex_grow: 1.0,
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                        ));
                    },
                    
                    ToolbarItem::Custom { builder_fn } => {
                        builder_fn(toolbar);
                    },
                }
            }
        }).id()
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Convenience function to create a toolbar builder
pub fn toolbar<'a>(parent: &'a mut ChildSpawnerCommands<'a>) -> ToolbarBuilder<'a> {
    ToolbarBuilder::new(parent)
}

// ============================================================================
// PRESETS
// ============================================================================

pub mod presets {
    use super::*;
    
    /// Create a standard editor toolbar
    pub fn editor_toolbar(parent: &mut ChildSpawnerCommands) -> Entity {
        // Manual construction due to lifetime constraints
        let style = ToolbarStyle::Default;
        parent.spawn((
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
        )).with_children(|toolbar| {
            // Use our button helper functions from buttons module
            use crate::ui_toolbox::buttons::{spawn_button, ButtonStyle, ButtonSize};
            
            let spawn_separator = |parent: &mut ChildSpawnerCommands| {
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
            
            spawn_button(toolbar, "New", ButtonStyle::Secondary, ButtonSize::Small);
            spawn_button(toolbar, "Open", ButtonStyle::Secondary, ButtonSize::Small);
            spawn_button(toolbar, "Save", ButtonStyle::Secondary, ButtonSize::Small);
            spawn_separator(toolbar);
            spawn_button(toolbar, "Cut", ButtonStyle::Secondary, ButtonSize::Small);
            spawn_button(toolbar, "Copy", ButtonStyle::Secondary, ButtonSize::Small);
            spawn_button(toolbar, "Paste", ButtonStyle::Secondary, ButtonSize::Small);
            spawn_separator(toolbar);
            spawn_button(toolbar, "Undo", ButtonStyle::Secondary, ButtonSize::Small);
            spawn_button(toolbar, "Redo", ButtonStyle::Secondary, ButtonSize::Small);
            
            // Spacer
            toolbar.spawn((
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ));
            
            spawn_button(toolbar, "Help", ButtonStyle::Secondary, ButtonSize::Small);
        }).id()
    }
    
    /// Create a compact navigation toolbar
    pub fn navigation_toolbar(parent: &mut ChildSpawnerCommands) -> Entity {
        let style = ToolbarStyle::Compact;
        parent.spawn((
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
        )).with_children(|toolbar| {
            use crate::ui_toolbox::buttons::{spawn_button, ButtonStyle, ButtonSize};
            
            spawn_button(toolbar, "←", ButtonStyle::Ghost, ButtonSize::Small);
            spawn_button(toolbar, "→", ButtonStyle::Ghost, ButtonSize::Small);
            
            toolbar.spawn((
                Node {
                    width: Val::Px(1.0),
                    height: Val::Px(20.0),
                    margin: UiRect::horizontal(Val::Px(dimensions::MARGIN_SMALL)),
                    ..default()
                },
                BackgroundColor(colors::BORDER_DEFAULT),
            ));
            
            spawn_button(toolbar, "↑", ButtonStyle::Ghost, ButtonSize::Small);
            spawn_button(toolbar, "Home", ButtonStyle::Ghost, ButtonSize::Small);
            
            toolbar.spawn((
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ));
            
            spawn_button(toolbar, "⚙", ButtonStyle::Ghost, ButtonSize::Small);
        }).id()
    }
    
    /// Create a vertical action toolbar
    pub fn action_toolbar(parent: &mut ChildSpawnerCommands) -> Entity {
        let style = ToolbarStyle::Floating;
        parent.spawn((
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
        )).with_children(|toolbar| {
            use crate::ui_toolbox::buttons::{spawn_button, ButtonStyle, ButtonSize};
            
            spawn_button(toolbar, "Play", ButtonStyle::Primary, ButtonSize::Medium);
            spawn_button(toolbar, "Pause", ButtonStyle::Secondary, ButtonSize::Medium);
            spawn_button(toolbar, "Stop", ButtonStyle::Secondary, ButtonSize::Medium);
            
            toolbar.spawn((
                Node {
                    width: Val::Px(20.0),
                    height: Val::Px(1.0),
                    margin: UiRect::vertical(Val::Px(dimensions::MARGIN_SMALL)),
                    ..default()
                },
                BackgroundColor(colors::BORDER_DEFAULT),
            ));
            
            spawn_button(toolbar, "Settings", ButtonStyle::Secondary, ButtonSize::Medium);
            
            toolbar.spawn((
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ));
            
            spawn_button(toolbar, "Exit", ButtonStyle::Danger, ButtonSize::Medium);
        }).id()
    }
}