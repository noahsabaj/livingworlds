//! Law effect definitions
//!
//! Contains types related to the mechanical effects that laws
//! have on nations when enacted.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::simulation::PressureType;

/// Mechanical effects that laws have on nations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawEffects {
    // Economic modifiers
    pub tax_efficiency_modifier: f32,
    pub trade_income_modifier: f32,
    pub industrial_output_modifier: f32,
    pub agricultural_output_modifier: f32,
    pub wealth_inequality_change: f32,

    // Military modifiers
    pub mobilization_speed_modifier: f32,
    pub army_morale_modifier: f32,
    pub naval_tradition_modifier: f32,
    pub defensive_bonus_modifier: f32,
    pub expansion_desire_modifier: f32,

    // Social modifiers
    pub stability_change: f32,
    pub legitimacy_change: f32,
    pub happiness_modifier: f32,
    pub population_growth_modifier: f32,
    pub technology_rate_modifier: f32,
    pub cultural_conversion_modifier: f32,

    // Political modifiers
    pub corruption_change: f32,
    pub centralization_change: f32,
    pub reform_resistance_change: f32,
    pub diplomatic_reputation_change: f32,
    pub administrative_efficiency_modifier: f32,
    pub maintenance_cost_modifier: f32,
    pub revolt_risk_change: f32,

    // Pressure modifiers - how this law affects various pressures
    pub pressure_modifiers: HashMap<PressureType, f32>,

    // Special flags
    pub allows_slavery: Option<bool>,
    pub allows_free_speech: Option<bool>,
    pub allows_private_property: Option<bool>,
    pub allows_religious_freedom: Option<bool>,
}

impl Default for LawEffects {
    fn default() -> Self {
        Self {
            tax_efficiency_modifier: 0.0,
            trade_income_modifier: 0.0,
            industrial_output_modifier: 0.0,
            agricultural_output_modifier: 0.0,
            wealth_inequality_change: 0.0,
            mobilization_speed_modifier: 0.0,
            army_morale_modifier: 0.0,
            naval_tradition_modifier: 0.0,
            defensive_bonus_modifier: 0.0,
            expansion_desire_modifier: 0.0,
            stability_change: 0.0,
            legitimacy_change: 0.0,
            happiness_modifier: 0.0,
            population_growth_modifier: 0.0,
            technology_rate_modifier: 0.0,
            cultural_conversion_modifier: 0.0,
            corruption_change: 0.0,
            centralization_change: 0.0,
            reform_resistance_change: 0.0,
            diplomatic_reputation_change: 0.0,
            administrative_efficiency_modifier: 0.0,
            maintenance_cost_modifier: 0.0,
            revolt_risk_change: 0.0,
            pressure_modifiers: HashMap::new(),
            allows_slavery: None,
            allows_free_speech: None,
            allows_private_property: None,
            allows_religious_freedom: None,
        }
    }
}

impl LawEffects {
    /// Apply diminishing returns to a modifier value
    /// Multiple similar effects have reduced cumulative impact
    fn apply_diminishing_returns(current: f32, addition: f32) -> f32 {
        // First law: 100% effectiveness
        // Second law: 75% effectiveness
        // Third law: 50% effectiveness
        // Fourth+: 25% effectiveness
        let num_existing = (current.abs() / 0.1).floor() as i32;
        let effectiveness = match num_existing {
            0 => 1.0,
            1 => 0.75,
            2 => 0.5,
            _ => 0.25,
        };

        current + (addition * effectiveness)
    }

