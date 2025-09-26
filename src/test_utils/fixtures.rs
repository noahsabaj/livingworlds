//! Test fixtures and data
//!
//! Common test data and fixtures used across multiple tests.

use crate::nations::laws::registry::LawRegistry;
use crate::nations::laws::types::{LawId, LawEffects};
use crate::nations::laws::definitions::economic::taxation::*;
use crate::nations::laws::definitions::social::healthcare::*;

/// Initialize test laws for testing
pub fn initialize_test_laws(registry: &mut LawRegistry) {
    // Add some test laws with known effects
    registry.register_law(
        LawId::MinimalTaxation,
        create_minimal_taxation_law()
    );

    registry.register_law(
        LawId::ModerateTaxation,
        create_moderate_taxation_law()
    );

    registry.register_law(
        LawId::HeavyTaxation,
        create_heavy_taxation_law()
    );

    registry.register_law(
        LawId::UniversalHealthcare,
        create_universal_healthcare_law()
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