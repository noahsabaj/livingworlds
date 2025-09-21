//! Geographic feature name generation
//!
//! This module handles name generation for natural geographic features
//! including rivers, mountains, oceans, deserts, and forests.

use super::core::NameGenerator;

/// Generate a river name with appropriate suffixes
pub fn generate_river_name(generator: &mut NameGenerator) -> String {
    use super::data::*;
    let root = generator.random_choice(RIVER_ROOTS);
    let suffix = generator.random_choice(&["", " River", " Stream", " Creek", " Rapids", " Falls"]);
    format!("{}{}", root, suffix)
}

/// Generate a mountain name with appropriate suffixes
pub fn generate_mountain_name(generator: &mut NameGenerator) -> String {
    use super::data::*;
    let root = generator.random_choice(MOUNTAIN_ROOTS);
    let suffix =
        generator.random_choice(&["", " Peak", " Mountain", " Ridge", " Summit", " Spire"]);
    format!("{}{}", root, suffix)
}

/// Generate an ocean name with appropriate suffixes
pub fn generate_ocean_name(generator: &mut NameGenerator) -> String {
    use super::data::*;
    let root = generator.random_choice(OCEAN_ROOTS);
    let suffix = generator.random_choice(&[" Ocean", " Sea", " Waters", " Depths", " Expanse"]);
    format!("{}{}", root, suffix)
}

/// Generate a desert name with appropriate suffixes
pub fn generate_desert_name(generator: &mut NameGenerator) -> String {
    use super::data::*;
    let root = generator.random_choice(DESERT_ROOTS);
    let suffix = generator.random_choice(&[" Desert", " Wastes", " Sands", " Barrens", " Expanse"]);
    format!("{}{}", root, suffix)
}

/// Generate a forest name with appropriate suffixes
pub fn generate_forest_name(generator: &mut NameGenerator) -> String {
    use super::data::*;
    let root = generator.random_choice(FOREST_ROOTS);
    let suffix = generator.random_choice(&[" Forest", " Woods", " Grove", " Wildwood", " Thicket"]);
    format!("{}{}", root, suffix)
}
