//! Core types for the animation system

use bevy::prelude::*;
use std::time::Duration;

/// Unique identifier for animations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct AnimationId(pub u64);

impl AnimationId {
    /// Generate a new unique animation ID
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for AnimationId {
    fn default() -> Self {
        Self::new()
    }
}

/// What property to animate
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum AnimationTarget {
    /// Animate position (translation)
    Position {
        from: Vec3,
        to: Vec3,
    },
    /// Animate scale
    Scale {
        from: Vec3,
        to: Vec3,
    },
    /// Animate rotation
    Rotation {
        from: Quat,
        to: Quat,
    },
    /// Animate background color for UI elements
    BackgroundColor {
        from: Color,
        to: Color,
    },
    /// Animate text color
    TextColor {
        from: Color,
        to: Color,
    },
    /// Animate opacity (alpha channel)
    Opacity {
        from: f32,
        to: f32,
    },
    /// Animate width for UI elements
    Width {
        from: Val,
        to: Val,
    },
    /// Animate height for UI elements
    Height {
        from: Val,
        to: Val,
    },
}

/// Current state of an animation
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum AnimationState {
    /// Animation is waiting to start
    Pending,
    /// Animation is currently playing
    Playing,
    /// Animation is paused
    Paused,
    /// Animation has completed
    Completed,
    /// Animation was cancelled
    Cancelled,
}

/// How the animation should repeat
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum AnimationRepeatMode {
    /// Play once and stop
    Once,
    /// Loop from start when reaching the end
    Loop,
    /// Reverse direction when reaching the end
    PingPong,
    /// Loop a specific number of times
    Count(u32),
}

/// Direction of animation playback
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum AnimationDirection {
    /// Play forward (from -> to)
    Forward,
    /// Play backward (to -> from)
    Reverse,
}

impl AnimationTarget {
    /// Create a fade-in animation target
    pub fn fade_in() -> Self {
        Self::Opacity { from: 0.0, to: 1.0 }
    }

    /// Create a fade-out animation target
    pub fn fade_out() -> Self {
        Self::Opacity { from: 1.0, to: 0.0 }
    }

    /// Create a scale-in animation target
    pub fn scale_in() -> Self {
        Self::Scale {
            from: Vec3::ZERO,
            to: Vec3::ONE,
        }
    }

    /// Create a scale-out animation target
    pub fn scale_out() -> Self {
        Self::Scale {
            from: Vec3::ONE,
            to: Vec3::ZERO,
        }
    }

    /// Create a slide animation target
    pub fn slide(from: Vec2, to: Vec2) -> Self {
        Self::Position {
            from: from.extend(0.0),
            to: to.extend(0.0),
        }
    }

    /// Create a color transition animation target
    pub fn color_transition(from: Color, to: Color) -> Self {
        Self::BackgroundColor { from, to }
    }

    /// Get the interpolated value at time t (0.0 to 1.0)
    pub fn interpolate(&self, t: f32) -> AnimationValue {
        let t = t.clamp(0.0, 1.0);

        match *self {
            Self::Position { from, to } => {
                AnimationValue::Position(from.lerp(to, t))
            }
            Self::Scale { from, to } => {
                AnimationValue::Scale(from.lerp(to, t))
            }
            Self::Rotation { from, to } => {
                AnimationValue::Rotation(from.slerp(to, t))
            }
            Self::BackgroundColor { from, to } => {
                let from_linear = from.to_linear();
                let to_linear = to.to_linear();
                let color = from_linear.mix(&to_linear, t);
                AnimationValue::BackgroundColor(color.into())
            }
            Self::TextColor { from, to } => {
                let from_linear = from.to_linear();
                let to_linear = to.to_linear();
                let color = from_linear.mix(&to_linear, t);
                AnimationValue::TextColor(color.into())
            }
            Self::Opacity { from, to } => {
                AnimationValue::Opacity(from.lerp(to, t))
            }
            Self::Width { from, to } => {
                AnimationValue::Width(interpolate_val(from, to, t))
            }
            Self::Height { from, to } => {
                AnimationValue::Height(interpolate_val(from, to, t))
            }
        }
    }
}

/// The current value of an animation
#[derive(Debug, Clone, Copy)]
pub enum AnimationValue {
    Position(Vec3),
    Scale(Vec3),
    Rotation(Quat),
    BackgroundColor(Color),
    TextColor(Color),
    Opacity(f32),
    Width(Val),
    Height(Val),
}

/// Helper function to interpolate Val types
fn interpolate_val(from: Val, to: Val, t: f32) -> Val {
    match (from, to) {
        (Val::Px(from_px), Val::Px(to_px)) => {
            Val::Px(from_px.lerp(to_px, t))
        }
        (Val::Percent(from_pct), Val::Percent(to_pct)) => {
            Val::Percent(from_pct.lerp(to_pct, t))
        }
        (Val::Vw(from_vw), Val::Vw(to_vw)) => {
            Val::Vw(from_vw.lerp(to_vw, t))
        }
        (Val::Vh(from_vh), Val::Vh(to_vh)) => {
            Val::Vh(from_vh.lerp(to_vh, t))
        }
        // For mismatched types, just return the target value at t >= 0.5
        _ => if t >= 0.5 { to } else { from }
    }
}

/// Animation system configuration
#[derive(Resource, Debug, Clone)]
pub struct AnimationConfig {
    /// Global time scale for all animations (1.0 = normal speed)
    pub time_scale: f32,
    /// Whether animations are globally paused
    pub paused: bool,
    /// Enable debug visualization of animations
    pub debug_mode: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            time_scale: 1.0,
            paused: false,
            debug_mode: false,
        }
    }
}