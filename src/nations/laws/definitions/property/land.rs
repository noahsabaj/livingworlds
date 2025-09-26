//! Land ownership law definitions
//!
//! Laws governing land ownership and property rights.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All land ownership laws
pub static LAND_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(6000),
            category: LawCategory::Property,
            name: "Private Property".to_string(),
            description: "Individuals can own land and resources".to_string(),
            effects: LawEffects {
                industrial_output_modifier: 0.2,
                agricultural_output_modifier: 0.15,
                happiness_modifier: 0.1,
                revolt_risk_change: -0.05,
                corruption_change: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(6001), LawId::new(6002)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 1.0),
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Socialist, -0.8),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.6,
            is_constitutional: true,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(6001),
            category: LawCategory::Property,
            name: "Collective Ownership".to_string(),
            description: "Land owned collectively by communities".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.15,
                stability_change: 0.1,
                industrial_output_modifier: -0.1,
                agricultural_output_modifier: 0.05,
                corruption_change: -0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(6000)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Socialist, 0.9),
                (GovernmentCategory::Anarchist, 0.8),
                (GovernmentCategory::Corporate, -0.9),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.4,
            is_constitutional: true,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(6002),
            category: LawCategory::Property,
            name: "Feudal Land Rights".to_string(),
            description: "Land owned by nobility, worked by serfs".to_string(),
            effects: LawEffects {
                agricultural_output_modifier: 0.1,
                happiness_modifier: -0.2,
                revolt_risk_change: 0.15,
                maintenance_cost_modifier: -0.15,
                legitimacy_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::GovernmentCategory(GovernmentCategory::Monarchic)],
            conflicts_with: vec![LawId::new(6000), LawId::new(6001)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Monarchic, 0.9),
                (GovernmentCategory::Aristocratic, 1.0),
                (GovernmentCategory::Democratic, -0.8),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.2,
            is_constitutional: true,
            available_from_year: 0,
        },
    ]
});