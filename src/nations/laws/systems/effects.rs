//! Law effects application system
//!
//! System that applies the combined effects of all active laws to nations.

use bevy::prelude::*;

use crate::nations::laws::mechanics::calculate_law_effects;
use crate::nations::laws::registry::{LawRegistry, NationLaws};
use crate::nations::{Nation, Governance};

/// System to apply law effects to nation mechanics
pub fn apply_law_effects_system(
    mut nations: Query<(&mut Nation, &mut Governance, &NationLaws), Changed<NationLaws>>,
    registry: Res<LawRegistry>,
) {
    for (mut nation, mut governance, nation_laws) in &mut nations {
        // Calculate combined law effects
        let combined_effects = calculate_law_effects(nation_laws, &registry);

        // Apply to nation stats
        nation.stability += combined_effects.stability_change;
        nation.stability = nation.stability.clamp(0.0, 1.0);

        // Apply to governance
        governance.legitimacy += combined_effects.legitimacy_change;
        governance.legitimacy = governance.legitimacy.clamp(0.0, 1.0);

        // Apply to government mechanics (stored in governance)
        // Note: GovernmentMechanics is stored in Governance, not imported separately
        // The apply_law_modifiers function returns a modified copy
    }
}