//! Arts funding law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// Arts funding laws
pub static ARTS_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(10002),
            category: LawCategory::Cultural,
            name: "State Arts Patronage".to_string(),
            description: "Government funding for cultural activities".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.15,
                legitimacy_change: 0.05,
                maintenance_cost_modifier: 0.05,
                cultural_conversion_modifier: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Monarchic, 0.6),
                (GovernmentCategory::Socialist, 0.5),
                (GovernmentCategory::Corporate, -0.4),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.7,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});