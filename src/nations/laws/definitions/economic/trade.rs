//! Trade law definitions
//!
//! Laws governing international commerce, tariffs, and trade policies.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};
use crate::simulation::PressureType;

/// All trade-related laws
pub static TRADE_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(1003),
            category: LawCategory::Economic,
            name: "Free Trade".to_string(),
            description: "Minimal restrictions on international commerce".to_string(),
            effects: LawEffects {
                trade_income_modifier: 0.3,
                industrial_output_modifier: -0.1,
                wealth_inequality_change: 0.1,
                diplomatic_reputation_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1004), LawId::new(1005)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 0.9),
                (GovernmentCategory::Democratic, 0.5),
                (GovernmentCategory::Autocratic, -0.3),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(1004),
            category: LawCategory::Economic,
            name: "Protective Tariffs".to_string(),
            description: "High taxes on imports to protect domestic industry".to_string(),
            effects: LawEffects {
                trade_income_modifier: -0.2,
                industrial_output_modifier: 0.2,
                diplomatic_reputation_change: -0.1,
                pressure_modifiers: HashMap::from([
                    (PressureType::EconomicStrain, -0.1),
                ]),
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1003), LawId::new(1005)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Autocratic, 0.6),
                (GovernmentCategory::Socialist, 0.4),
                (GovernmentCategory::Corporate, -0.5),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(1005),
            category: LawCategory::Economic,
            name: "Trade Embargo".to_string(),
            description: "Complete prohibition on trade with certain nations".to_string(),
            effects: LawEffects {
                trade_income_modifier: -0.4,
                diplomatic_reputation_change: -0.2,
                stability_change: 0.1,
                pressure_modifiers: HashMap::from([
                    (PressureType::MilitaryVulnerability, -0.1),
                ]),
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::MinimumStability(0.4)],
            conflicts_with: vec![LawId::new(1003), LawId::new(1004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Autocratic, 0.7),
                (GovernmentCategory::Theocratic, 0.5),
                (GovernmentCategory::Democratic, -0.4),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});