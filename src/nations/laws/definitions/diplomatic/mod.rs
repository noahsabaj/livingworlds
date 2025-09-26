//! Diplomatic law definitions gateway

// Private modules
mod treaties;
mod trade;
mod alliances;

pub use treaties::TREATY_LAWS;
pub use trade::TRADE_DIPLOMATIC_LAWS;
pub use alliances::ALLIANCE_LAWS;

use crate::nations::laws::types::Law;

/// Get all diplomatic laws
pub fn get_all_diplomatic_laws() -> Vec<&'static Law> {
    let mut laws = Vec::new();
    laws.extend(TREATY_LAWS.iter());
    laws.extend(TRADE_DIPLOMATIC_LAWS.iter());
    laws.extend(ALLIANCE_LAWS.iter());
    laws
}