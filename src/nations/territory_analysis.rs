//! Territory analysis for nation label positioning and scaling
//!
//! This module calculates geographic properties of nation territories
//! to enable proper label spanning across controlled provinces.

use bevy::prelude::*;
use std::collections::HashSet;

use crate::world::ProvinceStorage;

/// Computed territory metrics for a nation
///
/// This is now a Component attached to nation entities rather than a cached resource.
#[derive(Debug, Clone, Component, Reflect, Resource, Default)]
#[reflect(Component)]
pub struct TerritoryMetrics {
    /// Bounding box of all controlled provinces (min, max)
    pub bounds: (Vec2, Vec2),
    /// Geographic centroid of the territory
    pub centroid: Vec2,
    /// Number of provinces in the territory
    pub province_count: usize,
    /// Maximum distance from centroid to any province (for scaling)
    pub max_radius: f32,
    /// Whether the territory is contiguous or has disconnected parts
    pub is_contiguous: bool,
    /// Major territory clusters for disconnected empires
    pub clusters: Vec<TerritoryCluster>,
}

/// A contiguous cluster of provinces (for handling disconnected territories)
#[derive(Debug, Clone, Reflect)]
pub struct TerritoryCluster {
    /// Provinces in this cluster
    pub province_ids: HashSet<u32>,
    /// Cluster centroid
    pub centroid: Vec2,
    /// Cluster bounds
    pub bounds: (Vec2, Vec2),
    /// Size relative to total territory (0.0 to 1.0)
    pub relative_size: f32,
}

impl TerritoryMetrics {
    /// Calculate territory metrics for a nation
    ///
    /// Uses direct province iteration instead of ownership cache for single source of truth.
    pub fn calculate(
        nation_entity: Entity,
        province_storage: &ProvinceStorage,
    ) -> Option<Self> {
        // Collect province IDs owned by this nation directly from province storage
        let province_ids: HashSet<u32> = province_storage
            .provinces
            .iter()
            .filter(|p| p.owner_entity == Some(nation_entity))
            .map(|p| p.id.value())
            .collect();

        if province_ids.is_empty() {
            return None;
        }

        // Calculate bounds and centroid in a single pass
        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut max_radius: f32 = 0.0;

        // First pass: calculate bounds and centroid
        for province in province_storage.provinces.iter() {
            if province.owner_entity == Some(nation_entity) {
                let pos = province.position;
                min_x = min_x.min(pos.x);
                max_x = max_x.max(pos.x);
                min_y = min_y.min(pos.y);
                max_y = max_y.max(pos.y);
                sum_x += pos.x;
                sum_y += pos.y;
            }
        }

        let valid_provinces = province_ids.len();
        let bounds = (Vec2::new(min_x, min_y), Vec2::new(max_x, max_y));
        let centroid = Vec2::new(
            sum_x / valid_provinces as f32,
            sum_y / valid_provinces as f32,
        );

        // Second pass: calculate max radius from centroid
        for province in province_storage.provinces.iter() {
            if province.owner_entity == Some(nation_entity) {
                let distance = centroid.distance(province.position);
                max_radius = max_radius.max(distance);
            }
        }

        // Detect clusters for disconnected territories
        let clusters = detect_territory_clusters(
            &province_ids,
            province_storage,
            valid_provinces,
        );

        let is_contiguous = clusters.len() <= 1;

        Some(TerritoryMetrics {
            bounds,
            centroid,
            province_count: valid_provinces,
            max_radius,
            is_contiguous,
            clusters,
        })
    }

    /// Get the width of the territory bounding box
    pub fn width(&self) -> f32 {
        self.bounds.1.x - self.bounds.0.x
    }

    /// Get the height of the territory bounding box
    pub fn height(&self) -> f32 {
        self.bounds.1.y - self.bounds.0.y
    }

    /// Get the area of the bounding box
    pub fn bounding_area(&self) -> f32 {
        self.width() * self.height()
    }

