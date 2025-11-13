//! Shared types for all menu systems
//!
//! This module contains components and types that are used across
//! multiple menu implementations. It provides the common vocabulary
//! for menu interactions and actions.

use bevy::prelude::*;

/// Component for menu buttons that defines their action
#[derive(Component)]
pub struct MenuButton {
    pub action: MenuAction,
    pub enabled: bool,
}

/// Marker component for button text entities
#[derive(Component)]
pub struct ButtonText;

/// Actions that menu buttons can trigger
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuAction {
    // Main menu actions
    NewWorld,
    LoadGame,
    Settings,
    Mods,
    Exit,

    // Pause menu actions
    Resume,
    SaveGame,
    BackToMainMenu,
}

/// Event to trigger settings menu spawning
#[derive(Message)]
pub struct SpawnSettingsMenuEvent;

/// Event to trigger save browser spawning
#[derive(Message)]
pub struct SpawnSaveBrowserEvent;
