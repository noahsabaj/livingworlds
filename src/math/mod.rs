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
//! - [`interpolation`] - Game-specific interpolation, smoothing, and blending functions
//! - [`distance`] - Game-specific distance functions (hex distance, falloff, influence)
//! - [`angles`] - Game-specific angle calculations and utilities
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
//! The interpolation module provides GAME-SPECIFIC interpolation functions for Living Worlds.
//! For basic linear interpolation, use Bevy's built-in methods directly:
//! - `a.lerp(b, t)` for most types (f32, Vec2, Vec3, Color, etc.)
//! - `pos1.lerp(pos2, t)` for position interpolation
//! - `color1.lerp(color2, t)` for basic color transitions
//!
//! ### Basic Usage
//! ```rust
//! use crate::math::{smoothstep, lerp_exp_vec3, asymmetric_smooth, lerp_color};
//!
//! // Basic interpolation - use Bevy directly
//! let value = 0.0_f32.lerp(100.0, 0.5); // Returns 50.0
//! let position = start_pos.lerp(end_pos, t);
//!
//! // Game-specific functions from this module
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
//! let tension = asymmetric_smooth(
//!     current_tension,
//!     target_tension,
//!     rise_rate,  // 2.0 = fast rise
//!     fall_rate,  // 0.3 = slow fall
//!     delta_time
//! );
//!
//! // Linear color space interpolation
//! let blended_color = lerp_color(color_a, color_b, t);
//!
//! // Weighted blending of multiple values
//! use crate::math::{weighted_blend, weighted_blend_colors};
//! let result = weighted_blend(&[(value1, weight1), (value2, weight2)]);
//! let color = weighted_blend_colors(&[(color1, weight1), (color2, weight2)]);
//!
//! // Utility functions
//! use crate::math::{inverse_lerp, remap};
//! let t = inverse_lerp(min, max, value); // Get t from value
//! let remapped = remap(value, old_min, old_max, new_min, new_max);
//! ```
//!
//! ### Game-Specific Functions
//! #### Advanced Smoothing
//! - `lerp_exp(current, target, smoothing, dt)` - Frame-rate independent smoothing
//! - `lerp_exp_vec3(current, target, smoothing, dt)` - Exponential for Vec3
//! - `asymmetric_smooth(current, target, rise, fall, dt)` - Different rise/fall rates
//! - `exponential_smooth(current, target, factor)` - Simple exponential blend
//!
//! #### S-Curve Interpolation
//! - `smoothstep(edge0, edge1, x)` - S-curve smoothing
//! - `smootherstep(edge0, edge1, x)` - Smoother S-curve (Ken Perlin's version)
//!
//! #### Multi-Value Blending
//! - `weighted_blend(values_weights)` - Multi-value blending with normalization
//! - `weighted_blend_colors(colors_weights)` - Multi-color blending in linear space
//! - `weighted_blend_vec2/vec3` - Position blending with weights
//!
//! #### Color & Range Utilities
//! - `lerp_color(a, b, t)` - Linear color space interpolation
//! - `inverse_lerp(a, b, value)` - Get t from interpolated value
//! - `remap(value, old_min, old_max, new_min, new_max)` - Range remapping
//!
//! ### Common Use Cases
//! 1. **Camera Movement**: `lerp_exp_vec3()` for smooth following
//! 2. **UI Animations**: `smoothstep()` for fade effects
//! 3. **World Tension**: `asymmetric_smooth()` for physics-like behavior
//! 4. **Climate**: `exponential_smooth()` for rainfall blending
//! 5. **Terrain Colors**: `lerp_color()` for accurate biome transitions
//!
//! ---
//!
//! ## Distance Calculations (`distance` module)
//!
//! ### Overview
//! The distance module provides GAME-SPECIFIC distance functions for Living Worlds.
//! For basic distance calculations, use Bevy's Vec2/Vec3 methods directly:
//! - `pos1.distance(pos2)` for Euclidean distance
//! - `pos1.distance_squared(pos2)` for faster comparisons
//! - `(pos1 - pos2).abs().x + (pos1 - pos2).abs().y` for Manhattan distance
//!
//! ### Basic Usage
//! ```rust
//! use crate::math::{hex_distance, gaussian_falloff};
//!
//! // Basic distance - use Bevy directly
//! let dist = pos1.distance(pos2);
//! let dist_sq = pos1.distance_squared(pos2); // Faster for comparisons
//!
//! // Game-specific functions from this module
//! let hex_steps = hex_distance(col1, row1, col2, row2);
//! let influence = gaussian_falloff(distance, sigma);
//!
//! // Find closest point from a list
//! use crate::math::{find_closest, find_within_radius};
//! let (idx, dist) = find_closest(target, &points).unwrap();
//! let nearby = find_within_radius(center, &points, radius);
//! ```
//!
//! ### Game-Specific Functions
//! - **Hexagonal Distance**: Complex grid distance calculation for hex grids
//! - **Falloff Functions**: Influence calculations for minerals, territories, etc.
//! - **Boundary Detection**: Distance from map edges for continent generation
//! - **Spatial Queries**: Efficient nearest-neighbor and radius searches
//!
//! ### Falloff Functions
//! - `linear_falloff(dist, max)` - Linear decrease from 1.0 to 0.0
//! - `quadratic_falloff(dist, max)` - Quadratic decrease (smoother)
//! - `gaussian_falloff(dist, sigma)` - Bell curve falloff
//! - `inverse_square_falloff(dist, scale)` - Physical falloff (light, gravity)
//! - `smooth_falloff(dist, inner, outer)` - Smooth transition between radii
//!
//! ### Common Use Cases
//! 1. **Province Selection**: `pos1.distance(pos2)` for mouse picking
//! 2. **Mineral Influence**: `gaussian_falloff()` for ore vein effects
//! 3. **Ocean Distance**: BFS with `hex_distance()` for water proximity
//! 4. **Map Falloff**: `normalized_edge_distance()` for island generation
//! 5. **Pathfinding**: `hex_distance()` for hexagonal grid heuristics
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

// Game-specific interpolation and smoothing exports
pub use interpolation::{
    asymmetric_smooth, exponential_smooth, inverse_lerp, lerp, lerp_color, lerp_exp, lerp_exp_vec2,
    lerp_exp_vec3, remap, smootherstep, smoothstep, weighted_blend, weighted_blend_colors,
    weighted_blend_vec2, weighted_blend_vec3,
};

// Game-specific distance calculation exports
pub use distance::{
    calculate_influence, distance_from_rect_edge, find_closest, find_within_radius,
    gaussian_falloff, hex_distance, hex_distance_world, inverse_square_falloff, linear_falloff,
    normalized_edge_distance, quadratic_falloff, smooth_falloff, FalloffType,
};

// Game-specific angle calculation exports
pub use angles::{
    angle_in_range, angle_variation, angular_distance, fast_cos, fast_sin, lerp_angle,
    movement_vector, positions_around_circle, smoothstep_angle,
};
