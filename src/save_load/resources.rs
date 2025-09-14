//! Resource types for save/load system
//!
//! This module defines resources used for managing save/load state.

use super::{SaveGameData, SaveGameInfo, AUTO_SAVE_INTERVAL};
use bevy::prelude::*;

/// Tracks available save files
#[derive(Resource, Default)]
pub struct SaveGameList {
    pub saves: Vec<SaveGameInfo>,
}

/// Auto-save timer resource
#[derive(Resource)]
pub struct AutoSaveTimer {
    pub timer: Timer,
    pub enabled: bool,
}

impl Default for AutoSaveTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(AUTO_SAVE_INTERVAL, TimerMode::Repeating),
            enabled: true,
        }
    }
}

/// Save browser UI state
#[derive(Resource, Default)]
pub struct SaveBrowserState {
    pub is_open: bool,
    pub selected_save: Option<usize>,
}

/// Resource to track save dialog state
#[derive(Resource, Default)]
pub struct SaveDialogState {
    pub is_open: bool,
    pub selected_save: Option<String>,
    pub search_filter: String,
}

/// Pending load data to be applied when LoadingWorld state is entered
#[derive(Resource)]
pub struct PendingLoadData(pub SaveGameData);
