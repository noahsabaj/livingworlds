//! Judicial system law definitions
//!
//! Laws governing court systems and legal procedures.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All judicial system laws
pub static JUDICIAL_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(5004),
            category: LawCategory::Criminal,
            name: "Trial by Jury".to_string(),
            description: "Citizens judge criminal cases".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.2,
                legitimacy_change: 0.15,
                maintenance_cost_modifier: 0.05,
                corruption_change: -0.1,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(2)],
            conflicts_with: vec![LawId::new(5005), LawId::new(5006)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 1.0),
                (GovernmentCategory::Oligarchic, 0.3),
                (GovernmentCategory::Authoritarian, -0.7),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.7,
            is_constitutional: true,
            available_from_year: 100,
        },
        Law {
            id: LawId::new(5005),
            category: LawCategory::Criminal,
            name: "Authoritarian Courts".to_string(),
            description: "State-controlled judicial system".to_string(),
            effects: LawEffects {
                revolt_risk_change: -0.2,
                happiness_modifier: -0.15,
                corruption_change: 0.2,
                maintenance_cost_modifier: -0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(5004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Authoritarian, 0.9),
                (GovernmentCategory::Fascist, 0.8),
                (GovernmentCategory::Democratic, -1.0),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.2,
            is_constitutional: true,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(5006),
            category: LawCategory::Criminal,
            name: "Trial by Ordeal".to_string(),
            description: "Divine judgment through physical trials".to_string(),
            effects: LawEffects {
                technology_rate_modifier: -0.15,
                stability_change: 0.1,
                legitimacy_change: 0.05,
                happiness_modifier: -0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(5004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Theocratic, 0.8),
                (GovernmentCategory::Tribal, 0.7),
                (GovernmentCategory::Technocratic, -1.0),
            ]),
            complexity: LawComplexity::Trivial,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});