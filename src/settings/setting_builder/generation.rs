//! Settings generation utilities and support types
//!
//! This module provides the infrastructure that supports the macro system,
//! including configuration types and generation helpers.

use crate::settings::types::SettingType;
use crate::ui::ValueFormat;
use bevy::prelude::*;

/// Configuration for a complete settings tab
#[derive(Debug, Clone)]
pub struct TabConfig {
    /// Name of the tab (used for function generation)
    pub name: String,
    /// The settings type this tab operates on
    pub settings_type: String,
    /// List of sections within this tab
    pub sections: Vec<SectionConfig>,
}

/// Configuration for a settings section
#[derive(Debug, Clone)]
pub struct SectionConfig {
    /// Display name for the section
    pub name: String,
    /// List of controls in this section
    pub controls: Vec<ControlConfig>,
    /// Whether this section has preset buttons
    pub has_presets: bool,
}

/// Configuration for individual setting controls
#[derive(Debug, Clone)]
pub struct ControlConfig {
    /// Type of control (cycle, toggle, slider)
    pub control_type: ControlType,
    /// Display label for the control
    pub label: String,
    /// Field name in the settings struct
    pub field_name: String,
    /// The SettingType enum variant
    pub setting_type: SettingType,
}

/// Types of setting controls that can be generated
#[derive(Debug, Clone)]
pub enum ControlType {
    /// Cycle through enum values (buttons with < Value >)
    Cycle,
    /// Boolean toggle checkbox
    Toggle,
    /// Numeric slider with range and format
    Slider {
        min: f32,
        max: f32,
        format: ValueFormat,
    },
    /// Graphics preset buttons
    Presets(Vec<String>),
}

/// Main generator for settings tabs
pub struct SettingGenerator;

