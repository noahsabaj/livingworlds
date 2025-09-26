//! Climate effects and rain shadow simulation
//!
//! Applies realistic climate effects including rain shadows from mountains
//! to create desert placement and moisture distribution.

use crate::world::provinces::Province;
use crate::world::terrain::TerrainType;

/// Processes climate effects on provinces
pub struct ClimateProcessor;

impl ClimateProcessor {
    /// Apply rain shadow effect for realistic desert placement
    ///
    /// Converts mid-elevation terrain (Chaparral) to desert when it's in the
    /// "shadow" of mountains, simulating how mountains block moisture.
    pub fn apply_rain_shadow(provinces: &mut Vec<Province>) {
        // Find provinces that should become deserts (in rain shadow)
        let mut to_convert = Vec::new();

        for (idx, province) in provinces.iter().enumerate() {
            // Only consider Chaparral (mid-elevation dry areas)
            if province.terrain != TerrainType::Chaparral {
                continue;
            }

            // Check if there are mountains nearby
            let mut mountain_count = 0;
            let mut _total_neighbors = 0;

            // Check immediate neighbors using precomputed indices
            for &neighbor_idx_opt in &province.neighbor_indices {
                if let Some(neighbor_idx) = neighbor_idx_opt {
                    _total_neighbors += 1;
                    let neighbor = &provinces[neighbor_idx];
                    if neighbor.terrain == TerrainType::Alpine
                        || neighbor.terrain == TerrainType::Tundra
                    {
                        mountain_count += 1;
                    }
                }
            }

            // If surrounded by mountains (rain shadow), convert to desert
            // Also convert if isolated mid-elevation area (likely arid)
            if mountain_count >= 2 || (mountain_count >= 1 && province.elevation.value() > 0.3) {
                to_convert.push(idx);
            }
        }

        // Convert appropriate provinces to desert
        for idx in to_convert {
            provinces[idx].terrain = TerrainType::SubtropicalDesert;
        }
    }

    /// Calculate moisture level based on distance from water
    pub fn calculate_moisture(_province: &Province, water_distance: f32) -> f32 {
        // Moisture decreases with distance from water
        1.0 - (water_distance / 100.0).min(1.0)
    }

    /// Determine if a province should become a desert based on conditions
    pub fn should_be_desert(elevation: f32, moisture: f32, rain_shadow: bool) -> bool {
        // Desert conditions: rain shadow effect or low moisture at mid-elevation
        rain_shadow && moisture < 0.3 || (elevation > 0.2 && elevation < 0.4 && moisture < 0.2)
    }
}