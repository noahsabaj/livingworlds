//! World generation utilities for testing
//!
//! Functions for creating small test worlds suitable for unit and integration tests.

use bevy::prelude::*;
use crate::world::provinces::types::{Province, ProvinceStorage};

/// Generate a small test world for testing
pub fn generate_test_world(province_count: usize) -> ProvinceStorage {
    let mut provinces = Vec::with_capacity(province_count);

    for i in 0..province_count {
        provinces.push(Province {
            id: i as u32,
            position: Vec2::new(
                (i % 100) as f32 * 10.0,
                (i / 100) as f32 * 10.0
            ),
            terrain: crate::world::terrain::types::TerrainType::Plains,
            elevation: 0.5,
            temperature: 0.5,
            humidity: 0.5,
            population: 1000,
            development: 0.1,
            agriculture: 0.5,
            fresh_water_distance: 1,
            ocean_distance: 10,
            culture: None,
        });
    }

    ProvinceStorage::from_vec(provinces)
}