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
//! ```rust,no_run
//! commands.entity(ui_element)
//!     .insert(Animation::fade_in(Duration::from_secs_f32(0.3)))
//!     .insert(Animation::slide_from(Vec2::new(-300.0, 0.0), Duration::from_secs_f32(0.5)));
//! ```no_run

// GATEWAY ARCHITECTURE - Pure exports only

mod types;
mod components;
mod systems;
mod builder;
mod easing;
mod plugin;

// Core types
pub use types::{
    AnimationTarget, AnimationState, AnimationRepeatMode, AnimationConfig,
};

// Components
pub use components::{
    Animation, UIAnimationPlayer, AnimationSequence, EasingFunction,
};

// Systems (for advanced users who want custom scheduling)

// Builder API
pub use builder::{
    AnimationBuilder, SequenceBuilder,
    presets, AnimationCommandsExt,
};

// Easing functions (for custom easing)

// Plugin
pub use plugin::AnimationPlugin;