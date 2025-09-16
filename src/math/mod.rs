//! # Math Module - Single Source of Truth for All Mathematical Operations
//!
//! This module is the **EXCLUSIVE** source for all mathematical operations in Living Worlds.
//!
//! ## ⚠️ CRITICAL DIRECTIVES
//!
//! - **NO parallel implementations** - All math MUST come from this module
//! - **NO local copies** - Import from `crate::math`, never reimplement
//! - **NO direct external crate usage** - Only this module uses `noise` crate, etc.
//!
//! ## Module Structure
//!
//! - [`hexagon`] - All hexagon geometry calculations (flat-top, odd-q offset)
//! - [`perlin`] - All noise generation (terrain, clouds, everything)
//! - [`interpolation`] - All interpolation, smoothing, and blending operations
//! - [`distance`] - All distance calculations (Euclidean, Manhattan, hexagonal, etc.)
//! - [`angles`] - All angle calculations, trigonometry, and rotation utilities
//! - [`random`] - All random number generation and distribution utilities
//!
//! ---
//!
//! ## Hexagon Geometry (`hexagon` module)
//!
//! ### Overview
//! The hexagon module provides ALL hexagon calculations for the flat-top, odd-q offset
//! coordinate system used throughout Living Worlds. This is the ONLY source for hex math.
//!
//! ### ⛔ FORBIDDEN
//! - Reimplementing hex position calculations elsewhere
//! - Duplicating SQRT_3 or other hex constants
//! - Local point-in-hexagon tests
//! - Any hexagon math outside this module
//!
//! ### Basic Usage
//! ```rust
//! use crate::math::{Hexagon, calculate_grid_position, HEX_SIZE, SQRT_3};
//!
//! // Calculate world position from grid coordinates
//! let (x, y) = calculate_grid_position(col, row, HEX_SIZE);
//!
//! // Create a hexagon at a position
//! let hex = Hexagon::new(Vec2::new(100.0, 100.0));
//!
//! // Test if point is inside hexagon
//! if hex.contains_point(mouse_pos) {
//!     // Handle click
//! }
//!
//! // Get hexagon vertices for rendering
//! let vertices = hex.vertices();
//!
//! // Get neighbor positions
//! let neighbors = get_neighbor_positions(col, row);
//! ```
//!
//! ### Constants Available
//! - `HEX_SIZE` - Default hexagon radius (50.0 pixels)
//! - `SQRT_3` - √3 constant for hex calculations
//! - `VERTICES_PER_HEX` - 7 vertices (center + 6 corners)
//! - `TRIANGLES_PER_HEX` - 6 triangles for rendering
//! - `INDICES_PER_HEX` - 18 indices for triangle mesh
//!
//! ### Key Functions
//! - `calculate_grid_position(col, row, size)` - Grid to world coordinates
//! - `world_to_grid(x, y, size)` - World to grid coordinates
//! - `get_neighbor_positions(col, row)` - Get all 6 neighbor coordinates
//! - `validate_position(col, row, width, height)` - Check if position is valid
//! - `quantize_position(x, y, size)` - Snap to nearest hex center
//!
//! ---
//!
//! ## Perlin Noise Generation (`perlin` module)
//!
//! ### Overview
//! The perlin module provides ALL noise generation for Living Worlds. It handles terrain
//! generation, cloud patterns, and any other procedural noise needs. This is the ONLY
//! source for noise generation.
//!
//! ### ⛔ FORBIDDEN
//! - Using `noise` crate directly anywhere else
//! - Implementing custom noise functions
//! - Duplicating noise constants or parameters
//! - Any form of noise generation outside this module
//!
//! ### Basic Usage
//! ```rust
//! use crate::math::{PerlinNoise, TerrainPreset, CloudPreset, FbmSettings};
//!
//! // Create noise generator with seed
//! let noise = PerlinNoise::with_seed(world_seed);
//!
//! // Generate terrain elevation (0.0 to 1.0)
//! let elevation = noise.sample_terrain(x, y);
//!
//! // Generate terrain with preset
//! let elevation = noise.sample_terrain_preset(x, y, TerrainPreset::Continents);
//!
//! // Generate cloud density
//! let cloud_density = noise.sample_clouds(x, y, CloudPreset::Fluffy);
//!
//! // Custom FBM for special effects
//! let value = noise.sample_fbm(x, y, FbmSettings {
//!     octaves: 6,
//!     frequency: 0.02,
//!     persistence: 0.5,
//!     lacunarity: 2.0,
//! });
//!
//! // Use builder pattern for advanced configuration
//! use crate::math::PerlinBuilder;
//! let custom_noise = PerlinBuilder::new()
//!     .with_seed(12345)
//!     .with_frequency(0.015)
//!     .with_octaves(8)
//!     .build();
//! ```
//!
//! ### Terrain Presets
//! - `TerrainPreset::Default` - Balanced terrain for most worlds
//! - `TerrainPreset::Continents` - Large continental masses
//! - `TerrainPreset::Islands` - Archipelago with many islands
//! - `TerrainPreset::Mountains` - Mountainous terrain with peaks
//!
//! ### Cloud Presets
//! - `CloudPreset::Fluffy` - Cumulus-like puffy clouds
//! - `CloudPreset::Wispy` - Thin, stretched cirrus clouds
//! - `CloudPreset::Dense` - Thick overcast coverage
//! - `CloudPreset::Storm` - Heavy storm clouds
//!
//! ### Key Methods
//! - `sample_terrain(x, y)` - Combined terrain generation
//! - `sample_clouds(x, y, preset)` - Cloud generation with presets
//! - `sample_fbm(x, y, settings)` - Fractal Brownian Motion
//! - `sample_ridged(x, y, frequency)` - Ridge noise for mountains
//! - `sample_billow(x, y, frequency)` - Billow noise for clouds
//!
//! ---
//!
//! ## Interpolation & Smoothing (`interpolation` module)
//!
//! ### Overview
//! The interpolation module provides ALL interpolation, smoothing, and blending operations
//! for Living Worlds. This includes animation smoothing, color blending, and all forms of
//! value transitions. This is the ONLY source for interpolation math.
//!
//! ### ⛔ FORBIDDEN
//! - Manual lerp implementations like `a * (1.0 - t) + b * t`
//! - Local smoothstep or easing functions
//! - Duplicate exponential smoothing code
//! - Any interpolation math outside this module
//!
//! ### Basic Usage
//! ```rust
//! use crate::math::{lerp, lerp_vec3, smoothstep, lerp_exp, lerp_color};
//!
//! // Linear interpolation
//! let value = lerp(0.0, 100.0, 0.5); // Returns 50.0
//! let position = lerp_vec3(start_pos, end_pos, t);
//!
//! // Smooth S-curve transitions
//! let smooth_t = smoothstep(0.0, 1.0, raw_t);
//! let smoother_t = smootherstep(0.0, 1.0, raw_t);
//!
//! // Frame-rate independent exponential smoothing (for animations)
//! let camera_pos = lerp_exp_vec3(
//!     current_position,
//!     target_position,
//!     smoothing_factor, // 8.0 = smooth, 15.0 = snappy
//!     delta_time
//! );
//!
//! // Asymmetric smoothing (different rise/fall rates)
//! use crate::math::asymmetric_smooth;
//! let tension = asymmetric_smooth(
//!     current_tension,
//!     target_tension,
//!     rise_rate,  // 2.0 = fast rise
//!     fall_rate,  // 0.3 = slow fall
//!     delta_time
//! );
//!
//! // Color interpolation
//! let blended_color = lerp_color(color_a, color_b, t);
//!
//! // Weighted blending of multiple values
//! use crate::math::{weighted_blend, weighted_blend_colors};
//! let result = weighted_blend(&values, &weights);
//! let color = weighted_blend_colors(&colors, &weights);
//!
//! // Utility functions
//! use crate::math::{inverse_lerp, remap};
//! let t = inverse_lerp(min, max, value); // Get t from value
//! let remapped = remap(value, old_min, old_max, new_min, new_max);
//! ```
//!
//! ### Key Functions
//!
//! #### Basic Interpolation
//! - `lerp(a, b, t)` - Linear interpolation for f32
//! - `lerp_vec2(a, b, t)` - Linear interpolation for Vec2
//! - `lerp_vec3(a, b, t)` - Linear interpolation for Vec3
//! - `lerp_color(a, b, t)` - Color space interpolation
//!
//! #### Smoothing Functions
//! - `smoothstep(edge0, edge1, x)` - S-curve smoothing
//! - `smootherstep(edge0, edge1, x)` - Smoother S-curve
//! - `lerp_exp(current, target, smoothing, dt)` - Exponential smoothing
//! - `lerp_exp_vec3(current, target, smoothing, dt)` - Exponential for Vec3
//!
//! #### Advanced Functions
//! - `exponential_smooth(current, target, factor)` - Simple exponential blend
//! - `asymmetric_smooth(current, target, rise, fall, dt)` - Different rates
//! - `weighted_blend(values, weights)` - Multi-value blending
//! - `weighted_blend_colors(colors, weights)` - Multi-color blending
//!
//! #### Utility Functions
//! - `inverse_lerp(a, b, value)` - Get t from interpolated value
//! - `remap(value, old_min, old_max, new_min, new_max)` - Range remapping
//!
//! ### Common Use Cases
//! 1. **Camera Movement**: `lerp_exp_vec3()` for smooth following
//! 2. **UI Animations**: `smoothstep()` for fade effects
//! 3. **World Tension**: `asymmetric_smooth()` for physics
//! 4. **Climate**: `exponential_smooth()` for rainfall blending
//! 5. **Terrain Colors**: `lerp_color()` for biome transitions
//!
//! ---
//!
//! ## Distance Calculations (`distance` module)
//!
//! ### Overview
//! The distance module provides ALL distance calculations for Living Worlds. This includes
//! geometric distances (Euclidean, Manhattan, Chebyshev), hexagonal grid distances, and
//! specialized game mechanics like falloff and influence calculations.
//!
//! ### ⛔ FORBIDDEN
//! - Manual distance calculations like `sqrt((x2-x1)² + (y2-y1)²)`
//! - Using Vec2::distance() directly (use our wrappers for consistency)
//! - Implementing custom falloff functions
//! - Any distance math outside this module
//!
//! ### Basic Usage
//! ```rust
//! use crate::math::{
//!     euclidean_vec2, euclidean_squared_vec2,
//!     manhattan_vec2, chebyshev_vec2,
//!     hex_distance, gaussian_falloff
//! };
//!
//! // Standard Euclidean distance
//! let dist = euclidean_vec2(pos1, pos2);
//!
//! // Squared distance (faster for comparisons)
//! if euclidean_squared_vec2(a, b) < radius_squared {
//!     // Point is within radius
//! }
//!
//! // Manhattan distance for grid movement
//! let grid_dist = manhattan_vec2(start, end);
//!
//! // Hexagon grid distance
//! let hex_steps = hex_distance(col1, row1, col2, row2);
//!
//! // Distance with falloff for influence
//! let influence = gaussian_falloff(distance, sigma);
//!
//! // Find closest point from a list
//! use crate::math::{find_closest, find_within_radius};
//! let (idx, dist) = find_closest(target, &points).unwrap();
//! let nearby = find_within_radius(center, &points, radius);
//! ```
//!
//! ### Distance Types
//! - **Euclidean**: Standard straight-line distance (Pythagorean theorem)
//! - **Manhattan**: Grid-based "taxicab" distance (sum of coordinate differences)
//! - **Chebyshev**: Maximum coordinate difference (king moves in chess)
//! - **Hexagonal**: Minimum steps between hexagons in odd-q offset grid
//!
//! ### Falloff Functions
//! - `linear_falloff(dist, max)` - Linear decrease from 1.0 to 0.0
//! - `quadratic_falloff(dist, max)` - Quadratic decrease (smoother)
//! - `gaussian_falloff(dist, sigma)` - Bell curve falloff
//! - `inverse_square_falloff(dist, scale)` - Physical falloff (light, gravity)
//! - `smooth_falloff(dist, inner, outer)` - Smooth transition between radii
//!
//! ### Performance Functions
//! - `euclidean_squared_*()` - Avoids sqrt for comparison only
//! - `batch_distances()` - Calculate many distances efficiently
//! - `find_within_radius()` - Spatial queries with early termination
//!
//! ### Common Use Cases
//! 1. **Province Selection**: `euclidean_vec2()` for mouse picking
//! 2. **Mineral Influence**: `gaussian_falloff()` for ore vein effects
//! 3. **Ocean Distance**: BFS with `hex_distance()` for water proximity
//! 4. **Map Falloff**: `normalized_edge_distance()` for island generation
//! 5. **Pathfinding**: `manhattan_vec2()` or `hex_distance()` for heuristics
//!
//! ---
//!
//! ## Performance Notes
//!
//! All functions in this module are:
//! - **Inlined** for zero overhead
//! - **SIMD-friendly** where applicable
//! - **Const where possible** for compile-time evaluation
//! - **Thread-safe** without synchronization overhead
//!
//! ## Testing
//!
//! Each submodule includes comprehensive unit tests. Run with:
//! ```bash
//! cargo test math
//! ```

