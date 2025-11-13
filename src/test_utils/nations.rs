//! Nation testing utilities
//!
//! Functions for spawning and manipulating test nations.

use bevy::prelude::*;
use crate::nations::{Nation, NationId, NationLaws, NationPersonality, GovernmentType};
use crate::name_generator::Culture;

/// Spawn a test nation with configurable parameters
pub fn spawn_test_nation(
    app: &mut App,
    name: &str,
    government: GovernmentType,
) -> Entity {
    app.world_mut().spawn((
        Nation {
            id: NationId::new(0),
            name: name.to_string(),
            adjective: format!("{}n", name), // Simple adjective form
            capital_province: 0,
            color: Color::srgb(1.0, 0.0, 0.0),
            treasury: 1000.0,
            tax_rate: 0.2,
            military_strength: 100.0,
            stability: 0.5,
            culture: Culture::Western, // Default test culture
            personality: NationPersonality::balanced(),
        },
        NationLaws::default(),
    )).id()
}
