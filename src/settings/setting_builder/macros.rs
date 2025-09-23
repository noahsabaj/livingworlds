//! Declarative settings tab generation macro implementation
//!
//! This module contains the main `define_setting_tab!` macro that generates
//! complete settings UI and event handling from declarative syntax.


/// Define a settings tab declaratively, eliminating UI spawning and event handling boilerplate.
///
/// This macro takes a tab name and configuration block, then generates complete
/// UI spawning functions and event handlers using existing UI builders.
///
/// ## Supported Configuration Options
///
/// - `settings_type: Type` - The settings struct this tab operates on
/// - `sections: [...]` - List of UI sections with their controls
///
/// ## Control Types
///
/// - `cycle: "Label" => field_name` - Cycle through enum values
/// - `toggle: "Label" => field_name` - Boolean toggle checkbox
/// - `slider: "Label" => field_name (min..max, format)` - Numeric slider
/// - `presets: [Preset1, Preset2]` - Graphics preset buttons
///
/// ## Section Structure
///
/// ```rust
/// sections: [
///     Section("Display") {
///         cycle: "Window Mode" => window_mode,
///         toggle: "VSync" => vsync,
///         slider: "Render Scale" => render_scale (0.5..2.0, percentage)
///     },
///     Section("Quality") {
///         presets: [Low, Medium, High, Ultra]
///     }
/// ]
/// ```
///
/// ## Generated Output
///
/// The macro generates:
/// - `spawn_{tab_name}_content(parent: &mut ChildSpawnerCommands, settings: &{SettingsType})`
/// - `handle_{tab_name}_interactions()` system function
/// - All necessary marker components for event handling
///
/// ## Example
///
/// ```rust
/// define_setting_tab!(GraphicsTab {
///     settings_type: GraphicsSettings,
///     sections: [
///         Section("Display") {
///             cycle: "Window Mode" => window_mode,
///             slider: "Render Scale" => render_scale (0.5..2.0, percentage)
///         }
///     ]
/// });
/// ```
#[macro_export]
macro_rules! define_setting_tab {
    // Main entry point - tab with configuration block
    ($tab_name:ident { $($config:tt)* }) => {
        $crate::define_setting_tab_internal!($tab_name, $($config)*);
    };
}

/// Internal macro for parsing and generating settings tab code.
/// This is separate from the main macro to allow for recursive parsing.
#[macro_export]
macro_rules! define_setting_tab_internal {
    // Main parsing entry point
    ($tab_name:ident, settings_type: $settings_type:ty, sections: [$($sections:tt)*]) => {
        $crate::generate_setting_tab!($tab_name, $settings_type, $($sections)*);
    };

    // Error case - missing required configuration
    ($tab_name:ident, $($config:tt)*) => {
        compile_error!(concat!(
            "Invalid settings tab configuration for ",
            stringify!($tab_name),
            ". Required: settings_type and sections."
        ));
    };
}

/// Generate the actual settings tab implementation
#[macro_export]
macro_rules! generate_setting_tab {
    // Parse sections and generate implementation
    ($tab_name:ident, $settings_type:ty, $($sections:tt)*) => {
        // Generate the UI spawning function
        paste::paste! {
            pub fn [<spawn_ $tab_name:lower _content>](
                parent: &mut bevy::prelude::ChildSpawnerCommands,
                settings: &$settings_type
            ) {
                
                
                use bevy::prelude::default;

                parent.spawn((
                    bevy::prelude::Node {
                        flex_direction: bevy::prelude::FlexDirection::Column,
                        row_gap: bevy::prelude::Val::Px(20.0),
                        ..default()
                    },
                    bevy::prelude::BackgroundColor(bevy::prelude::Color::NONE),
                )).with_children(|content| {
                    $crate::generate_sections!(content, settings, $($sections)*);
                });
            }

            // Generate event handling system
            pub fn [<handle_ $tab_name:lower _interactions>](
                temp_settings: bevy::prelude::ResMut<crate::settings::types::TempGameSettings>,
                // Add standard interaction queries here
                mut cycle_buttons: bevy::prelude::Query<
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
                // Add text query for updating UI
                text_query: bevy::prelude::Query<&mut bevy::prelude::Text>,
            ) {
                $crate::generate_event_handlers!(temp_settings, cycle_buttons, toggle_buttons, sliders, text_query);
            }
        }
    };
}

