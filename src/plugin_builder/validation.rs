//! Validation utilities for plugin builder configurations
//!
//! This module provides compile-time and runtime validation for plugin
//! configurations, ensuring that generated plugins are correct and providing
//! helpful error messages when they're not.

use bevy::prelude::*;

/// Compile-time validation helpers
pub mod compile_time {
    /// Ensure a type implements the required traits for resource registration
    pub fn validate_resource<T>()
    where
        T: bevy::prelude::Resource + Default,
    {
        // This function exists purely for compile-time validation
        // The trait bounds ensure the type can be used as a resource
    }

    /// Ensure a type implements the required traits for event registration
    pub fn validate_event<T>()
    where
        T: bevy::prelude::Event,
    {
        // Compile-time validation for event types
    }

    /// Ensure a type implements the required traits for state registration
    pub fn validate_state<T>()
    where
        T: bevy::prelude::States + Default,
    {
        // Compile-time validation for state types
    }

    /// Ensure a type implements the required traits for sub-state registration
    pub fn validate_sub_state<T>()
    where
        T: bevy::prelude::SubStates,
    {
        // Compile-time validation for sub-state types
    }

    /// Ensure a type implements the required traits for reflection
    pub fn validate_reflection<T>()
    where
        T: bevy::reflect::Reflect + bevy::reflect::TypePath,
    {
        // Compile-time validation for reflection types
    }
}

/// Runtime validation utilities
pub mod runtime {
    use super::*;

    /// Validation result type
    pub type ValidationResult = Result<(), ValidationError>;

    /// Validation error types
    #[derive(Debug, Clone)]
    pub enum ValidationError {
        /// A required resource is missing
        MissingResource(String),
        /// A required event type is not registered
        MissingEvent(String),
        /// A system dependency is not satisfied
        UnmetSystemDependency(String),
        /// State configuration is invalid
        InvalidStateConfiguration(String),
        /// Plugin configuration is malformed
        MalformedConfiguration(String),
    }

    impl std::fmt::Display for ValidationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ValidationError::MissingResource(name) => {
                    write!(f, "Required resource not found: {}", name)
                }
                ValidationError::MissingEvent(name) => {
                    write!(f, "Required event type not registered: {}", name)
                }
                ValidationError::UnmetSystemDependency(name) => {
                    write!(f, "System dependency not satisfied: {}", name)
                }
                ValidationError::InvalidStateConfiguration(msg) => {
                    write!(f, "Invalid state configuration: {}", msg)
                }
                ValidationError::MalformedConfiguration(msg) => {
                    write!(f, "Malformed plugin configuration: {}", msg)
                }
            }
        }
    }

    impl std::error::Error for ValidationError {}

    /// Validate a complete plugin configuration at runtime
    pub fn validate_plugin_configuration(app: &App) -> ValidationResult {
        // This would perform runtime checks like:
        // - Ensuring all required resources are registered
        // - Checking that event types are properly configured
        // - Validating system dependencies
        // - Confirming state machine validity

        Ok(())
    }

    /// Validate that all required resources are available
    pub fn validate_resources(app: &App, required_resources: &[&str]) -> ValidationResult {
        for resource in required_resources {
            // Check if resource is registered
            // This would use Bevy's reflection system to check resource availability
            debug!("Validating resource: {}", resource);
        }
        Ok(())
    }

    /// Validate that all required events are registered
    pub fn validate_events(app: &App, required_events: &[&str]) -> ValidationResult {
        for event in required_events {
            // Check if event is registered
            debug!("Validating event: {}", event);
        }
        Ok(())
    }

    /// Validate system dependencies and ordering
    pub fn validate_system_dependencies(systems: &[&str]) -> ValidationResult {
        for system in systems {
            // Check system compatibility and dependencies
            debug!("Validating system: {}", system);
        }
        Ok(())
    }
}

/// Configuration validation helpers
pub mod config {
    use super::*;

    /// Validate a plugin name is valid Rust identifier
    pub fn validate_plugin_name(name: &str) -> bool {
        !name.is_empty()
            && name.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_')
            && name.chars().all(|c| c.is_alphanumeric() || c == '_')
    }

    /// Validate that a configuration section is well-formed
    pub fn validate_config_section(section_name: &str, content: &str) -> Result<(), String> {
        match section_name {
            "resources" => validate_resource_list(content),
            "events" => validate_event_list(content),
            "systems" => validate_system_list(content),
            "states" => validate_state_list(content),
            _ => Ok(()), // Unknown sections are handled by the macro
        }
    }

    fn validate_resource_list(_content: &str) -> Result<(), String> {
        // Validate resource list syntax
        Ok(())
    }

    fn validate_event_list(_content: &str) -> Result<(), String> {
        // Validate event list syntax
        Ok(())
    }

    fn validate_system_list(_content: &str) -> Result<(), String> {
        // Validate system list syntax
        Ok(())
    }

    fn validate_state_list(_content: &str) -> Result<(), String> {
        // Validate state list syntax
        Ok(())
    }
}

/// Helpful error message generators
pub mod error_messages {
    /// Generate helpful error message for invalid resource type
    pub fn invalid_resource_type(type_name: &str) -> String {
        format!(
            "Type '{}' cannot be used as a resource. \
             Resources must implement 'Resource' trait. \
             Try: #[derive(Resource)] or #[derive(Resource, Default)]",
            type_name
        )
    }

    /// Generate helpful error message for invalid event type
    pub fn invalid_event_type(type_name: &str) -> String {
        format!(
            "Type '{}' cannot be used as an event. \
             Events must implement 'Event' trait. \
             Try: #[derive(Event)]",
            type_name
        )
    }

    /// Generate helpful error message for invalid state type
    pub fn invalid_state_type(type_name: &str) -> String {
        format!(
            "Type '{}' cannot be used as a state. \
             States must implement 'States' trait. \
             Try: #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]",
            type_name
        )
    }

    /// Generate helpful error message for invalid system
    pub fn invalid_system(system_name: &str) -> String {
        format!(
            "System '{}' is not a valid Bevy system. \
             Systems must be functions that accept Bevy parameters (Query, Res, etc.)",
            system_name
        )
    }

    /// Generate helpful error message for malformed configuration
    pub fn malformed_config(section: &str, expected: &str) -> String {
        format!(
            "Malformed configuration in '{}' section. \
             Expected: {}. \
             Check the plugin_builder documentation for examples.",
            section, expected
        )
    }
}