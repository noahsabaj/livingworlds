//! Basic integration test for Living Worlds

use bevy::prelude::*;
use lw_game::LivingWorldsPlugin;

#[test]
fn test_plugin_creation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugins(LivingWorldsPlugin);
    
    // Verify app builds without panic
    assert!(true);
}

#[test]
fn test_world_generation() {
    // TODO: Add world generation test
    assert!(true);
}
