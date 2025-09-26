//! Anti-corruption law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// Anti-corruption laws
pub static CORRUPTION_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(11002),
            category: LawCategory::Administrative,
            name: "Anti-Corruption Agency".to_string(),
            description: "Independent body fighting corruption".to_string(),
            effects: LawEffects {
                corruption_change: -0.3,
                administrative_efficiency_modifier: 0.1,
                maintenance_cost_modifier: 0.05,
                happiness_modifier: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Technocratic, 0.6),
                (GovernmentCategory::Oligarchic, -0.7),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.7,
            is_constitutional: false,
            available_from_year: 150,
        },
        Law {
            id: LawId::new(11004),
            category: LawCategory::Administrative,
            name: "Transparency Laws".to_string(),
            description: "Public access to government records".to_string(),
            effects: LawEffects {
                corruption_change: -0.2,
                happiness_modifier: 0.1,
                administrative_efficiency_modifier: -0.05,
                diplomatic_reputation_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.9),
                (GovernmentCategory::Anarchist, 0.7),
                (GovernmentCategory::Authoritarian, -0.8),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.8,
            is_constitutional: false,
            available_from_year: 100,
        },
        Law {
            id: LawId::new(11005),
            category: LawCategory::Administrative,
            name: "Patronage System".to_string(),
            description: "Government positions as political rewards".to_string(),
            effects: LawEffects {
                corruption_change: 0.3,
                stability_change: 0.05,
                administrative_efficiency_modifier: -0.15,
                revolt_risk_change: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(11000)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Oligarchic, 0.8),
                (GovernmentCategory::Aristocratic, 0.7),
                (GovernmentCategory::Technocratic, -0.9),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});