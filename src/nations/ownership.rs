//! Province ownership query utilities
//!
//! This module provides O(1) functions to query province ownership using ECS relationships.
//!
//! ## Design
//!
//! Province ownership uses Bevy's entity relationships:
//! - `ControlledBy(nation_entity)` on province entities (source of truth)
//! - `Controls` on nation entities (auto-maintained by Bevy)
//!
//! All queries are O(1) via the Controls component.

use bevy::prelude::*;
use crate::relationships::{Controls, ControlledBy};
use crate::world::ProvinceData;

/// Get province entities owned by a nation
///
/// O(1) - Bevy automatically maintains the Controls list.
#[inline]
pub fn get_nation_provinces(
    controls_query: &Query<&Controls>,
    nation_entity: Entity,
) -> Vec<Entity> {
    controls_query
        .get(nation_entity)
        .map(|controls| controls.provinces().to_vec())
        .unwrap_or_default()
}

/// Count provinces owned by a nation (O(1))
#[inline]
pub fn get_nation_province_count(
    controls_query: &Query<&Controls>,
    nation_entity: Entity,
) -> usize {
    controls_query
        .get(nation_entity)
        .map(|controls| controls.province_count())
        .unwrap_or(0)
}

/// Check if a nation owns any provinces (O(1))
#[inline]
pub fn nation_has_territory(
    controls_query: &Query<&Controls>,
    nation_entity: Entity,
) -> bool {
    controls_query
        .get(nation_entity)
        .map(|controls| controls.has_provinces())
        .unwrap_or(false)
}

/// Check if a nation owns a specific province
#[inline]
pub fn nation_owns_province(
    controlled_by_query: &Query<&ControlledBy>,
    province_entity: Entity,
    nation_entity: Entity,
) -> bool {
    controlled_by_query
        .get(province_entity)
        .map(|cb| cb.0 == nation_entity)
        .unwrap_or(false)
}

/// Get the owner of a province
#[inline]
pub fn get_province_owner(
    controlled_by_query: &Query<&ControlledBy>,
    province_entity: Entity,
) -> Option<Entity> {
    controlled_by_query.get(province_entity).ok().map(|cb| cb.0)
}

/// Get the geographic bounds of a nation's territory
///
/// Returns (min, max) Vec2 representing the bounding box.
pub fn get_nation_bounds(
    controls_query: &Query<&Controls>,
    province_data_query: &Query<&ProvinceData>,
    nation_entity: Entity,
) -> Option<(Vec2, Vec2)> {
    let controls = controls_query.get(nation_entity).ok()?;

    if !controls.has_provinces() {
        return None;
    }

    let mut min = Vec2::new(f32::MAX, f32::MAX);
    let mut max = Vec2::new(f32::MIN, f32::MIN);

    for &province_entity in controls.provinces() {
        if let Ok(data) = province_data_query.get(province_entity) {
            min.x = min.x.min(data.position.x);
            min.y = min.y.min(data.position.y);
            max.x = max.x.max(data.position.x);
            max.y = max.y.max(data.position.y);
        }
    }

    Some((min, max))
}

/// Calculate the centroid of a nation's territory
pub fn get_nation_centroid(
    controls_query: &Query<&Controls>,
    province_data_query: &Query<&ProvinceData>,
    nation_entity: Entity,
) -> Option<Vec2> {
    let controls = controls_query.get(nation_entity).ok()?;

    if !controls.has_provinces() {
        return None;
    }

    let mut sum = Vec2::ZERO;
    let mut count = 0;

    for &province_entity in controls.provinces() {
        if let Ok(data) = province_data_query.get(province_entity) {
            sum += data.position;
            count += 1;
        }
    }

    if count > 0 {
        Some(sum / count as f32)
    } else {
        None
    }
}
