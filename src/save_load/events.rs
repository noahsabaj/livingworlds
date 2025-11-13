//! Event types for save/load system
//!
//! This module defines all events used for triggering save and load operations.

use bevy::prelude::*;
use std::path::PathBuf;

/// Event to trigger saving the game
#[derive(Message)]
pub struct SaveGameEvent {
    pub slot_name: String,
}

/// Event to trigger loading a game
#[derive(Message)]
pub struct LoadGameEvent {
    pub save_path: PathBuf,
}

/// Event sent when save completes
#[derive(Message)]
pub struct SaveCompleteEvent {
    pub success: bool,
    pub message: String,
}

/// Event sent when load completes
#[derive(Message)]
pub struct LoadCompleteEvent {
    pub success: bool,
    pub message: String,
}

/// Event to trigger deleting a save file
#[derive(Message)]
pub struct DeleteSaveEvent {
    pub save_path: PathBuf,
    pub save_name: String,
}

/// Event to open the save dialog
#[derive(Message)]
pub struct OpenSaveDialogEvent;

/// Event to close the save dialog
#[derive(Message)]
pub struct CloseSaveDialogEvent;
