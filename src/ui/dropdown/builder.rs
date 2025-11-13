//! Builder API for dropdowns

use bevy::prelude::*;
use super::components::*;
use super::types::*;

/// Builder for creating dropdowns
pub struct DropdownBuilder<T: DropdownValue> {
    items: Vec<T>,
    selected: Option<usize>,
    style: DropdownStyle,
    config: DropdownConfig,
    on_change: Option<Box<dyn Fn(&[T]) + Send + Sync>>,
}

impl<T: DropdownValue> DropdownBuilder<T> {
    /// Create a new dropdown builder
    pub fn new() -> Self {
        Self {
            items: vec![],
            selected: None,
            style: DropdownStyle::default(),
            config: DropdownConfig::default(),
            on_change: None,
        }
    }

    /// Set items
    pub fn items(mut self, items: Vec<T>) -> Self {
        self.items = items;
        self
    }

    /// Set selected index
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    /// Enable search
    pub fn searchable(mut self) -> Self {
        self.config.searchable = true;
        self
    }

    /// Enable multi-select
    pub fn multi_select(mut self) -> Self {
        self.config.multi_select = true;
        self.config.close_on_select = false;
        self
    }

    /// Set change callback
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(&[T]) + Send + Sync + 'static
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Build the dropdown bundle
    pub fn build(self) -> DropdownBundle<T> {
        let mut dropdown = Dropdown::new(self.items);
        dropdown.style = self.style;
        dropdown.config = self.config;
        dropdown.on_change = self.on_change;

        if let Some(index) = self.selected {
            dropdown.selected = vec![index];
        }

        DropdownBundle {
            dropdown,
            node: Node {
                width: Val::Px(200.0),
                height: Val::Px(32.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            border_color: BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
            interaction: Interaction::None,
        }
    }
}

/// Helper function to create a dropdown
pub fn dropdown<T: DropdownValue>(items: Vec<T>) -> DropdownBuilder<T> {
    DropdownBuilder::new().items(items)
}