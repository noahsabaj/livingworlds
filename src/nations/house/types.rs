//! Core house and ruler type definitions
//!
//! This module contains the fundamental data structures for noble houses
//! and their rulers, without any generation or behavioral logic.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::traits::HouseTraits;
use crate::nations::NationId;

/// A ruling house (dynasty/family) that controls a nation
///
/// Uses Bevy 0.16 Component Hooks for automatic cleanup tracking
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct House {
    pub nation_id: NationId,

    /// The house name (e.g., "Blackwater", "Ironhold", "Jalindan-Gatha")
    pub name: String,

    /// Full formal name (e.g., "House Blackwater of the Northern Reach")
    pub full_name: String,

    /// Current ruler of the house
    pub ruler: Ruler,

    /// House motto or words (e.g., "Blood Before Gold", "Honor Through Strength")
    pub motto: String,

    /// Inheritable traits that affect behavior
    pub traits: HouseTraits,

    /// How long this house has ruled (in years)
    pub years_in_power: u32,

    /// Legitimacy of rule (0.0 to 1.0)
    pub legitimacy: f32,

    /// Prestige and reputation
    pub prestige: f32,
}

// Manual Component implementation to register lifecycle hooks (Bevy 0.16)
impl Component for House {
    const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
    type Mutability = bevy::ecs::component::Mutable;

    fn on_add() -> Option<bevy::ecs::component::ComponentHook> {
        Some(|mut world, bevy::ecs::component::HookContext { entity, .. }| {
            // Log dynasty creation
            if let Some(house) = world.get::<House>(entity) {
                info!("Dynasty founded: {} (Nation ID {})", house.full_name, house.nation_id.value());
            }
        })
    }

    fn on_remove() -> Option<bevy::ecs::component::ComponentHook> {
        Some(|mut world, bevy::ecs::component::HookContext { entity, .. }| {
            // Log dynasty fall
            if let Some(house) = world.get::<House>(entity) {
                warn!("Dynasty fallen: {} - ruled for {} years",
                    house.full_name, house.years_in_power);
            }
        })
    }
}

/// The current ruler of a house
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct Ruler {
    pub name: String,
    pub title: String, // King, Queen, Emperor, Duke, etc.
    pub age: u32,
    pub years_ruling: u32,
    pub personality: RulerPersonality,
}

/// Individual ruler personality (distinct from house traits)
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct RulerPersonality {
    pub competence: f32,  // How effective they are at ruling
    pub ambition: f32,    // Personal drive for power/glory
    pub temperament: f32, // Calm vs volatile
    pub honor: f32,       // How much they value oaths and reputation
}

impl RulerPersonality {
    pub fn random(rng: &mut impl rand::Rng) -> Self {
        Self {
            competence: rng.gen_range(0.2..1.0),
            ambition: rng.gen_range(0.0..1.0),
            temperament: rng.gen_range(-1.0..1.0),
            honor: rng.gen_range(0.0..1.0),
        }
    }
}
