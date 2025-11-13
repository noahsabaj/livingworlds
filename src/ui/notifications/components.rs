//! Component markers for notification entities.

use bevy::prelude::*;
use std::time::Duration;

/// Marks the root notification container entity
#[derive(Component)]
pub struct NotificationContainer;

/// Marks a notification toast entity
#[derive(Component)]
pub struct NotificationToast {
    /// When this toast was spawned
    pub spawned_at: f64,
}

/// Marks a persistent notification banner entity
#[derive(Component)]
pub struct NotificationBanner;

/// Timer component for auto-dismissing notifications
#[derive(Component)]
pub struct ToastTimer {
    /// How long until this notification should be dismissed
    pub timer: Timer,
}

impl ToastTimer {
    /// Create a new timer with the specified duration
    pub fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

/// Marks the dismiss button on a notification
#[derive(Component)]
pub struct NotificationDismissButton {
    /// The notification entity this button dismisses
    pub target: Entity,
}
