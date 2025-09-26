//! Language policy law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// Language policy laws
pub static LANGUAGE_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(10000),
            category: LawCategory::Cultural,
            name: "Official Language".to_string(),
            description: "Single mandated language for government".to_string(),
            effects: LawEffects {
                cultural_conversion_modifier: 0.3,
                stability_change: 0.05,
                happiness_modifier: -0.05,
                administrative_efficiency_modifier: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(10001)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Fascist, 0.7),
                (GovernmentCategory::Monarchic, 0.5),
                (GovernmentCategory::Democratic, -0.2),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(10001),
            category: LawCategory::Cultural,
            name: "Multilingualism".to_string(),
            description: "Multiple official languages recognized".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.1,
                cultural_conversion_modifier: -0.2,
                administrative_efficiency_modifier: -0.05,
                diplomatic_reputation_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(10000)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Anarchist, 0.6),
                (GovernmentCategory::Fascist, -0.6),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 50,
        },
    ]
});