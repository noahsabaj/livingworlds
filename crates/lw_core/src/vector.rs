//! Fixed-point vector types for 2D and 3D math
//!
//! These vectors use Fixed32 for deterministic calculations across platforms.

use crate::Fixed32;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// 2D vector using fixed-point arithmetic
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vec2fx {
    pub x: Fixed32,
    pub y: Fixed32,
}

impl Vec2fx {
    /// Zero vector
    pub const ZERO: Self = Self {
        x: Fixed32::ZERO,
        y: Fixed32::ZERO,
    };
    
    /// One vector (1, 1)
    pub const ONE: Self = Self {
        x: Fixed32::ONE,
        y: Fixed32::ONE,
    };
    
    /// Unit X vector (1, 0)
    pub const UNIT_X: Self = Self {
        x: Fixed32::ONE,
        y: Fixed32::ZERO,
    };
    
    /// Unit Y vector (0, 1)
    pub const UNIT_Y: Self = Self {
        x: Fixed32::ZERO,
        y: Fixed32::ONE,
    };
    
    /// Create a new vector
    #[inline]
    pub const fn new(x: Fixed32, y: Fixed32) -> Self {
        Self { x, y }
    }
    
    /// Create from integers
    #[inline]
    pub fn from_ints(x: i32, y: i32) -> Self {
        Self {
            x: Fixed32::from_num(x),
            y: Fixed32::from_num(y),
        }
    }
    
    /// Create from floats (for initialization only)
    #[inline]
    pub fn from_floats(x: f32, y: f32) -> Self {
        Self {
            x: Fixed32::from_float(x),
            y: Fixed32::from_float(y),
        }
    }
    
    /// Dot product
    #[inline]
    pub fn dot(self, other: Self) -> Fixed32 {
        self.x * other.x + self.y * other.y
    }
    
    /// Cross product (returns scalar for 2D)
    #[inline]
    pub fn cross(self, other: Self) -> Fixed32 {
        self.x * other.y - self.y * other.x
    }
    
    /// Squared length (avoids sqrt)
    #[inline]
    pub fn length_squared(self) -> Fixed32 {
        self.dot(self)
    }
    
    /// Length (magnitude)
    #[inline]
    pub fn length(self) -> Fixed32 {
        self.length_squared().sqrt()
    }
    
    /// Distance to another vector
    #[inline]
    pub fn distance(self, other: Self) -> Fixed32 {
        (other - self).length()
    }
    
    /// Squared distance (avoids sqrt)
    #[inline]
    pub fn distance_squared(self, other: Self) -> Fixed32 {
        (other - self).length_squared()
    }
    
    /// Normalize to unit length
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > Fixed32::EPSILON {
            self / len
        } else {
            Self::ZERO
        }
    }
    
    /// Check if approximately zero
    #[inline]
    pub fn is_zero(self) -> bool {
        self.x.abs() < Fixed32::EPSILON && self.y.abs() < Fixed32::EPSILON
    }
    
    /// Linear interpolation
    #[inline]
    pub fn lerp(self, other: Self, t: Fixed32) -> Self {
        self + (other - self) * t
    }
    
    /// Rotate by angle (in radians)
    pub fn rotate(self, angle: Fixed32) -> Self {
        // TODO: Implement fixed-point sin/cos
        // For now, convert to float, rotate, and convert back
        let cos_a = Fixed32::from_float(angle.to_f32().cos());
        let sin_a = Fixed32::from_float(angle.to_f32().sin());
        
        Self {
            x: self.x * cos_a - self.y * sin_a,
            y: self.x * sin_a + self.y * cos_a,
        }
    }
    
    /// Perpendicular vector (rotated 90 degrees CCW)
    #[inline]
    pub fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
    
    /// Absolute value of components
    #[inline]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
    
    /// Component-wise min
    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self {
            x: if self.x < other.x { self.x } else { other.x },
            y: if self.y < other.y { self.y } else { other.y },
        }
    }
    
    /// Component-wise max
    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self {
            x: if self.x > other.x { self.x } else { other.x },
            y: if self.y > other.y { self.y } else { other.y },
        }
    }
    
    /// Clamp components between min and max
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }
}

