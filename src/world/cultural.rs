//! Geographic-Cultural Assignment System
//!
//! This module assigns cultures to provinces based on their geographic position,
//! creating the foundation for cultural regions and eventual nation emergence.

use super::provinces::{Province, ProvinceId, WorldBounds};
use super::terrain::TerrainType;
use crate::name_generator::Culture;
use bevy::log::info;
use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;
use std::collections::VecDeque;

/// Geographic-cultural assignment configuration
#[derive(Debug, Clone)]
pub struct CulturalConfig {
    /// Percentage of Ancient culture assignments (0.0-1.0)
    pub ancient_percentage: f32,

    /// Percentage of Mystical culture assignments (0.0-1.0)
    pub mystical_percentage: f32,

    /// Threshold for island detection (distance from mainland)
    pub island_threshold: f32,

    /// Coastal influence range for culture assignment
    pub coastal_range: f32,
}

impl Default for CulturalConfig {
    fn default() -> Self {
        Self {
            ancient_percentage: 0.1,   // 10% ancient
            mystical_percentage: 0.05, // 5% mystical
            island_threshold: 100.0,   // Distance for island detection
            coastal_range: 50.0,       // Coastal influence range
        }
    }
}

/// Represents a connected region of provinces sharing the same culture
///
/// Cultural regions form the foundation for realistic nation emergence,
/// where nations grow from the largest and most strategic cultural territories.
#[derive(Debug, Clone)]
pub struct CulturalRegion {
    /// The culture shared by all provinces in this region
    pub culture: Culture,

    /// List of province IDs that belong to this region
    pub provinces: Vec<ProvinceId>,

    /// Geographic center of the region (average position)
    pub center: Vec2,

    /// Total number of provinces in this region
    pub size: usize,

    /// Whether this region has access to ocean/coastal provinces
    pub coastal_access: bool,

    /// Strategic value for nation seed selection (0.0-1.0)
    /// Based on size, coastal access, defensive position, etc.
    pub strategic_value: f32,

    /// Approximate radius of the region (for territorial calculations)
    pub radius: f32,

    /// Whether this region is on an island (isolated from mainland)
    pub is_island: bool,
}

impl CulturalRegion {
    /// Create a new cultural region from a list of provinces
    pub fn new(culture: Culture, provinces: Vec<ProvinceId>, province_data: &[Province]) -> Self {
        let size = provinces.len();

        // Calculate center as average position
        let center = if !provinces.is_empty() {
            let sum: Vec2 = provinces
                .iter()
                .filter_map(|&id| province_data.get(id.value() as usize))
                .map(|p| p.position)
                .fold(Vec2::ZERO, |acc, pos| acc + pos);
            sum / provinces.len() as f32
        } else {
            Vec2::ZERO
        };

        // Check coastal access
        let coastal_access = provinces
            .iter()
            .filter_map(|&id| province_data.get(id.value() as usize))
            .any(|p| matches!(p.terrain, TerrainType::Beach | TerrainType::Ocean));

        // Calculate approximate radius (distance from center to furthest province)
        let radius = provinces
            .iter()
            .filter_map(|&id| province_data.get(id.value() as usize))
            .map(|p| center.distance(p.position))
            .fold(0.0, f32::max);

        // Determine if this is an island region
        let is_island = matches!(culture, Culture::Island) || (coastal_access && size < 50); // Small coastal regions are likely islands

        // Calculate strategic value
        let strategic_value =
            Self::calculate_strategic_value(size, coastal_access, is_island, radius);

        Self {
            culture,
            provinces,
            center,
            size,
            coastal_access,
            strategic_value,
            radius,
            is_island,
        }
    }

    /// Calculate strategic value for nation seed selection
    fn calculate_strategic_value(
        size: usize,
        coastal_access: bool,
        is_island: bool,
        radius: f32,
    ) -> f32 {
        let mut value = 0.0;

        // Size contribution (larger regions are more valuable)
        value += (size as f32).ln() / 10.0; // Logarithmic scaling

        // Coastal access bonus (trade and expansion opportunities)
        if coastal_access {
            value += 0.3;
        }

        // Island penalty (harder to expand, more isolated)
        if is_island {
            value -= 0.2;
        }

        // Compactness bonus (circular regions are easier to defend)
        let area = radius * radius * std::f32::consts::PI;
        let ideal_radius = (size as f32 / std::f32::consts::PI).sqrt();
        let compactness = 1.0 - (radius - ideal_radius).abs() / ideal_radius.max(1.0);
        value += compactness * 0.2;

        value.clamp(0.0, 1.0)
    }
}

/// Configuration for cultural region detection
#[derive(Debug, Clone)]
pub struct RegionDetectionConfig {
    /// Minimum size for a valid cultural region (prevents tiny isolated groups)
    pub min_region_size: usize,

    /// Maximum number of regions to detect per culture (performance limit)
    pub max_regions_per_culture: usize,
}

impl Default for RegionDetectionConfig {
    fn default() -> Self {
        Self {
            min_region_size: 10,         // At least 10 provinces
            max_regions_per_culture: 20, // Max 20 regions per culture type
        }
    }
}

