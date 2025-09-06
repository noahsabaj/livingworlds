//! Mathematics library for Living Worlds
//!
//! Provides fixed-point arithmetic, vectors, and deterministic RNG
//! for consistent cross-platform simulation.

pub mod fixed;
pub mod vector;
pub mod random;

pub use fixed::Fixed32;
pub use vector::{Vec2fx, Vec3fx};
pub use random::DeterministicRNG;
