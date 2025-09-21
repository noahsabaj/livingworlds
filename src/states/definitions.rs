//! Core State Definitions for Living Worlds
//!
//! This module contains all state enums, resources, events, and components
//! that form the vocabulary of the state management system.

use bevy::prelude::*;

/// Main game states that control the overall flow of the application
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    /// Initial loading/splash screen state
    #[default]
    Loading,

    /// Main menu - entry point after loading
    MainMenu,

    /// World configuration - player sets generation parameters
    WorldConfiguration,

    /// World generation - creating the world
    WorldGeneration,

    /// Unified loading screen (for both world generation and save loading)
    LoadingWorld,

    /// Active gameplay - world simulation running
    InGame,

    /// Game is paused (ESC menu)
    Paused,

    /// World generation failed - shows error dialog
    WorldGenerationFailed,
}

/// Sub-states for menu navigation (only active during MainMenu state)
/// Note: For Paused state, we'll use a separate system
#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::MainMenu)]
pub enum MenuState {
    #[default]
    /// No menu active
    None,

    /// Main menu root
    Main,

    /// Settings menu (graphics, audio, etc.)
    Settings,

    /// Load game browser
    LoadBrowser,

    /// Save game dialog
    SaveDialog,

    /// Statistics/Observatory view
    Statistics,

    /// Help/Controls overlay
    Help,

    /// Credits screen
    Credits,

    /// Exit confirmation dialog
    ExitConfirmation,
}

/// Settings menu tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SettingsTab {
    #[default]
    Graphics,
    Audio,
    Interface,
    Performance,
    Controls,
}

/// Resource for tracking the current settings tab
#[derive(Resource, Default)]
pub struct CurrentSettingsTab(pub SettingsTab);

/// Event for requesting state transitions with validation
#[derive(Event)]
pub struct RequestStateTransition {
    pub to: GameState,
    pub from: GameState,
}

/// Event triggered when world generation should begin
#[derive(Event, Default)]
pub struct StartWorldGeneration;

/// Event for menu navigation
#[derive(Event)]
pub enum MenuEvent {
    Open(MenuState),
    Close,
    Back,
}

/// Marker component for the world mesh entity
#[derive(Component)]
pub struct WorldMeshEntity;

/// Resource to track if there's a saved world to continue
#[derive(Resource, Default)]
pub struct SavedWorldExists(pub bool);

/// Resource to track if world generation is in progress
#[derive(Resource, Default)]
pub struct WorldGenerationInProgress(pub bool);

/// Resource to signal that we need to generate a world in LoadingWorld state
#[derive(Resource, Default)]
pub struct PendingWorldGeneration {
    pub pending: bool,
    pub delay_timer: f32, // Delay before starting generation to allow loading screen to render
}
