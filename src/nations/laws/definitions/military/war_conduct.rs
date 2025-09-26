//! War conduct law definitions
//!
//! Laws governing warfare doctrines and rules of engagement.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All war conduct laws
pub static WAR_CONDUCT_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(2006),
            category: LawCategory::Military,
            name: "Total War Doctrine".to_string(),
            description: "All resources devoted to warfare when at war".to_string(),
            effects: LawEffects {
                mobilization_speed_modifier: 0.5,
                industrial_output_modifier: 0.3,
                expansion_desire_modifier: 0.3,
                happiness_modifier: -0.2,
                diplomatic_reputation_change: -0.2,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::GovernmentCategory(GovernmentCategory::Autocratic)],
            conflicts_with: vec![LawId::new(2007)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Autocratic, 0.9),
                (GovernmentCategory::Democratic, -0.6),
                (GovernmentCategory::Anarchist, -0.8),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 150,
        },
        Law {
            id: LawId::new(2007),
            category: LawCategory::Military,
            name: "Laws of War".to_string(),
            description: "Military follows international humanitarian rules".to_string(),
            effects: LawEffects {
                diplomatic_reputation_change: 0.25,
                army_morale_modifier: -0.05,
                stability_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(2006)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Anarchist, 0.4),
                (GovernmentCategory::Autocratic, -0.5),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.7,
            is_constitutional: false,
            available_from_year: 100,
        },
    ]
});