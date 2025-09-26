//! Infrastructure module gateway for world infrastructure systems
//!
//! This module manages roads, trade routes, and development visualization
//! for the world overlay system.

// Private modules
mod analysis;
mod storage;

// Public exports - controlled API surface
pub use analysis::analyze_infrastructure;
pub use storage::{InfrastructureStorage, ProvinceInfrastructure};