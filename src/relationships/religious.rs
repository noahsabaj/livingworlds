//! Religious Relationships - Faith influence and religious structures
//!
//! This module defines relationships for religions, their influence over provinces,
//! and the spread of faith throughout the world.

use bevy::prelude::*;

// ================================================================================================
// RELIGIOUS INFLUENCE RELATIONSHIPS
// ================================================================================================

/// A religion has influence in a province
/// Religious spread and conversion mechanics
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = InfluencedByReligions)]
pub struct InfluencesProvince(pub Entity);

/// Reverse relationship: A province is influenced by religions
/// Automatically maintained by Bevy when `InfluencesProvince` is added
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = InfluencesProvince, linked_spawn)]
pub struct InfluencedByReligions(Vec<Entity>); // Private for safety - Bevy handles internal access

impl InfluencedByReligions {
    /// Get read-only access to religions influencing this province (convenience method)
    pub fn religions(&self) -> &[Entity] {
        &self.0
    }

    /// Get the number of religions influencing this province
    pub fn count(&self) -> usize {
        self.0.len()
    }

    /// Check if province is influenced by a specific religion
    pub fn influenced_by(&self, religion: Entity) -> bool {
        self.0.contains(&religion)
    }

    /// Check if province has any religious influence
    pub fn has_religious_influence(&self) -> bool {
        !self.0.is_empty()
    }
}

// ================================================================================================
// RELIGIOUS ENTITIES
// ================================================================================================

/// Marker component for religion entities
#[derive(Component, Debug, Clone)]
pub struct Religion {
    pub name: String,
    pub religion_type: ReligionType,
    pub founding_year: u32,
    pub core_teachings: Vec<String>,
    pub spread_rate: f32, // How quickly it spreads (0.0 to 1.0)
    pub tolerance: f32,   // How tolerant of other religions (0.0 to 1.0)
}

/// Religious influence data in a specific province
#[derive(Component, Debug, Clone)]
pub struct ReligiousInfluence {
    pub religion: Entity,
    pub influence_strength: f32, // 0.0 = no influence, 1.0 = complete dominance
    pub follower_count: u32,     // Number of followers in this province
    pub religious_buildings: u32, // Temples, churches, etc.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReligionType {
    Monotheistic,  // Single deity
    Polytheistic,  // Multiple deities
    Animistic,     // Nature spirits
    Ancestral,     // Ancestor worship
    Philosophical, // Philosophy-based
}

// ================================================================================================
// RELIGIOUS DATA
// ================================================================================================

/// Provincial religious status
#[derive(Component, Debug, Clone)]
pub struct ReligiousStatus {
    /// Dominant religion in this province
    pub dominant_religion: Option<Entity>,
    /// Religious diversity (0.0 = uniform, 1.0 = very diverse)
    pub diversity: f32,
    /// Overall religious tension level
    pub tension: f32,
    /// Number of different religions present
    pub religion_count: u32,
}

// ================================================================================================
// QUERY SYSTEMS - Religious demographics
// ================================================================================================

/// Query function for religious influence across all provinces
/// Returns list of (religion, religion_name, influenced_province) tuples
pub fn query_religious_influence(
    religions_query: &Query<(Entity, &Religion, &InfluencesProvince)>,
) -> Vec<(Entity, String, Entity)> {
    religions_query
        .iter()
        .map(|(religion_entity, religion, influences_province)| {
            (
                religion_entity,
                religion.name.clone(),
                influences_province.0,
            )
        })
        .collect()
}

/// Query function for provincial religious influence
/// Returns list of (province, religions) pairs
pub fn query_provincial_religions(
    provinces_query: &Query<(Entity, &InfluencedByReligions)>,
) -> Vec<(Entity, Vec<Entity>)> {
    provinces_query
        .iter()
        .map(|(province_entity, influenced_by)| {
            (province_entity, influenced_by.religions().to_vec())
        })
        .collect()
}

/// Find all provinces influenced by a religion
pub fn find_religion_provinces(
    religion_entity: Entity,
    religions_query: &Query<&InfluencesProvince>,
) -> Vec<Entity> {
    religions_query
        .iter_many([religion_entity])
        .map(|influences| influences.0)
        .collect()
}

/// Get the dominant religion in a province
pub fn get_dominant_religion(
    province_entity: Entity,
    religious_influences: &Query<&ReligiousInfluence>,
) -> Option<Entity> {
    religious_influences
        .iter()
        .filter(|influence| {
            // Check if this influence applies to our province
            // This would need to be cross-referenced with the relationship system
            true // Placeholder
        })
        .max_by(|a, b| {
            a.influence_strength
                .partial_cmp(&b.influence_strength)
                .unwrap()
        })
        .map(|influence| influence.religion)
}

