//! Worship law definitions
//!
//! Laws governing religious practices and worship requirements.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};
use crate::simulation::PressureType;

/// All worship-related laws
pub static WORSHIP_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(4003),
            category: LawCategory::Religious,
            name: "Mandatory Worship".to_string(),
            description: "Citizens required to attend religious services".to_string(),
            effects: LawEffects {
                legitimacy_change: 0.15,
                happiness_modifier: -0.1,
                stability_change: 0.1,
                industrial_output_modifier: -0.05,
                pressure_modifiers: HashMap::from([
                    (PressureType::LegitimacyCrisis, -0.15),
                ]),
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::HasLaw(LawId::new(4000))],
            conflicts_with: vec![LawId::new(4004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Theocratic, 0.9),
                (GovernmentCategory::Tribal, 0.5),
                (GovernmentCategory::Democratic, -0.6),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(4004),
            category: LawCategory::Religious,
            name: "Freedom of Worship".to_string(),
            description: "Citizens free to practice religion as they choose".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.2,
                stability_change: -0.05,
                trade_income_modifier: 0.05,
                diplomatic_reputation_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(4003), LawId::new(4005)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.9),
                (GovernmentCategory::Anarchist, 0.8),
                (GovernmentCategory::Theocratic, -0.7),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.7,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(4005),
            category: LawCategory::Religious,
            name: "Religious Holidays".to_string(),
            description: "State-mandated religious observances and holidays".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.1,
                industrial_output_modifier: -0.1,
                stability_change: 0.05,
                cultural_conversion_modifier: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(4004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Theocratic, 0.7),
                (GovernmentCategory::Monarchic, 0.5),
                (GovernmentCategory::Corporate, -0.4),
            ]),
            complexity: LawComplexity::Trivial,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});