impl SettingGenerator {
    /// Generate UI spawning function for a settings tab
    pub fn generate_spawn_function(config: &TabConfig) -> String {
        let function_name = format!("spawn_{}_content", config.name.to_lowercase());
        let settings_type = &config.settings_type;

        format!(
            r#"
pub fn {function_name}(
    parent: &mut bevy::prelude::ChildSpawner,
    settings: &{settings_type}
) {{
    use crate::ui::{{
        SliderBuilder, ButtonBuilder, PanelBuilder, LabelBuilder,
        ButtonStyle, ValueFormat, PanelStyle
    }};
    use crate::settings::{{components::*, types::SettingType}};

    parent.spawn((
        bevy::prelude::Node {{
            flex_direction: bevy::prelude::FlexDirection::Column,
            row_gap: bevy::prelude::Val::Px(20.0),
            ..default()
        }},
        bevy::prelude::BackgroundColor(bevy::prelude::Color::NONE),
    )).with_children(|content| {{
        {sections}
    }});
}}
"#,
            function_name = function_name,
            settings_type = settings_type,
            sections = config
                .sections
                .iter()
                .map(|section| Self::generate_section(section))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    /// Generate a single section within a settings tab
    fn generate_section(section: &SectionConfig) -> String {
        format!(
            r#"
        // Section: {section_name}
        content.spawn((
            bevy::prelude::Node {{
                flex_direction: bevy::prelude::FlexDirection::Column,
                row_gap: bevy::prelude::Val::Px(15.0),
                margin: bevy::prelude::UiRect::bottom(bevy::prelude::Val::Px(25.0)),
                ..default()
            }},
            bevy::prelude::BackgroundColor(bevy::prelude::Color::NONE),
        )).with_children(|section| {{
            // Section title
            section.spawn((
                bevy::prelude::Text::new("{section_name}"),
                bevy::prelude::TextFont {{
                    font_size: 20.0,
                    ..default()
                }},
                bevy::prelude::TextColor(crate::ui::styles::colors::TEXT_PRIMARY),
            ));

            {controls}
        }});
"#,
            section_name = section.name,
            controls = section
                .controls
                .iter()
                .map(|control| Self::generate_control(control))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    /// Generate individual control UI
    fn generate_control(control: &ControlConfig) -> String {
        match &control.control_type {
            ControlType::Cycle => {
                format!(
                    r#"
            // Cycle control: {label} using dropdown component
            {{
                let options = settings.{field}.get_all_options();
                let current_value = settings.{field}.as_str();
                let selected_index = options.iter().position(|opt| opt == &current_value).unwrap_or(0);

                crate::ui::dropdown::DropdownBuilder::new(options)
                    .width(bevy::prelude::Val::Px(200.0))
                    .selected_index(selected_index)
                    .with_marker(crate::settings::components::CycleButton {{
                        setting_type: crate::settings::types::SettingType::{setting_type:?}
                    }})
                    .build(section);
            }}
"#,
                    label = control.label,
                    field = control.field_name,
                    setting_type = control.setting_type
                )
            }
            ControlType::Toggle => {
                format!(
                    r#"
            // Toggle control: {label}
            create_toggle_row(section, "{label}", settings.{field}, crate::settings::types::SettingType::{setting_type:?});
"#,
                    label = control.label,
                    field = control.field_name,
                    setting_type = control.setting_type
                )
            }
            ControlType::Slider { min, max, format } => {
                format!(
                    r#"
            // Slider control: {label}
            crate::ui::SliderBuilder::new({min}..{max})
                .with_label("{label}")
                
                .with_format(crate::ui::ValueFormat::{format:?})
                .with_marker(crate::settings::components::SettingsSlider {{
                    setting_type: crate::settings::types::SettingType::{setting_type:?}
                }})
                .build(section);
"#,
                    label = control.label,
                    min = min,
                    max = max,
                    format = format,
                    setting_type = control.setting_type
                )
            }
            ControlType::Presets(presets) => {
                format!(
                    r#"
            // Preset buttons
            section.spawn((
                bevy::prelude::Node {{
                    flex_direction: bevy::prelude::FlexDirection::Row,
                    column_gap: bevy::prelude::Val::Px(10.0),
                    margin: bevy::prelude::UiRect::bottom(bevy::prelude::Val::Px(15.0)),
                    ..default()
                }},
                bevy::prelude::BackgroundColor(bevy::prelude::Color::NONE),
            )).with_children(|preset_row| {{
                {preset_buttons}
            }});
"#,
                    preset_buttons = presets
                        .iter()
                        .map(|preset| format!(
                            r#"
                crate::ui::ButtonBuilder::new("{preset}")
                    .style(crate::ui::ButtonStyle::Secondary)
                    .with_marker(crate::settings::components::PresetButton {{
                        preset: crate::settings::types::GraphicsPreset::{preset}
                    }})
                    .build(preset_row);
"#,
                            preset = preset
                        ))
                        .collect::<Vec<_>>()
                        .join("")
                )
            }
        }
    }

    /// Generate event handling function for a settings tab
    pub fn generate_event_handler(config: &TabConfig) -> String {
        let function_name = format!("handle_{}_interactions", config.name.to_lowercase());

        format!(
            r#"
pub fn {function_name}(
    mut temp_settings: bevy::prelude::ResMut<crate::settings::types::TempGameSettings>,
    cycle_buttons: bevy::prelude::Query<
        (&bevy::prelude::Interaction, &crate::settings::components::CycleButton, &bevy::prelude::Children),
        (bevy::prelude::Changed<bevy::prelude::Interaction>, bevy::prelude::With<bevy::prelude::Button>)
    >,
    mut toggle_buttons: bevy::prelude::Query<
        (&bevy::prelude::Interaction, &mut crate::settings::components::ToggleButton),
        (bevy::prelude::Changed<bevy::prelude::Interaction>, bevy::prelude::With<bevy::prelude::Button>)
    >,
    sliders: bevy::prelude::Query<
        (&crate::ui::Slider, &crate::settings::components::SettingsSlider),
        bevy::prelude::Changed<crate::ui::Slider>
    >,
    mut text_query: bevy::prelude::Query<&mut bevy::prelude::Text>,
) {{
    // Handle cycle button interactions
    for (interaction, cycle_button, children) in &cycle_buttons {{
        if *interaction == bevy::prelude::Interaction::Pressed {{
            {cycle_handlers}
        }}
    }}

    // Handle toggle button interactions
    for (interaction, mut toggle_button) in &mut toggle_buttons {{
        if *interaction == bevy::prelude::Interaction::Pressed {{
            toggle_button.enabled = !toggle_button.enabled;
            {toggle_handlers}
        }}
    }}

    // Handle slider interactions
    for (slider, settings_slider) in &sliders {{
        {slider_handlers}
    }}
}}
"#,
            function_name = function_name,
            cycle_handlers = Self::generate_cycle_handlers(config),
            toggle_handlers = Self::generate_toggle_handlers(config),
            slider_handlers = Self::generate_slider_handlers(config)
        )
    }

    /// Generate cycle button event handlers
    fn generate_cycle_handlers(config: &TabConfig) -> String {
        config
            .sections
            .iter()
            .flat_map(|section| &section.controls)
            .filter(|control| matches!(control.control_type, ControlType::Cycle))
            .map(|control| {
                format!(
                    r#"
            if cycle_button.setting_type == crate::settings::types::SettingType::{setting_type:?} {{
                temp_settings.0.graphics.{field} = temp_settings.0.graphics.{field}.cycle();
                // Update button text
                for child in children {{
                    if let Ok(mut text) = text_query.get_mut(*child) {{
                        **text = format!("< {{}} >", temp_settings.0.graphics.{field}.as_str());
                    }}
                }}
            }}
"#,
                    setting_type = control.setting_type,
                    field = control.field_name
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// Generate toggle button event handlers
    fn generate_toggle_handlers(config: &TabConfig) -> String {
        config
            .sections
            .iter()
            .flat_map(|section| &section.controls)
            .filter(|control| matches!(control.control_type, ControlType::Toggle))
            .map(|control| {
                format!(
                    r#"
            if toggle_button.setting_type == crate::settings::types::SettingType::{setting_type:?} {{
                temp_settings.0.graphics.{field} = toggle_button.enabled;
            }}
"#,
                    setting_type = control.setting_type,
                    field = control.field_name
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// Generate slider event handlers
    fn generate_slider_handlers(config: &TabConfig) -> String {
        config
            .sections
            .iter()
            .flat_map(|section| &section.controls)
            .filter(|control| matches!(control.control_type, ControlType::Slider { .. }))
            .map(|control| {
                format!(
                    r#"
        if settings_slider.setting_type == crate::settings::types::SettingType::{setting_type:?} {{
            temp_settings.0.graphics.{field} = slider.value;
        }}
"#,
                    setting_type = control.setting_type,
                    field = control.field_name
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

/// Validation results for settings configuration
#[derive(Debug)]
pub enum GenerationResult {
    Success(String),
    Error(String),
}

/// Helper function to create toggle row (matches existing pattern)
pub fn generate_toggle_helper() -> &'static str {
    r#"
/// Helper function to create toggle row (replicates existing pattern)
fn create_toggle_row(
    parent: &mut bevy::prelude::ChildSpawner,
    label: &str,
    enabled: bool,
    setting_type: crate::settings::types::SettingType,
) {
    parent.spawn((
        bevy::prelude::Node {
            flex_direction: bevy::prelude::FlexDirection::Row,
            justify_content: bevy::prelude::JustifyContent::SpaceBetween,
            align_items: bevy::prelude::AlignItems::Center,
            margin: bevy::prelude::UiRect::bottom(bevy::prelude::Val::Px(15.0)),
            ..default()
        },
        bevy::prelude::BackgroundColor(bevy::prelude::Color::NONE),
    )).with_children(|row| {
        // Label
        row.spawn((
            bevy::prelude::Text::new(label),
            bevy::prelude::TextFont {
                font_size: 18.0,
                ..default()
            },
            bevy::prelude::TextColor(crate::ui::styles::colors::TEXT_PRIMARY),
        ));

        // Toggle button
        crate::ui::ButtonBuilder::new(if enabled { "✓" } else { "" })
            .style(if enabled {
                crate::ui::ButtonStyle::Success
            } else {
                crate::ui::ButtonStyle::Secondary
            })
            .with_marker(crate::settings::components::ToggleButton {
                setting_type,
                enabled,
            })
            .build(row);
    });
}
"#
}
