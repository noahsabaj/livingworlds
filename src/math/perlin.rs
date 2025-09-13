//! Single source of truth for all Perlin noise generation in Living Worlds
//!
//! This module provides ready-to-use noise functions for terrain, clouds, and other
//! procedural generation needs. No configuration required - just create and use!
//!
//! # Quick Start
//!
//! ```rust
//! use crate::math::perlin::PerlinNoise;
//!
//! // Create a noise generator with a seed
//! let noise = PerlinNoise::new(12345);
//!
//! // Generate terrain height at a position (returns 0.0 to 1.0)
//! let height = noise.sample_terrain(100.0, 200.0);
//!
//! // Generate cloud density
//! let cloud = noise.sample_clouds(50.0, 75.0, CloudPreset::Fluffy);
//! ```
//!
//! # Advanced Usage
//!
//! ```rust
//! // Custom FBM (Fractal Brownian Motion) settings
//! let height = noise.sample_fbm(x, y, FbmSettings {
//!     octaves: 8,
//!     frequency: 0.01,
//!     persistence: 0.6,
//!     lacunarity: 2.0,
//! });
//!
//! // Ridge noise for mountains
//! let mountain = noise.sample_ridged(x, y, 0.02);
//!
//! // Using builder pattern for custom configuration
//! let custom_noise = PerlinNoise::builder()
//!     .seed(9999)
//!     .octaves(10)
//!     .frequency(0.005)
//!     .build();
//! ```

use noise::{Perlin, NoiseFn};
use bevy::prelude::*;

// ============================================================================
// NOISE CONSTANTS - Centralized parameters for all noise generation
// ============================================================================

/// Default number of octaves for terrain generation
pub const DEFAULT_OCTAVES: u32 = 6;

/// Base frequency for terrain noise - tuned for hexagon scale
pub const TERRAIN_FREQUENCY: f64 = 0.015;

/// Continental shelf frequency (large landmasses)
pub const CONTINENTAL_FREQUENCY: f64 = 0.004;

/// Mountain ridge frequency
pub const MOUNTAIN_FREQUENCY: f64 = 0.025;

/// Default persistence (amplitude decay per octave)
pub const DEFAULT_PERSISTENCE: f64 = 0.5;

/// Default lacunarity (frequency multiplier per octave)
pub const DEFAULT_LACUNARITY: f64 = 2.0;

/// Cloud generation base frequency
pub const CLOUD_FREQUENCY: f64 = 0.008;

/// Maximum value for normalization calculations
const NORMALIZATION_MAX: f64 = 1.0;

/// Minimum value for normalization calculations
const NORMALIZATION_MIN: f64 = -1.0;

// ============================================================================
// CORE PERLIN NOISE STRUCT
// ============================================================================

/// Main Perlin noise generator - your one-stop shop for all noise needs
///
/// This struct wraps the underlying noise library and provides convenient
/// sampling methods for different use cases. Thread-safe and efficient.
#[derive(Clone)]
pub struct PerlinNoise {
    /// The underlying Perlin noise generator
    perlin: Perlin,
    /// Seed used for reproducibility
    seed: u32,
    /// Default octaves for FBM
    default_octaves: u32,
    /// Default frequency
    default_frequency: f64,
    /// Default persistence
    default_persistence: f64,
    /// Default lacunarity
    default_lacunarity: f64,
}

