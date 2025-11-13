//! Test fixtures and data
//!
//! Common test data and fixtures used across multiple tests.

use crate::nations::{LawRegistry, LawId, LawEffects, Law, LawCategory, LawComplexity};

/// Initialize test laws for testing
pub fn initialize_test_laws(registry: &mut LawRegistry) {
    // Add some test laws with known effects
    registry.register_law(
        Law {
            id: LawId(1),
            name: "Minimal Taxation".to_string(),
            description: "Test law for minimal taxation".to_string(),
            category: LawCategory::Economic,
            effects: TestLawEffects::tax_law_1(),
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: Default::default(),
            complexity: LawComplexity::Simple,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 0,
        }
    );

    registry.register_law(
        Law {
            id: LawId(2),
            name: "Moderate Taxation".to_string(),
            description: "Test law for moderate taxation".to_string(),
            category: LawCategory::Economic,
            effects: TestLawEffects::tax_law_2(),
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: Default::default(),
            complexity: LawComplexity::Simple,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 0,
        }
    );

    registry.register_law(
        Law {
            id: LawId(3),
            name: "Heavy Taxation".to_string(),
            description: "Test law for heavy taxation".to_string(),
            category: LawCategory::Economic,
            effects: TestLawEffects::tax_law_3(),
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: Default::default(),
            complexity: LawComplexity::Simple,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 0,
        }
    );

    registry.register_law(
        Law {
            id: LawId(4),
            name: "Universal Healthcare".to_string(),
            description: "Test law for universal healthcare".to_string(),
            category: LawCategory::Social,
            effects: TestLawEffects::stability_law(),
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: Default::default(),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.7,
            is_constitutional: false,
            available_from_year: 0,
        }
    );
}

/// Test fixture for law effects
pub struct TestLawEffects;

impl TestLawEffects {
    pub fn tax_law_1() -> LawEffects {
        LawEffects {
            tax_efficiency_modifier: 0.1,
            ..Default::default()
        }
    }

    pub fn tax_law_2() -> LawEffects {
        LawEffects {
            tax_efficiency_modifier: 0.1,
            ..Default::default()
        }
    }

    pub fn tax_law_3() -> LawEffects {
        LawEffects {
            tax_efficiency_modifier: 0.1,
            ..Default::default()
        }
    }

    pub fn stability_law() -> LawEffects {
        LawEffects {
            stability_change: 0.2,
            ..Default::default()
        }
    }
}