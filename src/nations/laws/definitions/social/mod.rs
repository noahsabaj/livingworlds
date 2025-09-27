//! Social law definitions gateway
//!
//! Provides access to all social law categories including
//! healthcare, education, gender, and marriage laws.

// Private modules - gateway architecture
mod healthcare;
mod education;
mod gender;
mod marriage;

// Re-export social laws
pub use healthcare::HEALTHCARE_LAWS;
pub use education::EDUCATION_LAWS;
pub use gender::GENDER_LAWS;
pub use marriage::MARRIAGE_LAWS;

use crate::nations::laws::types::Law;
use once_cell::sync::Lazy;

/// All social laws combined
pub static SOCIAL_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    let mut laws = Vec::new();
    laws.extend(HEALTHCARE_LAWS.iter().cloned());
    laws.extend(EDUCATION_LAWS.iter().cloned());
    laws.extend(GENDER_LAWS.iter().cloned());
    laws.extend(MARRIAGE_LAWS.iter().cloned());
    laws
});

/// Get all social laws
pub fn get_all_social_laws() -> Vec<&'static Law> {
    SOCIAL_LAWS.iter().collect()
}