//! Validation utilities for settings builder configurations
//!
//! This module provides compile-time and runtime validation for settings tab
//! configurations, ensuring that generated tabs are correct and providing
//! helpful error messages when they're not.
//!
//! Based on the proven validation patterns from plugin_builder.

use crate::settings::types::SettingType;
use crate::ui::ValueFormat;
use bevy::prelude::*;

/// Compile-time validation helpers
pub mod compile_time {
    use super::*;

    /// Ensure a type can be used as a settings struct
    pub fn validate_settings_type<T>()
    where
        T: Clone + std::fmt::Debug + PartialEq,
    {
        // Compile-time validation for settings types
        // The trait bounds ensure the type can be used in the settings system
    }

    /// Validate that a field exists on a settings struct
    pub fn validate_field_access<T, F>(
        _settings_type: std::marker::PhantomData<T>,
        _field_accessor: F,
    ) where
        F: Fn(&T) -> &dyn std::fmt::Display,
    {
        // Compile-time validation for field access
        // This function exists purely for type checking
    }

    /// Validate range parameters for slider controls
    pub fn validate_slider_range(min: f32, max: f32) -> Result<(), &'static str> {
        if min >= max {
            return Err("Slider minimum must be less than maximum");
        }
        if !min.is_finite() || !max.is_finite() {
            return Err("Slider range must be finite numbers");
        }
        Ok(())
    }

    /// Validate value format compatibility
    pub fn validate_value_format(
        format: &ValueFormat,
        min: f32,
        max: f32,
    ) -> Result<(), &'static str> {
        match format {
            ValueFormat::Percentage => {
                if min < 0.0 || max > 1.0 {
                    return Err("Percentage format requires range 0.0..1.0");
                }
            }
            ValueFormat::Integer => {
                if (max - min) < 1.0 {
                    return Err("Integer format requires range >= 1.0");
                }
            }
            _ => {}
        }
        Ok(())
    }
}

/// Runtime validation utilities
pub mod runtime {
    use super::*;

    /// Validation result type
    pub type ValidationResult = Result<(), ValidationError>;

    /// Validation error types for settings
    #[derive(Debug, Clone)]
    pub enum ValidationError {
        /// Settings struct validation failed
        InvalidSettingsStruct(String),
        /// Field doesn't exist or isn't accessible
        InvalidField(String),
        /// Control configuration is invalid
        InvalidControl(String),
        /// Section configuration is malformed
        InvalidSection(String),
        /// Tab configuration is incomplete
        IncompleteTabConfig(String),
        /// Setting type mismatch
        SettingTypeMismatch(String),
    }

    impl std::fmt::Display for ValidationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ValidationError::InvalidSettingsStruct(msg) => {
                    write!(f, "Invalid settings struct: {}", msg)
                }
                ValidationError::InvalidField(field) => {
                    write!(f, "Invalid field access: {}", field)
                }
                ValidationError::InvalidControl(control) => {
                    write!(f, "Invalid control configuration: {}", control)
                }
                ValidationError::InvalidSection(section) => {
                    write!(f, "Invalid section configuration: {}", section)
                }
                ValidationError::IncompleteTabConfig(tab) => {
                    write!(f, "Incomplete tab configuration: {}", tab)
                }
                ValidationError::SettingTypeMismatch(msg) => {
                    write!(f, "Setting type mismatch: {}", msg)
                }
            }
        }
    }

    impl std::error::Error for ValidationError {}

    /// Validate a complete settings tab configuration
    pub fn validate_tab_configuration(
        tab_name: &str,
        settings_type: &str,
        sections: &[crate::settings::setting_builder::generation::SectionConfig],
    ) -> ValidationResult {
        // Validate tab name is a valid identifier
        if !is_valid_identifier(tab_name) {
            return Err(ValidationError::IncompleteTabConfig(format!(
                "Tab name '{}' is not a valid Rust identifier",
                tab_name
            )));
        }

        // Validate settings type exists
        if settings_type.is_empty() {
            return Err(ValidationError::InvalidSettingsStruct(
                "Settings type cannot be empty".to_string(),
            ));
        }

        // Validate each section
        for section in sections {
            validate_section_configuration(section)?;
        }

        // Check for duplicate setting types within the same tab
        let mut setting_types = std::collections::HashSet::new();
        for section in sections {
            for control in &section.controls {
                if !setting_types.insert(control.setting_type) {
                    return Err(ValidationError::SettingTypeMismatch(format!(
                        "Duplicate setting type: {:?}",
                        control.setting_type
                    )));
                }
            }
        }

        Ok(())
    }

    /// Validate a section configuration
    pub fn validate_section_configuration(
        section: &crate::settings::setting_builder::generation::SectionConfig,
    ) -> ValidationResult {
        // Section name validation
        if section.name.is_empty() {
            return Err(ValidationError::InvalidSection(
                "Section name cannot be empty".to_string(),
            ));
        }

        // Must have at least one control
        if section.controls.is_empty() && !section.has_presets {
            return Err(ValidationError::InvalidSection(format!(
                "Section '{}' must have at least one control or presets",
                section.name
            )));
        }

        // Validate each control
        for control in &section.controls {
            validate_control_configuration(control)?;
        }

        Ok(())
    }

    /// Validate a control configuration
    pub fn validate_control_configuration(
        control: &crate::settings::setting_builder::generation::ControlConfig,
    ) -> ValidationResult {
        // Label validation
        if control.label.is_empty() {
            return Err(ValidationError::InvalidControl(
                "Control label cannot be empty".to_string(),
            ));
        }

        // Field name validation
        if !is_valid_identifier(&control.field_name) {
            return Err(ValidationError::InvalidField(format!(
                "Field name '{}' is not a valid identifier",
                control.field_name
            )));
        }

        // Control type specific validation
        match &control.control_type {
            crate::settings::setting_builder::generation::ControlType::Slider {
                min,
                max,
                format,
            } => {
                compile_time::validate_slider_range(*min, *max).map_err(|e| {
                    ValidationError::InvalidControl(format!("Slider '{}': {}", control.label, e))
                })?;

                compile_time::validate_value_format(format, *min, *max).map_err(|e| {
                    ValidationError::InvalidControl(format!("Slider '{}': {}", control.label, e))
                })?;
            }
            crate::settings::setting_builder::generation::ControlType::Presets(presets) => {
                if presets.is_empty() {
                    return Err(ValidationError::InvalidControl(
                        "Presets list cannot be empty".to_string(),
                    ));
                }
                for preset in presets {
                    if !is_valid_identifier(preset) {
                        return Err(ValidationError::InvalidControl(format!(
                            "Preset name '{}' is not a valid identifier",
                            preset
                        )));
                    }
                }
            }
            _ => {} // Cycle and Toggle have no additional validation requirements
        }

        Ok(())
    }

    /// Check if a string is a valid Rust identifier
    fn is_valid_identifier(name: &str) -> bool {
        !name.is_empty()
            && name
                .chars()
                .next()
                .map_or(false, |c| c.is_alphabetic() || c == '_')
            && name.chars().all(|c| c.is_alphanumeric() || c == '_')
    }

    /// Validate that field names correspond to actual settings struct fields
    pub fn validate_field_mapping(field_name: &str, setting_type: SettingType) -> ValidationResult {
        // This would ideally use reflection or compile-time checks
        // to ensure field names match actual struct fields

        let expected_field = match setting_type {
            SettingType::WindowMode => "window_mode",
            SettingType::Resolution => "resolution",
            SettingType::VSync => "vsync",
            SettingType::RenderScale => "render_scale",
            SettingType::ShadowQuality => "shadow_quality",
            SettingType::MasterVolume => "master_volume",
            SettingType::SfxVolume | SettingType::SFXVolume => "sfx_volume",
            SettingType::UiScale | SettingType::UIScale => "ui_scale",
            SettingType::ShowFps | SettingType::ShowFPS => "show_fps",
            SettingType::ShowProvinceInfo => "show_province_info",
            SettingType::ShowTooltips => "show_tooltips",
            SettingType::TooltipDelay => "tooltip_delay",
            SettingType::EdgePanSpeed => "edge_pan_speed",
            SettingType::ZoomSensitivity => "zoom_sensitivity",
            SettingType::InvertZoom => "invert_zoom",
            SettingType::CameraSpeed => "camera_speed",
            SettingType::ZoomSpeed => "zoom_speed",
            SettingType::MuteWhenUnfocused => "mute_when_unfocused",
        };

        if field_name != expected_field {
            return Err(ValidationError::SettingTypeMismatch(format!(
                "Field '{}' does not match expected field '{}' for setting type {:?}",
                field_name, expected_field, setting_type
            )));
        }

        Ok(())
    }
}

