//! Law definitions gateway
//!
//! Organizes all law definitions into category subfolders,
//! providing clean access to the hundreds of laws in the system.

// Private category modules - gateway architecture
mod economic;
mod military;
mod social;
// TODO: Implement the following categories
// mod religious;
// mod criminal;
// mod property;
// mod immigration;
// mod environmental;
// mod technology;
// mod cultural;
// mod administrative;
// mod diplomatic;

use crate::nations::laws::types::{Law, LawId, LawCategory};

// Re-export all law collections from new modules
pub use economic::ECONOMIC_LAWS;
pub use military::MILITARY_LAWS;
pub use social::SOCIAL_LAWS;

/// Get all laws across all categories
pub fn get_all_laws() -> Vec<&'static Law> {
    let mut laws = Vec::new();
    laws.extend(ECONOMIC_LAWS.iter());
    laws.extend(MILITARY_LAWS.iter());
    laws.extend(SOCIAL_LAWS.iter());
    // TODO: Add other categories as they're implemented
    laws
}

/// Get all laws in a specific category
pub fn get_category_laws(category: LawCategory) -> Vec<&'static Law> {
    match category {
        LawCategory::Economic => ECONOMIC_LAWS.iter().collect(),
        LawCategory::Military => MILITARY_LAWS.iter().collect(),
        LawCategory::Social => SOCIAL_LAWS.iter().collect(),
        // TODO: Implement other categories
        _ => Vec::new(),
    }
}

/// Get a specific law by ID
pub fn get_law_by_id(id: LawId) -> Option<&'static Law> {
    get_all_laws().into_iter().find(|law| law.id == id)
}

// Law definitions are now properly modularized in this directory
// The old categories.rs file can be safely removed