    /// Add another law's effects with diminishing returns
    pub fn add_with_diminishing_returns(&mut self, other: &LawEffects) {
        // Economic modifiers
        self.tax_efficiency_modifier = Self::apply_diminishing_returns(
            self.tax_efficiency_modifier, other.tax_efficiency_modifier
        );
        self.trade_income_modifier = Self::apply_diminishing_returns(
            self.trade_income_modifier, other.trade_income_modifier
        );
        self.industrial_output_modifier = Self::apply_diminishing_returns(
            self.industrial_output_modifier, other.industrial_output_modifier
        );
        self.agricultural_output_modifier = Self::apply_diminishing_returns(
            self.agricultural_output_modifier, other.agricultural_output_modifier
        );

        // Direct additions (no diminishing returns for these)
        self.wealth_inequality_change += other.wealth_inequality_change;
        self.stability_change += other.stability_change;
        self.legitimacy_change += other.legitimacy_change;
        self.corruption_change += other.corruption_change;
        self.centralization_change += other.centralization_change;

        // Military modifiers with diminishing returns
        self.mobilization_speed_modifier = Self::apply_diminishing_returns(
            self.mobilization_speed_modifier, other.mobilization_speed_modifier
        );
        self.army_morale_modifier = Self::apply_diminishing_returns(
            self.army_morale_modifier, other.army_morale_modifier
        );

        // Social modifiers with diminishing returns
        self.happiness_modifier = Self::apply_diminishing_returns(
            self.happiness_modifier, other.happiness_modifier
        );
        self.population_growth_modifier = Self::apply_diminishing_returns(
            self.population_growth_modifier, other.population_growth_modifier
        );
        self.technology_rate_modifier = Self::apply_diminishing_returns(
            self.technology_rate_modifier, other.technology_rate_modifier
        );

        // Other modifiers
        self.naval_tradition_modifier += other.naval_tradition_modifier;
        self.defensive_bonus_modifier += other.defensive_bonus_modifier;
        self.expansion_desire_modifier += other.expansion_desire_modifier;
        self.cultural_conversion_modifier += other.cultural_conversion_modifier;
        self.reform_resistance_change += other.reform_resistance_change;
        self.diplomatic_reputation_change += other.diplomatic_reputation_change;

        // Administrative modifiers with diminishing returns
        self.administrative_efficiency_modifier = Self::apply_diminishing_returns(
            self.administrative_efficiency_modifier, other.administrative_efficiency_modifier
        );
        self.maintenance_cost_modifier = Self::apply_diminishing_returns(
            self.maintenance_cost_modifier, other.maintenance_cost_modifier
        );
        self.revolt_risk_change += other.revolt_risk_change;

        // Merge pressure modifiers
        for (pressure_type, modifier) in &other.pressure_modifiers {
            *self.pressure_modifiers.entry(*pressure_type).or_insert(0.0) += modifier;
        }

        // Boolean flags - OR operation
        if let Some(val) = other.allows_slavery {
            self.allows_slavery = Some(self.allows_slavery.unwrap_or(false) || val);
        }
        if let Some(val) = other.allows_free_speech {
            self.allows_free_speech = Some(self.allows_free_speech.unwrap_or(false) || val);
        }
        if let Some(val) = other.allows_private_property {
            self.allows_private_property = Some(self.allows_private_property.unwrap_or(false) || val);
        }
        if let Some(val) = other.allows_religious_freedom {
            self.allows_religious_freedom = Some(self.allows_religious_freedom.unwrap_or(false) || val);
        }
    }

    /// Combine with another law's effects, returning a new instance
    pub fn combine_with(&self, other: &LawEffects) -> Self {
        let mut result = self.clone();
        result.add_with_diminishing_returns(other);
        result
    }

    /// Subtract effects (for repeals) without diminishing returns
    pub fn subtract(&mut self, other: &LawEffects) {
        // Simple subtraction for all numeric fields
        self.tax_efficiency_modifier -= other.tax_efficiency_modifier;
        self.trade_income_modifier -= other.trade_income_modifier;
        self.industrial_output_modifier -= other.industrial_output_modifier;
        self.agricultural_output_modifier -= other.agricultural_output_modifier;
        self.wealth_inequality_change -= other.wealth_inequality_change;

        self.mobilization_speed_modifier -= other.mobilization_speed_modifier;
        self.army_morale_modifier -= other.army_morale_modifier;
        self.naval_tradition_modifier -= other.naval_tradition_modifier;
        self.defensive_bonus_modifier -= other.defensive_bonus_modifier;
        self.expansion_desire_modifier -= other.expansion_desire_modifier;

        self.stability_change -= other.stability_change;
        self.legitimacy_change -= other.legitimacy_change;
        self.happiness_modifier -= other.happiness_modifier;
        self.population_growth_modifier -= other.population_growth_modifier;
        self.technology_rate_modifier -= other.technology_rate_modifier;
        self.cultural_conversion_modifier -= other.cultural_conversion_modifier;

        self.corruption_change -= other.corruption_change;
        self.centralization_change -= other.centralization_change;
        self.reform_resistance_change -= other.reform_resistance_change;
        self.diplomatic_reputation_change -= other.diplomatic_reputation_change;
        self.administrative_efficiency_modifier -= other.administrative_efficiency_modifier;
        self.maintenance_cost_modifier -= other.maintenance_cost_modifier;
        self.revolt_risk_change -= other.revolt_risk_change;

        // Subtract pressure modifiers
        for (pressure_type, modifier) in &other.pressure_modifiers {
            if let Some(current) = self.pressure_modifiers.get_mut(pressure_type) {
                *current -= modifier;
                if current.abs() < 0.001 {
                    self.pressure_modifiers.remove(pressure_type);
                }
            }
        }

        // Boolean flags - reset to None if the removed law was setting them
        // (This is simplified - in practice might need more complex logic)
        if other.allows_slavery.is_some() {
            self.allows_slavery = None;
        }
        if other.allows_free_speech.is_some() {
            self.allows_free_speech = None;
        }
        if other.allows_private_property.is_some() {
            self.allows_private_property = None;
        }
        if other.allows_religious_freedom.is_some() {
            self.allows_religious_freedom = None;
        }
    }
}

/// Weights for different groups' influence on law passage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopularityWeights {
    pub popular_weight: f32,
    pub elite_weight: f32,
    pub military_weight: f32,
    pub religious_weight: f32,
    pub merchant_weight: f32,
}