impl PerlinNoise {
    /// Create a new Perlin noise generator with the given seed
    ///
    /// # Example
    /// ```
    /// let noise = PerlinNoise::new(12345);
    /// let value = noise.sample(10.0, 20.0);
    /// ```
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new(seed),
            seed,
            default_octaves: DEFAULT_OCTAVES,
            default_frequency: TERRAIN_FREQUENCY,
            default_persistence: DEFAULT_PERSISTENCE,
            default_lacunarity: DEFAULT_LACUNARITY,
        }
    }

    /// Create a new generator with explicit seed (alias for clarity)
    pub fn with_seed(seed: u32) -> Self {
        Self::new(seed)
    }

    /// Get a builder for custom configuration
    pub fn builder() -> PerlinBuilder {
        PerlinBuilder::default()
    }

    /// Get the seed used by this generator
    pub fn seed(&self) -> u32 {
        self.seed
    }

    // ========================================================================
    // BASIC SAMPLING METHODS
    // ========================================================================

    /// Sample raw Perlin noise at a position (returns -1.0 to 1.0)
    ///
    /// Use this when you need the raw noise values without normalization.
    pub fn sample_raw(&self, x: f64, y: f64) -> f64 {
        self.perlin.get([x, y])
    }

    /// Sample normalized Perlin noise at a position (returns 0.0 to 1.0)
    ///
    /// This is the most common sampling method for general use.
    pub fn sample(&self, x: f64, y: f64) -> f64 {
        Self::normalize_to_01(self.sample_raw(x, y))
    }

    /// Sample with custom frequency (returns 0.0 to 1.0)
    pub fn sample_scaled(&self, x: f64, y: f64, frequency: f64) -> f64 {
        Self::normalize_to_01(self.perlin.get([x * frequency, y * frequency]))
    }

    // ========================================================================
    // FRACTAL BROWNIAN MOTION (FBM)
    // ========================================================================

    /// Sample using Fractal Brownian Motion with default settings
    ///
    /// FBM combines multiple octaves of noise for more natural-looking results.
    /// Returns values normalized to [0.0, 1.0].
    pub fn sample_fbm_default(&self, x: f64, y: f64) -> f64 {
        self.sample_fbm(
            x,
            y,
            FbmSettings {
                octaves: self.default_octaves,
                frequency: self.default_frequency,
                persistence: self.default_persistence,
                lacunarity: self.default_lacunarity,
            },
        )
    }

    /// Sample using Fractal Brownian Motion with custom settings
    ///
    /// # Example
    /// ```
    /// let value = noise.sample_fbm(x, y, FbmSettings {
    ///     octaves: 8,
    ///     frequency: 0.01,
    ///     persistence: 0.6,
    ///     lacunarity: 2.0,
    /// });
    /// ```
    pub fn sample_fbm(&self, x: f64, y: f64, settings: FbmSettings) -> f64 {
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = settings.frequency;
        let mut max_amplitude = 0.0;

        for _ in 0..settings.octaves {
            value += self.perlin.get([x * frequency, y * frequency]) * amplitude;
            max_amplitude += amplitude;
            amplitude *= settings.persistence;
            frequency *= settings.lacunarity;
        }

        // Normalize to [-1, 1] then to [0, 1]
        let normalized = value / max_amplitude;
        Self::normalize_to_01(normalized)
    }

    // ========================================================================
    // SPECIALIZED TERRAIN SAMPLING
    // ========================================================================

    /// Sample terrain with continental and detail layers
    ///
    /// This is the main method for terrain generation. It combines:
    /// - Continental shelf (large landmasses)
    /// - Detail terrain (hills and valleys)
    /// - Mountain ridges (sharp peaks)
    ///
    /// Returns elevation from 0.0 (ocean floor) to 1.0 (mountain peak).
    pub fn sample_terrain(&self, x: f64, y: f64) -> f64 {
        // Continental shelf layer (40% influence)
        let continental = self.sample_scaled(x, y, CONTINENTAL_FREQUENCY) * 0.4;

        // Detail terrain layer (40% influence)
        let detail = self.sample_fbm_default(x, y) * 0.4;

        // Mountain ridge layer (20% influence)
        let ridge = self.sample_ridged(x, y, MOUNTAIN_FREQUENCY) * 0.2;

        // Combine and ensure [0, 1] range
        (continental + detail + ridge).clamp(0.0, 1.0)
    }

    /// Sample terrain with preset configuration
    pub fn sample_terrain_preset(&self, x: f64, y: f64, preset: TerrainPreset) -> f64 {
        match preset {
            TerrainPreset::Continents => {
                // Large continental masses with less detail
                let continental = self.sample_scaled(x, y, CONTINENTAL_FREQUENCY * 0.5) * 0.6;
                let detail = self.sample_fbm(
                    x,
                    y,
                    FbmSettings {
                        octaves: 4,
                        frequency: TERRAIN_FREQUENCY * 0.5,
                        persistence: 0.4,
                        lacunarity: 2.0,
                    },
                ) * 0.4;
                (continental + detail).clamp(0.0, 1.0)
            }
            TerrainPreset::Islands => {
                // Archipelago with many small islands
                let base = self.sample_fbm(
                    x,
                    y,
                    FbmSettings {
                        octaves: 6,
                        frequency: TERRAIN_FREQUENCY * 2.0,
                        persistence: 0.6,
                        lacunarity: 2.5,
                    },
                );
                // Apply threshold for island creation
                if base > 0.45 {
                    (base - 0.45) * 2.0 + 0.5
                } else {
                    base * 0.5
                }
            }
            TerrainPreset::Mountains => {
                // Mountainous terrain with many peaks
                let base = self.sample_fbm_default(x, y) * 0.3;
                let ridges = self.sample_ridged(x, y, MOUNTAIN_FREQUENCY * 1.5) * 0.7;
                (base + ridges).clamp(0.0, 1.0)
            }
        }
    }

    // ========================================================================
    // RIDGE NOISE (For Mountains)
    // ========================================================================

    /// Generate ridge noise for mountain peaks
    ///
    /// Ridge noise creates sharp mountain ridges by taking the absolute value
    /// of noise and inverting it. Perfect for realistic mountain ranges.
    pub fn sample_ridged(&self, x: f64, y: f64, frequency: f64) -> f64 {
        let noise = self.perlin.get([x * frequency, y * frequency]);
        // Ridge function: 1 - |noise|
        let ridge = 1.0 - noise.abs();
        // Square it for sharper peaks
        let sharp_ridge = ridge * ridge;
        // Normalize to [0, 1]
        sharp_ridge.clamp(0.0, 1.0)
    }

    // ========================================================================
    // CLOUD GENERATION
    // ========================================================================

    /// Sample cloud density with preset patterns
    pub fn sample_clouds(&self, x: f64, y: f64, preset: CloudPreset) -> f64 {
        match preset {
            CloudPreset::Wispy => {
                // Thin, stretched clouds
                let base = self.sample_fbm(
                    x,
                    y,
                    FbmSettings {
                        octaves: 3,
                        frequency: CLOUD_FREQUENCY * 2.0,
                        persistence: 0.3,
                        lacunarity: 3.0,
                    },
                );
                // Threshold for wispy effect
                if base > 0.4 {
                    (base - 0.4) * 1.5
                } else {
                    0.0
                }
            }
            CloudPreset::Fluffy => {
                // Standard cumulus clouds
                self.sample_billow(x, y, CLOUD_FREQUENCY)
            }
            CloudPreset::Storm => {
                // Dense storm clouds
                let base = self.sample_fbm(
                    x,
                    y,
                    FbmSettings {
                        octaves: 5,
                        frequency: CLOUD_FREQUENCY * 0.7,
                        persistence: 0.7,
                        lacunarity: 2.0,
                    },
                );
                // Make denser
                (base * 1.3).clamp(0.0, 1.0)
            }
        }
    }

    /// Generate billow noise for fluffy clouds
    ///
    /// Billow noise is like FBM but uses absolute values for a puffy appearance.
    pub fn sample_billow(&self, x: f64, y: f64, frequency: f64) -> f64 {
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut freq = frequency;
        let mut max_amplitude = 0.0;

        for _ in 0..4 {
            // Use absolute value for billow effect
            value += self.perlin.get([x * freq, y * freq]).abs() * amplitude;
            max_amplitude += amplitude;
            amplitude *= 0.5;
            freq *= 2.0;
        }

        (value / max_amplitude).clamp(0.0, 1.0)
    }

    // ========================================================================
    // DOMAIN WARPING (For Coastlines)
    // ========================================================================

    /// Apply domain warping for more natural coastlines
    ///
    /// Domain warping distorts the input coordinates before sampling,
    /// creating more organic-looking features.
    pub fn sample_warped(&self, x: f64, y: f64, warp_strength: f64) -> f64 {
        // Get warping offsets
        let warp_x = self.perlin.get([x * 0.01, y * 0.01]) * warp_strength;
        let warp_y = self.perlin.get([x * 0.01 + 100.0, y * 0.01 + 100.0]) * warp_strength;

        // Sample with warped coordinates
        self.sample_terrain(x + warp_x, y + warp_y)
    }

    // ========================================================================
    // UTILITY METHODS
    // ========================================================================

    /// Normalize a value from [-1, 1] to [0, 1]
    pub fn normalize_to_01(value: f64) -> f64 {
        (value + 1.0) * 0.5
    }

    /// Normalize a value from [0, 1] to [-1, 1]
    pub fn normalize_from_01(value: f64) -> f64 {
        value * 2.0 - 1.0
    }

    /// Clamp and normalize any range to [0, 1]
    pub fn normalize_range(value: f64, min: f64, max: f64) -> f64 {
        ((value - min) / (max - min)).clamp(0.0, 1.0)
    }
}

