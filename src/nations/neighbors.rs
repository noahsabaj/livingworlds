//! Nation neighbor detection and relationship management
//!
//! This module tracks which nations border each other through:
//! - **Land borders**: Direct province adjacency (LandNeighborOf relationship)
//! - **Naval range**: Coastal provinces projecting power across water (NavalNeighborOf relationship)
//!
//! The neighbor relationships are event-driven, rebuilding when territory ownership changes.

use bevy::prelude::*;
use std::collections::HashSet;
use crate::nations::{Nation, TerritoryOwnershipChanged};
use crate::nations::relationships::{LandNeighborOf, NavalNeighborOf};
use crate::world::{ProvinceStorage, TerrainType};

/// Get military strengths of neighboring nations
///
/// Uses LandNeighbors and NavalNeighbors relationship components
pub fn get_neighbor_strengths(
    nation_entity: Entity,
    nations_query: &Query<(&Nation, Option<&crate::nations::relationships::LandNeighbors>, Option<&crate::nations::relationships::NavalNeighbors>)>,
) -> Vec<f32> {
    let Ok((_, land_neighbors, naval_neighbors)) = nations_query.get(nation_entity) else {
        return Vec::new();
    };

    // Combine both neighbor types
    let mut all_neighbors = HashSet::new();

    if let Some(land) = land_neighbors {
        all_neighbors.extend(land.0.iter().copied());
    }

    if let Some(naval) = naval_neighbors {
        all_neighbors.extend(naval.0.iter().copied());
    }

    all_neighbors.iter()
        .filter_map(|&neighbor_entity| {
            nations_query.get(neighbor_entity)
                .ok()
                .map(|(nation, _, _)| nation.military_strength)
        })
        .collect()
}

/// Event-driven neighbor relationship rebuild system
///
/// Creates LandNeighborOf and NavalNeighborOf relationships based on province adjacency
pub fn rebuild_neighbor_relationships_on_ownership_change(
    mut ownership_events: MessageReader<TerritoryOwnershipChanged>,
    nations_query: Query<Entity, With<Nation>>,
    mut commands: Commands,
    province_storage: Res<ProvinceStorage>,
) {
    if ownership_events.read().next().is_none() {
        return; // No ownership changes
    }

    info!("Territory ownership changed - rebuilding neighbor relationships");

    // Step 1: Clear all existing neighbor relationships
    for nation_entity in &nations_query {
        commands.entity(nation_entity)
            .remove::<LandNeighborOf>()
            .remove::<NavalNeighborOf>();
    }

    // Step 2: Detect land borders via province neighbors
    let mut land_neighbor_pairs = HashSet::new();

    for province in &province_storage.provinces {
        let Some(owner_entity) = province.owner_entity else { continue };

        // Skip ocean/river provinces
        if matches!(province.terrain, TerrainType::Ocean | TerrainType::River) {
            continue;
        }

        for neighbor_opt in &province.neighbors {
            if let Some(neighbor_id) = neighbor_opt {
                if let Some(neighbor_prov) = province_storage.provinces.get(neighbor_id.value() as usize) {
                    if let Some(neighbor_owner) = neighbor_prov.owner_entity {
                        if neighbor_owner != owner_entity {
                            // Add bidirectional pair (sorted to avoid duplicates)
                            let pair = if owner_entity.index() < neighbor_owner.index() {
                                (owner_entity, neighbor_owner)
                            } else {
                                (neighbor_owner, owner_entity)
                            };
                            land_neighbor_pairs.insert(pair);
                        }
                    }
                }
            }
        }
    }

    // Create bidirectional land neighbor relationships
    for (nation_a, nation_b) in land_neighbor_pairs {
        commands.entity(nation_a).insert(LandNeighborOf(nation_b));
        commands.entity(nation_b).insert(LandNeighborOf(nation_a));
    }

    info!("Created {} land neighbor relationships", land_neighbor_pairs.len() * 2);

    // Step 3: Detect naval range neighbors (TODO Phase 7.5: Implement BFS naval range detection)
    // For now, skip naval neighbors - will implement in follow-up
    debug!("Naval neighbor detection not yet implemented");
}
