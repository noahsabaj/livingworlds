//! Loading operation types and details

/// The type of loading operation currently in progress
#[derive(Debug, Clone, PartialEq, Default)]
pub enum LoadingOperation {
    /// No operation in progress
    #[default]
    None,

    /// Generating a new world from scratch
    GeneratingWorld,

    /// Loading a saved game
    LoadingSave,

    /// Applying mod configuration changes
    ApplyingMods,
}

/// Operation-specific details for different loading types
///
/// This structure contains contextual information that varies
/// based on the type of loading operation being performed.
#[derive(Debug, Clone, Default)]
pub struct LoadingDetails {
    // World generation specific
    /// Random seed used for world generation
    pub world_seed: Option<u32>,
    /// Size category of the world (Small, Medium, Large)
    pub world_size: Option<String>,

    // Save loading specific
    /// Name of the save file being loaded
    pub save_name: Option<String>,
    /// Game age in days from the save file
    pub game_days: Option<f32>,
    /// Human-readable file size
    pub file_size: Option<String>,
}
