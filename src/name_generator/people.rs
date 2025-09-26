//! Person name generation with cultural names and role-based titles
//!
//! This module handles generating person names that combine culturally
//! appropriate first and last names with role-based titles.

use super::core::NameGenerator;
use super::types::*;

/// Generate a person name with cultural names and role-based title
pub fn generate_person_name(
    generator: &mut NameGenerator,
    gender: Gender,
    culture: Culture,
    role: PersonRole,
) -> String {
    let (first_name, surname) = get_cultural_person_name(generator, gender, culture);
    let title = get_role_title(role, gender, culture);

    if title.is_empty() {
        format!("{} {}", first_name, surname)
    } else {
        format!("{} {} {}", title, first_name, surname)
    }
}

/// Get culturally appropriate first and last names
pub fn get_cultural_person_name(
    generator: &mut NameGenerator,
    gender: Gender,
    culture: Culture,
) -> (String, String) {
    match culture {
        Culture::Western => {
            use super::data::western_data::*;
            let first = match gender {
                Gender::Male => generator.random_choice(MALE_NAMES),
                Gender::Female => generator.random_choice(FEMALE_NAMES),
                Gender::Neutral => generator.random_choice(NEUTRAL_NAMES),
            };
            let surname = generator.random_choice(SURNAMES);
            (first.to_string(), surname.to_string())
        }
        Culture::Eastern => {
            use super::data::eastern_data::*;
            let first = match gender {
                Gender::Male => generator.random_choice(MALE_NAMES),
                Gender::Female => generator.random_choice(FEMALE_NAMES),
                Gender::Neutral => generator.random_choice(NEUTRAL_NAMES),
            };
            let surname = generator.random_choice(SURNAMES);
            (first.to_string(), surname.to_string())
        }
        Culture::Northern => {
            use super::data::northern_data::*;
            let first = match gender {
                Gender::Male => generator.random_choice(MALE_NAMES),
                Gender::Female => generator.random_choice(FEMALE_NAMES),
                Gender::Neutral => generator.random_choice(NEUTRAL_NAMES),
            };
            // Northern cultures often use clan names as surnames
            let surname = generator.random_choice(CLAN_NAMES);
            (first.to_string(), surname.to_string())
        }
        Culture::Southern => {
            use super::data::southern_data::*;
            let first = match gender {
                Gender::Male => generator.random_choice(MALE_NAMES),
                Gender::Female => generator.random_choice(FEMALE_NAMES),
                Gender::Neutral => generator.random_choice(NEUTRAL_NAMES),
            };
            let surname = generator.random_choice(SURNAMES);
            (first.to_string(), surname.to_string())
        }
        Culture::Desert => {
            use super::data::desert_data::*;
            let first = match gender {
                Gender::Male => generator.random_choice(MALE_NAMES),
                Gender::Female => generator.random_choice(FEMALE_NAMES),
                Gender::Neutral => generator.random_choice(NEUTRAL_NAMES),
            };
            let surname = generator.random_choice(SURNAMES);
            (first.to_string(), surname.to_string())
        }
        Culture::Island => {
            use super::data::island_data::*;
            let first = match gender {
                Gender::Male => generator.random_choice(MALE_NAMES),
                Gender::Female => generator.random_choice(FEMALE_NAMES),
                Gender::Neutral => generator.random_choice(NEUTRAL_NAMES),
            };
            let surname = generator.random_choice(CLAN_NAMES);
            (first.to_string(), surname.to_string())
        }
        Culture::Ancient => {
            use super::data::ancient_data::*;
            let first = match gender {
                Gender::Male => generator.random_choice(MALE_NAMES),
                Gender::Female => generator.random_choice(FEMALE_NAMES),
                Gender::Neutral => generator.random_choice(NEUTRAL_NAMES),
            };
            // Ancient cultures often use dynasty names
            let surname = generator.random_choice(DYNASTY_NAMES);
            (first.to_string(), surname.to_string())
        }
        Culture::Mystical => {
            use super::data::mystical_data::*;
            let first = match gender {
                Gender::Male => generator.random_choice(MALE_NAMES),
                Gender::Female => generator.random_choice(FEMALE_NAMES),
                Gender::Neutral => generator.random_choice(NEUTRAL_NAMES),
            };
            // Mystical cultures use order names as surnames
            let surname = generator.random_choice(ORDER_NAMES);
            (first.to_string(), surname.to_string())
        }
    }
}

/// Get role-based title that varies by culture and gender
///
/// IMPORTANT: PersonRole::Ruler will panic - use governance system instead
pub fn get_role_title(role: PersonRole, gender: Gender, culture: Culture) -> String {
    match role {
        PersonRole::Ruler => {
            panic!(
                "get_role_title() called with PersonRole::Ruler - \
                use governance::get_ruler_title() instead. \
                Ruler titles are based on government type, not culture."
            );
        }
        PersonRole::General => match culture {
            Culture::Eastern => "Shogun",
            Culture::Northern => "Warlord",
            Culture::Desert => "Commander",
            _ => "General",
        },
        PersonRole::Diplomat => match culture {
            Culture::Eastern => "Emissary",
            Culture::Desert => "Vizier",
            _ => "Ambassador",
        },
        PersonRole::Merchant => match culture {
            Culture::Desert => "Trader",
            Culture::Island => "Captain",
            _ => "Merchant",
        },
        PersonRole::Scholar => match culture {
            Culture::Eastern => "Sage",
            Culture::Desert => "Scholar",
            Culture::Mystical => "Lorekeeper",
            _ => "Scholar",
        },
        PersonRole::Priest => match (culture, gender) {
            (Culture::Western, Gender::Female) => "Priestess",
            (Culture::Eastern, _) => "Monk",
            (Culture::Northern, _) => "Shaman",
            (Culture::Desert, _) => "Imam",
            (Culture::Mystical, _) => "Oracle",
            _ => "Priest",
        },
        PersonRole::Explorer => match culture {
            Culture::Island => "Navigator",
            Culture::Northern => "Pathfinder",
            _ => "Explorer",
        },
        PersonRole::Commoner => "",
    }
    .to_string()
}
