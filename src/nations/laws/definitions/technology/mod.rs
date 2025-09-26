//! Technology law definitions gateway

// Private modules
mod research;
mod innovation;
mod education;

pub use research::RESEARCH_LAWS;
pub use innovation::INNOVATION_LAWS;
pub use education::EDUCATION_LAWS;

use crate::nations::laws::types::Law;

/// Get all technology laws
pub fn get_all_technology_laws() -> Vec<&'static Law> {
    let mut laws = Vec::new();
    laws.extend(RESEARCH_LAWS.iter());
    laws.extend(INNOVATION_LAWS.iter());
    laws.extend(EDUCATION_LAWS.iter());
    laws
}