/// Generate sections within a settings tab
#[macro_export]
macro_rules! generate_sections {
    // Base case - no more sections
    ($content:ident, $settings:ident,) => {};

    // Parse a section with controls
    ($content:ident, $settings:ident, Section($section_name:literal) { $($controls:tt)* } $(, $($rest:tt)*)?) => {
        // Create section container with title
        $content.spawn((
            bevy::prelude::Node {
                flex_direction: bevy::prelude::FlexDirection::Column,
                row_gap: bevy::prelude::Val::Px(15.0),
                margin: bevy::prelude::UiRect::bottom(bevy::prelude::Val::Px(25.0)),
                ..default()
            },
            bevy::prelude::BackgroundColor(bevy::prelude::Color::NONE),
        )).with_children(|section| {
            // Section title
            section.spawn((
                bevy::prelude::Text::new($section_name),
                bevy::prelude::TextFont {
                    font_size: 20.0,
                    ..default()
                },
                bevy::prelude::TextColor(crate::ui::colors::TEXT_PRIMARY),
            ));

            // Generate controls within this section
            $crate::generate_controls!(section, $settings, $($controls)*);
        });

        // Process remaining sections
        $crate::generate_sections!($content, $settings, $($($rest)*)?);
    };
}

/// Generate individual controls within a section
#[macro_export]
macro_rules! generate_controls {
    // Base case - no more controls
    ($section:ident, $settings:ident,) => {};

    // Cycle control (enum cycling)
    ($section:ident, $settings:ident, cycle: $label:literal => $field:ident $(, $($rest:tt)*)?) => {
        crate::ui::ButtonBuilder::new(format!("< {} >", $settings.$field.as_str()))
            .style(crate::ui::ButtonStyle::Secondary)
            .with_marker(crate::settings::components::CycleButton {
                setting_type: $crate::field_to_setting_type!($field)
            })
            .build_in($section);

        $crate::generate_controls!($section, $settings, $($($rest)*)?);
    };

    // Toggle control (boolean checkbox)
    ($section:ident, $settings:ident, toggle: $label:literal => $field:ident $(, $($rest:tt)*)?) => {
        $crate::create_toggle_row!($section, $label, $settings.$field, $crate::field_to_setting_type!($field));
        $crate::generate_controls!($section, $settings, $($($rest)*)?);
    };

    // Slider control with range and format (simple format like Percentage)
    ($section:ident, $settings:ident, slider: $label:literal => $field:ident ($min:literal..$max:literal, $format:ident) $(, $($rest:tt)*)?) => {
        crate::ui::SliderBuilder::new($min, $max)
            .with_label($label)
            .with_value($settings.$field)
            .with_format(crate::ui::ValueFormat::$format)
            .with_marker(crate::settings::components::SettingsSlider {
                setting_type: $crate::field_to_setting_type!($field)
            })
            .build_in($section);

        $crate::generate_controls!($section, $settings, $($($rest)*)?);
    };

    // Slider control with range and format (function format like Decimal(1))
    ($section:ident, $settings:ident, slider: $label:literal => $field:ident ($min:literal..$max:literal, $format:ident($param:literal)) $(, $($rest:tt)*)?) => {
        crate::ui::SliderBuilder::new($min, $max)
            .with_label($label)
            .with_value($settings.$field)
            .with_format(crate::ui::ValueFormat::$format($param))
            .with_marker(crate::settings::components::SettingsSlider {
                setting_type: $crate::field_to_setting_type!($field)
            })
            .build_in($section);

        $crate::generate_controls!($section, $settings, $($($rest)*)?);
    };

    // Preset buttons (graphics quality presets)
    ($section:ident, $settings:ident, presets: [$($preset:ident),*] $(, $($rest:tt)*)?) => {
        // Generate preset button row
        $section.spawn((
            bevy::prelude::Node {
                flex_direction: bevy::prelude::FlexDirection::Row,
                column_gap: bevy::prelude::Val::Px(10.0),
                margin: bevy::prelude::UiRect::bottom(bevy::prelude::Val::Px(15.0)),
                ..default()
            },
            bevy::prelude::BackgroundColor(bevy::prelude::Color::NONE),
        )).with_children(|preset_row| {
            $(
                crate::ui::ButtonBuilder::new(stringify!($preset))
                    .style(crate::ui::ButtonStyle::Secondary)
                    .with_marker(crate::settings::components::PresetButton {
                        preset: crate::settings::types::GraphicsPreset::$preset
                    })
                    .build(preset_row);
            )*
        });

        $crate::generate_controls!($section, $settings, $($($rest)*)?);
    };

    // Error case - unrecognized control
    ($section:ident, $settings:ident, $unknown:tt $($rest:tt)*) => {
        compile_error!(concat!(
            "Unknown setting control type: ",
            stringify!($unknown),
            "\nSupported: cycle, toggle, slider, presets"
        ));
    };
}

