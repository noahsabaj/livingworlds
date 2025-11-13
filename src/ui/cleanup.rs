//! UI Cleanup Utilities
//!
//! Generic systems for cleaning up UI entities following Living Worlds' zero-duplication philosophy.
//! This module eliminates the need for duplicate cleanup functions across UI modules.

use bevy::prelude::*;

/// Generic system to despawn all entities with a specific root component.
///
/// This function eliminates duplicate cleanup boilerplate across all UI systems.
/// Use with any component that marks UI root entities for automatic cleanup.
///
/// # Examples
/// ```ignore
/// // In define_plugin! macro:
/// on_exit: {
///     GameState::Settings => [despawn_entities::<SettingsMenuRoot>]
/// }
///
/// // Or as a standalone system:
/// app.add_systems(OnExit(GameState::Menu), despawn_entities::<MenuRoot>);
/// ```ignore
///
/// # Performance
/// This is a zero-cost abstraction - compiles to identical assembly as manual implementations.
///
/// # Type Safety
/// The generic parameter ensures compile-time verification that only valid component types are used.
pub fn despawn_entities<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// Note: despawn_ui_entities is now provided by bevy-ui-builders crate
// This module only provides the generic despawn_entities function for non-UI cleanup

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component)]
    struct TestUIRoot;

    #[test]
    fn test_despawn_entities_compiles() {
        // This test ensures the generic function compiles correctly
        // Runtime testing happens in integration tests
        let _system = despawn_entities::<TestUIRoot>;
        // despawn_ui_entities is provided by bevy-ui-builders crate
        // but we can use despawn_entities as a local alternative
        let _alias = despawn_entities::<TestUIRoot>;
    }
}
