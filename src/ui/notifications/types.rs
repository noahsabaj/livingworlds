//! Notification types and events.

use bevy::prelude::*;
use std::time::Duration;

/// Event to trigger a notification display.
///
/// Send this event from anywhere to show a notification to the user.
///
/// # Examples
///
/// ```rust
/// // Show a temporary warning
/// commands.trigger(ShowNotification {
///     message: "Settings are temporary (persistence disabled)".into(),
///     notification_type: NotificationType::Warning,
///     duration: Some(Duration::from_secs(5)),
///     position: NotificationPosition::TopCenter,
/// });
///
/// // Show a persistent banner
/// commands.trigger(ShowNotification {
///     message: "Steam Workshop integration coming soon".into(),
///     notification_type: NotificationType::Info,
///     duration: None, // Persistent until dismissed
///     position: NotificationPosition::Banner,
/// });
/// ```
#[derive(Message, Clone)]
pub struct ShowNotification {
    /// The message to display
    pub message: String,

    /// Visual style and semantic meaning
    pub notification_type: NotificationType,

    /// How long to show the notification
    /// - Some(duration): Auto-dismiss after duration
    /// - None: Persistent until user dismisses
    pub duration: Option<Duration>,

    /// Where to position the notification
    pub position: NotificationPosition,
}

impl ShowNotification {
    /// Quick constructor for temporary info toast (5 seconds, top center)
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            notification_type: NotificationType::Info,
            duration: Some(Duration::from_secs(5)),
            position: NotificationPosition::TopCenter,
        }
    }

    /// Quick constructor for temporary warning toast (5 seconds, top center)
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            notification_type: NotificationType::Warning,
            duration: Some(Duration::from_secs(5)),
            position: NotificationPosition::TopCenter,
        }
    }

    /// Quick constructor for temporary error toast (8 seconds, top center)
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            notification_type: NotificationType::Error,
            duration: Some(Duration::from_secs(8)),
            position: NotificationPosition::TopCenter,
        }
    }

    /// Quick constructor for temporary success toast (3 seconds, bottom right)
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            notification_type: NotificationType::Success,
            duration: Some(Duration::from_secs(3)),
            position: NotificationPosition::BottomRight,
        }
    }

    /// Quick constructor for persistent banner (until dismissed)
    pub fn banner(message: impl Into<String>, notification_type: NotificationType) -> Self {
        Self {
            message: message.into(),
            notification_type,
            duration: None,
            position: NotificationPosition::Banner,
        }
    }

    /// Quick constructor for "feature disabled" warning
    pub fn feature_disabled(feature: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::warning(format!("{} disabled ({})", feature.into(), reason.into()))
    }

    /// Quick constructor for "coming soon" info banner
    pub fn coming_soon(feature: impl Into<String>) -> Self {
        Self::banner(
            format!("{} coming soon", feature.into()),
            NotificationType::Info,
        )
    }
}

/// Visual style and semantic meaning of a notification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    /// Blue - General informational messages
    Info,

    /// Yellow - Warnings or degraded functionality
    Warning,

    /// Red - Errors or failures
    Error,

    /// Green - Successful operations
    Success,
}

impl NotificationType {
    /// Get the background color for this notification type
    pub fn background_color(&self) -> Color {
        match self {
            Self::Info => Color::srgba(0.2, 0.4, 0.8, 0.95),       // Blue
            Self::Warning => Color::srgba(0.9, 0.7, 0.2, 0.95),    // Yellow
            Self::Error => Color::srgba(0.8, 0.2, 0.2, 0.95),      // Red
            Self::Success => Color::srgba(0.2, 0.7, 0.3, 0.95),    // Green
        }
    }

    /// Get the text color for this notification type
    pub fn text_color(&self) -> Color {
        match self {
            Self::Info => Color::WHITE,
            Self::Warning => Color::srgb(0.1, 0.1, 0.1),  // Dark text on yellow
            Self::Error => Color::WHITE,
            Self::Success => Color::WHITE,
        }
    }

    /// Get the icon text for this notification type
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Info => "ℹ",
            Self::Warning => "⚠",
            Self::Error => "✖",
            Self::Success => "✓",
        }
    }
}

/// Where to position the notification on screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationPosition {
    /// Top center - good for general notifications
    TopCenter,

    /// Bottom right - good for success confirmations
    BottomRight,

    /// Full-width banner at top - good for persistent "coming soon" messages
    Banner,
}
