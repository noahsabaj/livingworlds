//! Law definitions gateway
//!
//! Organizes all law definitions into category subfolders,
//! providing clean access to the hundreds of laws in the system.

// Private category modules - gateway architecture
mod economic;
mod military;
mod social;
mod religious;
mod criminal;
mod property;
mod immigration;
mod environmental;
mod technology;
mod cultural;
mod administrative;
mod diplomatic;

use crate::nations::laws::types::{Law, LawId, LawCategory};

// Re-export all law collections
pub use economic::get_all_economic_laws;
pub use military::get_all_military_laws;
pub use social::get_all_social_laws;
pub use religious::get_all_religious_laws;
pub use criminal::get_all_criminal_laws;
pub use property::get_all_property_laws;
pub use immigration::get_all_immigration_laws;
pub use environmental::get_all_environmental_laws;
pub use technology::get_all_technology_laws;
pub use cultural::get_all_cultural_laws;
pub use administrative::get_all_administrative_laws;
pub use diplomatic::get_all_diplomatic_laws;

/// Get all laws across all categories
pub fn get_all_laws() -> Vec<&'static Law> {
    let mut laws = Vec::new();
    laws.extend(get_all_economic_laws());
    laws.extend(get_all_military_laws());
    laws.extend(get_all_social_laws());
    laws.extend(get_all_religious_laws());
    laws.extend(get_all_criminal_laws());
    laws.extend(get_all_property_laws());
    laws.extend(get_all_immigration_laws());
    laws.extend(get_all_environmental_laws());
    laws.extend(get_all_technology_laws());
    laws.extend(get_all_cultural_laws());
    laws.extend(get_all_administrative_laws());
    laws.extend(get_all_diplomatic_laws());
    laws
}

/// Get all laws in a specific category
pub fn get_category_laws(category: LawCategory) -> Vec<&'static Law> {
    match category {
        LawCategory::Economic => get_all_economic_laws(),
        LawCategory::Military => get_all_military_laws(),
        LawCategory::Social => get_all_social_laws(),
        LawCategory::Religious => get_all_religious_laws(),
        LawCategory::Criminal => get_all_criminal_laws(),
        LawCategory::Property => get_all_property_laws(),
        LawCategory::Immigration => get_all_immigration_laws(),
        LawCategory::Environmental => get_all_environmental_laws(),
        LawCategory::Technology => get_all_technology_laws(),
        LawCategory::Cultural => get_all_cultural_laws(),
        LawCategory::Administrative => get_all_administrative_laws(),
        LawCategory::Diplomatic => get_all_diplomatic_laws(),
        _ => Vec::new(),
    }
}

/// Get a specific law by ID
pub fn get_law_by_id(id: LawId) -> Option<&'static Law> {
    get_all_laws().into_iter().find(|law| law.id == id)
}

// Keep the temporary exports from old categories.rs
// TODO: Remove these once fully modularized
pub use super::categories::{ECONOMIC_LAWS, MILITARY_LAWS, SOCIAL_LAWS};