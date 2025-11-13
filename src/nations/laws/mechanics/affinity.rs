//! Government law affinity
//!
//! Determines how well laws align with different government types.

use crate::nations::GovernmentType;
use crate::nations::laws::types::Law;

/// Get government affinity for a law
pub fn get_government_law_affinity(
    law: &Law,
    government_type: GovernmentType,
) -> f32 {
    let category = government_type.category();
    law.government_affinity
        .get(&category)
        .copied()
        .unwrap_or(0.0)
}