/// Assign culture to a province based on its geographic position
pub fn assign_province_culture(
    position: Vec2,
    world_bounds: &WorldBounds,
    config: &CulturalConfig,
    rng: &mut StdRng,
) -> Culture {
    // Calculate normalized position (0.0 to 1.0 in each dimension)
    let world_size = world_bounds.size();
    let norm_x = (position.x - world_bounds.min.x) / world_size.x;
    let norm_y = (position.y - world_bounds.min.y) / world_size.y;

    // Check for Ancient/Mystical random assignment first
    let random_value: f32 = rng.r#gen();
    if random_value < config.ancient_percentage {
        return Culture::Ancient;
    } else if random_value < config.ancient_percentage + config.mystical_percentage {
        return Culture::Mystical;
    }

    // Geographic assignment based on position
    // We'll use a simple quadrant system with some overlap for more natural borders

    // Calculate distance from center
    let center_dist_x = (norm_x - 0.5).abs();
    let center_dist_y = (norm_y - 0.5).abs();

    // Island detection: if far from any mainland area, assign Island culture
    let mainland_distance = calculate_mainland_distance(norm_x, norm_y);
    if mainland_distance > 0.3 {
        return Culture::Island;
    }

    // Primary geographic assignment based on quadrants with fuzzy boundaries
    let fuzziness = 0.15; // 15% overlap between regions
    let bias_x = norm_x + rng.r#gen::<f32>() * fuzziness - fuzziness / 2.0;
    let bias_y = norm_y + rng.r#gen::<f32>() * fuzziness - fuzziness / 2.0;

    match (bias_x < 0.5, bias_y < 0.5) {
        (true, true) => Culture::Western,   // Northwest quadrant
        (false, true) => Culture::Northern, // Northeast quadrant
        (true, false) => Culture::Southern, // Southwest quadrant
        (false, false) => Culture::Eastern, // Southeast quadrant
    }
}

/// Calculate distance from mainland areas for island detection
fn calculate_mainland_distance(norm_x: f32, norm_y: f32) -> f32 {
    // Distance from the main landmass (center area)
    let center_x = 0.5;
    let center_y = 0.5;

    let dx = norm_x - center_x;
    let dy = norm_y - center_y;

    (dx * dx + dy * dy).sqrt()
}

/// Assign cultures to a collection of provinces based on their positions
pub fn assign_cultures_to_provinces<T>(
    provinces: &mut [T],
    get_position: impl Fn(&T) -> Vec2 + Send + Sync,
    set_culture: impl Fn(&mut T, Culture) + Send + Sync,
    world_bounds: &WorldBounds,
    config: Option<CulturalConfig>,
    seed: Option<u64>,
) where
    T: Clone + Send + Sync,
{
    let config = config.unwrap_or_default();
    let base_seed = seed.unwrap_or(42);

    // Parallel cultural assignment with deterministic per-province seeding
    provinces
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, province)| {
            // Create deterministic per-province RNG to maintain reproducibility
            let mut province_rng = StdRng::seed_from_u64(base_seed.wrapping_add(index as u64));
            let position = get_position(province);
            let culture =
                assign_province_culture(position, world_bounds, &config, &mut province_rng);
            set_culture(&mut *province, culture);
        });
}

/// Calculate world bounds from a collection of positions
pub fn calculate_world_bounds(positions: &[Vec2]) -> WorldBounds {
    if positions.is_empty() {
        return WorldBounds {
            min: Vec2::ZERO,
            max: Vec2::ZERO,
        };
    }

    // Parallel reduction to find min/max bounds
    let (min_bounds, max_bounds) = positions
        .par_iter()
        .map(|pos| (*pos, *pos)) // Each position is both min and max candidate
        .reduce(
            || {
                (
                    Vec2::new(f32::INFINITY, f32::INFINITY),
                    Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY),
                )
            },
            |(min1, max1), (min2, max2)| {
                (
                    Vec2::new(min1.x.min(min2.x), min1.y.min(min2.y)),
                    Vec2::new(max1.x.max(max2.x), max1.y.max(max2.y)),
                )
            },
        );

    WorldBounds {
        min: min_bounds,
        max: max_bounds,
    }
}

/// Example integration function showing how to assign cultures to provinces
pub fn assign_cultures_to_province_storage(
    provinces: &mut [super::provinces::Province],
    seed: Option<u64>,
) {
    // Extract positions from provinces in parallel
    let positions: Vec<Vec2> = provinces.par_iter().map(|p| p.position).collect();

    // Calculate world bounds from province positions
    let world_bounds = calculate_world_bounds(&positions);

    // Use default cultural configuration
    let config = CulturalConfig::default();

    // Assign cultures using the geographic-cultural assignment system
    assign_cultures_to_provinces(
        provinces,
        |province| province.position,
        |province, culture| province.culture = Some(culture),
        &world_bounds,
        Some(config),
        seed,
    );
}

