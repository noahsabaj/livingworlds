//! Property law definitions gateway
//!
//! Provides access to all property law categories including
//! land ownership, inheritance, and intellectual property laws.

// Private modules - gateway architecture
mod land;
mod inheritance;
mod intellectual;

// Re-export property laws
pub use land::LAND_LAWS;
pub use inheritance::INHERITANCE_LAWS;
pub use intellectual::INTELLECTUAL_LAWS;

use crate::nations::laws::types::Law;

/// Get all property laws
pub fn get_all_property_laws() -> Vec<&'static Law> {
    let mut laws = Vec::new();
    laws.extend(LAND_LAWS.iter());
    laws.extend(INHERITANCE_LAWS.iter());
    laws.extend(INTELLECTUAL_LAWS.iter());
    laws
}