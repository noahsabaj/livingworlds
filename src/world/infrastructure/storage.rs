//! Infrastructure data storage for runtime visualization
//!
//! This module provides efficient storage of infrastructure data that's computed
//! during world generation and used for real-time overlay visualization without
//! recalculation.

use bevy::prelude::*;
use crate::world::ProvinceId;
use std::collections::HashMap;

/// Infrastructure data for a single province
#[derive(Debug, Clone, Copy)]
pub struct ProvinceInfrastructure {
    /// Overall connectivity score (0.0 = isolated, 1.0 = major hub)
    pub connectivity: f32,

    /// Road network density (roads / max possible roads)
    pub road_density: f32,

    /// Trade volume flowing through province
    pub trade_volume: f32,

    /// Development level (0-5 scale)
    /// 0: Wilderness, 1: Rural, 2: Developing, 3: Developed, 4: Urban, 5: Metropolis
    pub development_level: u8,

    /// Whether this province is a major connectivity hub
    pub is_hub: bool,

    /// Number of road connections
    pub road_connections: u8,

    /// Number of trade routes
    pub trade_routes: u8,
}

impl ProvinceInfrastructure {
    /// Create infrastructure data for an undeveloped province
    pub fn wilderness() -> Self {
        Self {
            connectivity: 0.0,
            road_density: 0.0,
            trade_volume: 0.0,
            development_level: 0,
            is_hub: false,
            road_connections: 0,
            trade_routes: 0,
        }
    }

    /// Create infrastructure data with basic development
    pub fn rural(road_connections: u8) -> Self {
        Self {
            connectivity: 0.2 + (road_connections as f32 * 0.1).min(0.3),
            road_density: (road_connections as f32 / 6.0).min(0.5), // Max 6 hex neighbors
            trade_volume: 0.1,
            development_level: 1,
            is_hub: false,
            road_connections,
            trade_routes: 0,
        }
    }

    /// Calculate development level based on metrics
    pub fn calculate_development_level(&mut self) {
        self.development_level = if self.connectivity >= 0.8 {
            5 // Metropolis
        } else if self.connectivity >= 0.65 {
            4 // Urban
        } else if self.connectivity >= 0.5 {
            3 // Developed
        } else if self.connectivity >= 0.35 {
            2 // Developing
        } else if self.connectivity >= 0.2 {
            1 // Rural
        } else {
            0 // Wilderness
        };
    }
}

/// Resource storing infrastructure data for all provinces
#[derive(Resource, Default, Debug, Clone)]
pub struct InfrastructureStorage {
    /// Infrastructure data indexed by province ID
    pub infrastructure: HashMap<ProvinceId, ProvinceInfrastructure>,

    /// Global connectivity statistics
    pub avg_connectivity: f32,
    pub max_connectivity: f32,
    pub hub_count: usize,

    /// Network statistics
    pub total_roads: usize,
    pub total_trade_routes: usize,
}

impl InfrastructureStorage {
    /// Create a new empty infrastructure storage
    pub fn new() -> Self {
        Self::default()
    }

    /// Get infrastructure data for a province
    pub fn get(&self, province_id: ProvinceId) -> Option<&ProvinceInfrastructure> {
        self.infrastructure.get(&province_id)
    }

    /// Add or update infrastructure data for a province
    pub fn set(&mut self, province_id: ProvinceId, infra: ProvinceInfrastructure) {
        self.infrastructure.insert(province_id, infra);
    }

    /// Calculate global statistics after all provinces are processed
    pub fn calculate_statistics(&mut self) {
        if self.infrastructure.is_empty() {
            return;
        }

        let mut total_connectivity = 0.0;
        let mut max_conn: f32 = 0.0;
        let mut hubs = 0;
        let mut roads = 0;
        let mut routes = 0;

        for infra in self.infrastructure.values() {
            total_connectivity += infra.connectivity;
            max_conn = max_conn.max(infra.connectivity);
            if infra.is_hub {
                hubs += 1;
            }
            roads += infra.road_connections as usize;
            routes += infra.trade_routes as usize;
        }

        self.avg_connectivity = total_connectivity / self.infrastructure.len() as f32;
        self.max_connectivity = max_conn;
        self.hub_count = hubs;
        self.total_roads = roads / 2; // Each road counted twice (once per endpoint)
        self.total_trade_routes = routes / 2; // Each route counted twice
    }

    /// Normalize connectivity scores relative to the maximum
    pub fn normalized_connectivity(&self, connectivity: f32) -> f32 {
        if self.max_connectivity > 0.0 {
            connectivity / self.max_connectivity
        } else {
            connectivity
        }
    }
}