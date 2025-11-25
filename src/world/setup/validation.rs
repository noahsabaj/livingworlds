//! World generation validation utilities
//!
//! Handles validation of world generation settings and error types.

use std::fmt;

use super::super::WorldGenerationSettings;

/// Validation bounds
pub const MIN_CONTINENTS: u32 = 1;
pub const MAX_CONTINENTS: u32 = 100;
pub const MAX_OCEAN_COVERAGE: f32 = 0.95;
pub const MIN_OCEAN_COVERAGE: f32 = 0.05;
pub const MAX_RIVER_DENSITY: f32 = 1.0;
pub const MIN_RIVER_DENSITY: f32 = 0.0;

/// Custom error type for world setup failures
#[derive(Debug)]
pub enum WorldSetupError {
    /// Invalid settings provided
    InvalidSettings(String),
    /// World generation failed
    GenerationFailed(String),
    /// Mesh building failed
    MeshBuildingFailed(String),
    /// Empty world generated
    EmptyWorld,
    /// Resource insertion failed
    ResourceError(String),
}

impl fmt::Display for WorldSetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSettings(msg) => write!(f, "Invalid settings: {}", msg),
            Self::GenerationFailed(msg) => write!(f, "World generation failed: {}", msg),
            Self::MeshBuildingFailed(msg) => write!(f, "Mesh building failed: {}", msg),
            Self::EmptyWorld => write!(f, "Generated world has no provinces"),
            Self::ResourceError(msg) => write!(f, "Resource error: {}", msg),
        }
    }
}

impl std::error::Error for WorldSetupError {}

/// Validates world generation settings
pub fn validate_settings(settings: &WorldGenerationSettings) -> Result<(), WorldSetupError> {
    // Validate ocean coverage
    if !(MIN_OCEAN_COVERAGE..=MAX_OCEAN_COVERAGE).contains(&settings.ocean_coverage) {
        return Err(WorldSetupError::InvalidSettings(format!(
            "Ocean coverage must be between {} and {}",
            MIN_OCEAN_COVERAGE, MAX_OCEAN_COVERAGE
        )));
    }

    // Validate continent count
    if !(MIN_CONTINENTS..=MAX_CONTINENTS).contains(&settings.continent_count) {
        return Err(WorldSetupError::InvalidSettings(format!(
            "Continent count must be between {} and {}",
            MIN_CONTINENTS, MAX_CONTINENTS
        )));
    }

    // Validate river density
    if !(MIN_RIVER_DENSITY..=MAX_RIVER_DENSITY).contains(&settings.river_density) {
        return Err(WorldSetupError::InvalidSettings(format!(
            "River density must be between {} and {}",
            MIN_RIVER_DENSITY, MAX_RIVER_DENSITY
        )));
    }

    // Validate world name
    if settings.world_name.is_empty() {
        return Err(WorldSetupError::InvalidSettings(
            "World name cannot be empty".to_string(),
        ));
    }

    Ok(())
}

/// Count cultures in provinces for debugging
pub fn count_cultures(
    provinces: &[super::super::provinces::Province],
) -> std::collections::HashMap<String, usize> {
    use std::collections::HashMap;

    let mut counts = HashMap::new();

    for province in provinces {
        let culture_name = match province.culture {
            Some(culture) => format!("{:?}", culture),
            None => "None".to_string(),
        };
        *counts.entry(culture_name).or_insert(0) += 1;
    }

    counts
}
