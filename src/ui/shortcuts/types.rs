//! Core types for the shortcuts system

use bevy::prelude::*;
use std::fmt;

/// Unique identifier for a keyboard shortcut
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShortcutId {
    // File operations
    SaveGame,
    LoadGame,
    QuickSave,
    QuickLoad,
    NewWorld,

    // Camera controls
    CameraUp,
    CameraDown,
    CameraLeft,
    CameraRight,
    CameraZoomIn,
    CameraZoomOut,
    CameraReset,

    // Time controls
    Pause,
    SpeedUp,
    SlowDown,
    NormalSpeed,
    Speed1,
    Speed2,
    Speed3,
    Speed4,
    Speed5,

    // UI toggles
    ToggleHud,
    ToggleGrid,
    ToggleBorders,
    ToggleFps,
    ToggleDebug,
    ToggleDebugOverlay,
    ToggleFullscreen,

    // Map modes
    MapModePolitical,
    MapModeTerrain,
    MapModeMineral,
    MapModeInfrastructure,
    MapModeCultural,
    MapModeReligious,
    MapModeToggle,  // Quick toggle between Political and Terrain

    // Menus
    OpenMainMenu,
    OpenSettings,
    OpenHelp,
    CloseDialog,
    Escape,

    // Settings navigation
    SettingsNavigateNext,
    SettingsNavigatePrevious,
    SettingsActivate,
    SettingsActivateSpace, // Alternative activation with Space

    // Selection
    SelectNext,
    SelectPrevious,
    DeselectAll,

    // Dropdown navigation
    DropdownUp,
    DropdownDown,
    DropdownSelect,

    // Screenshots
    TakeScreenshot,
    RecordVideo,

    // Developer
    OpenConsole,
    ReloadUI,

    // Debug shortcuts (law system)
    DebugForceEnactLaw,
    DebugForceRepealLaw,
    DebugTriggerProposal,

    // Custom shortcuts (for mods)
    Custom(String),
}

impl fmt::Display for ShortcutId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(id) => write!(f, "Custom({})", id),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Modifier keys that can be combined with a key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ModifierKeys {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool, // Command on macOS
}

impl ModifierKeys {
    /// Create with no modifiers
    pub fn none() -> Self {
        Self::default()
    }

    /// Create with Ctrl
    pub fn ctrl() -> Self {
        Self {
            ctrl: true,
            ..Default::default()
        }
    }

    /// Create with Shift
    pub fn shift() -> Self {
        Self {
            shift: true,
            ..Default::default()
        }
    }

    /// Create with Alt
    pub fn alt() -> Self {
        Self {
            alt: true,
            ..Default::default()
        }
    }

    /// Add Ctrl to modifiers
    pub fn with_ctrl(mut self) -> Self {
        self.ctrl = true;
        self
    }

    /// Add Shift to modifiers
    pub fn with_shift(mut self) -> Self {
        self.shift = true;
        self
    }

    /// Add Alt to modifiers
    pub fn with_alt(mut self) -> Self {
        self.alt = true;
        self
    }

    /// Check if current modifiers match required
    pub fn matches(&self, keyboard: &ButtonInput<KeyCode>) -> bool {
        let ctrl_match = self.ctrl == (keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight));
        let shift_match = self.shift == (keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight));
        let alt_match = self.alt == (keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight));

        ctrl_match && shift_match && alt_match
    }

    /// Get display string for modifiers
    pub fn display_string(&self) -> String {
        let mut parts = Vec::new();
        if self.ctrl { parts.push("Ctrl"); }
        if self.shift { parts.push("Shift"); }
        if self.alt { parts.push("Alt"); }
        if self.meta { parts.push("Cmd"); }
        parts.join("+")
    }
}

/// A keyboard binding (key + modifiers)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub key: KeyCode,
    pub modifiers: ModifierKeys,
}

impl KeyBinding {
    /// Create a single key binding without modifiers
    pub fn single(key: KeyCode) -> Self {
        Self {
            key,
            modifiers: ModifierKeys::none(),
        }
    }

    /// Add Ctrl modifier
    pub fn with_ctrl(mut self) -> Self {
        self.modifiers.ctrl = true;
        self
    }

    /// Add Shift modifier
    pub fn with_shift(mut self) -> Self {
        self.modifiers.shift = true;
        self
    }

    /// Add Alt modifier
    pub fn with_alt(mut self) -> Self {
        self.modifiers.alt = true;
        self
    }

    /// Check if this binding is currently pressed
    pub fn is_pressed(&self, keyboard: &ButtonInput<KeyCode>) -> bool {
        keyboard.pressed(self.key) && self.modifiers.matches(keyboard)
    }

    /// Check if this binding was just pressed
    pub fn just_pressed(&self, keyboard: &ButtonInput<KeyCode>) -> bool {
        keyboard.just_pressed(self.key) && self.modifiers.matches(keyboard)
    }

    /// Get display string for the binding
    pub fn display_string(&self) -> String {
        let key_str = format!("{:?}", self.key).replace("Key", "");

        if self.modifiers == ModifierKeys::none() {
            key_str
        } else {
            format!("{}+{}", self.modifiers.display_string(), key_str)
        }
    }
}

/// Context in which a shortcut is active
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ShortcutContext {
    /// Active everywhere
    #[default]
    Global,
    /// Only in main menu
    MainMenu,
    /// Only during gameplay
    InGame,
    /// Only when a dialog is open
    Dialog,
    /// Only in world configuration
    WorldConfig,
    /// Only in settings
    Settings,
    /// Custom context (for mods)
    Custom(&'static str),
}

impl ShortcutContext {
    /// Check if this context matches another context
    pub fn matches(&self, other: &ShortcutContext) -> bool {
        match (self, other) {
            (Self::Global, _) | (_, Self::Global) => true,
            (a, b) => a == b,
        }
    }
}

/// Action to perform when shortcut is triggered
pub enum ShortcutAction {
    /// Send an event
    Event(Box<dyn Fn(&mut World) + Send + Sync>),
    /// Run a system
    System(Box<dyn System<In = (), Out = ()>>),
    /// Just track that it was pressed (manual handling)
    Track,
}

/// Event sent when a shortcut is triggered
#[derive(Message, Debug, Clone)]
pub struct ShortcutEvent {
    pub shortcut_id: ShortcutId,
    pub binding: KeyBinding,
    pub context: ShortcutContext,
}

/// Resource for configuring the shortcut system
#[derive(Resource, Debug, Clone)]
pub struct ShortcutConfig {
    /// Enable shortcut processing
    pub enabled: bool,
    /// Show shortcut hints in UI
    pub show_hints: bool,
    /// Allow runtime rebinding
    pub allow_rebinding: bool,
    /// Check for conflicts on registration
    pub check_conflicts: bool,
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            show_hints: true,
            allow_rebinding: false,
            check_conflicts: true,
        }
    }
}