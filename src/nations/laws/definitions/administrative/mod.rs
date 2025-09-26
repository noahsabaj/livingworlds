//! Administrative law definitions gateway

// Private modules
mod bureaucracy;
mod corruption;
mod taxation;

pub use bureaucracy::BUREAUCRACY_LAWS;
pub use corruption::CORRUPTION_LAWS;
pub use taxation::ADMIN_TAX_LAWS;

use crate::nations::laws::types::Law;

/// Get all administrative laws
pub fn get_all_administrative_laws() -> Vec<&'static Law> {
    let mut laws = Vec::new();
    laws.extend(BUREAUCRACY_LAWS.iter());
    laws.extend(CORRUPTION_LAWS.iter());
    laws.extend(ADMIN_TAX_LAWS.iter());
    laws
}