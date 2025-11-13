//! Administrative Relationships - Governance and provincial administration
//!
//! This module defines relationships for governing provinces, administrative roles,
//! and the hierarchy of governance from national to local levels.

use bevy::prelude::*;

// ================================================================================================
// PROVINCIAL ADMINISTRATION
// ================================================================================================

/// A governor administers specific provinces
/// Enables provincial administration and local governance mechanics
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = AdministeredBy)]
pub struct Administers(pub Entity);

/// Reverse relationship: A province is administered by governors
/// Automatically maintained by Bevy when `Administers` is added
/// NOTE: Should typically only contain one governor (validated by validation systems)
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = Administers, linked_spawn)]
pub struct AdministeredBy(Vec<Entity>); // Private for safety - Bevy handles internal access

impl AdministeredBy {
    /// Get read-only access to governors administering this province
    /// Should typically be only one governor
    pub fn governors(&self) -> &[Entity] {
        &self.0
    }

    /// Get the primary governor (first one)
    /// Returns None if no governor administers this province
    pub fn primary_governor(&self) -> Option<Entity> {
        self.0.first().copied()
    }

    /// Check if province is administered by a specific governor
    pub fn is_administered_by(&self, governor: Entity) -> bool {
        self.0.contains(&governor)
    }

    /// Check if province has any governor
    pub fn has_governor(&self) -> bool {
        !self.0.is_empty()
    }
}

// ================================================================================================
// ADMINISTRATIVE ENTITIES
// ================================================================================================

/// Marker component for governor entities
#[derive(Component, Debug, Clone)]
pub struct Governor {
    pub name: String,
    pub loyalty: f32,    // 0.0 = disloyal, 1.0 = completely loyal
    pub competence: f32, // 0.0 = incompetent, 1.0 = highly skilled
    pub corruption: f32, // 0.0 = honest, 1.0 = completely corrupt
}

/// Administrative efficiency data for provinces
#[derive(Component, Debug, Clone)]
pub struct AdministrativeEfficiency {
    /// Overall efficiency (0.0 = chaos, 1.0 = perfect administration)
    pub efficiency: f32,
    /// Tax collection rate (0.0 = no taxes, 1.0 = full collection)
    pub tax_collection: f32,
    /// Infrastructure maintenance quality
    pub infrastructure_maintenance: f32,
}

// ================================================================================================
// QUERY SYSTEMS - Administrative queries
// ================================================================================================

/// System for querying governor-province assignments
pub fn query_provincial_administration_system(
    governors_query: Query<(Entity, &Governor, Option<&Administers>)>,
) {
    for (governor_entity, governor, administers) in governors_query.iter() {
        let administered_province = administers.map(|a| a.0);
        debug!(
            "Governor {:?} ({}) administers province {:?}",
            governor_entity, governor.name, administered_province
        );
    }
}

/// Find the governor of a specific province
pub fn find_province_governor(
    province_entity: Entity,
    provinces_query: &Query<&AdministeredBy>,
) -> Option<Entity> {
    provinces_query
        .get(province_entity)
        .ok()
        .and_then(|administered_by| administered_by.primary_governor())
}

/// Get all provinces administered by a governor
pub fn get_governor_provinces(
    governor_entity: Entity,
    governors_query: &Query<&Administers>,
) -> Vec<Entity> {
    governors_query
        .iter_many([governor_entity])
        .map(|administers| administers.0)
        .collect()
}

// ================================================================================================
// ADMINISTRATIVE SYSTEMS
// ================================================================================================

/// Updates administrative efficiency based on governor competence
pub fn update_administrative_efficiency(
    mut provinces_query: Query<(
        Entity,
        &mut AdministrativeEfficiency,
        Option<&AdministeredBy>,
    )>,
    governors_query: Query<&Governor>,
) {
    // NOTE: Bevy queries should not be manually parallelized with Rayon
    // Bevy has its own parallel scheduling system
    for (province_entity, mut efficiency, administered_by) in &mut provinces_query {
        if let Some(administered_by) = administered_by {
            if let Some(governor_entity) = administered_by.primary_governor() {
                if let Ok(governor) = governors_query.get(governor_entity) {
                    // Calculate efficiency based on governor traits
                    let base_efficiency =
                        (governor.competence * (1.0 - governor.corruption)).max(0.1);
                    efficiency.efficiency = base_efficiency;

                    // Tax collection affected by loyalty and corruption
                    efficiency.tax_collection =
                        (governor.loyalty * (1.0 - governor.corruption * 0.5)).max(0.1);

                    // Infrastructure maintenance primarily based on competence
                    efficiency.infrastructure_maintenance = governor.competence.max(0.1);
                }
            } else {
                // No valid governor - poor administration
                efficiency.efficiency = 0.3;
                efficiency.tax_collection = 0.2;
                efficiency.infrastructure_maintenance = 0.2;
            }
        } else {
            // No governor assigned - minimal administration
            efficiency.efficiency = 0.1;
            efficiency.tax_collection = 0.1;
            efficiency.infrastructure_maintenance = 0.1;
        }
    }
}

// ================================================================================================
// ADMINISTRATIVE EVENTS
// ================================================================================================

/// Event fired when a governor is appointed to a province
#[derive(Message, Debug, Clone)]
pub struct GovernorAppointedEvent {
    pub governor: Entity,
    pub province: Entity,
    pub appointing_nation: Entity,
}

/// Event fired when a governor is dismissed
#[derive(Message, Debug, Clone)]
pub struct GovernorDismissedEvent {
    pub governor: Entity,
    pub province: Entity,
    pub reason: DismissalReason,
}

/// Event fired when corruption is detected
#[derive(Message, Debug, Clone)]
pub struct CorruptionDetectedEvent {
    pub governor: Entity,
    pub province: Entity,
    pub corruption_level: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DismissalReason {
    Corruption,
    Incompetence,
    Disloyalty,
    PoliticalChanges,
}

// ================================================================================================
// VALIDATION SYSTEMS
// ================================================================================================

/// Validates administrative assignments
pub fn validate_administrative_assignments(
    governors_query: Query<(Entity, Option<&Administers>)>,
    provinces_query: Query<Entity>,
) {
    let valid_provinces: std::collections::HashSet<Entity> = provinces_query.iter().collect();

    for (governor_entity, administers) in governors_query.iter() {
        if let Some(administers) = administers {
            if !valid_provinces.contains(&administers.0) {
                warn!(
                    "Governor {:?} administers invalid province {:?}",
                    governor_entity, administers.0
                );
            }
        }
    }
}
