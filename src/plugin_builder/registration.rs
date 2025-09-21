//! Registration utilities and helper types for plugin building
//!
//! This module provides utilities that support the macro system,
//! including configuration types and registration helpers.

use bevy::prelude::*;

/// Configuration for a system with optional conditions and ordering
#[derive(Debug, Clone)]
pub struct SystemConfig {
    /// Whether the system should be chained with the previous system
    pub chained: bool,
    /// State condition for the system (if any)
    pub condition: Option<String>,
    /// System ordering relative to other systems
    pub ordering: SystemOrdering,
}

/// System ordering options
#[derive(Debug, Clone, PartialEq)]
pub enum SystemOrdering {
    /// No specific ordering
    None,
    /// Chain with previous system
    Chain,
    /// Run in parallel with other systems
    Parallel,
}

/// State-based schedule configuration
#[derive(Debug, Clone)]
pub struct StateSchedule<S: States> {
    /// The state this schedule applies to
    pub state: S,
    /// Systems to run on entering this state
    pub on_enter: Vec<String>,
    /// Systems to run on exiting this state
    pub on_exit: Vec<String>,
}

/// Main plugin registrar that handles different registration types
pub struct PluginRegistrar;

impl PluginRegistrar {
    /// Register multiple resources with the app
    pub fn register_resources<T>(app: &mut App)
    where
        T: Resource + Default,
    {
        app.init_resource::<T>();
    }

    /// Register multiple events with the app
    pub fn register_events<T>(app: &mut App)
    where
        T: Event,
    {
        app.add_event::<T>();
    }

    /// Register reflection types with the app
    pub fn register_reflection<T>(app: &mut App)
    where
        T: bevy::reflect::Reflect + bevy::reflect::TypePath,
    {
        app.register_type::<T>();
    }

    /// Register a state with the app
    pub fn register_state<S>(app: &mut App)
    where
        S: States + Default,
    {
        app.init_state::<S>();
    }

    /// Register a sub-state with the app
    pub fn register_sub_state<S>(app: &mut App)
    where
        S: SubStates,
    {
        app.add_sub_state::<S>();
    }
}

/// Helper trait for system configuration
pub trait SystemConfigurable {
    /// Apply a condition to this system configuration
    fn with_condition(self, condition: &str) -> Self;

    /// Mark this system as chained
    fn as_chained(self) -> Self;

    /// Mark this system as parallel
    fn as_parallel(self) -> Self;
}

/// Validation utilities for plugin configuration
pub mod validation {
    use super::*;

    /// Validate that a plugin configuration is well-formed
    pub fn validate_plugin_config() -> Result<(), String> {
        // This would contain validation logic for:
        // - Ensuring state types are valid
        // - Checking system compatibility
        // - Validating resource/event types
        Ok(())
    }

    /// Validate that all required systems are properly registered
    pub fn validate_systems() -> Result<(), String> {
        // System validation logic
        Ok(())
    }

    /// Validate that state transitions are properly configured
    pub fn validate_state_config() -> Result<(), String> {
        // State validation logic
        Ok(())
    }
}

/// Documentation and example utilities
pub mod examples {
    //! Example plugin configurations for reference
    use bevy_plugin_builder::define_plugin;
    use bevy::prelude::*;

    // Test resource and event types
    #[derive(Resource, Default)]
    pub struct TestResource {
        pub value: i32,
    }

    #[derive(Event)]
    pub struct TestEvent {
        pub message: String,
    }

    // Test systems
    pub fn test_setup_system() {
        info!("Test plugin setup complete");
    }

    pub fn test_update_system(mut test_resource: ResMut<TestResource>) {
        test_resource.value += 1;
    }

    // Example using our new macro system
    define_plugin!(TestPlugin {
        resources: [TestResource],
        events: [TestEvent],
        startup: [test_setup_system],
        update: [test_update_system]
    });

    /// Example of a simple plugin with basic registration
    pub fn simple_plugin_example() -> &'static str {
        r#"
define_plugin!(SimplePlugin {
    resources: [MyResource],
    events: [MyEvent],
    startup: [setup_system],
    update: [update_system]
});
        "#
    }

    /// Example of a complex plugin with state management
    pub fn complex_plugin_example() -> &'static str {
        r#"
define_plugin!(ComplexPlugin {
    resources: [ComplexResource, AnotherResource],
    events: [ComplexEvent, StateChangedEvent],
    states: [MyGameState],
    sub_states: [MySubState],
    reflect: [MyComponent, MyResource],

    startup: [initialize_complex_system],

    update: [
        regular_update,
        (parallel_system1, parallel_system2).chain(),
        conditional_system.run_if(in_state(MyGameState::Playing))
    ],

    on_enter: {
        MyGameState::Playing => [enter_playing, setup_gameplay],
        MyGameState::Paused => [enter_paused]
    },

    on_exit: {
        MyGameState::Playing => [cleanup_gameplay],
        MyGameState::Paused => [exit_paused]
    },

    custom_init: |app| {
        // Custom initialization logic here
        if some_condition() {
            app.insert_resource(ConditionalResource::default());
        }
    }
});
        "#
    }
}