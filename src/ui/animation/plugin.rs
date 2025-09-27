//! Bevy plugin for the animation system

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::systems::*;
use super::components::{Animation, UIAnimationPlayer, AnimationSequence, AnimationComplete};
use super::types::AnimationConfig;

define_plugin!(AnimationPlugin {
    resources: [AnimationConfig],

    reflect: [
        Animation,
        UIAnimationPlayer,
        AnimationSequence,
        AnimationComplete
    ],

    update: [
        (
            // Core animation updates
            update_animations,

            // Sequence processing
            process_animation_sequences,
            advance_animation_sequences,

            // Cleanup
            cleanup_completed_animations,
        ).chain(),  // Run in order for proper sequencing

        // Global config application
        apply_animation_config.run_if(resource_changed::<AnimationConfig>),

        // Debug system (only in debug builds)
        #[cfg(debug_assertions)]
        debug_animations
    ]
});