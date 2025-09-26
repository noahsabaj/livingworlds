//! Test utilities for Living Worlds (gateway module)
//!
//! This module provides common test helpers, fixtures, and utilities
//! for testing game systems without requiring full graphics/audio.

#![cfg(test)]

// Private modules - gateway architecture
mod app;
mod assertions;
mod fixtures;
mod nations;
mod time;
mod world;

// Re-export all test utilities through the gateway
pub use app::{create_test_app, create_law_test_app};
pub use assertions::{assert_law_active, assert_law_not_active};
pub use fixtures::{initialize_test_laws, TestLawEffects};
pub use nations::spawn_test_nation;
pub use time::{advance_frames, advance_days};
pub use world::generate_test_world;