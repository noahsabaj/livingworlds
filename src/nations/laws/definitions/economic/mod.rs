//! Economic law definitions gateway
//!
//! Provides access to all economic law categories including
//! taxation, trade, labor, currency, and welfare laws.

// Private modules - gateway architecture
mod taxation;
mod trade;
mod labor;
mod currency;
mod market;
// TODO: Split the rest from categories.rs
// mod banking;
// mod welfare;

// Re-export economic laws
pub use taxation::TAX_LAWS;
pub use trade::TRADE_LAWS;
pub use labor::LABOR_LAWS;
pub use currency::CURRENCY_LAWS;
pub use market::MARKET_LAWS;
// pub use banking::BANKING_LAWS;
// pub use welfare::WELFARE_LAWS;

use crate::nations::laws::types::Law;
use once_cell::sync::Lazy;

/// All economic laws combined
pub static ECONOMIC_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    let mut laws = Vec::new();
    laws.extend(TAX_LAWS.iter().cloned());
    laws.extend(TRADE_LAWS.iter().cloned());
    laws.extend(LABOR_LAWS.iter().cloned());
    laws.extend(CURRENCY_LAWS.iter().cloned());
    laws.extend(MARKET_LAWS.iter().cloned());
    laws
});

/// Get all economic laws
pub fn get_all_economic_laws() -> Vec<&'static Law> {
    ECONOMIC_LAWS.iter().collect()
}