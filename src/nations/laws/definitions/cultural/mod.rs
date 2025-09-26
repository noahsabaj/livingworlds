//! Cultural law definitions gateway

// Private modules
mod language;
mod arts;
mod traditions;
mod media;

pub use language::LANGUAGE_LAWS;
pub use arts::ARTS_LAWS;
pub use traditions::TRADITION_LAWS;
pub use media::MEDIA_LAWS;

use crate::nations::laws::types::Law;

/// Get all cultural laws
pub fn get_all_cultural_laws() -> Vec<&'static Law> {
    let mut laws = Vec::new();
    laws.extend(LANGUAGE_LAWS.iter());
    laws.extend(ARTS_LAWS.iter());
    laws.extend(TRADITION_LAWS.iter());
    laws.extend(MEDIA_LAWS.iter());
    laws
}