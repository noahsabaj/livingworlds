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
        &mut Governance,
    )>,
) {
    for (nation, mut pressure, mut governance) in &mut nations {
        // Economic pressure from low treasury
        if nation.treasury < 100.0 {
            // Clamp to [0.0, 1.0] range to prevent excessive pressure from negative treasury
            pressure.economic_crisis = ((100.0 - nation.treasury) / 100.0).min(1.0).max(0.0);
        } else {
            pressure.economic_crisis *= 0.95; // Decay if economy good
        }

        // Military pressure from low strength (expecting normalized 0-1 value)
        let normalized_strength = (nation.military_strength / 100.0).min(1.0);
        if normalized_strength < 0.3 {
            pressure.military_defeat = ((0.3 - normalized_strength) / 0.3).min(1.0).max(0.0);
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

        // INSTITUTIONAL RECOVERY - Stable nations slowly rebuild state apparatus
        // This natural recovery prevents permanent failed states unless conditions remain terrible
        if governance.stability > 0.5 && nation.treasury > 200.0 {
            // Good conditions = faster recovery
            governance.institution_strength = (governance.institution_strength + 0.005).min(1.0);
        } else if governance.stability > 0.3 {
            // Moderate conditions = slow recovery
            governance.institution_strength = (governance.institution_strength + 0.001).min(1.0);
        } else if governance.institution_strength < 0.2 {
            // Failed state decay - institutions continue to crumble
            governance.institution_strength = (governance.institution_strength * 0.995).max(0.05);
        }

        // POWER CONSOLIDATION - New governments with resources can strengthen quickly
        if let Some(last_transition) = governance.last_transition {
            let days_since_transition = (time.current_day() as f32 - last_transition as f32).max(0.0);
            if days_since_transition < 180.0 {  // First 6 months critical
                if nation.treasury > 500.0 && nation.military_strength > 50.0 {
                    // Strong position = rapid consolidation
                    governance.stability = (governance.stability + 0.01).min(1.0);
                    governance.institution_strength = (governance.institution_strength + 0.003).min(1.0);
                } else if nation.treasury < 100.0 || nation.military_strength < 20.0 {
                    // Weak position = continued instability
                    governance.stability = (governance.stability * 0.99).max(0.1);
                }
            }
        }
    }
}