impl fmt::Display for Vec2fx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Arithmetic operators
impl Add for Vec2fx {
    type Output = Self;
    
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2fx {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2fx {
    type Output = Self;
    
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2fx {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<Fixed32> for Vec2fx {
    type Output = Self;
    
    #[inline]
    fn mul(self, rhs: Fixed32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<Fixed32> for Vec2fx {
    #[inline]
    fn mul_assign(&mut self, rhs: Fixed32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<Fixed32> for Vec2fx {
    type Output = Self;
    
    #[inline]
    fn div(self, rhs: Fixed32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<Fixed32> for Vec2fx {
    #[inline]
    fn div_assign(&mut self, rhs: Fixed32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vec2fx {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// 3D vector using fixed-point arithmetic
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vec3fx {
    pub x: Fixed32,
    pub y: Fixed32,
    pub z: Fixed32,
}

impl Vec3fx {
    /// Zero vector
    pub const ZERO: Self = Self {
        x: Fixed32::ZERO,
        y: Fixed32::ZERO,
        z: Fixed32::ZERO,
    };
    
    /// One vector (1, 1, 1)
    pub const ONE: Self = Self {
        x: Fixed32::ONE,
        y: Fixed32::ONE,
        z: Fixed32::ONE,
    };
    
    /// Unit X vector (1, 0, 0)
    pub const UNIT_X: Self = Self {
        x: Fixed32::ONE,
        y: Fixed32::ZERO,
        z: Fixed32::ZERO,
    };
    
    /// Unit Y vector (0, 1, 0)
    pub const UNIT_Y: Self = Self {
        x: Fixed32::ZERO,
        y: Fixed32::ONE,
        z: Fixed32::ZERO,
    };
    
    /// Unit Z vector (0, 0, 1)
    pub const UNIT_Z: Self = Self {
        x: Fixed32::ZERO,
        y: Fixed32::ZERO,
        z: Fixed32::ONE,
    };
    
    /// Create a new vector
    #[inline]
    pub const fn new(x: Fixed32, y: Fixed32, z: Fixed32) -> Self {
        Self { x, y, z }
    }
    
    /// Create from integers
    #[inline]
    pub fn from_ints(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: Fixed32::from_num(x),
            y: Fixed32::from_num(y),
            z: Fixed32::from_num(z),
        }
    }
    
    /// Dot product
    #[inline]
    pub fn dot(self, other: Self) -> Fixed32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    /// Cross product
    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    
    /// Squared length
    #[inline]
    pub fn length_squared(self) -> Fixed32 {
        self.dot(self)
    }
    
    /// Length
    #[inline]
    pub fn length(self) -> Fixed32 {
        self.length_squared().sqrt()
    }
    
    /// Normalize to unit length
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > Fixed32::EPSILON {
            self / len
        } else {
            Self::ZERO
        }
    }
}

// Implement arithmetic operators for Vec3fx similar to Vec2fx
impl Add for Vec3fx {
    type Output = Self;
    
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3fx {
    type Output = Self;
    
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Fixed32> for Vec3fx {
    type Output = Self;
    
    #[inline]
    fn mul(self, rhs: Fixed32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<Fixed32> for Vec3fx {
    type Output = Self;
    
    #[inline]
    fn div(self, rhs: Fixed32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vec3fx {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec2_operations() {
        let v1 = Vec2fx::from_ints(3, 4);
        let v2 = Vec2fx::from_ints(1, 2);
        
        assert_eq!((v1 + v2).x.to_f32(), 4.0);
        assert_eq!((v1 - v2).x.to_f32(), 2.0);
        assert_eq!(v1.dot(v2).to_f32(), 11.0);
        assert_eq!(v1.length().to_f32(), 5.0);
    }
    
    #[test]
    fn test_vec2_normalize() {
        let v = Vec2fx::from_ints(3, 4);
        let normalized = v.normalize();
        let len = normalized.length();
        
        assert!((len.to_f32() - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_vec3_cross_product() {
        let v1 = Vec3fx::UNIT_X;
        let v2 = Vec3fx::UNIT_Y;
        let cross = v1.cross(v2);
        
        assert_eq!(cross, Vec3fx::UNIT_Z);
    }
}