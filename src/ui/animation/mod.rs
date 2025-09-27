//! Declarative Animation System for Living Worlds
//!
//! This module provides a unified animation system that eliminates the need for
//! manual timer updates and interpolation code scattered throughout the codebase.
//!
//! # Features
//! - Component-based animations that automatically update
//! - Support for transform, color, and opacity animations
//! - Multiple easing functions (linear, ease-in-out, bounce, etc.)
//! - Animation sequences and parallel animations
//! - Auto-cleanup on completion
//!
//! # Usage
//! ```rust
//! commands.entity(ui_element)
//!     .insert(Animation::fade_in(Duration::from_secs_f32(0.3)))
//!     .insert(Animation::slide_from(Vec2::new(-300.0, 0.0), Duration::from_secs_f32(0.5)));
//! ```

// GATEWAY ARCHITECTURE - Pure exports only

mod types;
mod components;
mod systems;
mod builder;
mod easing;
mod plugin;

// Core types
pub use types::{
    AnimationTarget, AnimationState, AnimationRepeatMode,
    AnimationDirection, AnimationId, AnimationValue, AnimationConfig,
};

// Components
pub use components::{
    Animation, UIAnimationPlayer, AnimationSequence,
    AnimationComplete, AnimationBundle, EasingFunction,
};

// Systems (for advanced users who want custom scheduling)
pub use systems::{
    update_animations, cleanup_completed_animations,
    process_animation_sequences, advance_animation_sequences,
    pause_all_animations, resume_all_animations,
    cancel_entity_animations,
};

// Builder API
pub use builder::{
    AnimationBuilder, animate, SequenceBuilder,
    presets, AnimationCommandsExt,
};

// Easing functions (for custom easing)
pub use easing::{
    linear, ease_in, ease_out, ease_in_out,
    bounce, elastic, back_in_out, cubic_bezier,
    exp_in, exp_out, circ_in, circ_out,
    sine_in, sine_out, quart_in, quart_out,
    quint_in, quint_out,
};

// Plugin
pub use plugin::AnimationPlugin;