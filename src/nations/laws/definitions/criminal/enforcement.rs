//! Law enforcement definitions
//!
//! Laws governing policing and criminal enforcement.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All enforcement-related laws
pub static ENFORCEMENT_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(5007),
            category: LawCategory::Criminal,
            name: "Police State".to_string(),
            description: "Extensive surveillance and police presence".to_string(),
            effects: LawEffects {
                revolt_risk_change: -0.3,
                happiness_modifier: -0.2,
                stability_change: 0.15,
                maintenance_cost_modifier: 0.2,
                corruption_change: 0.15,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(5008)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Authoritarian, 1.0),
                (GovernmentCategory::Fascist, 0.9),
                (GovernmentCategory::Democratic, -0.8),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.2,
            is_constitutional: false,
            available_from_year: 50,
        },
        Law {
            id: LawId::new(5008),
            category: LawCategory::Criminal,
            name: "Community Policing".to_string(),
            description: "Local community-based law enforcement".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.1,
                stability_change: 0.05,
                corruption_change: -0.1,
                maintenance_cost_modifier: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(5007)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Anarchist, 0.7),
                (GovernmentCategory::Authoritarian, -0.6),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(5009),
            category: LawCategory::Criminal,
            name: "Secret Police".to_string(),
            description: "Covert enforcement and political surveillance".to_string(),
            effects: LawEffects {
                revolt_risk_change: -0.4,
                happiness_modifier: -0.3,
                stability_change: -0.1,
                corruption_change: 0.3,
                diplomatic_reputation_change: -0.3,
                maintenance_cost_modifier: 0.15,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::HasLaw(LawId::new(5007))],
            conflicts_with: vec![LawId::new(5008)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Fascist, 1.0),
                (GovernmentCategory::Authoritarian, 0.9),
                (GovernmentCategory::Democratic, -1.0),
            ]),
            complexity: LawComplexity::Revolutionary,
            base_popularity: 0.1,
            is_constitutional: false,
            available_from_year: 100,
        },
    ]
});