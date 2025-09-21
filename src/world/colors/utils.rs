//! Color utility functions and type-safe wrappers
//!
//! This module provides helper functions and type-safe wrappers for
//! color operations, including safe color construction and position hashing.

use bevy::prelude::Color;

/// Type-safe wrapper for stone abundance with validation
#[derive(Debug, Clone, Copy)]
pub struct StoneAbundance(u8);

impl StoneAbundance {
    pub fn new(value: u8) -> Self {
        Self(value.min(100))
    }

    pub fn normalized(&self) -> f32 {
        if self.0 == 0 {
            0.0 // Ocean/no stone
        } else {
            ((self.0 as f32 - 20.0) / 60.0).clamp(0.0, 1.0)
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

/// Safe color construction with automatic clamping
pub struct SafeColor;

impl SafeColor {
    #[inline]
    pub fn srgb(r: f32, g: f32, b: f32) -> Color {
        Color::srgb(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
    }

    #[inline]
    pub fn srgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color::srgba(
            r.clamp(0.0, 1.0),
            g.clamp(0.0, 1.0),
            b.clamp(0.0, 1.0),
            a.clamp(0.0, 1.0),
        )
    }
}

/// Position-based hash for deterministic variation
#[inline]
pub fn position_hash(x: f32, y: f32, seed: u32) -> f32 {
    // Simple deterministic hash function for position-based variation
    let x_int = (x * 1000.0) as u32;
    let y_int = (y * 1000.0) as u32;

    // Mix the coordinates and seed using bit operations
    let mut hash = seed;
    hash ^= x_int.wrapping_mul(0x9e3779b9);
    hash ^= y_int.wrapping_mul(0x85ebca6b);
    hash ^= hash >> 16;
    hash ^= hash << 13;
    hash ^= hash >> 17;

    // Convert to [0,1] then to [-1,1]
    let normalized = (hash as f32) / (u32::MAX as f32);
    normalized * 2.0 - 1.0
}
