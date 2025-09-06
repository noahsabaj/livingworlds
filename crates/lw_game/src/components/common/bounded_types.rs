//! Bounded Types for Fixed32
//! 
//! These types enforce bounds at compile time and runtime,
//! eliminating the repeated pattern of "field: Fixed32 // 0-1"

use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};

/// A percentage value bounded between 0 and 1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Percentage(Fixed32);

impl Percentage {
    pub const ZERO: Self = Self(Fixed32::ZERO);
    pub const ONE: Self = Self(Fixed32::ONE);
    
    /// Create a new Percentage, clamping to [0, 1]
    pub fn new(value: Fixed32) -> Self {
        Self(value.clamp(Fixed32::ZERO, Fixed32::ONE))
    }
    
    /// Create from a float, clamping to [0, 1]
    pub fn from_float(value: f32) -> Self {
        Self::new(Fixed32::from_float(value))
    }
    
    /// Get the inner value
    pub fn value(&self) -> Fixed32 {
        self.0
    }
    
    /// Convert to f32
    pub fn to_f32(&self) -> f32 {
        self.0.to_f32()
    }
    
    /// Invert the percentage (1 - x)
    pub fn invert(&self) -> Self {
        Self(Fixed32::ONE - self.0)
    }
}

impl Default for Percentage {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<f32> for Percentage {
    fn from(value: f32) -> Self {
        Self::from_float(value)
    }
}

impl Add for Percentage {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self::new(self.0 + other.0)
    }
}

impl Sub for Percentage {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self::new(self.0 - other.0)
    }
}

impl Mul<Fixed32> for Percentage {
    type Output = Fixed32;
    
    fn mul(self, other: Fixed32) -> Fixed32 {
        self.0 * other
    }
}

/// A unit interval value (alias for Percentage)
pub type UnitInterval = Percentage;

/// A bounded integer range
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundedInt<const MIN: i32, const MAX: i32> {
    value: i32,
}

impl<const MIN: i32, const MAX: i32> BoundedInt<MIN, MAX> {
    pub fn new(value: i32) -> Self {
        Self {
            value: value.clamp(MIN, MAX)
        }
    }
    
    pub fn value(&self) -> i32 {
        self.value
    }
    
    pub fn set(&mut self, value: i32) {
        self.value = value.clamp(MIN, MAX);
    }
}

/// Common bounded types
pub type TechLevel = BoundedInt<0, 100>;
pub type FortificationLevel = BoundedInt<0, 10>;
pub type AIPersonality = BoundedInt<0, 10>;

/// A positive Fixed32 value
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PositiveFixed(Fixed32);

impl PositiveFixed {
    pub const ZERO: Self = Self(Fixed32::ZERO);
    
    pub fn new(value: Fixed32) -> Option<Self> {
        if value >= Fixed32::ZERO {
            Some(Self(value))
        } else {
            None
        }
    }
    
    pub fn new_saturating(value: Fixed32) -> Self {
        Self(value.max(Fixed32::ZERO))
    }
    
    pub fn value(&self) -> Fixed32 {
        self.0
    }
}

impl Default for PositiveFixed {
    fn default() -> Self {
        Self::ZERO
    }
}