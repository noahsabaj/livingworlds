//! Fixed-point arithmetic for deterministic simulation
//!
//! This module provides Fixed32, a fixed-point number type with 16.16 bit representation
//! that ensures deterministic arithmetic across platforms.

use fixed::types::I16F16;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::iter::Sum;

/// Fixed-point number with 16 integer bits and 16 fractional bits
/// 
/// This type ensures deterministic arithmetic operations for game simulation.
/// Range: approximately -32768.0 to 32767.99998
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fixed32(I16F16);

// Custom serialization for Fixed32
impl Serialize for Fixed32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_bits().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Fixed32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bits = i32::deserialize(deserializer)?;
        Ok(Fixed32(I16F16::from_bits(bits)))
    }
}

impl Fixed32 {
    /// Zero constant
    pub const ZERO: Self = Self(I16F16::ZERO);
    
    /// One constant  
    pub const ONE: Self = Self(I16F16::ONE);
    
    /// Half constant (0.5)
    pub const HALF: Self = Self(I16F16::from_bits(0x8000)); // 0.5 in 16.16 fixed point
    
    /// Pi constant
    pub const PI: Self = Self(I16F16::PI);
    
    /// Tau constant (2π)
    pub const TAU: Self = Self(I16F16::TAU);
    
    /// Euler's number
    pub const E: Self = Self(I16F16::E);
    
    /// Minimum value
    pub const MIN: Self = Self(I16F16::MIN);
    
    /// Maximum value
    pub const MAX: Self = Self(I16F16::MAX);
    
    /// Smallest positive value
    pub const EPSILON: Self = Self(I16F16::DELTA);

    /// Create from an integer
    #[inline]
    pub const fn from_num(n: i32) -> Self {
        Self(I16F16::from_bits(n << 16))
    }
    
    /// Create from a compile-time floating point value
    #[inline]
    pub fn from_float(f: f32) -> Self {
        Self(I16F16::from_num(f))
    }
    
    /// Create from raw bits
    #[inline]
    pub const fn from_bits(bits: i32) -> Self {
        Self(I16F16::from_bits(bits))
    }
    
    /// Get raw bits representation
    #[inline]
    pub const fn to_bits(self) -> i32 {
        self.0.to_bits()
    }
    
    /// Get integer part
    #[inline]
    pub fn integer_part(self) -> i32 {
        self.0.to_bits() >> 16
    }
    
    /// Get fractional part as raw bits
    #[inline]
    pub fn fraction_part(self) -> u16 {
        (self.0.to_bits() & 0xFFFF) as u16
    }
    
    /// Convert to f32 (for display/debugging only, not for simulation)
    #[inline]
    pub fn to_f32(self) -> f32 {
        self.0.to_num::<f32>()
    }
    
    /// Absolute value
    #[inline]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
    
    /// Floor (round down)
    #[inline]
    pub fn floor(self) -> Self {
        Self(self.0.floor())
    }
    
    /// Ceiling (round up)
    #[inline]
    pub fn ceil(self) -> Self {
        Self(self.0.ceil())
    }
    
    /// Round to nearest integer
    #[inline]
    pub fn round(self) -> Self {
        Self(self.0.round())
    }
    
    /// Clamp value between min and max
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self(self.0.clamp(min.0, max.0))
    }
    
    /// Linear interpolation
    #[inline]
    pub fn lerp(self, other: Self, t: Self) -> Self {
        self + (other - self) * t
    }
    
    /// Square root (Newton-Raphson method for fixed-point)
    pub fn sqrt(self) -> Self {
        if self.0 <= I16F16::ZERO {
            return Self::ZERO;
        }
        
        // Initial guess
        let mut x = self;
        
        // Newton-Raphson iterations
        for _ in 0..8 {
            let prev = x;
            x = (x + self / x) / Self::from_num(2);
            
            // Check for convergence
            if (x - prev).abs() < Self::EPSILON {
                break;
            }
        }
        
        x
    }
}

impl Default for Fixed32 {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for Fixed32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.to_f32())
    }
}

// Arithmetic operators
impl Add for Fixed32 {
    type Output = Self;
    
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Fixed32 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Fixed32 {
    type Output = Self;
    
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Fixed32 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul for Fixed32 {
    type Output = Self;
    
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for Fixed32 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Div for Fixed32 {
    type Output = Self;
    
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for Fixed32 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl Neg for Fixed32 {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

// Conversions for convenience
impl From<i32> for Fixed32 {
    #[inline]
    fn from(n: i32) -> Self {
        Self::from_num(n)
    }
}

// Implement Sum trait for iterator operations
impl Sum for Fixed32 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Fixed32::ZERO, |acc, x| acc + x)
    }
}

impl<'a> Sum<&'a Fixed32> for Fixed32 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Fixed32::ZERO, |acc, x| acc + *x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_constants() {
        assert_eq!(Fixed32::ZERO.to_f32(), 0.0);
        assert_eq!(Fixed32::ONE.to_f32(), 1.0);
        assert_relative_eq!(Fixed32::HALF.to_f32(), 0.5, epsilon = 0.0001);
        assert_relative_eq!(Fixed32::PI.to_f32(), std::f32::consts::PI, epsilon = 0.0001);
    }
    
    #[test]
    fn test_arithmetic() {
        let a = Fixed32::from_num(10);
        let b = Fixed32::from_num(3);
        
        assert_eq!((a + b).to_f32(), 13.0);
        assert_eq!((a - b).to_f32(), 7.0);
        assert_eq!((a * b).to_f32(), 30.0);
        assert_relative_eq!((a / b).to_f32(), 10.0 / 3.0, epsilon = 0.0001);
    }
    
    #[test]
    fn test_fixed_precision() {
        let a = Fixed32::from_float(0.1);
        let mut sum = Fixed32::ZERO;
        
        // Add 0.1 ten times
        for _ in 0..10 {
            sum += a;
        }
        
        // Should be very close to 1.0 (fixed-point maintains precision)
        assert_relative_eq!(sum.to_f32(), 1.0, epsilon = 0.001);
    }
    
    #[test]
    fn test_sqrt() {
        let values = [4, 9, 16, 25, 100];
        let expected = [2.0, 3.0, 4.0, 5.0, 10.0];
        
        for (val, exp) in values.iter().zip(expected.iter()) {
            let fixed = Fixed32::from_num(*val);
            let result = fixed.sqrt();
            assert_relative_eq!(result.to_f32(), exp, epsilon = 0.01);
        }
    }
    
    #[test]
    fn test_lerp() {
        let a = Fixed32::from_num(0);
        let b = Fixed32::from_num(10);
        let t = Fixed32::from_float(0.3);
        
        let result = a.lerp(b, t);
        assert_relative_eq!(result.to_f32(), 3.0, epsilon = 0.01);
    }
    
    #[test]
    fn test_serialization() {
        let value = Fixed32::from_float(3.14159);
        let serialized = bincode::serialize(&value).unwrap();
        let deserialized: Fixed32 = bincode::deserialize(&serialized).unwrap();
        
        assert_eq!(value, deserialized);
    }
}