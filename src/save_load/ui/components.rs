//! UI component markers for save/load system
//!
//! This module contains all the marker components used to identify
//! UI elements in the save/load dialogs.

use bevy::prelude::*;
use std::path::PathBuf;
use super::SaveGameInfo;

// Browser components

/// Marker for save browser UI root
#[derive(Component)]
pub struct SaveBrowserRoot;

/// Marker for save slot buttons
#[derive(Component)]
pub struct SaveSlotButton {
    pub index: usize,
    pub save_info: SaveGameInfo,
}

/// Marker for delete save buttons
#[derive(Component)]
pub struct DeleteSaveButton {
    pub save_path: PathBuf,
    pub save_name: String,
}

/// Marker for load button in browser
#[derive(Component)]
pub struct LoadSelectedButton;

/// Marker for cancel button in browser
#[derive(Component)]
pub struct CancelBrowserButton;

// Save dialog components

/// Marker for save dialog UI
#[derive(Component)]
pub struct SaveDialogRoot;

/// Marker for save name input field
#[derive(Component)]
pub struct SaveNameInput;

/// Marker for save dialog confirm button
#[derive(Component)]
pub struct SaveDialogConfirmButton;

/// Marker for save dialog cancel button
#[derive(Component)]
pub struct SaveDialogCancelButton;

/// Marker for search input in save dialog
#[derive(Component)]
pub struct SaveSearchInput;

/// Marker for existing save list in save dialog
#[derive(Component)]
pub struct ExistingSavesList;

/// Marker for existing save item that can be clicked to overwrite
#[derive(Component)]
pub struct ExistingSaveItem {
    pub save_info: SaveGameInfo,
}

// Delete dialog components

/// Marker for the delete confirmation dialog
#[derive(Component)]
pub struct DeleteConfirmationDialog;

/// Delete confirmation button
#[derive(Component)]
pub struct ConfirmDeleteButton {
    pub save_path: PathBuf,
}

/// Cancel delete button
#[derive(Component)]
pub struct CancelDeleteButton;