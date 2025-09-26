//! Test application creation utilities
//!
//! Provides functions to create minimal Bevy apps for testing
//! without requiring graphics, audio, or other heavy subsystems.

use bevy::prelude::*;
use crate::nations::laws::registry::LawRegistry;
use crate::simulation::time::resources::GameTime;

/// Create a minimal Bevy app for testing without graphics/audio
pub fn create_test_app() -> App {
    let mut app = App::new();

    // Use MinimalPlugins for headless testing
    app.add_plugins(MinimalPlugins);

    // Add only essential plugins needed for game logic
    app.insert_resource(GameTime::default())
       .insert_resource(LawRegistry::new());

    app
}

/// Create a test app with law systems configured
pub fn create_law_test_app() -> App {
    let mut app = create_test_app();

    // Add law-specific events
    app.add_event::<crate::nations::laws::types::LawEnactmentEvent>()
       .add_event::<crate::nations::laws::types::LawRepealEvent>();

    // Initialize law registry with test laws
    let mut registry = LawRegistry::new();
    super::fixtures::initialize_test_laws(&mut registry);
    app.insert_resource(registry);

    app
}