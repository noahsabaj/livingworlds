//! Immigration law definitions gateway
//!
//! Provides access to all immigration law categories including
//! border control, citizenship, and refugee laws.

// Private modules - gateway architecture
mod borders;
mod citizenship;
mod refugees;

// Re-export immigration laws
pub use borders::BORDER_LAWS;
pub use citizenship::CITIZENSHIP_LAWS;
pub use refugees::REFUGEE_LAWS;

use crate::nations::laws::types::Law;

/// Get all immigration laws
pub fn get_all_immigration_laws() -> Vec<&'static Law> {
    let mut laws = Vec::new();
    laws.extend(BORDER_LAWS.iter());
    laws.extend(CITIZENSHIP_LAWS.iter());
    laws.extend(REFUGEE_LAWS.iter());
    laws
}