//! Law system update systems gateway
//!
//! Provides controlled access to ECS systems that handle law mechanics.

// Private modules - gateway architecture
mod debate;
mod effect_application;
mod effects;
mod proposal;
mod transitions;
mod validation;
mod voting;

// Re-export all systems
pub use debate::update_law_debates_system;
pub use effect_application::{apply_law_effects_to_nations, Economy};
pub use effects::apply_law_effects_system;
pub use proposal::propose_laws_system;
pub use transitions::{handle_government_transitions_system, update_law_cooldowns_system};

// Debug-only validation systems
#[cfg(debug_assertions)]
pub use validation::{validate_law_data_system, periodic_recalculation_system};

pub use voting::process_law_votes_system;