//! Market regulation law definitions
//!
//! Laws governing economic systems and market structures.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};
use crate::simulation::PressureType;

/// All market regulation laws
pub static MARKET_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(1013),
            category: LawCategory::Economic,
            name: "Laissez-Faire".to_string(),
            description: "Minimal government intervention in markets".to_string(),
            effects: LawEffects {
                industrial_output_modifier: 0.25,
                trade_income_modifier: 0.2,
                wealth_inequality_change: 0.3,
                corruption_change: 0.15,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1014), LawId::new(1015)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 1.0),
                (GovernmentCategory::Democratic, 0.3),
                (GovernmentCategory::Socialist, -0.9),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(1014),
            category: LawCategory::Economic,
            name: "Mixed Economy".to_string(),
            description: "Balance of private enterprise and government regulation".to_string(),
            effects: LawEffects {
                industrial_output_modifier: 0.1,
                stability_change: 0.1,
                wealth_inequality_change: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1013), LawId::new(1015)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Socialist, 0.3),
                (GovernmentCategory::Corporate, 0.2),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(1015),
            category: LawCategory::Economic,
            name: "Planned Economy".to_string(),
            description: "Government controls all production and distribution".to_string(),
            effects: LawEffects {
                industrial_output_modifier: -0.15,
                agricultural_output_modifier: 0.1,
                wealth_inequality_change: -0.4,
                stability_change: 0.2,
                trade_income_modifier: -0.3,
                pressure_modifiers: HashMap::from([
                    (PressureType::EconomicStrain, 0.1),
                ]),
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::GovernmentCategory(GovernmentCategory::Socialist)],
            conflicts_with: vec![LawId::new(1013), LawId::new(1014)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Socialist, 1.0),
                (GovernmentCategory::Autocratic, 0.4),
                (GovernmentCategory::Corporate, -1.0),
            ]),
            complexity: LawComplexity::Revolutionary,
            base_popularity: 0.3,
            is_constitutional: true,
            available_from_year: 100,
        },
    ]
});