//! Faith law definitions
//!
//! Laws governing state religion and faith requirements.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All faith-related laws
pub static FAITH_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(4000),
            category: LawCategory::Religious,
            name: "State Religion".to_string(),
            description: "Official religion enforced by the state".to_string(),
            effects: LawEffects {
                legitimacy_change: 0.2,
                stability_change: 0.15,
                cultural_conversion_modifier: 0.3,
                happiness_modifier: -0.05,
                allows_religious_freedom: Some(false),
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(4001), LawId::new(4002)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Theocratic, 1.0),
                (GovernmentCategory::Monarchic, 0.6),
                (GovernmentCategory::Democratic, -0.3),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.4,
            is_constitutional: true,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(4001),
            category: LawCategory::Religious,
            name: "Religious Pluralism".to_string(),
            description: "Multiple religions coexist under state protection".to_string(),
            effects: LawEffects {
                stability_change: -0.05,
                happiness_modifier: 0.15,
                cultural_conversion_modifier: -0.2,
                trade_income_modifier: 0.1,
                allows_religious_freedom: Some(true),
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(2)],
            conflicts_with: vec![LawId::new(4000), LawId::new(4002)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Corporate, 0.5),
                (GovernmentCategory::Theocratic, -0.9),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.6,
            is_constitutional: true,
            available_from_year: 100,
        },
        Law {
            id: LawId::new(4002),
            category: LawCategory::Religious,
            name: "State Atheism".to_string(),
            description: "Religion banned by government decree".to_string(),
            effects: LawEffects {
                technology_rate_modifier: 0.2,
                legitimacy_change: -0.15,
                stability_change: -0.1,
                happiness_modifier: -0.2,
                allows_religious_freedom: Some(false),
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::GovernmentCategory(GovernmentCategory::Socialist)],
            conflicts_with: vec![LawId::new(4000), LawId::new(4001)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Socialist, 0.7),
                (GovernmentCategory::Technocratic, 0.4),
                (GovernmentCategory::Theocratic, -1.0),
            ]),
            complexity: LawComplexity::Revolutionary,
            base_popularity: 0.2,
            is_constitutional: true,
            available_from_year: 200,
        },
    ]
});