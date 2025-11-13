//! Universal notification system for displaying toasts, banners, and alerts.
//!
//! This module provides a unified way to show notifications to users anywhere
//! in the application. Simply send a `ShowNotification` event and the system
//! will handle displaying it with appropriate styling and behavior.
//!
//! # Examples
//!
//! ## Quick Notifications
//!
//! ```rust,no_run
//! use crate::ui::notifications::ShowNotification;
//!
//! // Temporary info toast (5 seconds)
//! commands.trigger(ShowNotification::info("World generated successfully"));
//!
//! // Warning toast (5 seconds)
//! commands.trigger(ShowNotification::warning("Settings are temporary"));
//!
//! // Error toast (8 seconds)
//! commands.trigger(ShowNotification::error("Failed to load save file"));
//!
//! // Success toast (3 seconds)
//! commands.trigger(ShowNotification::success("Settings applied"));
//! ```no_run
//!
//! ## Feature Flags
//!
//! ```rust,no_run
//! // Show "feature disabled" warning
//! commands.trigger(ShowNotification::feature_disabled(
//!     "Settings persistence",
//!     "pending Bevy 0.17 compatibility"
//! ));
//!
//! // Show "coming soon" banner
//! commands.trigger(ShowNotification::coming_soon("Steam Workshop integration"));
//! ```no_run
//!
//! ## Custom Configuration
//!
//! ```rust,no_run
//! use crate::ui::notifications::*;
//! use std::time::Duration;
//!
//! commands.trigger(ShowNotification {
//!     message: "Custom notification".into(),
//!     notification_type: NotificationType::Info,
//!     duration: Some(Duration::from_secs(10)), // 10 seconds
//!     position: NotificationPosition::BottomRight,
//! });
//! ```no_run
//!
//! # Architecture
//!
//! - **Event-driven**: Send `ShowNotification` events from anywhere
//! - **Auto-dismissal**: Optional timers for temporary notifications
//! - **Manual dismissal**: Click X button or wait for timer
//! - **Z-index layering**: Appears above game UI but below critical dialogs
//! - **Persistent container**: Always present, notifications spawn as children

// Module declarations
mod components;
mod plugin;
mod spawning;
mod styles;
mod systems;
mod types;

// Public API exports
pub use plugin::NotificationPlugin;
pub use types::{NotificationPosition, NotificationType, ShowNotification};

// Internal exports for testing
#[cfg(test)]
pub use components::*;
#[cfg(test)]
pub use spawning::*;
