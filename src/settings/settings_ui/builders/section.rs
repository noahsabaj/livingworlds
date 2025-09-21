//! Settings Section Builder
//!
//! Builder for creating consistent settings sections with headers and organized content.

use crate::ui::{dimensions, LabelBuilder, LabelStyle, PanelBuilder, PanelStyle, ChildBuilder};
use bevy::prelude::*;

/// Component marker for settings sections
#[derive(Component, Debug)]
pub struct SettingsSection {
    pub title: String,
}

/// Builder for creating settings sections with proper UI patterns
pub struct SettingSectionBuilder {
    title: String,
    gap: Val,
    padding: UiRect,
    show_title: bool,
    panel_style: PanelStyle,
    width: Val,
    margin: UiRect,
}

impl SettingSectionBuilder {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            gap: Val::Px(dimensions::MARGIN_MEDIUM),
            padding: UiRect::all(Val::Px(dimensions::PANEL_PADDING)),
            show_title: true,
            panel_style: PanelStyle::Default,
            width: Val::Percent(100.0),
            margin: UiRect::bottom(Val::Px(dimensions::MARGIN_LARGE)),
        }
    }

    pub fn gap(mut self, gap: Val) -> Self {
        self.gap = gap;
        self
    }

    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }

    pub fn no_title(mut self) -> Self {
        self.show_title = false;
        self
    }

    pub fn style(mut self, style: PanelStyle) -> Self {
        self.panel_style = style;
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

    /// Build section container only - content can be added later
    pub fn build(self, parent: &mut ChildBuilder) -> Entity {
        let title = self.title.clone();

        PanelBuilder::new()
            .style(self.panel_style)
            .width(self.width)
            .padding(self.padding)
            .margin(self.margin)
            .flex_direction(FlexDirection::Column)
            .row_gap(self.gap)
            .build_with_children(parent, |section| {
                if self.show_title {
                    LabelBuilder::new(&title)
                        .style(LabelStyle::Heading)
                        .margin(UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)))
                        .build(section);
                }
            })
    }

    /// Build section with content using a closure
    pub fn build_with_children(
        self,
        parent: &mut ChildBuilder,
        children: impl FnOnce(&mut ChildBuilder),
    ) -> Entity {
        let title = self.title.clone();

        PanelBuilder::new()
            .style(self.panel_style)
            .width(self.width)
            .padding(self.padding)
            .margin(self.margin)
            .flex_direction(FlexDirection::Column)
            .row_gap(self.gap)
            .build_with_children(parent, |section| {
                if self.show_title {
                    LabelBuilder::new(&title)
                        .style(LabelStyle::Heading)
                        .margin(UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)))
                        .build(section);
                }

                // Add user-provided content
                children(section);
            })
    }
}
