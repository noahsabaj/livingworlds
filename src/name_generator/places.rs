//! Place name generation for provinces and cities
//!
//! This module handles generating names for settlements and geographic places,
//! combining regional characteristics with cultural naming styles.

use super::core::NameGenerator;
use super::types::*;
use super::utils::*;

/// Generate a province name combining geographic and cultural elements
pub fn generate_province_name(
    generator: &mut NameGenerator,
    region: Region,
    culture: Culture,
) -> String {
    let cultural_style = get_cultural_place_style(generator, culture);
    let geographical_prefix = match region {
        Region::Coastal => generator.random_choice(&["Port", "Bay", "Cape", "Harbor", "Coast"]),
        Region::Mountain => {
            generator.random_choice(&["Mount", "Peak", "Highland", "Ridge", "Summit"])
        }
        Region::Desert => generator.random_choice(&["Oasis", "Dune", "Sand", "Mirage", "Dust"]),
        Region::Forest => generator.random_choice(&["Wood", "Grove", "Timber", "Sylvan", "Green"]),
        Region::Plains => {
            generator.random_choice(&["Field", "Prairie", "Meadow", "Steppe", "Grass"])
        }
        Region::River => generator.random_choice(&["River", "Ford", "Bridge", "Delta", "Banks"]),
        Region::Arctic => generator.random_choice(&["Frost", "Ice", "Snow", "Glacier", "White"]),
        Region::Tropical => {
            generator.random_choice(&["Palm", "Jungle", "Tropic", "Rain", "Monsoon"])
        }
        Region::Valley => generator.random_choice(&["Vale", "Valley", "Glen", "Hollow", "Dell"]),
        Region::Island => generator.random_choice(&["Isle", "Atoll", "Key", "Cay", "Rock"]),
    };

    format!("{} {}", geographical_prefix, cultural_style)
}

/// Generate a city name with size-appropriate modifiers
pub fn generate_city_name(
    generator: &mut NameGenerator,
    size: CitySize,
    culture: Culture,
) -> String {
    let base_name = get_cultural_place_style(generator, culture);

    match size {
        CitySize::Hamlet => {
            let suffix = generator.random_choice(&["stead", "thorpe", "ham", "cot", "ton"]);
            format!("{}{}", base_name, suffix)
        }
        CitySize::Village => base_name,
        CitySize::Town => {
            if generator.random_bool() {
                let prefix = generator.random_choice(&["Market ", "Old ", "New "]);
                format!("{}{}", prefix, base_name)
            } else {
                base_name
            }
        }
        CitySize::City => {
            if generator.random_bool() {
                let suffix = generator.random_choice(&[" City", "polis", "grad", "burg"]);
                format!("{}{}", base_name, suffix)
            } else {
                base_name
            }
        }
        CitySize::Metropolis => {
            let prefix = generator.random_choice(&["Great ", "Grand ", "Imperial ", "Royal "]);
            format!("{}{}", prefix, base_name)
        }
    }
}

/// Get culturally appropriate place name style
pub fn get_cultural_place_style(generator: &mut NameGenerator, culture: Culture) -> String {
    match culture {
        Culture::Western => {
            use super::data::western_data::*;
            let prefix = generator.random_choice(PLACE_PREFIXES);
            let suffix = generator.random_choice(PLACE_SUFFIXES);
            format!("{}{}", capitalize_first(prefix), suffix)
        }
        Culture::Eastern => {
            use super::data::eastern_data::*;
            generator.random_choice(PLACE_NAMES).to_string()
        }
        Culture::Northern => {
            use super::data::northern_data::*;
            generator.random_choice(PLACE_NAMES).to_string()
        }
        Culture::Southern => {
            use super::data::southern_data::*;
            generator.random_choice(PLACE_NAMES).to_string()
        }
        Culture::Desert => {
            use super::data::desert_data::*;
            generator.random_choice(PLACE_NAMES).to_string()
        }
        Culture::Island => {
            use super::data::island_data::*;
            generator.random_choice(PLACE_NAMES).to_string()
        }
        Culture::Ancient => {
            use super::data::ancient_data::*;
            generator.random_choice(PLACE_NAMES).to_string()
        }
        Culture::Mystical => {
            use super::data::mystical_data::*;
            generator.random_choice(PLACE_NAMES).to_string()
        }
    }
}
