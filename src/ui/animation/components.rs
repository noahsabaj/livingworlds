//! Component definitions for the animation system

use bevy::prelude::*;
use std::time::Duration;
use super::types::*;

/// Component that defines an animation to be played on an entity
#[derive(Component, Debug, Clone, Reflect)]
pub struct Animation {
    /// Unique ID for this animation
    pub id: AnimationId,
    /// What property to animate
    pub target: AnimationTarget,
    /// How long the animation should take
    pub duration: Duration,
    /// Optional delay before starting
    pub delay: Duration,
    /// Easing function to use (default: linear)
    pub easing: EasingFunction,
    /// How the animation should repeat
    pub repeat_mode: AnimationRepeatMode,
    /// Current state of the animation
    pub state: AnimationState,
    /// Whether to remove the component when complete
    pub auto_cleanup: bool,
}

impl Animation {
    /// Create a new animation
    pub fn new(target: AnimationTarget, duration: Duration) -> Self {
        Self {
            id: AnimationId::new(),
            target,
            duration,
            delay: Duration::ZERO,
            easing: EasingFunction::Linear,
            repeat_mode: AnimationRepeatMode::Once,
            state: AnimationState::Pending,
            auto_cleanup: true,
        }
    }

    /// Create a fade-in animation
    pub fn fade_in(duration: Duration) -> Self {
        Self::new(AnimationTarget::fade_in(), duration)
            .with_easing(EasingFunction::EaseInOut)
    }

    /// Create a fade-out animation
    pub fn fade_out(duration: Duration) -> Self {
        Self::new(AnimationTarget::fade_out(), duration)
            .with_easing(EasingFunction::EaseInOut)
    }

    /// Create a scale-in animation
    pub fn scale_in(duration: Duration) -> Self {
        Self::new(AnimationTarget::scale_in(), duration)
            .with_easing(EasingFunction::BackInOut)
    }

    /// Create a scale-out animation
    pub fn scale_out(duration: Duration) -> Self {
        Self::new(AnimationTarget::scale_out(), duration)
            .with_easing(EasingFunction::BackInOut)
    }

    /// Create a slide animation
    pub fn slide(from: Vec2, to: Vec2, duration: Duration) -> Self {
        Self::new(AnimationTarget::slide(from, to), duration)
            .with_easing(EasingFunction::EaseInOut)
    }

    /// Create a color transition animation
    pub fn color_transition(from: Color, to: Color, duration: Duration) -> Self {
        Self::new(AnimationTarget::color_transition(from, to), duration)
            .with_easing(EasingFunction::EaseInOut)
    }

    /// Set the easing function
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    /// Set the delay before starting
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Set the repeat mode
    pub fn with_repeat(mut self, repeat_mode: AnimationRepeatMode) -> Self {
        self.repeat_mode = repeat_mode;
        self
    }

    /// Disable auto-cleanup (component remains after animation completes)
    pub fn keep_component(mut self) -> Self {
        self.auto_cleanup = false;
        self
    }
}

/// Component that tracks animation playback progress
#[derive(Component, Debug, Clone, Reflect)]
pub struct UIAnimationPlayer {
    /// Time elapsed since animation started (excluding delay)
    pub elapsed: Duration,
    /// Time spent in delay phase
    pub delay_elapsed: Duration,
    /// Current animation direction (for PingPong mode)
    pub direction: AnimationDirection,
    /// Number of loops completed
    pub loops_completed: u32,
    /// Current interpolation value (0.0 to 1.0)
    pub progress: f32,
}

impl Default for UIAnimationPlayer {
    fn default() -> Self {
        Self {
            elapsed: Duration::ZERO,
            delay_elapsed: Duration::ZERO,
            direction: AnimationDirection::Forward,
            loops_completed: 0,
            progress: 0.0,
        }
    }
}

/// Component for sequencing multiple animations
#[derive(Component, Debug, Clone, Reflect)]
pub struct AnimationSequence {
    /// Animations to play in sequence
    pub animations: Vec<Animation>,
    /// Index of current animation
    pub current: usize,
    /// Whether to loop the entire sequence
    pub loop_sequence: bool,
}

impl AnimationSequence {
    /// Create a new animation sequence
    pub fn new(animations: Vec<Animation>) -> Self {
        Self {
            animations,
            current: 0,
            loop_sequence: false,
        }
    }

    /// Create a sequence that loops
    pub fn looping(mut self) -> Self {
        self.loop_sequence = true;
        self
    }

    /// Get the current animation if any
    pub fn current_animation(&self) -> Option<&Animation> {
        self.animations.get(self.current)
    }

    /// Get the current animation mutably if any
    pub fn current_animation_mut(&mut self) -> Option<&mut Animation> {
        self.animations.get_mut(self.current)
    }

    /// Advance to next animation in sequence
    pub fn advance(&mut self) -> bool {
        self.current += 1;
        if self.current >= self.animations.len() {
            if self.loop_sequence {
                self.current = 0;
                true
            } else {
                false
            }
        } else {
            true
        }
    }
}

/// Marker component for animations that have completed
#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct AnimationComplete {
    pub animation_id: AnimationId,
}

/// Bundle for easily adding animations to entities
#[derive(Bundle)]
pub struct AnimationBundle {
    pub animation: Animation,
    pub player: UIAnimationPlayer,
}

impl AnimationBundle {
    /// Create a new animation bundle
    pub fn new(animation: Animation) -> Self {
        Self {
            animation,
            player: UIAnimationPlayer::default(),
        }
    }
}

/// Easing functions for animations
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum EasingFunction {
    /// Linear interpolation
    Linear,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end)
    EaseOut,
    /// Ease in and out (slow start and end)
    EaseInOut,
    /// Bounce effect
    Bounce,
    /// Elastic spring effect
    Elastic,
    /// Back effect (overshoot)
    BackInOut,
    /// Custom cubic bezier curve
    CubicBezier(f32, f32, f32, f32),
}

impl EasingFunction {
    /// Apply the easing function to a progress value
    pub fn ease(&self, t: f32) -> f32 {
        use super::easing;

        let t = t.clamp(0.0, 1.0);

        match self {
            Self::Linear => t,
            Self::EaseIn => easing::ease_in(t),
            Self::EaseOut => easing::ease_out(t),
            Self::EaseInOut => easing::ease_in_out(t),
            Self::Bounce => easing::bounce(t),
            Self::Elastic => easing::elastic(t),
            Self::BackInOut => easing::back_in_out(t),
            Self::CubicBezier(x1, y1, x2, y2) => {
                easing::cubic_bezier(*x1, *y1, *x2, *y2, t)
            }
        }
    }
}