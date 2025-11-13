//! Plugin for the notification system.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

use super::systems::*;
use super::types::*;

/// Plugin providing universal notification system
///
/// Handles displaying toasts, banners, and other notifications
/// anywhere in the application via `ShowNotification` events.
///
/// # Usage
///
/// ```rust
/// // From any system:
/// commands.trigger(ShowNotification::warning(
///     "Settings are temporary (persistence disabled)"
/// ));
///
/// // Or custom configuration:
/// commands.trigger(ShowNotification {
///     message: "Feature coming soon!".into(),
///     notification_type: NotificationType::Info,
///     duration: None, // Persistent
///     position: NotificationPosition::Banner,
/// });
/// ```
define_plugin!(NotificationPlugin {
    messages: [ShowNotification],

    startup: [setup_notification_container],

    update: [handle_notification_events, update_toast_timers, cleanup_empty_container]
});