/// Configuration validation helpers
pub mod config {
    use super::*;

    /// Validate macro syntax at compile time
    pub fn validate_macro_syntax(input: &str) -> Result<(), String> {
        // Basic syntax validation for the macro input
        // This would be expanded with more sophisticated parsing

        if !input.contains("settings_type:") {
            return Err("Missing required 'settings_type' field".to_string());
        }

        if !input.contains("sections:") {
            return Err("Missing required 'sections' field".to_string());
        }

        Ok(())
    }

    /// Validate section syntax
    pub fn validate_section_syntax(section_input: &str) -> Result<(), String> {
        if !section_input.starts_with("Section(") {
            return Err("Sections must start with 'Section(\"name\")'".to_string());
        }

        Ok(())
    }

    /// Validate control syntax
    pub fn validate_control_syntax(control_input: &str) -> Result<(), String> {
        let valid_controls = ["cycle:", "toggle:", "slider:", "presets:"];

        if !valid_controls
            .iter()
            .any(|prefix| control_input.trim().starts_with(prefix))
        {
            return Err(format!(
                "Invalid control type. Must be one of: {}",
                valid_controls.join(", ")
            ));
        }

        Ok(())
    }
}

/// Helpful error message generators
pub mod error_messages {
    /// Generate helpful error message for invalid settings type
    pub fn invalid_settings_type(type_name: &str) -> String {
        format!(
            "Type '{}' cannot be used as a settings type. \
             Settings types must implement Clone + Debug + PartialEq. \
             Try: #[derive(Clone, Debug, PartialEq)]",
            type_name
        )
    }

    /// Generate helpful error message for invalid field
    pub fn invalid_field(field_name: &str, settings_type: &str) -> String {
        format!(
            "Field '{}' does not exist on settings type '{}'. \
             Check that the field name matches exactly and is public.",
            field_name, settings_type
        )
    }

    /// Generate helpful error message for invalid control type
    pub fn invalid_control_type(control_type: &str) -> String {
        format!(
            "Invalid control type '{}'. \
             Valid control types: cycle, toggle, slider, presets. \
             Check the setting_builder documentation for examples.",
            control_type
        )
    }

    /// Generate helpful error message for slider configuration
    pub fn invalid_slider_config(label: &str, issue: &str) -> String {
        format!(
            "Invalid slider configuration for '{}': {}. \
             Slider format: slider: \"Label\" => field_name (min..max, format)",
            label, issue
        )
    }

    /// Generate helpful error message for malformed section
    pub fn malformed_section(section: &str) -> String {
        format!(
            "Malformed section '{}'. \
             Section format: Section(\"Name\") {{ control declarations }}. \
             Check the setting_builder documentation for examples.",
            section
        )
    }
}
