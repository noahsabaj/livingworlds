//! Color utility functions and type-safe wrappers
//!
//! This module provides helper functions and type-safe wrappers for
//! color operations, including safe color construction and position hashing.

use bevy::prelude::Color;
use crate::math::hash_random;

/// Type-safe wrapper for stone abundance with validation
#[derive(Debug, Clone, Copy)]
pub struct StoneAbundance(u8);

impl StoneAbundance {
    pub fn new(value: u8) -> Self {
        Self(value.min(100))
    }

    pub fn normalized(&self) -> f32 {
        if self.0 == 0 {
            0.0  // Ocean/no stone
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
        Color::srgba(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0), a.clamp(0.0, 1.0))
    }
}

/// Position-based hash for deterministic variation
#[inline]
pub fn position_hash(x: f32, y: f32, seed: u32) -> f32 {
    // Use centralized hash_random and convert from [0,1] to [-1,1]
    hash_random(x, y, seed) * 2.0 - 1.0
}