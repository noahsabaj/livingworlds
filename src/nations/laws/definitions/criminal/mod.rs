//! Criminal law definitions gateway
//!
//! Provides access to all criminal law categories including
//! punishment, judicial, and enforcement laws.

// Private modules - gateway architecture
mod punishment;
mod judicial;
mod enforcement;

// Re-export criminal laws
pub use punishment::PUNISHMENT_LAWS;
pub use judicial::JUDICIAL_LAWS;
pub use enforcement::ENFORCEMENT_LAWS;

use crate::nations::laws::types::Law;

/// Get all criminal laws
pub fn get_all_criminal_laws() -> Vec<&'static Law> {
    let mut laws = Vec::new();
    laws.extend(PUNISHMENT_LAWS.iter());
    laws.extend(JUDICIAL_LAWS.iter());
    laws.extend(ENFORCEMENT_LAWS.iter());
    laws
}