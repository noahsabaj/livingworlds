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
pub use types::{PressureLevel, PressureType, PressureVector};

pub use population::calculate_population_pressure;

pub use economic::calculate_economic_pressure;

pub use military::calculate_military_pressure;

pub use legitimacy::{
    calculate_legitimacy_pressure, RecentEvents, RulerPersonality,
};

pub use systems::{
    apply_pressure_effects, determine_ruler_personality, resolve_pressure_actions,
    update_nation_pressures,
};
