//! Military law definitions gateway
//!
//! Provides access to all military law categories including
//! conscription, organization, and war conduct laws.

// Private modules - gateway architecture
mod conscription;
mod organization;
mod war_conduct;
// TODO: Split the rest from categories.rs
// mod weapons;

// Re-export military laws
pub use conscription::CONSCRIPTION_LAWS;
pub use organization::ORGANIZATION_LAWS;
pub use war_conduct::WAR_CONDUCT_LAWS;
// pub use weapons::WEAPONS_LAWS;

use crate::nations::laws::types::Law;

/// Get all military laws
pub fn get_all_military_laws() -> Vec<&'static Law> {
    let mut laws = Vec::new();
    laws.extend(CONSCRIPTION_LAWS.iter());
    laws.extend(ORGANIZATION_LAWS.iter());
    laws.extend(WAR_CONDUCT_LAWS.iter());
    // laws.extend(WEAPONS_LAWS.iter());
    laws
}