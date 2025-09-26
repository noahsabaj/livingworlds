//! Nation testing utilities
//!
//! Functions for spawning and manipulating test nations.

use bevy::prelude::*;
use crate::nations::{Nation, NationId};
use crate::nations::laws::registry::NationLaws;
use crate::nations::governance::types::GovernmentType;

/// Spawn a test nation with configurable parameters
pub fn spawn_test_nation(
    app: &mut App,
    name: &str,
    government: GovernmentType,
) -> Entity {
    app.world_mut().spawn((
        Nation {
            id: NationId::new(),
            name: name.to_string(),
            capital_province: 0,
            color: Color::srgb(1.0, 0.0, 0.0),
            government_type: government,
            stability: 0.5,
            treasury: 1000.0,
            legitimacy: 0.7,
            development: 0.5,
            corruption: 0.1,
            technology_level: 1.0,
        },
        NationLaws::default(),
    )).id()
}