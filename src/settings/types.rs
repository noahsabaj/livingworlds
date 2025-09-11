//! Settings data types and structures
//!
//! This module contains all the core data structures for the settings system,
//! including the main GameSettings, individual setting categories, and various
//! enums for options.

use serde::{Deserialize, Serialize};
use bevy::prelude::*;

// ============================================================================
// MAIN SETTINGS STRUCTURES
// ============================================================================

/// Main settings structure containing all game settings
#[derive(Resource, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameSettings {
    pub graphics: GraphicsSettings,
    pub audio: AudioSettings,
    pub interface: InterfaceSettings,
    pub controls: ControlSettings,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            graphics: GraphicsSettings::default(),
            audio: AudioSettings::default(),
            interface: InterfaceSettings::default(),
            controls: ControlSettings::default(),
        }
    }
}

/// Temporary settings used while editing (before applying)
#[derive(Resource, Clone, Debug)]
pub struct TempGameSettings(pub GameSettings);

impl Default for TempGameSettings {
    fn default() -> Self {
        Self(GameSettings::default())
    }
}

// ============================================================================
// GRAPHICS SETTINGS
// ============================================================================

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub window_mode: WindowModeOption,
    pub resolution: ResolutionOption,
    pub vsync: bool,
    pub render_scale: f32,
    pub shadow_quality: QualityLevel,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            window_mode: WindowModeOption::Windowed,
            resolution: ResolutionOption::default(),
            vsync: true,
            render_scale: 1.0,
            shadow_quality: QualityLevel::Medium,
        }
    }
}

impl GraphicsSettings {
    /// Apply a graphics preset
    pub fn apply_preset(&mut self, preset: GraphicsPreset) {
        match preset {
            GraphicsPreset::Low => {
                self.render_scale = 0.75;
                self.shadow_quality = QualityLevel::Low;
                self.vsync = false;
            }
            GraphicsPreset::Medium => {
                self.render_scale = 0.9;
                self.shadow_quality = QualityLevel::Medium;
                self.vsync = true;
            }
            GraphicsPreset::High => {
                self.render_scale = 1.0;
                self.shadow_quality = QualityLevel::High;
                self.vsync = true;
            }
            GraphicsPreset::Ultra => {
                self.render_scale = 1.0;
                self.shadow_quality = QualityLevel::Ultra;
                self.vsync = true;
            }
        }
    }
    
    /// Determine which preset matches current settings (if any)
    pub fn current_preset(&self) -> Option<GraphicsPreset> {
        if self.render_scale == 0.75 && self.shadow_quality == QualityLevel::Low && !self.vsync {
            Some(GraphicsPreset::Low)
        } else if self.render_scale == 0.9 && self.shadow_quality == QualityLevel::Medium && self.vsync {
            Some(GraphicsPreset::Medium)
        } else if self.render_scale == 1.0 && self.shadow_quality == QualityLevel::High && self.vsync {
            Some(GraphicsPreset::High)
        } else if self.render_scale == 1.0 && self.shadow_quality == QualityLevel::Ultra && self.vsync {
            Some(GraphicsPreset::Ultra)
        } else {
            None // Custom settings
        }
    }
}

/// Graphics quality presets
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GraphicsPreset {
    Low,
    Medium,
    High,
    Ultra,
}

// ============================================================================
// AUDIO SETTINGS
// ============================================================================

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub mute_when_unfocused: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.7,
            sfx_volume: 1.0,
            mute_when_unfocused: false,
        }
    }
}

// ============================================================================
// INTERFACE SETTINGS
// ============================================================================

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterfaceSettings {
    pub ui_scale: f32,
    pub show_fps: bool,
    pub show_province_info: bool,
    pub tooltip_delay: f32,
    pub show_tooltips: bool,
}

impl Default for InterfaceSettings {
    fn default() -> Self {
        Self {
            ui_scale: 1.0,
            show_fps: false,
            show_province_info: true,
            tooltip_delay: 0.5,
            show_tooltips: true,
        }
    }
}

