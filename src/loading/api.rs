//! Public API functions for external loading system integration

use super::state::{LoadingDetails, LoadingOperation, LoadingState};

/// Update the loading state from external systems
///
/// This is the primary function for external systems to report
/// loading progress and update the status message displayed to users.
///
/// # Arguments
/// * `loading_state` - Mutable reference to the LoadingState resource
/// * `progress` - Progress value from 0.0 to 1.0 (automatically clamped)
/// * `message` - Status message to display to the user
pub fn set_loading_progress(
    loading_state: &mut LoadingState,
    progress: f32,
    message: impl Into<String>,
) {
    loading_state.progress = progress.clamp(0.0, 1.0);
    loading_state.current_step = message.into();
}

/// Start a world generation loading operation
///
/// Initializes the loading state for world generation with the
/// specified parameters. This sets up the UI to display world
/// generation specific information.
///
/// # Arguments
/// * `loading_state` - Mutable reference to the LoadingState resource
/// * `seed` - Random seed used for world generation
/// * `size` - Human-readable world size description
pub fn start_world_generation_loading(loading_state: &mut LoadingState, seed: u32, size: String) {
    loading_state.operation = LoadingOperation::GeneratingWorld;
    loading_state.progress = 0.0;
    loading_state.current_step = "Initializing world generation...".to_string();
    loading_state.details = LoadingDetails {
        world_seed: Some(seed),
        world_size: Some(size),
        save_name: None,
        game_days: None,
        file_size: None,
    };
}

/// Start a save loading operation
///
/// Initializes the loading state for save file loading with the
/// specified parameters. This sets up the UI to display save
/// loading specific information.
///
/// # Arguments
/// * `loading_state` - Mutable reference to the LoadingState resource
/// * `save_name` - Name of the save file being loaded
/// * `game_days` - Age of the world in game days
/// * `file_size` - Human-readable file size description
pub fn start_save_loading(
    loading_state: &mut LoadingState,
    save_name: String,
    game_days: f32,
    file_size: String,
) {
    loading_state.operation = LoadingOperation::LoadingSave;
    loading_state.progress = 0.0;
    loading_state.current_step = "Reading save file...".to_string();
    loading_state.details = LoadingDetails {
        world_seed: None,
        world_size: None,
        save_name: Some(save_name),
        game_days: Some(game_days),
        file_size: Some(file_size),
    };
}

/// Start a mod application loading operation
///
/// Initializes the loading state for applying mod configuration
/// changes. This sets up the UI to display mod application
/// specific information.
///
/// # Arguments
/// * `loading_state` - Mutable reference to the LoadingState resource
pub fn start_mod_application_loading(loading_state: &mut LoadingState) {
    loading_state.operation = LoadingOperation::ApplyingMods;
    loading_state.progress = 0.0;
    loading_state.current_step = "Applying mod configuration...".to_string();
    loading_state.details = LoadingDetails {
        world_seed: None,
        world_size: None,
        save_name: None,
        game_days: None,
        file_size: None,
    };
}