// INTERNAL MODULES - ALL PRIVATE
// Gateway Architecture: All submodules are private. External code must use
// the controlled re-exports below. This ensures all math operations go through
// this single gateway, maintaining our "single source of truth" principle.
mod angles;
mod distance;
mod hexagon;
mod interpolation;
mod perlin;
mod random;

// Test modules - only compiled in test mode
// TODO: Create these test modules when needed
// #[cfg(test)]
// mod distance_tests;
// #[cfg(test)]
// mod hexagon_tests;
// #[cfg(test)]
// mod interpolation_tests;
// #[cfg(test)]
// mod test_utils;

// Only these carefully selected exports are available to external code.
// This enforces our "single source of truth" principle - all math operations
// must go through this gateway module.

// Hexagon geometry exports
pub use hexagon::{
    calculate_grid_position, get_neighbor_positions, quantize_position, validate_position,
    world_to_grid, Hexagon, CORNERS, HALF, HEX_SIZE, INDICES_PER_HEX, SQRT_3, TRIANGLES_PER_HEX,
    VERTICES_PER_HEX,
};

// Perlin noise generation exports
pub use perlin::{CloudPreset, FbmSettings, PerlinBuilder, PerlinNoise, TerrainPreset};

// Interpolation and smoothing exports
pub use interpolation::{
    asymmetric_smooth, exponential_smooth, inverse_lerp, lerp, lerp_color, lerp_exp, lerp_exp_vec2,
    lerp_exp_vec3, lerp_vec2, lerp_vec3, remap, smootherstep, smoothstep, weighted_blend,
    weighted_blend_colors,
};

