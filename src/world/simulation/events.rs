//! Province change event system
//!
//! This module provides an event-driven architecture for tracking changes to provinces.
//! Instead of polling dirty flags, systems can react to specific change events,
//! improving performance and enabling better debugging and replay capabilities.

use crate::components::MineralType;
use crate::world::TerrainType;
use crate::world::{Abundance, Agriculture, Distance, ProvinceId};
use bevy::prelude::*;

/// Event fired when a province's population changes
#[derive(Event, Debug, Clone)]
pub struct ProvincePopulationChanged {
    pub province_id: ProvinceId,
    pub old_population: u32,
    pub new_population: u32,
    pub max_population: u32,
}

impl ProvincePopulationChanged {
    /// Calculate the percentage change in population
    pub fn percentage_change(&self) -> f32 {
        if self.old_population == 0 {
            100.0
        } else {
            ((self.new_population as f32 - self.old_population as f32) / self.old_population as f32)
                * 100.0
        }
    }

    /// Check if this was a growth event
    pub fn is_growth(&self) -> bool {
        self.new_population > self.old_population
    }

    /// Check if population is near capacity
    pub fn is_near_capacity(&self) -> bool {
        self.new_population as f32 >= self.max_population as f32 * 0.9
    }
}

/// Event fired when a province's terrain type changes
#[derive(Event, Debug, Clone)]
pub struct ProvinceTerrainChanged {
    pub province_id: ProvinceId,
    pub old_terrain: TerrainType,
    pub new_terrain: TerrainType,
}

impl ProvinceTerrainChanged {
    /// Check if this represents desertification
    pub fn is_desertification(&self) -> bool {
        matches!(
            self.new_terrain,
            TerrainType::ColdDesert
                | TerrainType::SubtropicalDesert
                | TerrainType::TropicalDesert
                | TerrainType::PolarDesert
        ) && !matches!(
            self.old_terrain,
            TerrainType::ColdDesert
                | TerrainType::SubtropicalDesert
                | TerrainType::TropicalDesert
                | TerrainType::PolarDesert
        )
    }

    /// Check if this represents deforestation
    pub fn is_deforestation(&self) -> bool {
        matches!(
            self.old_terrain,
            TerrainType::TropicalRainforest
                | TerrainType::TropicalSeasonalForest
                | TerrainType::TemperateRainforest
                | TerrainType::TemperateDeciduousForest
                | TerrainType::MediterraneanForest
                | TerrainType::BorealForest
                | TerrainType::Taiga
        ) && !matches!(
            self.new_terrain,
            TerrainType::TropicalRainforest
                | TerrainType::TropicalSeasonalForest
                | TerrainType::TemperateRainforest
                | TerrainType::TemperateDeciduousForest
                | TerrainType::MediterraneanForest
                | TerrainType::BorealForest
                | TerrainType::Taiga
        )
    }
}

/// Event fired when mineral resources are discovered or depleted in a province
#[derive(Event, Debug, Clone)]
pub struct ProvinceMineralsChanged {
    pub province_id: ProvinceId,
    pub mineral_type: MineralType,
    pub old_abundance: Abundance,
    pub new_abundance: Abundance,
}

impl ProvinceMineralsChanged {
    /// Check if this is a new discovery
    pub fn is_discovery(&self) -> bool {
        self.old_abundance.value() == 0 && self.new_abundance.value() > 0
    }

    /// Check if this is a depletion
    pub fn is_depletion(&self) -> bool {
        self.old_abundance.value() > 0 && self.new_abundance.value() == 0
    }

    /// Check if this is a significant change (>25% difference)
    pub fn is_significant(&self) -> bool {
        let old = self.old_abundance.value() as f32;
        let new = self.new_abundance.value() as f32;
        if old == 0.0 {
            new > 25.0
        } else {
            ((new - old).abs() / old) > 0.25
        }
    }
}

/// Event fired when a province's agriculture value changes
#[derive(Event, Debug, Clone)]
pub struct ProvinceAgricultureChanged {
    pub province_id: ProvinceId,
    pub old_agriculture: Agriculture,
    pub new_agriculture: Agriculture,
}

impl ProvinceAgricultureChanged {
    /// Check if land became fertile
    pub fn became_fertile(&self) -> bool {
        self.old_agriculture.is_barren() && self.new_agriculture.is_fertile()
    }

    /// Check if land became barren
    pub fn became_barren(&self) -> bool {
        self.old_agriculture.is_fertile() && self.new_agriculture.is_barren()
    }
}

/// Event fired when a province's fresh water access changes
#[derive(Event, Debug, Clone)]
pub struct ProvinceFreshWaterChanged {
    pub province_id: ProvinceId,
    pub old_distance: Distance,
    pub new_distance: Distance,
}

impl ProvinceFreshWaterChanged {
    /// Check if province gained fresh water access
    pub fn gained_water_access(&self) -> bool {
        self.old_distance.is_infinite() && !self.new_distance.is_infinite()
    }

    /// Check if province lost fresh water access
    pub fn lost_water_access(&self) -> bool {
        !self.old_distance.is_infinite() && self.new_distance.is_infinite()
    }

    /// Check if water got closer
    pub fn water_got_closer(&self) -> bool {
        self.new_distance.value() < self.old_distance.value()
    }
}

/// Event fired when any province data changes (catch-all for debugging)
#[derive(Event, Debug, Clone)]
pub struct ProvinceChanged {
    pub province_id: ProvinceId,
    pub change_type: ProvinceChangeType,
}

