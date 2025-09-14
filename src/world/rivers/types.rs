//! River system data structures

/// River system containing flow and delta information
#[derive(Debug, Clone, Default)]
pub struct RiverSystem {
    /// Province IDs that contain river tiles
    pub river_tiles: Vec<u32>,

    /// Province IDs where rivers meet the ocean (deltas)
    pub delta_tiles: Vec<u32>,

    /// Flow accumulation values per province (for river width)
    pub flow_accumulation: Vec<f32>,

    /// Flow direction for each province (to next downstream province)
    pub flow_direction: Vec<Option<u32>>,
}

impl RiverSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a province contains a river
    pub fn is_river(&self, province_id: u32) -> bool {
        self.river_tiles.contains(&province_id)
    }

    /// Check if a province is a river delta
    pub fn is_delta(&self, province_id: u32) -> bool {
        self.delta_tiles.contains(&province_id)
    }

    /// Get flow accumulation for a province
    pub fn get_flow(&self, province_id: u32) -> f32 {
        self.flow_accumulation
            .get(province_id as usize)
            .copied()
            .unwrap_or(0.0)
    }

    /// Get the downstream province for water flow
    pub fn get_downstream(&self, province_id: u32) -> Option<u32> {
        self.flow_direction
            .get(province_id as usize)
            .and_then(|&dir| dir)
    }

    /// Get the number of river tiles
    pub fn len(&self) -> usize {
        self.river_tiles.len()
    }
}
