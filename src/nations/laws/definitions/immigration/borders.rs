//! Border control law definitions
//!
//! Laws governing border security and movement control.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All border control laws
pub static BORDER_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(7000),
            category: LawCategory::Immigration,
            name: "Open Borders".to_string(),
            description: "Free movement across national boundaries".to_string(),
            effects: LawEffects {
                population_growth_modifier: 0.2,
                trade_income_modifier: 0.15,
                happiness_modifier: 0.05,
                stability_change: -0.1,
                cultural_conversion_modifier: -0.2,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(7001), LawId::new(7002)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.6),
                (GovernmentCategory::Anarchist, 0.9),
                (GovernmentCategory::Fascist, -0.9),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(7001),
            category: LawCategory::Immigration,
            name: "Controlled Borders".to_string(),
            description: "Regulated entry with documentation requirements".to_string(),
            effects: LawEffects {
                stability_change: 0.1,
                population_growth_modifier: 0.05,
                trade_income_modifier: 0.05,
                maintenance_cost_modifier: 0.05,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(2)],
            conflicts_with: vec![LawId::new(7000), LawId::new(7002)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.5),
                (GovernmentCategory::Monarchic, 0.4),
                (GovernmentCategory::Corporate, 0.6),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 50,
        },
        Law {
            id: LawId::new(7002),
            category: LawCategory::Immigration,
            name: "Closed Borders".to_string(),
            description: "Severe restrictions on entry and exit".to_string(),
            effects: LawEffects {
                stability_change: 0.15,
                population_growth_modifier: -0.15,
                trade_income_modifier: -0.2,
                revolt_risk_change: 0.1,
                cultural_conversion_modifier: 0.3,
                diplomatic_reputation_change: -0.2,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(7000), LawId::new(7001)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Fascist, 0.9),
                (GovernmentCategory::Authoritarian, 0.7),
                (GovernmentCategory::Democratic, -0.6),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});