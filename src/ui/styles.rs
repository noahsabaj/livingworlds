//! Standardized UI styles and constants for Living Worlds
//! 
//! This module provides a single source of truth for all UI styling,
//! ensuring visual consistency across the entire game interface.


/// Standardized color palette with semantic naming
pub mod colors {
    use bevy::prelude::Color;
    
    // Primary colors (blue theme for main actions)
    pub const PRIMARY: Color = Color::srgb(0.2, 0.4, 0.6);
    pub const PRIMARY_HOVER: Color = Color::srgb(0.25, 0.45, 0.65);
    pub const PRIMARY_PRESSED: Color = Color::srgb(0.15, 0.35, 0.55);
    
    // Secondary colors (gray theme for secondary actions)
    pub const SECONDARY: Color = Color::srgb(0.15, 0.15, 0.18);
    pub const SECONDARY_HOVER: Color = Color::srgb(0.2, 0.2, 0.23);
    pub const SECONDARY_PRESSED: Color = Color::srgb(0.1, 0.1, 0.13);
    
    // Danger colors (red theme for destructive actions)
    pub const DANGER: Color = Color::srgb(0.5, 0.2, 0.2);
    pub const DANGER_HOVER: Color = Color::srgb(0.6, 0.25, 0.25);
    pub const DANGER_PRESSED: Color = Color::srgb(0.4, 0.15, 0.15);
    
    // Success colors (green theme for positive actions)
    pub const SUCCESS: Color = Color::srgb(0.2, 0.4, 0.2);
    pub const SUCCESS_HOVER: Color = Color::srgb(0.25, 0.45, 0.25);
    pub const SUCCESS_PRESSED: Color = Color::srgb(0.15, 0.35, 0.15);
    
    // Warning colors (yellow theme for cautionary actions)
    pub const WARNING: Color = Color::srgb(0.5, 0.4, 0.2);
    pub const WARNING_HOVER: Color = Color::srgb(0.55, 0.45, 0.25);
    pub const WARNING_PRESSED: Color = Color::srgb(0.45, 0.35, 0.15);
    
    // UI background colors
    pub const BACKGROUND_DARK: Color = Color::srgb(0.05, 0.05, 0.05);
    pub const BACKGROUND_DARKER: Color = Color::srgb(0.03, 0.03, 0.03);  // Even darker than DARK
    pub const BACKGROUND_MEDIUM: Color = Color::srgb(0.08, 0.08, 0.1);
    pub const BACKGROUND_LIGHT: Color = Color::srgb(0.12, 0.12, 0.15);
    pub const SURFACE: Color = Color::srgb(0.1, 0.1, 0.12);  // Surface color for panels
    
    // Overlay colors
    pub const OVERLAY_DARK: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);
    pub const OVERLAY_MEDIUM: Color = Color::srgba(0.0, 0.0, 0.0, 0.5);
    pub const OVERLAY_LIGHT: Color = Color::srgba(0.0, 0.0, 0.0, 0.3);
    
    pub const TEXT_PRIMARY: Color = Color::srgb(0.9, 0.9, 0.9);
    pub const TEXT_SECONDARY: Color = Color::srgb(0.7, 0.7, 0.7);
    pub const TEXT_TERTIARY: Color = Color::srgb(0.5, 0.5, 0.5);
    pub const TEXT_MUTED: Color = Color::srgb(0.5, 0.5, 0.5);
    pub const TEXT_TITLE: Color = Color::srgb(0.9, 0.85, 0.7);
    
    // Border colors
    pub const BORDER_DEFAULT: Color = Color::srgb(0.3, 0.3, 0.35);
    pub const BORDER: Color = Color::srgb(0.3, 0.3, 0.35);   // Alias for BORDER_DEFAULT
    pub const BORDER_HOVER: Color = Color::srgb(0.4, 0.4, 0.45);
    pub const BORDER_ACTIVE: Color = Color::srgb(0.5, 0.5, 0.55);
}

/// Standardized dimensions for UI elements
pub mod dimensions {
    // Border widths (STANDARDIZED to 2px everywhere)
    pub const BORDER_WIDTH: f32 = 2.0;
    pub const BORDER_WIDTH_THIN: f32 = 1.0;
    pub const BORDER_WIDTH_THICK: f32 = 3.0;
    
    pub const BUTTON_HEIGHT: f32 = 45.0;
    pub const BUTTON_HEIGHT_SMALL: f32 = 35.0;
    pub const BUTTON_HEIGHT_LARGE: f32 = 55.0;
    
