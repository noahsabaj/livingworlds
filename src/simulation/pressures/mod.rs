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





pub use systems::{
    apply_pressure_effects, resolve_pressure_actions,
    run_pressure_systems_on_timer, update_nation_pressures, PressureSystemTimer,
};