// ============================================================================
// CONTROL SETTINGS
// ============================================================================

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlSettings {
    pub edge_pan_speed: f32,
    pub zoom_sensitivity: f32,
    pub invert_zoom: bool,
    pub camera_speed: f32,
    pub zoom_speed: f32,
}

impl Default for ControlSettings {
    fn default() -> Self {
        Self {
            edge_pan_speed: 1.0,
            zoom_sensitivity: 1.0,
            invert_zoom: false,
            camera_speed: 1.0,
            zoom_speed: 1.0,
        }
    }
}

// ============================================================================
// ENUMS FOR SETTINGS OPTIONS
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum WindowModeOption {
    Windowed,
    Borderless,
    Fullscreen,
}

impl WindowModeOption {
    pub fn cycle(&self) -> Self {
        match self {
            Self::Windowed => Self::Borderless,
            Self::Borderless => Self::Fullscreen,
            Self::Fullscreen => Self::Windowed,
        }
    }
    
    pub fn as_str(&self) -> &str {
        match self {
            Self::Windowed => "Windowed",
            Self::Borderless => "Borderless",
            Self::Fullscreen => "Fullscreen",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ResolutionOption {
    pub width: f32,
    pub height: f32,
}

impl ResolutionOption {
    pub fn common_resolutions() -> Vec<Self> {
        vec![
            Self { width: 1280.0, height: 720.0 },
            Self { width: 1600.0, height: 900.0 },
            Self { width: 1920.0, height: 1080.0 },
            Self { width: 2560.0, height: 1440.0 },
            Self { width: 3840.0, height: 2160.0 },
        ]
    }
    
    pub fn cycle(&self) -> Self {
        let resolutions = Self::common_resolutions();
        let current_idx = resolutions.iter()
            .position(|r| r.width == self.width && r.height == self.height)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % resolutions.len();
        resolutions[next_idx].clone()
    }
    
    pub fn as_str(&self) -> String {
        format!("{}x{}", self.width as i32, self.height as i32)
    }
}

impl Default for ResolutionOption {
    fn default() -> Self {
        Self { width: 1920.0, height: 1080.0 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum QualityLevel {
    Low,
    Medium,
    High,
    Ultra,
}

impl QualityLevel {
    pub fn cycle(&self) -> Self {
        match self {
            Self::Low => Self::Medium,
            Self::Medium => Self::High,
            Self::High => Self::Ultra,
            Self::Ultra => Self::Low,
        }
    }
    
    pub fn as_str(&self) -> &str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Ultra => "Ultra",
        }
    }
}

// ============================================================================
// SETTING TYPES ENUM
// ============================================================================

/// Types of settings that can be modified
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SettingType {
    // Graphics
    WindowMode,
    Resolution,
    VSync,
    RenderScale,
    ShadowQuality,
    // Audio
    MasterVolume,
    MusicVolume,
    SfxVolume,
    MuteWhenUnfocused,
    // Interface
    UiScale,
    ShowFps,
    ShowProvinceInfo,
    TooltipDelay,
    ShowTooltips,
    // Controls
    EdgePanSpeed,
    ZoomSensitivity,
    InvertZoom,
    CameraSpeed,
    ZoomSpeed,
    // Additional compatibility aliases
    SFXVolume,
    UIScale,
    ShowFPS,
}

// ============================================================================
// EVENTS
// ============================================================================

/// Event triggered when settings are changed
#[derive(Event)]
pub struct SettingsChanged;

/// Event for requesting resolution confirmation dialog
#[derive(Event)]
pub struct RequestResolutionConfirm;

// ============================================================================
// RESOURCES
// ============================================================================

/// Tracks whether settings have been modified
#[derive(Resource, Default)]
pub struct SettingsDirtyState {
    pub is_dirty: bool,
}

/// Countdown timer for resolution confirmation
#[derive(Resource)]
pub struct ResolutionConfirmation {
    pub timer: Timer,
    pub original_resolution: ResolutionOption,
    pub original_window_mode: WindowModeOption,
    pub active: bool,
}

impl Default for ResolutionConfirmation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(15.0, TimerMode::Once),
            original_resolution: ResolutionOption::default(),
            original_window_mode: WindowModeOption::Windowed,
            active: false,
        }
    }
}