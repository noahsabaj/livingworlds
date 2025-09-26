//! Military conscription law definitions
//!
//! Laws governing military service requirements and draft policies.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All conscription-related laws
pub static CONSCRIPTION_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(2000),
            category: LawCategory::Military,
            name: "Volunteer Army".to_string(),
            description: "Military service is entirely voluntary".to_string(),
            effects: LawEffects {
                army_morale_modifier: 0.2,
                mobilization_speed_modifier: -0.3,
                happiness_modifier: 0.1,
                tax_efficiency_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(2001), LawId::new(2002), LawId::new(2003)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Anarchist, 0.6),
                (GovernmentCategory::Autocratic, -0.4),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.8,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(2001),
            category: LawCategory::Military,
            name: "Limited Conscription".to_string(),
            description: "Selective military draft during wartime".to_string(),
            effects: LawEffects {
                mobilization_speed_modifier: 0.2,
                army_morale_modifier: -0.1,
                happiness_modifier: -0.05,
                defensive_bonus_modifier: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(2000), LawId::new(2002), LawId::new(2003)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.4),
                (GovernmentCategory::Monarchic, 0.5),
                (GovernmentCategory::Corporate, 0.3),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(2002),
            category: LawCategory::Military,
            name: "Universal Conscription".to_string(),
            description: "All citizens must serve in the military".to_string(),
            effects: LawEffects {
                mobilization_speed_modifier: 0.4,
                army_morale_modifier: -0.15,
                happiness_modifier: -0.15,
                defensive_bonus_modifier: 0.2,
                population_growth_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(2000), LawId::new(2001), LawId::new(2003)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Autocratic, 0.8),
                (GovernmentCategory::Socialist, 0.4),
                (GovernmentCategory::Democratic, -0.3),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(2003),
            category: LawCategory::Military,
            name: "Pacifist Constitution".to_string(),
            description: "Military force prohibited except for defense".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.2,
                stability_change: 0.2,
                expansion_desire_modifier: -0.8,
                defensive_bonus_modifier: -0.2,
                diplomatic_reputation_change: 0.3,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::MinimumStability(0.6)],
            conflicts_with: vec![LawId::new(2000), LawId::new(2001), LawId::new(2002)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.5),
                (GovernmentCategory::Anarchist, 0.7),
                (GovernmentCategory::Autocratic, -0.9),
            ]),
            complexity: LawComplexity::Revolutionary,
            base_popularity: 0.6,
            is_constitutional: true,
            available_from_year: 200,
        },
    ]
});