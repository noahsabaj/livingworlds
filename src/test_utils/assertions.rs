//! Test assertion utilities
//!
//! Common assertion helpers for testing game state.

use bevy::prelude::*;
use crate::nations::{NationLaws, LawId};

/// Assert that a nation has a specific law active
pub fn assert_law_active(app: &App, nation: Entity, law_id: LawId) {
    let laws = app.world()
        .entity(nation)
        .get::<NationLaws>()
        .expect("Nation should have NationLaws component");

    assert!(
        laws.is_active(law_id),
        "Law {:?} should be active but is not",
        law_id
    );
}

/// Assert that a nation does not have a specific law active
pub fn assert_law_not_active(app: &App, nation: Entity, law_id: LawId) {
    let laws = app.world()
        .entity(nation)
        .get::<NationLaws>()
        .expect("Nation should have NationLaws component");

    assert!(
        !laws.is_active(law_id),
        "Law {:?} should not be active but is",
        law_id
    );
}