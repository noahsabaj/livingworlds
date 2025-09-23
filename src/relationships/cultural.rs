//! Cultural Relationships - Cultural regions and identity
//!
//! This module defines relationships between provinces and cultural regions,
//! replacing the struct-based cultural region system with entity relationships.

use bevy::prelude::*;

// ================================================================================================
// CULTURAL REGION RELATIONSHIPS
// ================================================================================================

/// A province belongs to a cultural region
/// Replaces the struct-based cultural region system with entity relationships
///
/// Cultural regions group provinces with similar cultural characteristics,
/// affecting naming patterns, architectural styles, and political tendencies.
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = ContainsProvinces)]
pub struct BelongsToRegion(pub Entity);

/// Reverse relationship: A cultural region contains multiple provinces
/// Automatically maintained by Bevy when `BelongsToRegion` is added
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = BelongsToRegion, linked_spawn)]
pub struct ContainsProvinces(Vec<Entity>); // Private for safety - Bevy handles internal access

impl ContainsProvinces {
    /// Get read-only access to provinces in this cultural region (convenience method)
    pub fn provinces(&self) -> &[Entity] {
        &self.0
    }

    /// Get the number of provinces in this cultural region
    pub fn count(&self) -> usize {
        self.0.len()
    }

    /// Check if this cultural region contains a specific province
    pub fn contains_province(&self, province: Entity) -> bool {
        self.0.contains(&province)
    }

    /// Check if region has any provinces
    pub fn has_provinces(&self) -> bool {
        !self.0.is_empty()
    }
}

// ================================================================================================
// CULTURAL REGION ENTITIES
// ================================================================================================

/// Marker component for cultural region entities
/// These entities represent distinct cultural areas in the world
#[derive(Component, Debug, Clone)]
pub struct CulturalRegion {
    /// Name of the cultural region (e.g., "Northern Highlands", "Desert Kingdoms")
    pub name: String,
    /// Dominant culture type in this region
    pub dominant_culture: crate::name_generator::Culture,
    /// Geographic center of the region
    pub center: Vec2,
    /// Approximate radius of the region's influence
    pub influence_radius: f32,
}

/// Cultural coherence data for regions
/// Tracks how unified a cultural region is
#[derive(Component, Debug, Clone)]
pub struct CulturalCoherence {
    /// How unified the region is (0.0 = fragmented, 1.0 = totally unified)
    pub unity: f32,
    /// Number of provinces in this region
    pub province_count: u32,
    /// Number of different nations controlling provinces in this region
    pub controlling_nations: u32,
}

// ================================================================================================
// QUERY SYSTEMS - Cultural analysis
// ================================================================================================

/// Query function for cultural regions and their provinces
/// Returns list of (region_entity, region_name, province_count) tuples
pub fn query_cultural_regions(
    regions_query: &Query<(Entity, &CulturalRegion, &ContainsProvinces)>,
) -> Vec<(Entity, String, usize)> {
    regions_query
        .iter()
        .map(|(region_entity, region, contains)| {
            (region_entity, region.name.clone(), contains.count())
        })
        .collect()
}

/// Query function for province cultural region membership
/// Returns list of (province, region) pairs
pub fn query_province_regions(
    provinces_query: &Query<(Entity, Option<&BelongsToRegion>)>,
) -> Vec<(Entity, Entity)> {
    provinces_query
        .iter()
        .filter_map(|(province_entity, belongs_to)| belongs_to.map(|bt| (province_entity, bt.0)))
        .collect()
}

/// Get all provinces in a specific cultural region
pub fn get_provinces_in_region(
    region_entity: Entity,
    regions_query: &Query<&ContainsProvinces>,
) -> Option<Vec<Entity>> {
    regions_query
        .get(region_entity)
        .ok()
        .map(|contains| contains.provinces().to_vec())
}

/// Find cultural region by culture type
pub fn find_regions_by_culture(
    culture: crate::name_generator::Culture,
    regions_query: Query<(Entity, &CulturalRegion)>,
) -> Vec<Entity> {
    regions_query
        .iter()
        .filter_map(|(entity, region)| {
            if region.dominant_culture == culture {
                Some(entity)
            } else {
                None
            }
        })
        .collect()
}

// ================================================================================================
// CULTURAL ANALYSIS SYSTEMS
// ================================================================================================

