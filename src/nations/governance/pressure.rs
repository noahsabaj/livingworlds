//! Political pressure system
//!
//! This module handles the calculation and updating of political pressures
//! that influence government stability and transitions.

use bevy::prelude::*;
use super::types::{PoliticalPressure, Governance, GovernmentCategory};

/// Update political pressure based on various factors
pub fn update_political_pressure(
    time: Res<crate::simulation::GameTime>,
    mut nations: Query<(
        &crate::nations::Nation,
        &mut PoliticalPressure,
        &Governance,
    )>,
) {
    for (nation, mut pressure, governance) in &mut nations {
        // Economic pressure from low treasury
        if nation.treasury < 100.0 {
            pressure.economic_crisis = (100.0 - nation.treasury) / 100.0;
        } else {
            pressure.economic_crisis *= 0.95; // Decay if economy good
        }

        // Military pressure from low strength
        if nation.military_strength < 0.3 {
            pressure.military_defeat = (0.3 - nation.military_strength) / 0.3;
        } else {
            pressure.military_defeat *= 0.95;
        }

        // Revolutionary ideas spread over time (especially in certain governments)
        let revolution_spread = match governance.government_type.category() {
            GovernmentCategory::Anarchist => 0.01,
            GovernmentCategory::Socialist => 0.005,
            GovernmentCategory::Democratic => 0.002,
            _ => 0.001,
        };
        pressure.revolutionary_ideas = (pressure.revolutionary_ideas + revolution_spread).min(1.0);

        // Cultural shifts happen slowly
        pressure.cultural_shift += 0.0001;
        pressure.cultural_shift = pressure.cultural_shift.min(1.0);
    }
}