// ================================================================================================
// RELIGIOUS SYSTEMS
// ================================================================================================

/// Updates religious status for all provinces
pub fn update_religious_status(
    mut provinces_query: Query<(Entity, &mut ReligiousStatus, &InfluencedByReligions)>,
    religious_influences_query: Query<&ReligiousInfluence>,
) {
    // NOTE: Bevy queries should not be manually parallelized with Rayon
    // Bevy has its own parallel scheduling system
    for (province_entity, mut status, influenced_by) in &mut provinces_query {
            status.religion_count = influenced_by.0.len() as u32;

            if status.religion_count == 0 {
                status.dominant_religion = None;
                status.diversity = 0.0;
                status.tension = 0.0;
                return;
            }

            // Calculate diversity based on influence distribution
            status.diversity = if status.religion_count == 1 {
                0.0
            } else {
                // More religions = higher diversity
                (status.religion_count as f32 / 5.0).min(1.0)
            };

            // Calculate religious tension (higher with more competing religions)
            status.tension = if status.religion_count <= 1 {
                0.0
            } else {
                (status.diversity * 0.5 + (status.religion_count as f32 - 1.0) * 0.1).min(1.0)
            };

            // Find dominant religion (placeholder - would need proper influence data)
            status.dominant_religion = influenced_by.0.first().copied();
    }
}

/// Simulates religious spread between provinces
pub fn simulate_religious_spread(
    religions_query: Query<(Entity, &Religion)>,
    religious_influences_query: Query<&mut ReligiousInfluence>,
    provinces_query: Query<Entity>,
) {
    // Simplified religious spread simulation
    for (religion_entity, religion) in religions_query.iter() {
        // Religious spread logic would go here
        // This would involve checking neighboring provinces, calculating spread probability, etc.

        // For now, just a placeholder that demonstrates the system
        debug!(
            "Processing religious spread for religion: {}",
            religion.name
        );
    }
}

// ================================================================================================
// RELIGIOUS EVENTS
// ================================================================================================

/// Event fired when a religion gains significant influence in a province
#[derive(Event, Debug, Clone)]
pub struct ReligiousConversionEvent {
    pub province: Entity,
    pub religion: Entity,
    pub previous_dominant: Option<Entity>,
    pub influence_strength: f32,
}

/// Event fired when religious conflict arises
#[derive(Event, Debug, Clone)]
pub struct ReligiousConflictEvent {
    pub province: Entity,
    pub religion_a: Entity,
    pub religion_b: Entity,
    pub conflict_type: ConflictType,
}

/// Event fired when a new religion is founded
#[derive(Event, Debug, Clone)]
pub struct ReligionFoundedEvent {
    pub religion: Entity,
    pub founding_province: Entity,
    pub founder: Option<Entity>, // Person or entity who founded it
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictType {
    IdeologicalDispute,  // Theological disagreement
    ResourceCompetition, // Competing for followers/resources
    PoliticalTension,    // Religion used for political purposes
    Persecution,         // Active persecution of minority religion
}

// ================================================================================================
// VALIDATION SYSTEMS
// ================================================================================================

/// Validates religious relationships
pub fn validate_religious_relationships(
    religions_query: Query<(Entity, &InfluencesProvince)>,
    provinces_query: Query<Entity>,
) {
    let valid_provinces: std::collections::HashSet<Entity> = provinces_query.iter().collect();

    for (religion_entity, influences_province) in religions_query.iter() {
        if !valid_provinces.contains(&influences_province.0) {
            warn!(
                "Religion {:?} influences invalid province {:?}",
                religion_entity, influences_province.0
            );
        }
    }
}

/// Validates religious influence data consistency
pub fn validate_religious_influence_consistency(
    religious_influences_query: Query<&ReligiousInfluence>,
    religions_query: Query<Entity>,
) {
    let valid_religions: std::collections::HashSet<Entity> = religions_query.iter().collect();

    for influence in religious_influences_query.iter() {
        if !valid_religions.contains(&influence.religion) {
            warn!(
                "Religious influence references invalid religion {:?}",
                influence.religion
            );
        }

        if influence.influence_strength < 0.0 || influence.influence_strength > 1.0 {
            warn!(
                "Invalid influence strength: {} (should be 0.0-1.0)",
                influence.influence_strength
            );
        }
    }
}
