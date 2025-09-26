//! Clergy law definitions
//!
//! Laws governing religious leadership and clerical privileges.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All clergy-related laws
pub static CLERGY_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(4006),
            category: LawCategory::Religious,
            name: "Clerical Privilege".to_string(),
            description: "Religious leaders exempt from taxes and secular law".to_string(),
            effects: LawEffects {
                tax_efficiency_modifier: -0.15,
                legitimacy_change: 0.2,
                corruption_change: 0.1,
                stability_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(4007)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Theocratic, 1.0),
                (GovernmentCategory::Monarchic, 0.4),
                (GovernmentCategory::Socialist, -0.8),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(4007),
            category: LawCategory::Religious,
            name: "Secular Governance".to_string(),
            description: "Separation of religious and state authority".to_string(),
            effects: LawEffects {
                technology_rate_modifier: 0.1,
                legitimacy_change: -0.1,
                corruption_change: -0.05,
                tax_efficiency_modifier: 0.1,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(3)],
            conflicts_with: vec![LawId::new(4006), LawId::new(4008)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Technocratic, 0.8),
                (GovernmentCategory::Theocratic, -1.0),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.5,
            is_constitutional: true,
            available_from_year: 150,
        },
        Law {
            id: LawId::new(4008),
            category: LawCategory::Religious,
            name: "Divine Right".to_string(),
            description: "Ruler's authority comes directly from divine mandate".to_string(),
            effects: LawEffects {
                legitimacy_change: 0.3,
                stability_change: 0.2,
                reform_resistance_change: 0.3,
                happiness_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::GovernmentCategory(GovernmentCategory::Monarchic)],
            conflicts_with: vec![LawId::new(4007)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Monarchic, 0.9),
                (GovernmentCategory::Theocratic, 0.8),
                (GovernmentCategory::Democratic, -0.9),
            ]),
            complexity: LawComplexity::Revolutionary,
            base_popularity: 0.3,
            is_constitutional: true,
            available_from_year: 0,
        },
    ]
});