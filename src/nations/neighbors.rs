//! Nation neighbor detection and relationship management
//!
//! This module tracks which nations border each other through:
//! - **Land borders**: Direct province adjacency (LandNeighborOf relationship)
//! - **Naval range**: Coastal provinces projecting power across water (NavalNeighborOf relationship)
//!
//! The neighbor relationships are event-driven, rebuilding when territory ownership changes.

use bevy::prelude::*;
use std::collections::{HashSet, VecDeque};
use crate::nations::{Nation, TerritoryOwnershipChanged};
use crate::nations::relationships::{LandNeighborOf, NavalNeighborOf};
use crate::world::{ProvinceStorage, TerrainType};

/// Maximum distance in tiles for naval neighbor detection
const MAX_NAVAL_RANGE: usize = 10;

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
        all_neighbors.extend(land.neighbors().iter().copied());
    }

    if let Some(naval) = naval_neighbors {
        all_neighbors.extend(naval.neighbors().iter().copied());
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

    let relationship_count = land_neighbor_pairs.len();
    // Create bidirectional land neighbor relationships
    for (nation_a, nation_b) in land_neighbor_pairs {
        commands.entity(nation_a).insert(LandNeighborOf(nation_b));
        commands.entity(nation_b).insert(LandNeighborOf(nation_a));
    }

    info!("Created {} land neighbor relationships", relationship_count * 2);

    // Step 3: Detect naval range neighbors via BFS across ocean tiles
    let mut naval_neighbor_pairs = HashSet::new();

    // First, find all coastal provinces for each nation
    for province in &province_storage.provinces {
        let Some(owner_entity) = province.owner_entity else { continue };

        // Skip ocean provinces - we want coastal land provinces
        if province.terrain == TerrainType::Ocean {
            continue;
        }

        // Check if this province is coastal (has ocean neighbor)
        let is_coastal = province.neighbors.iter().any(|n| {
            n.and_then(|id| province_storage.provinces.get(id.value() as usize))
                .map(|p| p.terrain == TerrainType::Ocean)
                .unwrap_or(false)
        });

        if !is_coastal {
            continue;
        }

        // BFS from this coastal province across ocean tiles
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Seed BFS with adjacent ocean tiles at distance 1
        for neighbor_opt in &province.neighbors {
            if let Some(neighbor_id) = neighbor_opt {
                if let Some(neighbor_prov) = province_storage.provinces.get(neighbor_id.value() as usize) {
                    if neighbor_prov.terrain == TerrainType::Ocean {
                        queue.push_back((neighbor_id.value(), 1));
                    }
                }
            }
        }

        while let Some((province_id, distance)) = queue.pop_front() {
            if distance > MAX_NAVAL_RANGE || !visited.insert(province_id) {
                continue;
            }

            let Some(current_prov) = province_storage.provinces.get(province_id as usize) else {
                continue;
            };

            // Check if we reached another nation's coast
            if current_prov.terrain != TerrainType::Ocean {
                if let Some(other_owner) = current_prov.owner_entity {
                    if other_owner != owner_entity {
                        // Found a naval neighbor - add sorted pair to avoid duplicates
                        let pair = if owner_entity.index() < other_owner.index() {
                            (owner_entity, other_owner)
                        } else {
                            (other_owner, owner_entity)
                        };
                        naval_neighbor_pairs.insert(pair);
                    }
                }
                // Don't continue BFS through land
                continue;
            }

            // Continue BFS across ocean tiles
            for neighbor_opt in &current_prov.neighbors {
                if let Some(neighbor_id) = neighbor_opt {
                    let next_id = neighbor_id.value();
                    if !visited.contains(&next_id) {
                        queue.push_back((next_id, distance + 1));
                    }
                }
            }
        }
    }

    let naval_count = naval_neighbor_pairs.len();
    // Create bidirectional naval neighbor relationships
    for (nation_a, nation_b) in naval_neighbor_pairs {
        commands.entity(nation_a).insert(NavalNeighborOf(nation_b));
        commands.entity(nation_b).insert(NavalNeighborOf(nation_a));
    }

    info!("Created {} naval neighbor relationships", naval_count * 2);
}
