//! Infrastructure analysis and connectivity calculation
//!
//! This module analyzes infrastructure networks to calculate connectivity scores,
//! identify hubs, and determine development levels for provinces.

use bevy::prelude::*;
use crate::world::{Province, ProvinceId, ProvinceStorage};
use super::storage::{InfrastructureStorage, ProvinceInfrastructure};
use std::collections::{HashSet, HashMap};

/// Calculate infrastructure metrics for all provinces based on their connections
pub fn analyze_infrastructure(
    provinces: &[Province],
    province_storage: &ProvinceStorage,
) -> InfrastructureStorage {
    let mut storage = InfrastructureStorage::new();

    // First pass: Calculate basic connectivity from neighbors
    for province in provinces {
        let mut infra = calculate_basic_infrastructure(province, provinces, province_storage);
        storage.set(province.id, infra);
    }

    // Second pass: Identify hubs based on network centrality
    identify_infrastructure_hubs(&mut storage, provinces);

    // Calculate global statistics
    storage.calculate_statistics();

    storage
}

/// Calculate basic infrastructure for a single province
fn calculate_basic_infrastructure(
    province: &Province,
    all_provinces: &[Province],
    province_storage: &ProvinceStorage,
) -> ProvinceInfrastructure {
    // Count connections to owned neighbors (simplified road network)
    let mut road_connections = 0u8;
    let mut connected_to_capital = false;

    for neighbor_id_opt in &province.neighbors {
        if let Some(neighbor_id) = neighbor_id_opt {
            if let Some(&neighbor_idx) = province_storage.province_by_id.get(neighbor_id) {
                let neighbor = &all_provinces[neighbor_idx];

                // Count as road if both provinces are owned by same nation
                if province.owner.is_some() && province.owner == neighbor.owner {
                    road_connections += 1;
                }

                // Check if neighbor is a capital (simplified)
                if neighbor.population > 50000 {
                    connected_to_capital = true;
                }
            }
        }
    }

    // Calculate metrics based on connections and province properties
    let base_connectivity = (road_connections as f32 / 6.0).min(0.5); // Max 6 neighbors

    // Boost connectivity for high population provinces
    let population_factor = if province.population > 100000 {
        0.3
    } else if province.population > 50000 {
        0.2
    } else if province.population > 20000 {
        0.1
    } else {
        0.0
    };

    // Boost for capital connection
    let capital_boost = if connected_to_capital { 0.1 } else { 0.0 };

    // Boost for agricultural productivity (likely to have trade)
    let agriculture_boost = (province.agriculture.value() / 3.0) * 0.1;

    let connectivity = (base_connectivity + population_factor + capital_boost + agriculture_boost)
        .min(1.0);

    // Estimate trade volume based on population and agriculture
    let trade_volume = if province.population > 10000 {
        let pop_trade = (province.population as f32 / 200000.0).min(0.5);
        let agri_trade = (province.agriculture.value() / 3.0) * 0.3;
        (pop_trade + agri_trade).min(1.0)
    } else {
        0.0
    };

    let mut infra = ProvinceInfrastructure {
        connectivity,
        road_density: base_connectivity,
        trade_volume,
        development_level: 0,
        is_hub: false,
        road_connections,
        trade_routes: (trade_volume * 5.0) as u8, // Simplified trade route count
    };

    // Calculate development level
    infra.calculate_development_level();

    infra
}