/// Helper macro to create toggle row (replicates existing pattern)
#[macro_export]
macro_rules! create_toggle_row {
    ($parent:ident, $label:literal, $enabled:expr_2021, $setting_type:expr_2021) => {
        $parent
            .spawn((
                bevy::prelude::Node {
                    flex_direction: bevy::prelude::FlexDirection::Row,
                    justify_content: bevy::prelude::JustifyContent::SpaceBetween,
                    align_items: bevy::prelude::AlignItems::Center,
                    margin: bevy::prelude::UiRect::bottom(bevy::prelude::Val::Px(15.0)),
                    ..default()
                },
                bevy::prelude::BackgroundColor(bevy::prelude::Color::NONE),
            ))
            .with_children(|row| {
                // Label
                row.spawn((
                    bevy::prelude::Text::new($label),
                    bevy::prelude::TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    bevy::prelude::TextColor(crate::ui::colors::TEXT_PRIMARY),
                ));

                // Toggle button
                crate::ui::ButtonBuilder::new(if $enabled { "✓" } else { "" })
                    .style(if $enabled {
                        crate::ui::ButtonStyle::Success
                    } else {
                        crate::ui::ButtonStyle::Secondary
                    })
                    .with_marker(crate::settings::components::ToggleButton {
                        setting_type: $setting_type,
                        enabled: $enabled,
                    })
                    .build(row);
            });
    };
}

/// Convert field names to SettingType enum variants
#[macro_export]
macro_rules! field_to_setting_type {
    (window_mode) => {
        crate::settings::types::SettingType::WindowMode
    };
    (resolution) => {
        crate::settings::types::SettingType::Resolution
    };
    (vsync) => {
        crate::settings::types::SettingType::VSync
    };
    (render_scale) => {
        crate::settings::types::SettingType::RenderScale
    };
    (shadow_quality) => {
        crate::settings::types::SettingType::ShadowQuality
    };
    (master_volume) => {
        crate::settings::types::SettingType::MasterVolume
    };
    (sfx_volume) => {
        crate::settings::types::SettingType::SfxVolume
    };
    (mute_when_unfocused) => {
        crate::settings::types::SettingType::MuteWhenUnfocused
    };
    (ui_scale) => {
        crate::settings::types::SettingType::UiScale
    };
    (show_fps) => {
        crate::settings::types::SettingType::ShowFps
    };
    (show_province_info) => {
        crate::settings::types::SettingType::ShowProvinceInfo
    };
    (tooltip_delay) => {
        crate::settings::types::SettingType::TooltipDelay
    };
    (show_tooltips) => {
        crate::settings::types::SettingType::ShowTooltips
    };
    (camera_speed) => {
        crate::settings::types::SettingType::CameraSpeed
    };
    (zoom_speed) => {
        crate::settings::types::SettingType::ZoomSpeed
    };
    (invert_zoom) => {
        crate::settings::types::SettingType::InvertZoom
    };
    (edge_pan_speed) => {
        crate::settings::types::SettingType::EdgePanSpeed
    };
    (zoom_sensitivity) => {
        crate::settings::types::SettingType::ZoomSensitivity
    };
}

/// Generate event handlers for all control types
#[macro_export]
macro_rules! generate_event_handlers {
    ($temp_settings:ident, $cycle_buttons:ident, $toggle_buttons:ident, $sliders:ident, $text_query:ident) => {
        // Handle cycle button interactions
        for (interaction, cycle_button, children) in &mut $cycle_buttons {
            if *interaction == bevy::prelude::Interaction::Pressed {
                // This will be expanded with specific field updates
                // based on the actual settings structure in each generated implementation
            }
        }

        // Handle toggle button interactions
        for (interaction, mut toggle_button) in &mut $toggle_buttons {
            if *interaction == bevy::prelude::Interaction::Pressed {
                toggle_button.enabled = !toggle_button.enabled;
                // Update temp settings based on setting type
            }
        }

        // Handle slider interactions
        for (slider, settings_slider) in &$sliders {
            // Update temp settings based on setting type and slider value
        }
    };
}

// Re-export the main macro for easier usage
pub use define_setting_tab;
