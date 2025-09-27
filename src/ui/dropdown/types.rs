//! Core types for the dropdown system

use bevy::prelude::*;
use std::fmt::Debug;

/// Trait for values that can be used in a dropdown
pub trait DropdownValue: Clone + Debug + Send + Sync + 'static {
    /// Get display text for this value
    fn display_text(&self) -> String;

    /// Optional icon or color for this value
    fn display_color(&self) -> Option<Color> {
        None
    }

    /// Whether this value is selectable
    fn is_selectable(&self) -> bool {
        true
    }
}

// Implement for common types
impl DropdownValue for String {
    fn display_text(&self) -> String {
        self.clone()
    }
}

impl DropdownValue for &'static str {
    fn display_text(&self) -> String {
        self.to_string()
    }
}

impl<T: Debug + Clone + Send + Sync + 'static> DropdownValue for (String, T) {
    fn display_text(&self) -> String {
        self.0.clone()
    }
}

/// A single item in a dropdown
#[derive(Debug, Clone)]
pub struct DropdownItem<T: DropdownValue> {
    /// The value this item represents
    pub value: T,
    /// Optional icon
    pub icon: Option<Handle<Image>>,
    /// Whether this item is currently highlighted
    pub highlighted: bool,
    /// Whether this item is disabled
    pub disabled: bool,
}

impl<T: DropdownValue> DropdownItem<T> {
    /// Create a new dropdown item
    pub fn new(value: T) -> Self {
        Self {
            value,
            icon: None,
            highlighted: false,
            disabled: false,
        }
    }

    /// Set icon for this item
    pub fn with_icon(mut self, icon: Handle<Image>) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Disable this item
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Current state of a dropdown
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropdownState {
    /// Dropdown is closed
    Closed,
    /// Dropdown is open
    Open,
    /// Dropdown is animating open
    Opening,
    /// Dropdown is animating closed
    Closing,
}

/// Visual style for dropdowns
#[derive(Debug, Clone)]
pub struct DropdownStyle {
    /// Background color when closed
    pub background: Color,
    /// Background color when hovered
    pub background_hover: Color,
    /// Background color when open
    pub background_open: Color,
    /// Text color
    pub text_color: Color,
    /// Text color for disabled items
    pub text_color_disabled: Color,
    /// Border color
    pub border_color: Color,
    /// Border width
    pub border_width: f32,
    /// Corner radius
    pub corner_radius: f32,
    /// Padding
    pub padding: f32,
    /// Item height
    pub item_height: f32,
    /// Maximum visible items before scrolling
    pub max_visible_items: usize,
    /// Animation duration
    pub animation_duration: f32,
}

impl Default for DropdownStyle {
    fn default() -> Self {
        Self {
            background: Color::srgb(0.15, 0.15, 0.15),
            background_hover: Color::srgb(0.2, 0.2, 0.2),
            background_open: Color::srgb(0.18, 0.18, 0.18),
            text_color: Color::WHITE,
            text_color_disabled: Color::srgb(0.5, 0.5, 0.5),
            border_color: Color::srgb(0.3, 0.3, 0.3),
            border_width: 2.0,
            corner_radius: 4.0,
            padding: 8.0,
            item_height: 32.0,
            max_visible_items: 8,
            animation_duration: 0.2,
        }
    }
}

/// Configuration for dropdown behavior
#[derive(Resource, Debug, Clone)]
pub struct DropdownConfig {
    /// Allow search/filtering
    pub searchable: bool,
    /// Allow multiple selection
    pub multi_select: bool,
    /// Close on selection (for single select)
    pub close_on_select: bool,
    /// Close when clicking outside
    pub close_on_outside_click: bool,
    /// Enable keyboard navigation
    pub keyboard_nav: bool,
    /// Show item count
    pub show_count: bool,
}

impl Default for DropdownConfig {
    fn default() -> Self {
        Self {
            searchable: false,
            multi_select: false,
            close_on_select: true,
            close_on_outside_click: true,
            keyboard_nav: true,
            show_count: false,
        }
    }
}