/// Identify infrastructure hubs using simplified network centrality
fn identify_infrastructure_hubs(
    storage: &mut InfrastructureStorage,
    provinces: &[Province],
) {
    // Find provinces with high connectivity that connect to many other high-connectivity provinces
    let mut hub_candidates = Vec::new();

    for province in provinces {
        if let Some(infra) = storage.infrastructure.get(&province.id) {
            // Only consider well-connected provinces as potential hubs
            if infra.connectivity >= 0.6 {
                let mut connected_development = 0.0;
                let mut connection_count = 0;

                // Check neighbors' development
                for neighbor_id_opt in &province.neighbors {
                    if let Some(neighbor_id) = neighbor_id_opt {
                        if let Some(neighbor_infra) = storage.infrastructure.get(neighbor_id) {
                            connected_development += neighbor_infra.connectivity;
                            connection_count += 1;
                        }
                    }
                }

                if connection_count > 0 {
                    let avg_neighbor_development = connected_development / connection_count as f32;

                    // Hub if well-connected AND neighbors are also developed
                    if avg_neighbor_development >= 0.4 {
                        hub_candidates.push((province.id, infra.connectivity + avg_neighbor_development));
                    }
                }
            }
        }
    }

    // Sort by combined score and mark top provinces as hubs
    hub_candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Mark top 5% as hubs (or at least the top 10)
    let hub_count = (provinces.len() / 20).max(10).min(hub_candidates.len());

    for i in 0..hub_count {
        if let Some(infra) = storage.infrastructure.get_mut(&hub_candidates[i].0) {
            infra.is_hub = true;
            // Boost connectivity for hubs
            infra.connectivity = (infra.connectivity * 1.2).min(1.0);
        }
    }
}

/// Calculate infrastructure development along a path (for future pathfinding)
pub fn calculate_path_infrastructure(
    start: ProvinceId,
    end: ProvinceId,
    storage: &InfrastructureStorage,
    province_storage: &ProvinceStorage,
) -> f32 {
    // Simplified: just average the infrastructure of both endpoints
    let start_infra = storage.get(start).map(|i| i.connectivity).unwrap_or(0.0);
    let end_infra = storage.get(end).map(|i| i.connectivity).unwrap_or(0.0);

    (start_infra + end_infra) / 2.0
}

/// Find provinces that should be connected by major roads
pub fn find_major_road_candidates(
    provinces: &[Province],
    storage: &InfrastructureStorage,
) -> Vec<(ProvinceId, ProvinceId)> {
    use std::collections::HashMap;

    let mut roads = Vec::new();

    // Build HashMap for O(1) lookups instead of O(n) searches
    // This prevents O(n²) complexity that caused 19-minute load times!
    let province_lookup: HashMap<ProvinceId, usize> = provinces
        .iter()
        .enumerate()
        .map(|(idx, p)| (p.id, idx))
        .collect();

    // Connect high-population provinces that are nearby
    for i in 0..provinces.len() {
        let province_a = &provinces[i];
        if province_a.population < 20000 {
            continue;
        }

        for neighbor_id_opt in &province_a.neighbors {
            if let Some(neighbor_id) = neighbor_id_opt {
                // O(1) lookup instead of O(n) search!
                if let Some(&neighbor_idx) = province_lookup.get(neighbor_id) {
                    let neighbor = &provinces[neighbor_idx];
                    if neighbor.population >= 20000 && neighbor.id.value() > province_a.id.value() {
                        // Only add each road once (using ID comparison to avoid duplicates)
                        roads.push((province_a.id, *neighbor_id));
                    }
                }
            }
        }
    }

    roads
}

/// Update infrastructure metrics when roads are built or destroyed
pub fn update_infrastructure_for_road_change(
    province_id: ProvinceId,
    storage: &mut InfrastructureStorage,
    provinces: &[Province],
    province_storage: &ProvinceStorage,
    road_added: bool,
) {
    if let Some(&province_idx) = province_storage.province_by_id.get(&province_id) {
        let province = &provinces[province_idx];

        if let Some(infra) = storage.infrastructure.get_mut(&province_id) {
            // Adjust road connections
            if road_added {
                infra.road_connections = (infra.road_connections + 1).min(6);
            } else {
                infra.road_connections = infra.road_connections.saturating_sub(1);
            }

            // Recalculate connectivity
            infra.road_density = (infra.road_connections as f32 / 6.0).min(1.0);
            infra.connectivity = (infra.road_density * 0.5
                + infra.trade_volume * 0.3
                + if infra.is_hub { 0.2 } else { 0.0 })
                .min(1.0);

            infra.calculate_development_level();
        }
    }

    // Recalculate global statistics after change
    storage.calculate_statistics();
}