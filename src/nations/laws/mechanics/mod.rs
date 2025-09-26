//! Law mechanics gateway
//!
//! This module orchestrates all law mechanics calculations including
//! effects, modifiers, conflicts, popularity, and diplomatic impacts.

// Private modules - gateway architecture
mod effects;
mod modifiers;
mod conflicts;
mod popularity;
mod affinity;
mod suggestions;
mod diplomacy;

// Public exports - controlled API surface
pub use effects::{calculate_law_effects, apply_diminishing_returns};
pub use modifiers::apply_law_modifiers;
pub use conflicts::check_law_conflicts;
pub use popularity::{evaluate_law_popularity, calculate_popularity_weights};
pub use affinity::get_government_law_affinity;
pub use suggestions::suggest_laws_for_pressures;
pub use diplomacy::calculate_law_diplomatic_impact;