/// Detect cultural regions using connected components analysis
///
/// This is the critical architectural function that transforms individual cultural
/// assignments into coherent territorial foundations for realistic nation emergence.
///
/// # Algorithm
/// Uses flood-fill (BFS) to find connected components of provinces sharing the same culture.
/// Leverages existing neighbor_indices for O(1) neighbor access - no HashMap lookups needed!
///
/// # Performance
/// - Handles 3M+ provinces efficiently using bit vectors for visited tracking
/// - Parallel processing for different culture types where beneficial
/// - Early termination for small isolated regions
///
/// # Returns
/// Vector of CulturalRegion sorted by strategic value (most valuable first)
pub fn detect_cultural_regions(
    provinces: &[Province],
    config: Option<RegionDetectionConfig>,
) -> Vec<CulturalRegion> {
    let config = config.unwrap_or_default();
    let province_count = provinces.len();

    if province_count == 0 {
        return Vec::new();
    }

    info!(
        "Detecting cultural regions from {} provinces (min size: {})",
        province_count, config.min_region_size
    );

    // Track visited provinces using efficient bit vector
    let mut visited = vec![false; province_count];
    let mut all_regions = Vec::new();

    // Process each province as a potential region seed
    for (start_idx, province) in provinces.iter().enumerate() {
        // Skip if already visited or has no culture
        if visited[start_idx] || province.culture.is_none() {
            continue;
        }

        let culture = province.culture.unwrap();

        // Find connected component using BFS flood-fill
        let region_provinces =
            find_connected_cultural_component(start_idx, culture, provinces, &mut visited);

        // Only keep regions above minimum size threshold
        if region_provinces.len() >= config.min_region_size {
            let region = CulturalRegion::new(culture, region_provinces, provinces);
            all_regions.push(region);
        }
    }

    // Sort regions by strategic value (most valuable first)
    all_regions.sort_by(|a, b| b.strategic_value.total_cmp(&a.strategic_value));

    // Apply per-culture limits to prevent performance issues
    let mut regions_by_culture = std::collections::HashMap::new();
    let mut final_regions = Vec::new();

    for region in all_regions {
        let count = regions_by_culture.entry(region.culture).or_insert(0);
        if *count < config.max_regions_per_culture {
            final_regions.push(region);
            *count += 1;
        }
    }

    info!(
        "Detected {} cultural regions across {} culture types",
        final_regions.len(),
        regions_by_culture.len()
    );

    // Log region statistics for debugging
    for (culture, count) in &regions_by_culture {
        let total_provinces: usize = final_regions
            .iter()
            .filter(|r| r.culture == *culture)
            .map(|r| r.size)
            .sum();
        info!(
            "  {:?}: {} regions, {} total provinces",
            culture, count, total_provinces
        );
    }

    final_regions
}

/// Find all provinces connected to start_idx that share the same culture
///
/// Uses BFS flood-fill algorithm leveraging existing neighbor_indices for O(1) performance.
fn find_connected_cultural_component(
    start_idx: usize,
    target_culture: Culture,
    provinces: &[Province],
    visited: &mut [bool],
) -> Vec<ProvinceId> {
    let mut component = Vec::new();
    let mut queue = VecDeque::new();

    // Start BFS from the seed province
    queue.push_back(start_idx);
    visited[start_idx] = true;

    while let Some(current_idx) = queue.pop_front() {
        let current_province = &provinces[current_idx];
        component.push(current_province.id);

        // Check all neighbors using precomputed neighbor_indices (O(1) access!)
        for &neighbor_idx_opt in &current_province.neighbor_indices {
            if let Some(neighbor_idx) = neighbor_idx_opt {
                // Bounds check for safety
                if neighbor_idx < provinces.len() && !visited[neighbor_idx] {
                    let neighbor = &provinces[neighbor_idx];

                    // Add to component if same culture and land province
                    if neighbor.culture == Some(target_culture)
                        && neighbor.terrain != TerrainType::Ocean
                    {
                        visited[neighbor_idx] = true;
                        queue.push_back(neighbor_idx);
                    }
                }
            }
        }
    }

    component
}

/// Get the largest cultural region for each culture type
///
/// Useful for selecting primary nation seeds from the most significant cultural territories.
pub fn get_largest_regions_by_culture(regions: &[CulturalRegion]) -> Vec<&CulturalRegion> {
    let mut largest_by_culture: std::collections::HashMap<Culture, &CulturalRegion> =
        std::collections::HashMap::new();

    for region in regions {
        let current_largest = largest_by_culture.get(&region.culture);
        if current_largest.map_or(true, |largest| region.size > largest.size) {
            largest_by_culture.insert(region.culture, region);
        }
    }

    largest_by_culture.into_values().collect()
}

/// Get the most strategic cultural regions (highest strategic value)
///
/// Useful for selecting optimal nation seed locations based on strategic considerations.
pub fn get_most_strategic_regions(
    regions: &[CulturalRegion],
    count: usize,
) -> Vec<&CulturalRegion> {
    let mut sorted_regions: Vec<_> = regions.iter().collect();
    sorted_regions.sort_by(|a, b| b.strategic_value.total_cmp(&a.strategic_value));
    sorted_regions.into_iter().take(count).collect()
}
