//! Environmental law definitions gateway
//!
//! Provides access to environmental protection and resource laws.

// Private modules - gateway architecture
mod conservation;
mod pollution;
mod resources;

// Re-export environmental laws
pub use conservation::CONSERVATION_LAWS;
pub use pollution::POLLUTION_LAWS;
pub use resources::RESOURCE_LAWS;

use crate::nations::laws::types::Law;

/// Get all environmental laws
pub fn get_all_environmental_laws() -> Vec<&'static Law> {
    let mut laws = Vec::new();
    laws.extend(CONSERVATION_LAWS.iter());
    laws.extend(POLLUTION_LAWS.iter());
    laws.extend(RESOURCE_LAWS.iter());
    laws
}