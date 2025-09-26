//! Administrative taxation law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// Administrative tax laws
pub static ADMIN_TAX_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(11003),
            category: LawCategory::Administrative,
            name: "Centralized Tax Collection".to_string(),
            description: "National agency collects all taxes".to_string(),
            effects: LawEffects {
                tax_efficiency_modifier: 0.2,
                administrative_efficiency_modifier: 0.1,
                corruption_change: -0.05,
                maintenance_cost_modifier: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Technocratic, 0.7),
                (GovernmentCategory::Democratic, 0.5),
                (GovernmentCategory::Anarchist, -0.8),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 100,
        },
    ]
});