//! Religious law definitions gateway
//!
//! Provides access to all religious law categories including
//! faith, worship, clergy, and tolerance laws.

// Private modules - gateway architecture
mod faith;
mod worship;
mod clergy;
mod tolerance;

// Re-export religious laws
pub use faith::FAITH_LAWS;
pub use worship::WORSHIP_LAWS;
pub use clergy::CLERGY_LAWS;
pub use tolerance::TOLERANCE_LAWS;

use crate::nations::laws::types::Law;

/// Get all religious laws
pub fn get_all_religious_laws() -> Vec<&'static Law> {
    let mut laws = Vec::new();
    laws.extend(FAITH_LAWS.iter());
    laws.extend(WORSHIP_LAWS.iter());
    laws.extend(CLERGY_LAWS.iter());
    laws.extend(TOLERANCE_LAWS.iter());
    laws
}