// ============================================================================
// BUILDER PATTERN
// ============================================================================

/// Builder for creating customized PerlinNoise instances
#[derive(Default)]
pub struct PerlinBuilder {
    seed: Option<u32>,
    octaves: Option<u32>,
    frequency: Option<f64>,
    persistence: Option<f64>,
    lacunarity: Option<f64>,
}

impl PerlinBuilder {
    /// Set the seed
    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set default octaves
    pub fn octaves(mut self, octaves: u32) -> Self {
        self.octaves = Some(octaves);
        self
    }

    /// Set default frequency
    pub fn frequency(mut self, frequency: f64) -> Self {
        self.frequency = Some(frequency);
        self
    }

    /// Set default persistence
    pub fn persistence(mut self, persistence: f64) -> Self {
        self.persistence = Some(persistence);
        self
    }

    /// Set default lacunarity
    pub fn lacunarity(mut self, lacunarity: f64) -> Self {
        self.lacunarity = Some(lacunarity);
        self
    }

    /// Build the PerlinNoise instance
    pub fn build(self) -> PerlinNoise {
        let seed = self.seed.unwrap_or(0);
        let mut noise = PerlinNoise::new(seed);

        if let Some(octaves) = self.octaves {
            noise.default_octaves = octaves;
        }
        if let Some(frequency) = self.frequency {
            noise.default_frequency = frequency;
        }
        if let Some(persistence) = self.persistence {
            noise.default_persistence = persistence;
        }
        if let Some(lacunarity) = self.lacunarity {
            noise.default_lacunarity = lacunarity;
        }

        noise
    }
}

