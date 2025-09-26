//! Bureaucracy law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// Bureaucracy laws
pub static BUREAUCRACY_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(11000),
            category: LawCategory::Administrative,
            name: "Meritocratic Bureaucracy".to_string(),
            description: "Civil service based on examinations".to_string(),
            effects: LawEffects {
                administrative_efficiency_modifier: 0.25,
                corruption_change: -0.15,
                maintenance_cost_modifier: 0.1,
                technology_rate_modifier: 0.05,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(2)],
            conflicts_with: vec![LawId::new(11001)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Technocratic, 0.9),
                (GovernmentCategory::Democratic, 0.6),
                (GovernmentCategory::Aristocratic, -0.7),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 100,
        },
        Law {
            id: LawId::new(11001),
            category: LawCategory::Administrative,
            name: "Hereditary Offices".to_string(),
            description: "Administrative positions inherited".to_string(),
            effects: LawEffects {
                stability_change: 0.1,
                corruption_change: 0.2,
                administrative_efficiency_modifier: -0.1,
                legitimacy_change: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(11000)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Aristocratic, 0.9),
                (GovernmentCategory::Monarchic, 0.7),
                (GovernmentCategory::Democratic, -0.8),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});