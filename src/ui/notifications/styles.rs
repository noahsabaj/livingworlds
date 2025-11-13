//! Styling constants for notifications.

use bevy::prelude::*;

/// Z-index for notification layer (between HUD and modals)
/// Re-exported from main UI styles module
pub const NOTIFICATION_Z_INDEX: i32 = crate::ui::layers::NOTIFICATION;

/// Toast notification dimensions
pub mod toast {
    use bevy::prelude::*;

    /// Maximum width of a toast notification
    pub const MAX_WIDTH: Val = Val::Px(400.0);

    /// Minimum height of a toast notification
    pub const MIN_HEIGHT: Val = Val::Px(60.0);

    /// Padding inside the toast
    pub const PADDING: UiRect = UiRect::all(Val::Px(16.0));

    /// Gap between icon and text
    pub const ICON_GAP: Val = Val::Px(12.0);

    /// Border radius for rounded corners
    pub const BORDER_RADIUS: BorderRadius = BorderRadius::all(Val::Px(8.0));

    /// Icon font size
    pub const ICON_SIZE: f32 = 24.0;

    /// Message text font size
    pub const TEXT_SIZE: f32 = 16.0;

    /// Gap between stacked toasts
    pub const STACK_GAP: Val = Val::Px(12.0);
}

/// Banner notification dimensions
pub mod banner {
    use bevy::prelude::*;

    /// Height of banner
    pub const HEIGHT: Val = Val::Px(48.0);

    /// Padding inside the banner
    pub const PADDING: UiRect = UiRect::all(Val::Px(12.0));

    /// Gap between icon and text
    pub const ICON_GAP: Val = Val::Px(12.0);

    /// Icon font size
    pub const ICON_SIZE: f32 = 20.0;

    /// Message text font size
    pub const TEXT_SIZE: f32 = 14.0;
}

/// Animation durations
pub mod animation {
    use std::time::Duration;

    /// Fade in duration
    pub const FADE_IN: Duration = Duration::from_millis(200);

    /// Fade out duration
    pub const FADE_OUT: Duration = Duration::from_millis(150);

    /// Slide in duration
    pub const SLIDE_IN: Duration = Duration::from_millis(250);
}
