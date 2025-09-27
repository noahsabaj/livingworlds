//! Law popularity evaluation
//!
//! Evaluates how popular laws are with different population groups
//! and calculates popularity weights based on government type.

use crate::nations::{Nation, Governance, GovernmentType, GovernmentCategory};
use crate::nations::laws::types::{Law, LawCategory, LawPopularity, PopularityWeights};

/// Evaluate how popular a law would be with the population
pub fn evaluate_law_popularity(
    law: &Law,
    nation: &Nation,
    governance: &Governance,
    current_year: i32,
) -> LawPopularity {
    let base_pop = law.base_popularity;

    // Government affinity affects elite support
    let gov_category = governance.government_type.category();
    let gov_affinity = law.government_affinity
        .get(&gov_category)
        .copied()
        .unwrap_or(0.0);

    // Calculate support from different groups
    let mut popularity = LawPopularity {
        popular_support: base_pop,
        elite_support: (base_pop + gov_affinity) / 2.0,
        military_support: base_pop,
        religious_support: base_pop,
        merchant_support: base_pop,
    };

    // Modify based on law category and effects
    match law.category {
        LawCategory::Economic => {
            popularity.merchant_support += law.effects.trade_income_modifier;
            popularity.popular_support += law.effects.happiness_modifier;
        }
        LawCategory::Military => {
            popularity.military_support += law.effects.army_morale_modifier;
            popularity.military_support += law.effects.expansion_desire_modifier * 0.5;
        }
        LawCategory::Religious => {
            popularity.religious_support += 0.3;
            if law.effects.allows_religious_freedom == Some(true) {
                popularity.popular_support += 0.1;
            }
        }
        LawCategory::Social => {
            popularity.popular_support += law.effects.happiness_modifier * 2.0;
            popularity.popular_support -= law.effects.wealth_inequality_change;
        }
        _ => {}
    }

    // Nation personality affects support
    if law.effects.expansion_desire_modifier > 0.0 {
        popularity.military_support += nation.personality.aggression * 0.2;
    }
    if law.effects.trade_income_modifier > 0.0 {
        popularity.merchant_support += nation.personality.mercantilism * 0.2;
    }

    // Stability affects willingness to change
    let stability_modifier = if nation.stability > 0.7 {
        -0.1 // High stability = resistance to change
    } else if nation.stability < 0.3 {
        0.1 // Low stability = desperate for solutions
    } else {
        0.0
    };

    popularity.popular_support += stability_modifier;
    popularity.elite_support += stability_modifier;

    // Clamp all values to [0, 1]
    popularity.popular_support = popularity.popular_support.clamp(0.0, 1.0);
    popularity.elite_support = popularity.elite_support.clamp(0.0, 1.0);
    popularity.military_support = popularity.military_support.clamp(0.0, 1.0);
    popularity.religious_support = popularity.religious_support.clamp(0.0, 1.0);
    popularity.merchant_support = popularity.merchant_support.clamp(0.0, 1.0);

    popularity
}

/// Calculate popularity weights based on government type
pub fn calculate_popularity_weights(government_type: GovernmentType) -> PopularityWeights {
    match government_type.category() {
        GovernmentCategory::Democratic => PopularityWeights {
            popular_weight: 0.5,
            elite_weight: 0.2,
            military_weight: 0.1,
            religious_weight: 0.1,
            merchant_weight: 0.1,
        },
        GovernmentCategory::Autocratic => PopularityWeights {
            popular_weight: 0.1,
            elite_weight: 0.3,
            military_weight: 0.4,
            religious_weight: 0.1,
            merchant_weight: 0.1,
        },
        GovernmentCategory::Theocratic => PopularityWeights {
            popular_weight: 0.2,
            elite_weight: 0.1,
            military_weight: 0.1,
            religious_weight: 0.5,
            merchant_weight: 0.1,
        },
        GovernmentCategory::Socialist => PopularityWeights {
            popular_weight: 0.6,
            elite_weight: 0.1,
            military_weight: 0.1,
            religious_weight: 0.1,
            merchant_weight: 0.1,
        },
        GovernmentCategory::Corporate => PopularityWeights {
            popular_weight: 0.1,
            elite_weight: 0.2,
            military_weight: 0.1,
            religious_weight: 0.0,
            merchant_weight: 0.6,
        },
        GovernmentCategory::Anarchist => PopularityWeights {
            popular_weight: 0.8,
            elite_weight: 0.0,
            military_weight: 0.1,
            religious_weight: 0.05,
            merchant_weight: 0.05,
        },
        GovernmentCategory::Monarchic => PopularityWeights {
            popular_weight: 0.2,
            elite_weight: 0.4,
            military_weight: 0.2,
            religious_weight: 0.1,
            merchant_weight: 0.1,
        },
        GovernmentCategory::Technocratic => PopularityWeights {
            popular_weight: 0.2,
            elite_weight: 0.3,
            military_weight: 0.1,
            religious_weight: 0.0,
            merchant_weight: 0.4,
        },
        GovernmentCategory::Tribal => PopularityWeights {
            popular_weight: 0.3,
            elite_weight: 0.3,
            military_weight: 0.2,
            religious_weight: 0.2,
            merchant_weight: 0.0,
        },
    }
}