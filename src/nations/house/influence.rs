//! House influence and behavioral calculations
//!
//! This module contains methods for calculating how houses affect their
//! nations and interact with the world.

use super::types::House;

impl House {
    /// Calculate how much this house affects nation behavior
    ///
    /// A house's influence depends on:
    /// - Legitimacy: How accepted their rule is
    /// - Ruler competence: How effective the current ruler is
    /// - Prestige: The house's accumulated reputation
    ///
    /// Returns a value from 0.0 (no influence) to ~2.0 (absolute control)
    pub fn calculate_influence_on_nation(&self) -> f32 {
        self.legitimacy * self.ruler.personality.competence * (1.0 + self.prestige * 0.5)
    }

    /// Calculate military effectiveness modifier from house
    ///
    /// Houses with strong martial traditions provide bonuses to their armies
    pub fn military_modifier(&self) -> f32 {
        let base = 1.0;
        let martial_bonus = self.traits.martial * 0.3; // Up to 30% from traits
        let ruler_bonus = if self.ruler.personality.competence > 0.7 {
            0.1 // Competent rulers add 10%
        } else {
            0.0
        };

        base + martial_bonus + ruler_bonus
    }

    /// Calculate economic modifier from house
    ///
    /// Merchant houses and those with good stewardship boost the economy
    pub fn economic_modifier(&self) -> f32 {
        let base = 1.0;
        let steward_bonus = self.traits.stewardship * 0.25; // Up to 25% from traits
        let stability_factor = self.legitimacy * 0.15; // Up to 15% from stability

        base + steward_bonus + stability_factor
    }

    /// Calculate diplomatic reputation
    ///
    /// Affects how other nations view and interact with this house's nation
    pub fn diplomatic_reputation(&self) -> f32 {
        let base = self.prestige;
        let diplomacy_factor = self.traits.diplomacy * 0.5;
        let honor_factor = self.ruler.personality.honor * 0.3;
        let intrigue_penalty = self.traits.intrigue * -0.2; // Schemers are less trusted

        (base + diplomacy_factor + honor_factor + intrigue_penalty).clamp(0.0, 1.0)
    }

    /// Check if this house is likely to start wars
    ///
    /// Based on ruler ambition, house martial tradition, and legitimacy
    pub fn war_tendency(&self) -> f32 {
        let ambition_factor = self.ruler.personality.ambition * 0.4;
        let martial_factor = self.traits.martial * 0.3;
        let legitimacy_factor = (1.0 - self.legitimacy) * 0.2; // Weak rulers may start wars for glory
        let temperament_factor = (self.ruler.personality.temperament + 1.0) * 0.1; // Volatile rulers

        (ambition_factor + martial_factor + legitimacy_factor + temperament_factor).clamp(0.0, 1.0)
    }

    /// Calculate chance of internal stability issues
    ///
    /// Low legitimacy, incompetent rulers, and low prestige cause problems
    pub fn instability_risk(&self) -> f32 {
        let legitimacy_risk = (1.0 - self.legitimacy) * 0.4;
        let competence_risk = (1.0 - self.ruler.personality.competence) * 0.3;
        let prestige_risk = (1.0 - self.prestige) * 0.2;
        let intrigue_factor = self.traits.intrigue * 0.1; // Scheming houses have more internal conflict

        (legitimacy_risk + competence_risk + prestige_risk + intrigue_factor).clamp(0.0, 1.0)
    }

    /// Calculate technological advancement rate
    ///
    /// Learning-focused houses advance faster
    pub fn tech_advancement_rate(&self) -> f32 {
        let base = 1.0;
        let learning_bonus = self.traits.learning * 0.4; // Up to 40% faster
        let piety_factor = self.traits.piety * -0.1; // Religious conservatism slows progress

        (base + learning_bonus + piety_factor).max(0.5) // Never slower than 50%
    }
}

// Future expansions:
/*
impl House {
    /// Calculate succession crisis probability
    pub fn succession_crisis_chance(&self) -> f32 { ... }

    /// Calculate chance of producing a genius/terrible heir
    pub fn heir_quality_modifier(&self) -> f32 { ... }

    /// Calculate relationship with specific other houses
    pub fn relationship_with(&self, other: &House) -> f32 { ... }

    /// Calculate chance of religious conversion
    pub fn religious_flexibility(&self) -> f32 { ... }

    /// Calculate espionage effectiveness
    pub fn espionage_power(&self) -> f32 { ... }
}
*/