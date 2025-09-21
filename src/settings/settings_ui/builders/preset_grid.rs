//! Preset Grid Builder
//!
//! Builder for creating graphics preset button grids.

use crate::settings::{components::PresetButton, types::*};
use crate::ui::{dimensions, styles::colors, ButtonBuilder, ButtonStyle, PanelBuilder, PanelStyle, ChildBuilder};
use bevy::prelude::*;

/// Configuration for preset grid layout
#[derive(Debug, Clone, Copy)]
pub enum GridLayout {
    Row,      // Single row of buttons
    TwoByTwo, // 2x2 grid
    Column,   // Single column of buttons
}

/// Builder for creating graphics preset button grids using existing UI patterns
pub struct PresetGridBuilder {
    current_settings: GraphicsSettings,
    layout: GridLayout,
    button_width: Val,
    button_height: Val,
    gap: Val,
    width: Val,
    margin: UiRect,
}

impl PresetGridBuilder {
    pub fn new(current_settings: GraphicsSettings) -> Self {
        Self {
            current_settings,
            layout: GridLayout::Row,
            button_width: Val::Px(80.0),
            button_height: Val::Px(35.0),
            gap: Val::Px(dimensions::MARGIN_SMALL),
            width: Val::Percent(100.0),
            margin: UiRect::all(Val::Px(0.0)),
        }
    }

    pub fn layout(mut self, layout: GridLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn button_width(mut self, width: Val) -> Self {
        self.button_width = width;
        self
    }

    pub fn button_height(mut self, height: Val) -> Self {
        self.button_height = height;
        self
    }

    pub fn gap(mut self, gap: Val) -> Self {
        self.gap = gap;
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    pub fn build(self, parent: &mut ChildBuilder) -> Entity {
        let current_preset = self.current_settings.current_preset();

        match self.layout {
            GridLayout::Row => self.build_row_layout(parent, current_preset),
            GridLayout::TwoByTwo => self.build_grid_layout(parent, current_preset),
            GridLayout::Column => self.build_column_layout(parent, current_preset),
        }
    }

    fn build_row_layout(
        self,
        parent: &mut ChildBuilder,
        current_preset: Option<GraphicsPreset>,
    ) -> Entity {
        PanelBuilder::new()
            .style(PanelStyle::Transparent)
            .width(self.width)
            .margin(self.margin)
            .flex_direction(FlexDirection::Row)
            .justify_content(JustifyContent::Center)
            .column_gap(self.gap)
            .build_with_children(parent, |grid| {
                self.create_preset_buttons(grid, current_preset);
            })
    }

    fn build_column_layout(
        self,
        parent: &mut ChildBuilder,
        current_preset: Option<GraphicsPreset>,
    ) -> Entity {
        PanelBuilder::new()
            .style(PanelStyle::Transparent)
            .width(self.width)
            .margin(self.margin)
            .flex_direction(FlexDirection::Column)
            .justify_content(JustifyContent::Center)
            .row_gap(self.gap)
            .build_with_children(parent, |grid| {
                self.create_preset_buttons(grid, current_preset);
            })
    }

    fn build_grid_layout(
        self,
        parent: &mut ChildBuilder,
        current_preset: Option<GraphicsPreset>,
    ) -> Entity {
        PanelBuilder::new()
            .style(PanelStyle::Transparent)
            .width(self.width)
            .margin(self.margin)
            .flex_direction(FlexDirection::Column)
            .justify_content(JustifyContent::Center)
            .row_gap(self.gap)
            .build_with_children(parent, |grid| {
                // First row: Low and Medium
                PanelBuilder::new()
                    .style(PanelStyle::Transparent)
                    .flex_direction(FlexDirection::Row)
                    .justify_content(JustifyContent::Center)
                    .column_gap(self.gap)
                    .build_with_children(grid, |row| {
                        self.create_preset_button(row, GraphicsPreset::Low, current_preset);
                        self.create_preset_button(row, GraphicsPreset::Medium, current_preset);
                    });

                // Second row: High and Ultra
                PanelBuilder::new()
                    .style(PanelStyle::Transparent)
                    .flex_direction(FlexDirection::Row)
                    .justify_content(JustifyContent::Center)
                    .column_gap(self.gap)
                    .build_with_children(grid, |row| {
                        self.create_preset_button(row, GraphicsPreset::High, current_preset);
                        self.create_preset_button(row, GraphicsPreset::Ultra, current_preset);
                    });
            })
    }

    fn create_preset_buttons(
        &self,
        parent: &mut ChildBuilder,
        current_preset: Option<GraphicsPreset>,
    ) {
        self.create_preset_button(parent, GraphicsPreset::Low, current_preset);
        self.create_preset_button(parent, GraphicsPreset::Medium, current_preset);
        self.create_preset_button(parent, GraphicsPreset::High, current_preset);
        self.create_preset_button(parent, GraphicsPreset::Ultra, current_preset);
    }

    fn create_preset_button(
        &self,
        parent: &mut ChildBuilder,
        preset: GraphicsPreset,
        current_preset: Option<GraphicsPreset>,
    ) {
        let is_selected = current_preset == Some(preset);
        let button_style = if is_selected {
            ButtonStyle::Success
        } else {
            ButtonStyle::Secondary
        };

        let preset_name = match preset {
            GraphicsPreset::Low => "Low",
            GraphicsPreset::Medium => "Medium",
            GraphicsPreset::High => "High",
            GraphicsPreset::Ultra => "Ultra",
        };

        ButtonBuilder::new(preset_name)
            .style(button_style)
            .width(self.button_width)
            .height(self.button_height)
            .with_marker(PresetButton { preset })
            .build(parent);
    }
}
