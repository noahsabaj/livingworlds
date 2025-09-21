//! Pressure-driven emergence system gateway
//!
//! This module implements the pressure-driven emergence system where simple
//! pressure rules create complex civilization behaviors.

// PRIVATE MODULES
mod economic;
mod legitimacy;
mod military;
mod population;
mod systems;
mod types;

// PUBLIC EXPORTS
pub use types::{PressureLevel, PressureThreshold, PressureType, PressureVector};

pub use population::{calculate_population_pressure, PopulationPressure};

pub use economic::{calculate_economic_pressure, EconomicPressure};

pub use military::{calculate_military_pressure, MilitaryPressure};

pub use legitimacy::{
    calculate_legitimacy_pressure, LegitimacyPressure, RecentEvents, RulerPersonality,
};

pub use systems::{
    apply_pressure_effects, determine_ruler_personality, resolve_pressure_actions,
    update_nation_pressures,
};