/// Types of changes that can occur to a province
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvinceChangeType {
    Population,
    Terrain,
    Minerals,
    Agriculture,
    FreshWater,
    Infrastructure,
    Ownership,
    Other(String),
}

// BATCH EVENTS - For bulk operations

/// Event for batch population updates (e.g., from simulation tick)
#[derive(Event, Debug, Clone)]
pub struct BatchPopulationUpdate {
    pub updates: Vec<ProvincePopulationChanged>,
    pub simulation_year: u32,
}

/// Event for batch mineral discoveries (e.g., from exploration)
#[derive(Event, Debug, Clone)]
pub struct BatchMineralDiscovery {
    pub discoveries: Vec<ProvinceMineralsChanged>,
}

/// Plugin that registers all province event types
pub struct ProvinceEventsPlugin;

impl Plugin for ProvinceEventsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Individual change events
            .add_event::<ProvincePopulationChanged>()
            .add_event::<ProvinceTerrainChanged>()
            .add_event::<ProvinceMineralsChanged>()
            .add_event::<ProvinceAgricultureChanged>()
            .add_event::<ProvinceFreshWaterChanged>()
            .add_event::<ProvinceChanged>()
            // Batch events
            .add_event::<BatchPopulationUpdate>()
            .add_event::<BatchMineralDiscovery>()
            // Debug logging system (only in debug builds)
            .add_systems(
                Update,
                log_province_changes.run_if(resource_exists::<DebugProvinceEvents>),
            );
    }
}

/// Resource to enable debug logging of province events
#[derive(Resource, Default)]
pub struct DebugProvinceEvents {
    pub log_population: bool,
    pub log_terrain: bool,
    pub log_minerals: bool,
    pub log_agriculture: bool,
    pub log_water: bool,
}

impl DebugProvinceEvents {
    pub fn all() -> Self {
        Self {
            log_population: true,
            log_terrain: true,
            log_minerals: true,
            log_agriculture: true,
            log_water: true,
        }
    }
}

/// System that logs province changes for debugging
fn log_province_changes(
    debug: Res<DebugProvinceEvents>,
    mut population_events: EventReader<ProvincePopulationChanged>,
    mut terrain_events: EventReader<ProvinceTerrainChanged>,
    mut mineral_events: EventReader<ProvinceMineralsChanged>,
    mut agriculture_events: EventReader<ProvinceAgricultureChanged>,
    mut water_events: EventReader<ProvinceFreshWaterChanged>,
) {
    if debug.log_population {
        for event in population_events.read() {
            info!(
                "Province {} population: {} → {} ({:+.1}%)",
                event.province_id,
                event.old_population,
                event.new_population,
                event.percentage_change()
            );
        }
    }

    if debug.log_terrain {
        for event in terrain_events.read() {
            info!(
                "Province {} terrain: {:?} → {:?}",
                event.province_id, event.old_terrain, event.new_terrain
            );
        }
    }

    if debug.log_minerals {
        for event in mineral_events.read() {
            if event.is_discovery() {
                info!(
                    "Province {} discovered {:?} ({})",
                    event.province_id, event.mineral_type, event.new_abundance
                );
            } else if event.is_depletion() {
                info!(
                    "Province {} depleted {:?}",
                    event.province_id, event.mineral_type
                );
            }
        }
    }

    if debug.log_agriculture {
        for event in agriculture_events.read() {
            info!(
                "Province {} agriculture: {} → {}",
                event.province_id, event.old_agriculture, event.new_agriculture
            );
        }
    }

    if debug.log_water {
        for event in water_events.read() {
            if event.gained_water_access() {
                info!(
                    "Province {} gained water access (distance: {})",
                    event.province_id, event.new_distance
                );
            } else if event.lost_water_access() {
                info!("Province {} lost water access", event.province_id);
            }
        }
    }
}

/// Trait for types that can emit province events
pub trait ProvinceEventEmitter {
    fn emit_population_changed(&self, event: ProvincePopulationChanged);
    fn emit_terrain_changed(&self, event: ProvinceTerrainChanged);
    fn emit_minerals_changed(&self, event: ProvinceMineralsChanged);
    fn emit_agriculture_changed(&self, event: ProvinceAgricultureChanged);
    fn emit_fresh_water_changed(&self, event: ProvinceFreshWaterChanged);
}

/// Implementation for EventWriter (used in systems)
impl<'w> ProvinceEventEmitter for EventWriter<'w, ProvincePopulationChanged> {
    fn emit_population_changed(&self, event: ProvincePopulationChanged) {
        // Note: In newer Bevy versions, use .write() instead of .write()
        // For now, keeping .write() to match the codebase's current usage
        unsafe {
            (*(self as *const _ as *mut EventWriter<ProvincePopulationChanged>)).write(event);
        }
    }

    fn emit_terrain_changed(&self, _event: ProvinceTerrainChanged) {
        // Can't emit different event type from this writer
    }

    fn emit_minerals_changed(&self, _event: ProvinceMineralsChanged) {
        // Can't emit different event type from this writer
    }

    fn emit_agriculture_changed(&self, _event: ProvinceAgricultureChanged) {
        // Can't emit different event type from this writer
    }

    fn emit_fresh_water_changed(&self, _event: ProvinceFreshWaterChanged) {
        // Can't emit different event type from this writer
    }
}