    /// Estimate if this is a small, medium, or large nation
    pub fn size_category(&self) -> NationSizeCategory {
        match self.province_count {
            0..=5 => NationSizeCategory::Tiny,
            6..=20 => NationSizeCategory::Small,
            21..=50 => NationSizeCategory::Medium,
            51..=150 => NationSizeCategory::Large,
            _ => NationSizeCategory::Empire,
        }
    }

    /// Get optimal font size base value for this territory
    pub fn optimal_base_font_size(&self) -> f32 {
        // Base size on territory width, with adjustments for shape
        let width = self.width();
        let height = self.height();

        // Use the smaller dimension to ensure text fits
        let primary_dimension = width.min(height * 1.5); // Account for text being horizontal

        // Scale based on territory size - INCREASED for better readability (2.5x larger)
        let base = match self.size_category() {
            NationSizeCategory::Tiny => primary_dimension * 0.20,    // was 0.08
            NationSizeCategory::Small => primary_dimension * 0.25,   // was 0.10
            NationSizeCategory::Medium => primary_dimension * 0.30,  // was 0.12
            NationSizeCategory::Large => primary_dimension * 0.35,   // was 0.14
            NationSizeCategory::Empire => primary_dimension * 0.40,  // was 0.16
        };

        // Larger minimum for readability, higher maximum for empires
        base.clamp(24.0, 320.0)  // was 12.0, 200.0
    }
}

/// Categories for nation sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NationSizeCategory {
    Tiny,    // City-states, 1-5 provinces
    Small,   // Minor nations, 6-20 provinces
    Medium,  // Regional powers, 21-50 provinces
    Large,   // Major powers, 51-150 provinces
    Empire,  // Continental empires, 150+ provinces
}

/// Detect disconnected territory clusters using flood-fill
fn detect_territory_clusters(
    province_ids: &HashSet<u32>,
    province_storage: &ProvinceStorage,
    total_provinces: usize,
) -> Vec<TerritoryCluster> {
    let mut clusters = Vec::new();
    let mut visited = HashSet::new();

    for &province_id in province_ids {
        if visited.contains(&province_id) {
            continue;
        }

        // Start a new cluster from this province
        let mut cluster_provinces = HashSet::new();
        let mut to_visit = vec![province_id];
        let mut cluster_min = Vec2::new(f32::INFINITY, f32::INFINITY);
        let mut cluster_max = Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
        let mut cluster_sum = Vec2::ZERO;
        let mut cluster_count = 0;

        while let Some(current_id) = to_visit.pop() {
            if !visited.insert(current_id) {
                continue;
            }

            cluster_provinces.insert(current_id);

            // Direct array access - province_id IS the array index
            if let Some(province) = province_storage.provinces.get(current_id as usize) {
                // Update cluster metrics
                let pos = province.position;
                cluster_min = cluster_min.min(pos);
                cluster_max = cluster_max.max(pos);
                cluster_sum += pos;
                cluster_count += 1;

                // Check neighbors
                for neighbor_opt in &province.neighbors {
                    if let Some(neighbor_id) = neighbor_opt {
                        let neighbor_id_u32 = neighbor_id.value();
                        if province_ids.contains(&neighbor_id_u32) && !visited.contains(&neighbor_id_u32) {
                            to_visit.push(neighbor_id_u32);
                        }
                    }
                }
            }
        }

        if cluster_count > 0 {
            let centroid = cluster_sum / cluster_count as f32;
            let relative_size = cluster_count as f32 / total_provinces as f32;

            clusters.push(TerritoryCluster {
                province_ids: cluster_provinces,
                centroid,
                bounds: (cluster_min, cluster_max),
                relative_size,
            });
        }
    }

    // Sort clusters by size (largest first)
    clusters.sort_by(|a, b| {
        b.relative_size
            .partial_cmp(&a.relative_size)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    clusters
}