    pub const BUTTON_WIDTH_SMALL: f32 = 120.0;
    pub const BUTTON_WIDTH_MEDIUM: f32 = 160.0;
    pub const BUTTON_WIDTH_LARGE: f32 = 200.0;
    pub const BUTTON_WIDTH_XLARGE: f32 = 280.0;  // Menu buttons need this width
    
    // Dialog dimensions
    pub const DIALOG_WIDTH_SMALL: f32 = 350.0;
    pub const DIALOG_WIDTH_MEDIUM: f32 = 450.0;
    pub const DIALOG_WIDTH_LARGE: f32 = 550.0;
    pub const DIALOG_PADDING: f32 = 30.0;
    pub const DIALOG_SPACING: f32 = 20.0;
    
    // Font sizes
    pub const FONT_SIZE_SMALL: f32 = 14.0;
    pub const FONT_SIZE_NORMAL: f32 = 18.0;
    pub const FONT_SIZE_MEDIUM: f32 = 20.0;
    pub const FONT_SIZE_LARGE: f32 = 24.0;
    pub const FONT_SIZE_XLARGE: f32 = 28.0;
    pub const FONT_SIZE_TITLE: f32 = 32.0;
    pub const FONT_SIZE_HEADER: f32 = 48.0;
    pub const FONT_SIZE_HERO: f32 = 72.0;
    
    // Margins and padding
    pub const MARGIN_SMALL: f32 = 5.0;
    pub const MARGIN_MEDIUM: f32 = 10.0;
    pub const MARGIN_LARGE: f32 = 15.0;
    pub const MARGIN_XLARGE: f32 = 20.0;
    pub const MARGIN_XXLARGE: f32 = 30.0;
    pub const SEPARATOR_MARGIN: f32 = 8.0;  // Specific margin for separators

    // Padding values
    pub const PADDING_SMALL: f32 = 5.0;
    pub const PADDING_MEDIUM: f32 = 10.0;
    pub const PADDING_LARGE: f32 = 15.0;
    pub const PANEL_PADDING: f32 = 12.0;  // Standard padding for panels
    
    // Corner radius (for future rounded corners)
    pub const CORNER_RADIUS: f32 = 4.0;
}

/// Z-index layers for proper UI stacking
pub mod layers {
    pub const GAME_UI: i32 = 100;          // HUD, province info, etc.
    pub const MENU_BACKGROUND: i32 = 150;  // Menu backgrounds
    pub const MENU_CONTENT: i32 = 160;     // Menu buttons and content
    pub const SETTINGS: i32 = 200;         // Settings menu
    pub const MODAL_OVERLAY: i32 = 300;    // Modal dialog overlays
    pub const MODAL_CONTENT: i32 = 350;    // Modal dialog content
    pub const CRITICAL_DIALOG: i32 = 400;  // Exit confirmation, errors
    pub const TOOLTIP: i32 = 500;          // Tooltips above everything
}

/// Animation durations for transitions
pub mod animations {
    use std::time::Duration;
    
    pub const HOVER_TRANSITION: Duration = Duration::from_millis(150);
    pub const FADE_IN: Duration = Duration::from_millis(200);
    pub const FADE_OUT: Duration = Duration::from_millis(150);
    pub const DIALOG_APPEAR: Duration = Duration::from_millis(100);
    pub const DIALOG_DISMISS: Duration = Duration::from_millis(100);
}

/// Helper functions for creating styled UI elements
pub mod helpers {
    use bevy::prelude::*;
    
    use super::dimensions;
    
    /// Creates a standard UiRect for borders
    pub fn standard_border() -> UiRect {
        UiRect::all(Val::Px(dimensions::BORDER_WIDTH))
    }
    
    /// Creates a standard UiRect for padding
    pub fn standard_padding() -> UiRect {
        UiRect::all(Val::Px(dimensions::DIALOG_PADDING))
    }
    
    /// Creates a standard button node
    pub fn button_node(width: f32) -> Node {
        Node {
            width: Val::Px(width),
            height: Val::Px(dimensions::BUTTON_HEIGHT),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: standard_border(),
            ..default()
        }
    }
    
    /// Creates a standard dialog overlay node
    pub fn overlay_node() -> Node {
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }
    
    /// Creates a standard dialog container node
    pub fn dialog_container_node(width: f32) -> Node {
        Node {
            width: Val::Px(width),
            padding: standard_padding(),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            border: standard_border(),
            ..default()
        }
    }
}