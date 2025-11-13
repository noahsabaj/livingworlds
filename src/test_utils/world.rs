//! World generation utilities for testing
//!
//! Functions for creating small test worlds suitable for unit and integration tests.

use bevy::prelude::*;
use crate::world::{
    Province, ProvinceStorage, ProvinceId, TerrainType,
    Elevation, Agriculture, Distance, Abundance,
};

/// Generate a small test world for testing
pub fn generate_test_world(province_count: usize) -> ProvinceStorage {
    let mut provinces = Vec::with_capacity(province_count);

    for i in 0..province_count {
        provinces.push(Province {
            id: ProvinceId::new(i as u32),
            position: Vec2::new(
                (i % 100) as f32 * 10.0,
                (i / 100) as f32 * 10.0
            ),
            owner: None,
            culture: None,
            population: 1000,
            max_population: 10000,
            terrain: TerrainType::TemperateGrassland,
            elevation: Elevation::new(0.5),
            agriculture: Agriculture::new(0.5),
            fresh_water_distance: Distance::new(1.0),
            iron: Abundance::new(50),
            copper: Abundance::new(50),
            tin: Abundance::new(20),
            gold: Abundance::new(10),
            coal: Abundance::new(50),
            stone: Abundance::new(80),
            gems: Abundance::new(5),
            neighbors: [None; 6],
            neighbor_indices: [None; 6],
            version: 0,
            dirty: false,
        });
    }

    ProvinceStorage::from_provinces(provinces)
}