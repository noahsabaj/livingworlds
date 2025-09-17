//! Culture-specific name generation implementation
//!
//! This module contains the actual implementation logic that orchestrates
//! culture-specific generation by delegating to individual culture modules.

use super::super::core::NameGenerator;
use super::super::types::*;

/// Main entry point for nation name generation - delegates to culture modules
pub fn generate_nation_name(generator: &mut NameGenerator, culture: Culture) -> String {
    match culture {
        Culture::Western => super::western::generate_nation_name(generator),
        Culture::Eastern => super::eastern::generate_nation_name(generator),
        Culture::Northern => super::northern::generate_nation_name(generator),
        Culture::Southern => super::southern::generate_nation_name(generator),
        Culture::Desert => super::desert::generate_nation_name(generator),
        Culture::Island => super::island::generate_nation_name(generator),
        Culture::Ancient => super::ancient::generate_nation_name(generator),
        Culture::Mystical => super::mystical::generate_nation_name(generator),
    }
}

/// Main entry point for house name generation - delegates to culture modules
pub fn generate_house_name(generator: &mut NameGenerator, culture: Culture) -> String {
    match culture {
        Culture::Western => super::western::generate_house_name(generator),
        Culture::Eastern => super::eastern::generate_house_name(generator),
        Culture::Northern => super::northern::generate_house_name(generator),
        Culture::Southern => super::southern::generate_house_name(generator),
        Culture::Desert => super::desert::generate_house_name(generator),
        Culture::Island => super::island::generate_house_name(generator),
        Culture::Ancient => super::ancient::generate_house_name(generator),
        Culture::Mystical => super::mystical::generate_house_name(generator),
    }
}