//! Intellectual property law definitions
//!
//! Laws governing ideas, inventions, and creative works.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All intellectual property laws
pub static INTELLECTUAL_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(6006),
            category: LawCategory::Property,
            name: "Patent System".to_string(),
            description: "Exclusive rights for inventions".to_string(),
            effects: LawEffects {
                technology_rate_modifier: 0.25,
                industrial_output_modifier: 0.1,
                trade_income_modifier: 0.05,
                corruption_change: 0.05,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(3)],
            conflicts_with: vec![LawId::new(6007)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 0.9),
                (GovernmentCategory::Technocratic, 0.8),
                (GovernmentCategory::Socialist, -0.4),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 150,
        },
        Law {
            id: LawId::new(6007),
            category: LawCategory::Property,
            name: "Open Knowledge".to_string(),
            description: "Ideas freely shared without restrictions".to_string(),
            effects: LawEffects {
                technology_rate_modifier: 0.15,
                happiness_modifier: 0.1,
                trade_income_modifier: -0.05,
                diplomatic_reputation_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(6006)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Socialist, 0.8),
                (GovernmentCategory::Anarchist, 0.9),
                (GovernmentCategory::Corporate, -0.7),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(6008),
            category: LawCategory::Property,
            name: "Guild Secrets".to_string(),
            description: "Trade knowledge protected by guilds".to_string(),
            effects: LawEffects {
                industrial_output_modifier: 0.15,
                technology_rate_modifier: -0.1,
                trade_income_modifier: 0.1,
                corruption_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(6006), LawId::new(6007)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Oligarchic, 0.8),
                (GovernmentCategory::Corporate, 0.6),
                (GovernmentCategory::Democratic, -0.4),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});