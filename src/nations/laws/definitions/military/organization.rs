//! Military organization law definitions
//!
//! Laws governing military command structure and officer systems.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All military organization laws
pub static ORGANIZATION_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(2004),
            category: LawCategory::Military,
            name: "Professional Officer Corps".to_string(),
            description: "Military leadership requires formal training".to_string(),
            effects: LawEffects {
                army_morale_modifier: 0.15,
                naval_tradition_modifier: 0.1,
                tax_efficiency_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(2)],
            conflicts_with: vec![LawId::new(2005)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Monarchic, 0.7),
                (GovernmentCategory::Democratic, 0.5),
                (GovernmentCategory::Tribal, -0.4),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 100,
        },
        Law {
            id: LawId::new(2005),
            category: LawCategory::Military,
            name: "Elected Officers".to_string(),
            description: "Military units elect their own commanders".to_string(),
            effects: LawEffects {
                army_morale_modifier: 0.25,
                mobilization_speed_modifier: -0.2,
                stability_change: -0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(2004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Anarchist, 0.9),
                (GovernmentCategory::Democratic, 0.3),
                (GovernmentCategory::Autocratic, -0.8),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});