//! Setting Row Builder
//!
//! Builder for creating consistent setting rows with labels and controls.

use crate::settings::{components::*, types::*};
use crate::ui::{
    dimensions, ButtonBuilder, ButtonStyle, LabelBuilder, LabelStyle, PanelBuilder,
    PanelStyle, SliderBuilder, ValueFormat, ChildBuilder,
};
use bevy::prelude::*;

/// Control type configuration for setting rows
#[derive(Debug, Clone)]
pub enum ControlType {
    Cycle {
        current_value: String,
    },
    Toggle {
        enabled: bool,
    },
    Slider {
        value: f32,
        min: f32,
        max: f32,
        format: ValueFormat,
    },
}

/// Builder for creating individual setting rows with proper UI patterns
pub struct SettingRowBuilder {
    label: String,
    setting_type: SettingType,
    control_type: ControlType,
    width: Val,
    height: Val,
    margin: UiRect,
}

impl SettingRowBuilder {
    pub fn cycle(
        label: impl Into<String>,
        setting_type: SettingType,
        current_value: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            setting_type,
            control_type: ControlType::Cycle {
                current_value: current_value.into(),
            },
            width: Val::Percent(100.0),
            height: Val::Px(40.0),
            margin: UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)),
        }
    }

    pub fn slider(
        label: impl Into<String>,
        setting_type: SettingType,
        value: f32,
        min: f32,
        max: f32,
        format: ValueFormat,
    ) -> Self {
        Self {
            label: label.into(),
            setting_type,
            control_type: ControlType::Slider {
                value,
                min,
                max,
                format,
            },
            width: Val::Percent(100.0),
            height: Val::Px(40.0),
            margin: UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)),
        }
    }

    pub fn toggle(label: impl Into<String>, setting_type: SettingType, enabled: bool) -> Self {
        Self {
            label: label.into(),
            setting_type,
            control_type: ControlType::Toggle { enabled },
            width: Val::Percent(100.0),
            height: Val::Px(40.0),
            margin: UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)),
        }
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    pub fn build(self, parent: &mut ChildBuilder) -> Entity {
        let setting_type = self.setting_type.clone();
        let control_type = self.control_type;

        PanelBuilder::new()
            .style(PanelStyle::Transparent)
            .width(self.width)
            .height(self.height)
            .margin(self.margin)
            .flex_direction(FlexDirection::Row)
            .justify_content(JustifyContent::SpaceBetween)
            .align_items(AlignItems::Center)
            .padding(UiRect::horizontal(Val::Px(dimensions::MARGIN_SMALL)))
            .build_with_children(parent, |row| {
                // Label on the left
                LabelBuilder::new(&self.label)
                    .style(LabelStyle::Body)
                    .build(row);

                // Control on the right
                match control_type {
                    ControlType::Cycle { current_value } => {
                        ButtonBuilder::new(format!("< {} >", current_value))
                            .style(ButtonStyle::Secondary)
                            .width(Val::Px(200.0))
                            .with_marker(CycleButton { setting_type })
                            .build(row);
                    }
                    ControlType::Toggle { enabled } => {
                        ButtonBuilder::new(if enabled { "X" } else { "" })
                            .style(if enabled {
                                ButtonStyle::Success
                            } else {
                                ButtonStyle::Secondary
                            })
                            .width(Val::Px(40.0))
                            .height(Val::Px(30.0))
                            .with_marker(ToggleButton {
                                setting_type,
                                enabled,
                            })
                            .build(row);
                    }
                    ControlType::Slider {
                        value,
                        min,
                        max,
                        format,
                    } => {
                        // Use SliderBuilder for proper slider creation
                        SliderBuilder::new(min, max)
                            .with_value(value)
                            .width(Val::Px(200.0))
                            .with_format(format)
                            .with_marker(SettingsSlider { setting_type })
                            .build(row);
                    }
                }
            })
    }
}
