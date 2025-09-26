//! Religious tolerance law definitions
//!
//! Laws governing religious tolerance and minority rights.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All tolerance-related laws
pub static TOLERANCE_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(4009),
            category: LawCategory::Religious,
            name: "Religious Persecution".to_string(),
            description: "Active suppression of minority faiths".to_string(),
            effects: LawEffects {
                stability_change: -0.2,
                happiness_modifier: -0.15,
                revolt_risk_change: 0.2,
                cultural_conversion_modifier: 0.4,
                diplomatic_reputation_change: -0.2,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::HasLaw(LawId::new(4000))],
            conflicts_with: vec![LawId::new(4010), LawId::new(4011)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Theocratic, 0.6),
                (GovernmentCategory::Fascist, 0.8),
                (GovernmentCategory::Democratic, -0.9),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.2,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(4010),
            category: LawCategory::Religious,
            name: "Religious Tolerance".to_string(),
            description: "Protection of religious minorities from discrimination".to_string(),
            effects: LawEffects {
                stability_change: 0.1,
                happiness_modifier: 0.1,
                trade_income_modifier: 0.1,
                diplomatic_reputation_change: 0.15,
                cultural_conversion_modifier: -0.3,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(4009)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Monarchic, 0.3),
                (GovernmentCategory::Theocratic, -0.5),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 50,
        },
        Law {
            id: LawId::new(4011),
            category: LawCategory::Religious,
            name: "Forced Conversion".to_string(),
            description: "Mandatory conversion to state religion".to_string(),
            effects: LawEffects {
                cultural_conversion_modifier: 0.6,
                happiness_modifier: -0.3,
                revolt_risk_change: 0.3,
                stability_change: -0.15,
                legitimacy_change: -0.1,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::HasLaw(LawId::new(4000))],
            conflicts_with: vec![LawId::new(4010), LawId::new(4004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Theocratic, 0.7),
                (GovernmentCategory::Fascist, 0.6),
                (GovernmentCategory::Democratic, -1.0),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.1,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});