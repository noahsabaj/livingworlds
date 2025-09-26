//! Citizenship law definitions
//!
//! Laws governing citizenship acquisition and rights.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All citizenship laws
pub static CITIZENSHIP_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(7003),
            category: LawCategory::Immigration,
            name: "Birthright Citizenship".to_string(),
            description: "Anyone born in the nation gains citizenship".to_string(),
            effects: LawEffects {
                population_growth_modifier: 0.1,
                happiness_modifier: 0.1,
                stability_change: 0.05,
                cultural_conversion_modifier: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(7004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Socialist, 0.6),
                (GovernmentCategory::Aristocratic, -0.7),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.7,
            is_constitutional: true,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(7004),
            category: LawCategory::Immigration,
            name: "Blood Citizenship".to_string(),
            description: "Citizenship through ancestry only".to_string(),
            effects: LawEffects {
                cultural_conversion_modifier: 0.4,
                stability_change: 0.1,
                population_growth_modifier: -0.05,
                revolt_risk_change: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(7003)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Fascist, 0.9),
                (GovernmentCategory::Aristocratic, 0.7),
                (GovernmentCategory::Democratic, -0.5),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.4,
            is_constitutional: true,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(7005),
            category: LawCategory::Immigration,
            name: "Naturalization Path".to_string(),
            description: "Process for immigrants to gain citizenship".to_string(),
            effects: LawEffects {
                population_growth_modifier: 0.15,
                trade_income_modifier: 0.05,
                happiness_modifier: 0.05,
                maintenance_cost_modifier: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Corporate, 0.5),
                (GovernmentCategory::Fascist, -0.4),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 50,
        },
    ]
});