/// Updates cultural coherence data for all regions
pub fn update_cultural_coherence(
    mut regions_query: Query<(Entity, &mut CulturalCoherence, &ContainsProvinces)>,
    provinces_query: Query<Option<&crate::relationships::ControlledBy>>,
) {
    // NOTE: Bevy queries should not be manually parallelized with Rayon
    // Bevy has its own parallel scheduling system
    for (_region_entity, mut coherence, contains_provinces) in &mut regions_query {
            let province_count = contains_provinces.count() as u32;
            coherence.province_count = province_count;

            if province_count == 0 {
                coherence.unity = 0.0;
                coherence.controlling_nations = 0;
                return;
            }

            // Count unique nations controlling provinces in this region
            let mut controlling_nations = std::collections::HashSet::new();
            for &province_entity in contains_provinces.provinces() {
                if let Ok(Some(controlled_by)) = provinces_query.get(province_entity) {
                    controlling_nations.insert(controlled_by.0);
                }
            }

            coherence.controlling_nations = controlling_nations.len() as u32;

            // Calculate unity: fewer controlling nations = higher unity
            coherence.unity = if coherence.controlling_nations <= 1 {
                1.0
            } else {
                1.0 / (coherence.controlling_nations as f32).sqrt()
            };
    }
}

/// Detects cultural tensions based on political fragmentation
pub fn detect_cultural_tensions(
    regions_query: Query<(Entity, &CulturalRegion, &CulturalCoherence)>,
) -> Vec<CulturalTensionEvent> {
    let mut tensions = Vec::new();

    for (region_entity, region, coherence) in regions_query.iter() {
        // High fragmentation = cultural tension
        if coherence.controlling_nations >= 3 && coherence.unity < 0.5 {
            tensions.push(CulturalTensionEvent {
                region: region_entity,
                region_name: region.name.clone(),
                tension_level: 1.0 - coherence.unity,
                fragmentation_cause: FragmentationCause::PoliticalDivision,
            });
        }
    }

    tensions
}

// ================================================================================================
// CULTURAL EVENTS - Cultural changes
// ================================================================================================

/// Event fired when cultural tensions arise in a region
#[derive(Event, Debug, Clone)]
pub struct CulturalTensionEvent {
    pub region: Entity,
    pub region_name: String,
    pub tension_level: f32, // 0.0 = no tension, 1.0 = maximum tension
    pub fragmentation_cause: FragmentationCause,
}

/// Event fired when a cultural region unifies under one nation
#[derive(Event, Debug, Clone)]
pub struct CulturalUnificationEvent {
    pub region: Entity,
    pub region_name: String,
    pub unifying_nation: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FragmentationCause {
    PoliticalDivision,    // Multiple nations in one cultural region
    MilitaryConquest,     // Recent conquest creating instability
    CulturalAssimilation, // Cultural boundaries shifting
}

// ================================================================================================
// VALIDATION SYSTEMS - Cultural integrity
// ================================================================================================

/// Validates that cultural regions have valid province assignments
pub fn validate_cultural_regions(
    regions_query: Query<(Entity, &ContainsProvinces)>,
    provinces_query: Query<Entity>,
) {
    let valid_provinces: std::collections::HashSet<Entity> = provinces_query.iter().collect();

    for (region_entity, contains_provinces) in regions_query.iter() {
        for &province_entity in &contains_provinces.0 {
            if !valid_provinces.contains(&province_entity) {
                warn!(
                    "Cultural region {:?} contains invalid province {:?}",
                    region_entity, province_entity
                );
            }
        }
    }
}

/// Ensures no province belongs to multiple cultural regions
pub fn validate_exclusive_cultural_membership(provinces_query: Query<(Entity, &BelongsToRegion)>) {
    // Pre-allocate HashMap with capacity based on province count
    let province_count = provinces_query.iter().len();
    let mut province_regions = std::collections::HashMap::with_capacity(province_count);

    for (province_entity, belongs_to_region) in provinces_query.iter() {
        if let Some(existing_region) = province_regions.insert(province_entity, belongs_to_region.0)
        {
            error!(
                "Province {:?} belongs to multiple cultural regions: {:?} and {:?}",
                province_entity, existing_region, belongs_to_region.0
            );
        }
    }
}