// ============================================================================
// CONFIGURATION STRUCTS
// ============================================================================

/// Settings for Fractal Brownian Motion
#[derive(Debug, Clone, Copy)]
pub struct FbmSettings {
    /// Number of noise layers to combine
    pub octaves: u32,
    /// Base frequency for the first octave
    pub frequency: f64,
    /// Amplitude multiplier per octave (0.0 to 1.0)
    pub persistence: f64,
    /// Frequency multiplier per octave (typically 2.0)
    pub lacunarity: f64,
}

impl Default for FbmSettings {
    fn default() -> Self {
        Self {
            octaves: DEFAULT_OCTAVES,
            frequency: TERRAIN_FREQUENCY,
            persistence: DEFAULT_PERSISTENCE,
            lacunarity: DEFAULT_LACUNARITY,
        }
    }
}

// ============================================================================
// PRESET ENUMS
// ============================================================================

/// Preset terrain generation patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainPreset {
    /// Large continental landmasses
    Continents,
    /// Archipelago with many islands
    Islands,
    /// Mountainous terrain with many peaks
    Mountains,
}

/// Preset cloud generation patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudPreset {
    /// Thin, stretched clouds
    Wispy,
    /// Fluffy cumulus clouds
    Fluffy,
    /// Dense storm clouds
    Storm,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise_creation() {
        let noise = PerlinNoise::new(12345);
        assert_eq!(noise.seed(), 12345);
    }

    #[test]
    fn test_normalization() {
        assert_eq!(PerlinNoise::normalize_to_01(-1.0), 0.0);
        assert_eq!(PerlinNoise::normalize_to_01(1.0), 1.0);
        assert_eq!(PerlinNoise::normalize_to_01(0.0), 0.5);
    }

    #[test]
    fn test_sample_range() {
        let noise = PerlinNoise::new(42);
        for x in 0..10 {
            for y in 0..10 {
                let value = noise.sample(x as f64, y as f64);
                assert!(value >= 0.0 && value <= 1.0, "Sample out of range: {}", value);
            }
        }
    }

    #[test]
    fn test_terrain_range() {
        let noise = PerlinNoise::new(99);
        for x in 0..10 {
            for y in 0..10 {
                let value = noise.sample_terrain(x as f64 * 10.0, y as f64 * 10.0);
                assert!(value >= 0.0 && value <= 1.0, "Terrain out of range: {}", value);
            }
        }
    }

    #[test]
    fn test_builder() {
        let noise = PerlinNoise::builder()
            .seed(777)
            .octaves(8)
            .frequency(0.01)
            .build();

        assert_eq!(noise.seed(), 777);
        assert_eq!(noise.default_octaves, 8);
        assert_eq!(noise.default_frequency, 0.01);
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let noise = PerlinNoise::new(123);
        let noise_clone = noise.clone();

        let handle = thread::spawn(move || {
            noise_clone.sample(10.0, 20.0)
        });

        let result1 = noise.sample(10.0, 20.0);
        let result2 = handle.join().unwrap();

        assert_eq!(result1, result2, "Thread safety issue: different results");
    }
}