// Distance calculation exports
pub use distance::{
    batch_distances,
    batch_distances_squared,
    // Game utilities
    calculate_influence,
    chebyshev_2d,
    chebyshev_vec2,
    // Edge and wrapping distances
    distance_from_rect_edge,
    euclidean_2d,
    euclidean_squared_2d,
    euclidean_squared_vec2,
    euclidean_vec2,
    euclidean_vec3,
    find_closest,
    find_within_radius,
    gaussian_falloff,
    // Hexagon distances
    hex_distance,
    hex_distance_world,
    inverse_square_falloff,
    // Falloff functions
    linear_falloff,
    manhattan_2d,
    manhattan_3d,
    manhattan_vec2,
    normalized_edge_distance,
    quadratic_falloff,
    smooth_falloff,
    toroidal_distance_2d,
    wrapping_distance_2d,
    FalloffType,
};

// Angle and trigonometry exports
pub use angles::{
    // Calculations
    angle_between,
    angle_in_range,
    angle_variation,
    angular_distance,
    // Conversions
    degrees_to_radians,
    fast_cos,
    // Trigonometric helpers
    fast_sin,
    // Interpolation
    lerp_angle,
    movement_vector,
    // Normalization
    normalize_angle,
    normalize_angle_signed,
    position_on_circle,
    positions_around_circle,
    radians_to_degrees,
    sin_cos,
    smoothstep_angle,
    // Vector operations
    unit_vector_from_angle,
    vector_from_angle,
    wrap_degrees,
    wrap_degrees_signed,
    DEG_TO_RAD,
    HALF_PI,
    PI,
    QUARTER_PI,
    RAD_TO_DEG,
    TAU,
};

// Random generation exports
pub use random::{
    choose,
    choose_multiple,
    // RNG creation
    create_rng,
    create_rng_multi,
    // Deterministic
    hash_random,
    hash_random_int,
    random_01,
    random_11,
    random_bool,
    random_color_variation,
    random_exponential,
    random_hex_offset,
    // Distributions
    random_normal,
    random_point_in_circle,
    // Geometric
    random_point_in_rect,
    random_point_on_circle,
    random_range,
    random_spaced_positions,
    random_unit_vector,
    // Game utilities
    random_variation,
    random_vector,
    random_weighted_index,
    shuffle,
};
