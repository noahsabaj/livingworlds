//! Component definitions for the dropdown system

use bevy::prelude::*;
use super::types::*;

/// Main dropdown component
#[derive(Component)]
pub struct Dropdown<T: DropdownValue> {
    /// All available items
    pub items: Vec<DropdownItem<T>>,
    /// Currently selected index(es)
    pub selected: Vec<usize>,
    /// Current state
    pub state: DropdownState,
    /// Visual style
    pub style: DropdownStyle,
    /// Configuration
    pub config: DropdownConfig,
    /// Search query
    pub search_query: String,
    /// Filtered item indices
    pub filtered_indices: Vec<usize>,
    /// Currently highlighted item (for keyboard nav)
    pub highlighted_index: Option<usize>,
    /// Callback when selection changes
    pub on_change: Option<Box<dyn Fn(&[T]) + Send + Sync>>,
}

impl<T: DropdownValue> Dropdown<T> {
    /// Create a new dropdown with items
    pub fn new(items: Vec<T>) -> Self {
        let items = items.into_iter().map(DropdownItem::new).collect();
        Self {
            items,
            selected: vec![],
            state: DropdownState::Closed,
            style: DropdownStyle::default(),
            config: DropdownConfig::default(),
            search_query: String::new(),
            filtered_indices: vec![],
            highlighted_index: None,
            on_change: None,
        }
    }

    /// Get currently selected values
    pub fn selected_values(&self) -> Vec<T> {
        self.selected
            .iter()
            .filter_map(|&idx| self.items.get(idx).map(|item| item.value.clone()))
            .collect()
    }

    /// Set selected index
    pub fn set_selected(&mut self, index: usize) {
        if self.config.multi_select {
            if !self.selected.contains(&index) {
                self.selected.push(index);
            }
        } else {
            self.selected = vec![index];
        }

        if let Some(on_change) = &self.on_change {
            on_change(&self.selected_values());
        }
    }

    /// Toggle selection for an index
    pub fn toggle_selected(&mut self, index: usize) {
        if let Some(pos) = self.selected.iter().position(|&idx| idx == index) {
            self.selected.remove(pos);
        } else {
            self.set_selected(index);
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected.clear();
        if let Some(on_change) = &self.on_change {
            on_change(&[]);
        }
    }

    /// Open the dropdown
    pub fn open(&mut self) {
        self.state = DropdownState::Opening;
        self.update_filter();
    }

    /// Close the dropdown
    pub fn close(&mut self) {
        self.state = DropdownState::Closing;
        self.search_query.clear();
        self.highlighted_index = None;
    }

    /// Toggle open/closed
    pub fn toggle(&mut self) {
        match self.state {
            DropdownState::Closed => self.open(),
            DropdownState::Open => self.close(),
            _ => {} // Ignore if animating
        }
    }

    /// Update filtered indices based on search query
    pub fn update_filter(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_indices = (0..self.items.len()).collect();
        } else {
            let query = self.search_query.to_lowercase();
            self.filtered_indices = self.items
                .iter()
                .enumerate()
                .filter(|(_, item)| {
                    item.value.display_text().to_lowercase().contains(&query)
                })
                .map(|(idx, _)| idx)
                .collect();
        }
    }

    /// Move highlight up
    pub fn highlight_previous(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }

        self.highlighted_index = match self.highlighted_index {
            None => Some(self.filtered_indices.len() - 1),
            Some(0) => Some(self.filtered_indices.len() - 1),
            Some(idx) => Some(idx - 1),
        };
    }

    /// Move highlight down
    pub fn highlight_next(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }

        self.highlighted_index = match self.highlighted_index {
            None => Some(0),
            Some(idx) if idx >= self.filtered_indices.len() - 1 => Some(0),
            Some(idx) => Some(idx + 1),
        };
    }

    /// Select highlighted item
    pub fn select_highlighted(&mut self) {
        if let Some(highlight_idx) = self.highlighted_index {
            if let Some(&item_idx) = self.filtered_indices.get(highlight_idx) {
                if !self.items[item_idx].disabled {
                    self.set_selected(item_idx);
                }
            }
        }
    }
}

/// Marker for the dropdown menu (popup) entity
#[derive(Component)]
pub struct DropdownMenu {
    /// Entity of the parent dropdown
    pub dropdown_entity: Entity,
}

/// Marker for the selected value display
#[derive(Component)]
pub struct DropdownSelected {
    /// Entity of the parent dropdown
    pub dropdown_entity: Entity,
}

/// Marker for open dropdowns
#[derive(Component)]
pub struct DropdownOpen;

/// Marker for dropdown search input
#[derive(Component)]
pub struct DropdownSearch {
    /// Entity of the parent dropdown
    pub dropdown_entity: Entity,
}

/// Bundle for spawning a dropdown
#[derive(Bundle)]
pub struct DropdownBundle<T: DropdownValue> {
    /// The dropdown component
    pub dropdown: Dropdown<T>,
    /// UI node
    pub node: Node,
    /// Background color
    pub background_color: BackgroundColor,
    /// Border color
    pub border_color: BorderColor,
    /// Interaction component
    pub interaction: Interaction,
}