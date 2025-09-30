//! Builder API for creating animations with a fluent interface

use bevy::prelude::*;
use std::time::Duration;
use super::components::*;
use super::types::*;

/// Builder for creating animations with a fluent API
pub struct AnimationBuilder {
    target: AnimationTarget,
    duration: Duration,
    delay: Duration,
    easing: EasingFunction,
    repeat_mode: AnimationRepeatMode,
    auto_cleanup: bool,
}

impl AnimationBuilder {
    /// Create a new animation builder
    pub fn new(target: AnimationTarget, duration: Duration) -> Self {
        Self {
            target,
            duration,
            delay: Duration::ZERO,
            easing: EasingFunction::EaseInOut,
            repeat_mode: AnimationRepeatMode::Once,
            auto_cleanup: true,
        }
    }

    /// Set the delay before the animation starts
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Set the easing function
    pub fn easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    /// Make the animation loop infinitely
    pub fn loop_infinite(mut self) -> Self {
        self.repeat_mode = AnimationRepeatMode::Loop;
        self
    }

    /// Make the animation ping-pong (reverse on completion)
    pub fn ping_pong(mut self) -> Self {
        self.repeat_mode = AnimationRepeatMode::PingPong;
        self
    }

    /// Make the animation repeat a specific number of times
    pub fn repeat(mut self, count: u32) -> Self {
        self.repeat_mode = AnimationRepeatMode::Count(count);
        self
    }

    /// Keep the animation component after completion (don't auto-cleanup)
    pub fn keep_component(mut self) -> Self {
        self.auto_cleanup = false;
        self
    }

    /// Build the animation component
    pub fn build(self) -> Animation {
        Animation {
            id: AnimationId::new(),
            target: self.target,
            duration: self.duration,
            delay: self.delay,
            easing: self.easing,
            repeat_mode: self.repeat_mode,
            state: AnimationState::Pending,
            auto_cleanup: self.auto_cleanup,
        }
    }

    /// Build the animation (automatically includes UIAnimationPlayer via Required Components)
    pub fn bundle(self) -> Animation {
        self.build()
    }
}

/// Helper function to animate any entity
/// Automatically adds UIAnimationPlayer along with Animation
pub fn animate(entity: Entity, commands: &mut Commands, animation: Animation) {
    commands.entity(entity).insert((animation, UIAnimationPlayer::default()));
}

/// Sequence builder for chaining animations
pub struct SequenceBuilder {
    animations: Vec<Animation>,
    loop_sequence: bool,
}

impl SequenceBuilder {
    /// Create a new sequence builder
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            loop_sequence: false,
        }
    }

    /// Add an animation to the sequence
    pub fn then(mut self, animation: Animation) -> Self {
        self.animations.push(animation);
        self
    }

    /// Add a delay between animations
    pub fn wait(mut self, duration: Duration) -> Self {
        // Add a no-op animation that just waits
        self.animations.push(
            Animation::new(AnimationTarget::Opacity { from: 1.0, to: 1.0 }, duration)
        );
        self
    }

    /// Make the entire sequence loop
    pub fn loop_sequence(mut self) -> Self {
        self.loop_sequence = true;
        self
    }

    /// Build the animation sequence
    pub fn build(self) -> AnimationSequence {
        let mut sequence = AnimationSequence::new(self.animations);
        if self.loop_sequence {
            sequence = sequence.looping();
        }
        sequence
    }
}

/// Common animation presets
pub mod presets {
    use super::*;

    /// Fade in animation
    pub fn fade_in(duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(AnimationTarget::fade_in(), duration)
            .easing(EasingFunction::EaseInOut)
    }

    /// Fade out animation
    pub fn fade_out(duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(AnimationTarget::fade_out(), duration)
            .easing(EasingFunction::EaseInOut)
    }

    /// Scale in animation (grow from 0 to full size)
    pub fn scale_in(duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(AnimationTarget::scale_in(), duration)
            .easing(EasingFunction::BackInOut)
    }

    /// Scale out animation (shrink from full size to 0)
    pub fn scale_out(duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(AnimationTarget::scale_out(), duration)
            .easing(EasingFunction::BackInOut)
    }

    /// Slide animation
    pub fn slide(from: Vec2, to: Vec2, duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(AnimationTarget::slide(from, to), duration)
            .easing(EasingFunction::EaseInOut)
    }

