//! Innovation policy law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// Innovation policy laws
pub static INNOVATION_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(9002),
            category: LawCategory::Technology,
            name: "Innovation Hubs".to_string(),
            description: "Dedicated zones for technological development".to_string(),
            effects: LawEffects {
                technology_rate_modifier: 0.25,
                industrial_output_modifier: 0.05,
                maintenance_cost_modifier: 0.15,
                trade_income_modifier: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Technocratic, 0.9),
                (GovernmentCategory::Corporate, 0.7),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 200,
        },
        Law {
            id: LawId::new(9004),
            category: LawCategory::Technology,
            name: "Technology Restrictions".to_string(),
            description: "Limits on dangerous technologies".to_string(),
            effects: LawEffects {
                technology_rate_modifier: -0.15,
                stability_change: 0.1,
                revolt_risk_change: -0.05,
                happiness_modifier: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Theocratic, 0.6),
                (GovernmentCategory::Tribal, 0.5),
                (GovernmentCategory::Technocratic, -0.9),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(9005),
            category: LawCategory::Technology,
            name: "Open Source Mandate".to_string(),
            description: "Government technology must be open source".to_string(),
            effects: LawEffects {
                technology_rate_modifier: 0.1,
                corruption_change: -0.1,
                maintenance_cost_modifier: -0.05,
                diplomatic_reputation_change: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.6),
                (GovernmentCategory::Anarchist, 0.8),
                (GovernmentCategory::Corporate, -0.5),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 200,
        },
    ]
});