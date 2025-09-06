//! Infrastructure domain module - human-built structures and networks
//!
//! Infrastructure components are only added to entities when actually built,
//! following proper ECS composition patterns.

pub mod transport;
pub mod utilities;
pub mod defense;
pub mod urban;

// Re-export key types for convenience
pub use transport::*;
pub use utilities::*;
pub use defense::*;
pub use urban::*;

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};

/// Marker component indicating infrastructure is present
/// Only added when infrastructure is actually built
#[derive(Component, Debug, Default, Clone, Serialize, Deserialize)]
pub struct InfrastructurePresent;

/// Infrastructure quality and maintenance tracking
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureCondition {
    pub quality: Fixed32,           // 0.0 (ruins) to 1.0 (perfect)
    pub maintenance_cost: Fixed32,  // Monthly cost to maintain
    pub decay_rate: Fixed32,        // How fast it deteriorates
    pub last_maintenance: u64,      // Game tick of last maintenance
}