    /// Slide in from left
    pub fn slide_in_left(distance: f32, duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(
            AnimationTarget::Position {
                from: Vec3::new(-distance, 0.0, 0.0),
                to: Vec3::ZERO,
            },
            duration,
        )
        .easing(EasingFunction::EaseOut)
    }

    /// Slide in from right
    pub fn slide_in_right(distance: f32, duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(
            AnimationTarget::Position {
                from: Vec3::new(distance, 0.0, 0.0),
                to: Vec3::ZERO,
            },
            duration,
        )
        .easing(EasingFunction::EaseOut)
    }

    /// Slide in from top
    pub fn slide_in_top(distance: f32, duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(
            AnimationTarget::Position {
                from: Vec3::new(0.0, distance, 0.0),
                to: Vec3::ZERO,
            },
            duration,
        )
        .easing(EasingFunction::EaseOut)
    }

    /// Slide in from bottom
    pub fn slide_in_bottom(distance: f32, duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(
            AnimationTarget::Position {
                from: Vec3::new(0.0, -distance, 0.0),
                to: Vec3::ZERO,
            },
            duration,
        )
        .easing(EasingFunction::EaseOut)
    }

    /// Pulse animation (scale up and down)
    pub fn pulse(scale_factor: f32, duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(
            AnimationTarget::Scale {
                from: Vec3::ONE,
                to: Vec3::ONE * scale_factor,
            },
            duration,
        )
        .easing(EasingFunction::EaseInOut)
        .ping_pong()
    }

    /// Shake animation (quick left-right movement)
    pub fn shake(intensity: f32, duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(
            AnimationTarget::Position {
                from: Vec3::new(-intensity, 0.0, 0.0),
                to: Vec3::new(intensity, 0.0, 0.0),
            },
            duration,
        )
        .easing(EasingFunction::EaseInOut)
        .repeat(3)
    }

    /// Color flash animation
    pub fn color_flash(from: Color, to: Color, duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(AnimationTarget::color_transition(from, to), duration)
            .easing(EasingFunction::EaseInOut)
            .ping_pong()
    }

    /// Bounce in animation
    pub fn bounce_in(duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(AnimationTarget::scale_in(), duration)
            .easing(EasingFunction::Bounce)
    }

    /// Elastic scale animation
    pub fn elastic_scale(duration: Duration) -> AnimationBuilder {
        AnimationBuilder::new(AnimationTarget::scale_in(), duration)
            .easing(EasingFunction::Elastic)
    }
}

/// Extension trait for Commands to add animation methods
pub trait AnimationCommandsExt {
    /// Animate an entity with the given animation
    fn animate(&mut self, entity: Entity, animation: Animation);

    /// Start a fade in animation on an entity
    fn fade_in(&mut self, entity: Entity, duration: Duration);

    /// Start a fade out animation on an entity
    fn fade_out(&mut self, entity: Entity, duration: Duration);

    /// Start a scale animation on an entity
    fn scale_to(&mut self, entity: Entity, scale: Vec3, duration: Duration);

    /// Start a position animation on an entity
    fn move_to(&mut self, entity: Entity, position: Vec3, duration: Duration);
}

impl AnimationCommandsExt for Commands<'_, '_> {
    fn animate(&mut self, entity: Entity, animation: Animation) {
        self.entity(entity).insert((animation, UIAnimationPlayer::default()));
    }

    fn fade_in(&mut self, entity: Entity, duration: Duration) {
        self.animate(entity, Animation::fade_in(duration));
    }

    fn fade_out(&mut self, entity: Entity, duration: Duration) {
        self.animate(entity, Animation::fade_out(duration));
    }

    fn scale_to(&mut self, entity: Entity, scale: Vec3, duration: Duration) {
        self.animate(
            entity,
            Animation::new(
                AnimationTarget::Scale {
                    from: Vec3::ONE,
                    to: scale,
                },
                duration,
            ),
        );
    }

    fn move_to(&mut self, entity: Entity, position: Vec3, duration: Duration) {
        self.animate(
            entity,
            Animation::new(
                AnimationTarget::Position {
                    from: Vec3::ZERO,
                    to: position,
                },
                duration,
            ),
        );
    }
}