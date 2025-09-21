//! UI component markers for loading screen elements

use bevy::prelude::*;

/// Marker component for the root loading screen container
#[derive(Component)]
pub struct LoadingScreenRoot;

/// Marker component for the progress bar element
#[derive(Component)]
pub struct LoadingProgressBar;

/// Marker component for the status text element
#[derive(Component)]
pub struct LoadingStatusText;

/// Marker component for the cancel generation button
#[derive(Component)]
pub struct CancelGenerationButton;
