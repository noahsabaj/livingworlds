//! Province generation gateway module
//!
//! This module orchestrates the complex process of procedural province generation,
//! breaking down the monolithic generation logic into focused, maintainable components.
//!
//! # Architecture
//! Following the gateway pattern, all submodules are private and accessed only through
//! controlled public exports. This ensures clean separation of concerns and prevents
//! tight coupling between generation components.

// PRIVATE modules - gateway architecture enforcement
mod builder;
mod continents;
mod elevation_processor;
mod ocean_systems;
mod terrain_classifier;
mod island_filter;
mod climate_effects;
mod neighbor_calculator;
mod gpu_accelerator;

// PUBLIC exports - controlled API surface
pub use builder::ProvinceBuilder;
pub use builder::{provinces_to_bundles, set_neighbor_entities};
pub use ocean_systems::calculate_ocean_depths;
pub use neighbor_calculator::precompute_neighbor_indices;