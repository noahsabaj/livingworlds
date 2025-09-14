//! Core name generator implementation
//!
//! This module contains the main NameGenerator struct and all the generation
//! logic that combines data from various sources to create names.

use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashSet;

// Import through parent module gateway
use super::types::*;
use super::utils::*;
use super::data;

/// Main name generator with optional deterministic seeding
#[derive(Resource, Clone)]
pub struct NameGenerator {
    rng: Option<StdRng>,
    used_names: HashSet<String>,
}

impl Default for NameGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl NameGenerator {
    pub fn new() -> Self {
        Self {
            rng: None,
            used_names: HashSet::new(),
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: Some(StdRng::seed_from_u64(seed)),
            used_names: HashSet::new(),
        }
    }

    /// Clear the used names cache
    pub fn clear_cache(&mut self) {
        self.used_names.clear();
    }

    pub fn names_generated(&self) -> usize {
        self.used_names.len()
    }

    pub fn generate(&mut self, name_type: NameType) -> String {
        let name = match name_type {
            NameType::World => self.generate_world_name(),
            NameType::Nation { culture } => self.generate_nation_name(culture),
            NameType::Province { region, culture } => self.generate_province_name(region, culture),
            NameType::City { size, culture } => self.generate_city_name(size, culture),
            NameType::Person { gender, culture, role } => self.generate_person_name(gender, culture, role),
            NameType::River => self.generate_river_name(),
            NameType::Mountain => self.generate_mountain_name(),
            NameType::Ocean => self.generate_ocean_name(),
            NameType::Desert => self.generate_desert_name(),
            NameType::Forest => self.generate_forest_name(),
        };

        // Ensure uniqueness by appending Roman numerals if needed
        self.ensure_unique(name)
    }

    pub fn generate_related_name(&mut self, parent_name: &str, relation: NameRelation) -> String {
        let name = match relation {
            NameRelation::NewSettlement => format!("New {}", parent_name),
            NameRelation::OldSettlement => format!("Old {}", parent_name),
            NameRelation::ChildCity => {
                let suffixes = ["ton", "ville", "burg", "shire", "ford", "haven", "bridge"];
                let suffix = self.random_choice(&suffixes);
                format!("{}{}", parent_name, suffix)
            },
            NameRelation::TwinCity => {
                let prefixes = ["North", "South", "East", "West", "Upper", "Lower", "Greater", "Lesser"];
                let prefix = self.random_choice(&prefixes);
                format!("{} {}", prefix, parent_name)
            },
            NameRelation::RivalCity => {
                let prefixes = ["Fort", "Port", "Mount", "Saint", "New", "Royal"];
                let prefix = self.random_choice(&prefixes);
                format!("{} {}", prefix, parent_name)
            },
        };

        self.ensure_unique(name)
    }

    // ========================================================================
    // ========================================================================

    fn generate_world_name(&mut self) -> String {
        use super::data::*;

        let pattern = self.random_range(0, 6);
        match pattern {
            0 => {
                // Prefix + Root + Suffix pattern
                let prefix = self.random_choice(PREFIXES);
                let root = self.random_choice(ROOTS);
                let suffix = self.random_choice(SUFFIXES);
                join_name_parts(&[prefix, root, suffix])
            },
            1 => {
                // Single epic name
                self.random_choice(EPIC_NAMES).to_string()
            },
            2 => {
                // The + Adjective + Noun pattern
                let adjective = self.random_choice(ADJECTIVES);
                let noun = self.random_choice(NOUNS);
                format!("The {} {}", adjective, noun)
            },
            3 => {
                // Root + numeric designation
                let root = self.random_choice(ROOTS);
                let number = self.random_range(1, 13) as u32;
                format!("{} {}", root, to_roman_numeral(number))
            },
            4 => {
                // Compound name (two roots)
                let first = self.random_choice(ROOTS);
                let second = self.random_choice(ROOTS);
                if first == second {
                    format!("{} Prime", first)
                } else {
                    format!("{}{}", first, second.to_lowercase())
                }
            },
            _ => {
                // Just a root with optional prefix
                let root = self.random_choice(ROOTS);
                if self.random_bool() {
                    let prefix = self.random_choice(PREFIXES);
                    format!("{} {}", prefix, root)
                } else {
                    root.to_string()
                }
            },
        }
    }

    // ========================================================================
    // ========================================================================

    fn generate_nation_name(&mut self, culture: Culture) -> String {
        match culture {
            Culture::Western => self.generate_western_nation(),
            Culture::Eastern => self.generate_eastern_nation(),
            Culture::Northern => self.generate_northern_nation(),
            Culture::Southern => self.generate_southern_nation(),
            Culture::Desert => self.generate_desert_nation(),
            Culture::Island => self.generate_island_nation(),
            Culture::Ancient => self.generate_ancient_nation(),
            Culture::Mystical => self.generate_mystical_nation(),
        }
    }

    fn generate_western_nation(&mut self) -> String {
        use super::data::cultures::western_data::*;
        let patterns = [
            "Kingdom of {}", "{} Empire", "Republic of {}", "{} Federation",
            "Commonwealth of {}", "{} Union", "Principality of {}", "{} Dominion",
            "Duchy of {}", "{} Confederation", "Free State of {}", "{} Alliance",
        ];
        let pattern = self.random_choice(&patterns);
        let root = self.random_choice(NATION_ROOTS);
        pattern.replace("{}", root)
    }

    fn generate_eastern_nation(&mut self) -> String {
        use super::data::cultures::eastern_data::*;
        let dynasty = self.random_choice(DYNASTY_NAMES);
        let patterns = [
            "{} Dynasty", "{} Shogunate", "Empire of the {} Sun",
            "{} Kingdom", "{} Celestial Empire", "{} Heavenly Kingdom",
            "Divine {} Empire", "{} Sacred Realm",
        ];
        let pattern = self.random_choice(&patterns);
        pattern.replace("{}", dynasty)
    }

    fn generate_northern_nation(&mut self) -> String {
        use super::data::cultures::northern_data::*;
        let clan = self.random_choice(CLAN_NAMES);
        let patterns = [
            "{} Clans", "Tribes of {}", "{} Jarldom", "{} Federation",
            "United {}", "{} Confederation", "Great {}", "{} Horde",
        ];
        let pattern = self.random_choice(&patterns);
        pattern.replace("{}", clan)
    }

    fn generate_southern_nation(&mut self) -> String {
        use super::data::cultures::southern_data::*;
        let root = self.random_choice(NATION_ROOTS);
        let patterns = [
            "{} Republic", "{} Confederation", "Free Cities of {}",
            "{} League", "{} Sultanate", "{} City-States",
            "Merchant Republic of {}", "{} Trade Federation",
        ];
        let pattern = self.random_choice(&patterns);
        pattern.replace("{}", root)
    }

    fn generate_desert_nation(&mut self) -> String {
        use super::data::cultures::desert_data::*;
        let root = self.random_choice(NATION_ROOTS);
        let patterns = [
            "{} Caliphate", "Emirate of {}", "{} Sultanate",
            "{} Khanate", "Tribes of {}", "{} Oasis Federation",
            "Great {} Desert", "{} Nomad Confederation",
        ];
        let pattern = self.random_choice(&patterns);
        pattern.replace("{}", root)
    }

    fn generate_island_nation(&mut self) -> String {
        use super::data::cultures::island_data::*;
        let root = self.random_choice(NATION_ROOTS);
        let patterns = [
            "{} Islands", "Kingdom of {}", "{} Confederation",
            "United Isles of {}", "{} Archipelago", "{} Island Federation",
            "Maritime Republic of {}", "{} Ocean Empire",
        ];
        let pattern = self.random_choice(&patterns);
        pattern.replace("{}", root)
    }

    fn generate_ancient_nation(&mut self) -> String {
        use super::data::cultures::ancient_data::*;
        let root = self.random_choice(NATION_ROOTS);
        let patterns = [
            "Empire of {}", "{} Hegemony", "{} Dominion",
            "The {} Imperium", "Realm of {}", "Ancient {}",
            "Lost Kingdom of {}", "Eternal {} Empire",
        ];
        let pattern = self.random_choice(&patterns);
        pattern.replace("{}", root)
    }

    fn generate_mystical_nation(&mut self) -> String {
        use super::data::cultures::mystical_data::*;
        let root = self.random_choice(NATION_ROOTS);
        let patterns = [
            "{} Covenant", "Circle of {}", "{} Conclave",
            "Order of {}", "{} Sanctum", "Arcane {} Empire",
            "Mystical Realm of {}", "{} Magocracy",
        ];
        let pattern = self.random_choice(&patterns);
        pattern.replace("{}", root)
    }

    // ========================================================================
    // PROVINCE & CITY NAME GENERATION
    // ========================================================================

    fn generate_province_name(&mut self, region: Region, culture: Culture) -> String {
        let cultural_style = self.get_cultural_place_style(culture);
        let geographical_prefix = match region {
            Region::Coastal => self.random_choice(&["Port", "Bay", "Cape", "Harbor", "Coast"]),
            Region::Mountain => self.random_choice(&["Mount", "Peak", "Highland", "Ridge", "Summit"]),
            Region::Desert => self.random_choice(&["Oasis", "Dune", "Sand", "Mirage", "Dust"]),
            Region::Forest => self.random_choice(&["Wood", "Grove", "Timber", "Sylvan", "Green"]),
            Region::Plains => self.random_choice(&["Field", "Prairie", "Meadow", "Steppe", "Grass"]),
            Region::River => self.random_choice(&["River", "Ford", "Bridge", "Delta", "Banks"]),
            Region::Arctic => self.random_choice(&["Frost", "Ice", "Snow", "Glacier", "White"]),
            Region::Tropical => self.random_choice(&["Palm", "Jungle", "Tropic", "Rain", "Monsoon"]),
            Region::Valley => self.random_choice(&["Vale", "Valley", "Glen", "Hollow", "Dell"]),
            Region::Island => self.random_choice(&["Isle", "Atoll", "Key", "Cay", "Rock"]),
        };

        format!("{} {}", geographical_prefix, cultural_style)
    }

    fn generate_city_name(&mut self, size: CitySize, culture: Culture) -> String {
        let base_name = self.get_cultural_place_style(culture);

        match size {
            CitySize::Hamlet => {
                let suffix = self.random_choice(&["stead", "thorpe", "ham", "cot", "ton"]);
                format!("{}{}", base_name, suffix)
            },
            CitySize::Village => base_name,
            CitySize::Town => {
                if self.random_bool() {
                    let prefix = self.random_choice(&["Market ", "Old ", "New "]);
                    format!("{}{}", prefix, base_name)
                } else {
                    base_name
                }
            },
            CitySize::City => {
                if self.random_bool() {
                    let suffix = self.random_choice(&[" City", "polis", "grad", "burg"]);
                    format!("{}{}", base_name, suffix)
                } else {
                    base_name
                }
            },
            CitySize::Metropolis => {
                let prefix = self.random_choice(&["Great ", "Grand ", "Imperial ", "Royal "]);
                format!("{}{}", prefix, base_name)
            },
        }
    }

    fn get_cultural_place_style(&mut self, culture: Culture) -> String {
        match culture {
            Culture::Western => {
                use super::data::cultures::western_data::*;
                let prefix = self.random_choice(PLACE_PREFIXES);
                let suffix = self.random_choice(PLACE_SUFFIXES);
                format!("{}{}", capitalize_first(prefix), suffix)
            },
            Culture::Eastern => {
                use super::data::cultures::eastern_data::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
            Culture::Northern => {
                use super::data::cultures::northern_data::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
            Culture::Southern => {
                use super::data::cultures::southern_data::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
            Culture::Desert => {
                use super::data::cultures::desert_data::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
            Culture::Island => {
                use super::data::cultures::island_data::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
            Culture::Ancient => {
                use super::data::cultures::ancient_data::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
            Culture::Mystical => {
                use super::data::cultures::mystical_data::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
        }
    }

    // ========================================================================
    // ========================================================================

    fn generate_person_name(&mut self, gender: Gender, culture: Culture, role: PersonRole) -> String {
        let (first_name, surname) = self.get_cultural_person_name(gender, culture);
        let title = self.get_role_title(role, gender, culture);

        if title.is_empty() {
            format!("{} {}", first_name, surname)
        } else {
            format!("{} {} {}", title, first_name, surname)
        }
    }

    fn get_cultural_person_name(&mut self, gender: Gender, culture: Culture) -> (String, String) {
        match culture {
            Culture::Western => {
                use super::data::cultures::western_data::*;
                let first = match gender {
                    Gender::Male => self.random_choice(MALE_NAMES),
                    Gender::Female => self.random_choice(FEMALE_NAMES),
                    Gender::Neutral => self.random_choice(NEUTRAL_NAMES),
                };
                let surname = self.random_choice(SURNAMES);
                (first.to_string(), surname.to_string())
            },
            Culture::Eastern => {
                use super::data::cultures::eastern_data::*;
                let first = match gender {
                    Gender::Male => self.random_choice(MALE_NAMES),
                    Gender::Female => self.random_choice(FEMALE_NAMES),
                    Gender::Neutral => self.random_choice(NEUTRAL_NAMES),
                };
                let surname = self.random_choice(SURNAMES);
                (first.to_string(), surname.to_string())
            },
            Culture::Northern => {
                use super::data::cultures::northern_data::*;
                let first = match gender {
                    Gender::Male => self.random_choice(MALE_NAMES),
                    Gender::Female => self.random_choice(FEMALE_NAMES),
                    Gender::Neutral => self.random_choice(NEUTRAL_NAMES),
                };
                // Northern cultures often use clan names as surnames
                let surname = self.random_choice(CLAN_NAMES);
                (first.to_string(), surname.to_string())
            },
            Culture::Southern => {
                use super::data::cultures::southern_data::*;
                let first = match gender {
                    Gender::Male => self.random_choice(MALE_NAMES),
                    Gender::Female => self.random_choice(FEMALE_NAMES),
                    Gender::Neutral => self.random_choice(NEUTRAL_NAMES),
                };
                let surname = self.random_choice(SURNAMES);
                (first.to_string(), surname.to_string())
            },
            Culture::Desert => {
                use super::data::cultures::desert_data::*;
                let first = match gender {
                    Gender::Male => self.random_choice(MALE_NAMES),
                    Gender::Female => self.random_choice(FEMALE_NAMES),
                    Gender::Neutral => self.random_choice(NEUTRAL_NAMES),
                };
                let surname = self.random_choice(SURNAMES);
                (first.to_string(), surname.to_string())
            },
            Culture::Island => {
                use super::data::cultures::island_data::*;
                let first = match gender {
                    Gender::Male => self.random_choice(MALE_NAMES),
                    Gender::Female => self.random_choice(FEMALE_NAMES),
                    Gender::Neutral => self.random_choice(NEUTRAL_NAMES),
                };
                let surname = self.random_choice(CLAN_NAMES);
                (first.to_string(), surname.to_string())
            },
            Culture::Ancient => {
                use super::data::cultures::ancient_data::*;
                let first = match gender {
                    Gender::Male => self.random_choice(MALE_NAMES),
                    Gender::Female => self.random_choice(FEMALE_NAMES),
                    Gender::Neutral => self.random_choice(NEUTRAL_NAMES),
                };
                // Ancient cultures often use dynasty names
                let surname = self.random_choice(DYNASTY_NAMES);
                (first.to_string(), surname.to_string())
            },
            Culture::Mystical => {
                use super::data::cultures::mystical_data::*;
                let first = match gender {
                    Gender::Male => self.random_choice(MALE_NAMES),
                    Gender::Female => self.random_choice(FEMALE_NAMES),
                    Gender::Neutral => self.random_choice(NEUTRAL_NAMES),
                };
                // Mystical cultures use order names as surnames
                let surname = self.random_choice(ORDER_NAMES);
                (first.to_string(), surname.to_string())
            },
        }
    }

    fn get_role_title(&self, role: PersonRole, gender: Gender, culture: Culture) -> String {
        match role {
            PersonRole::Ruler => match (culture, gender) {
                (Culture::Western, Gender::Male) => "King",
                (Culture::Western, Gender::Female) => "Queen",
                (Culture::Eastern, Gender::Male) => "Emperor",
                (Culture::Eastern, Gender::Female) => "Empress",
                (Culture::Northern, _) => "Jarl",
                (Culture::Southern, _) => "Consul",
                (Culture::Desert, Gender::Male) => "Sultan",
                (Culture::Desert, Gender::Female) => "Sultana",
                (Culture::Island, _) => "Chief",
                (Culture::Ancient, _) => "Pharaoh",
                (Culture::Mystical, _) => "Archmage",
                _ => "Sovereign",
            },
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
        }.to_string()
    }

    // ========================================================================
    // ========================================================================

    fn generate_river_name(&mut self) -> String {
        use super::data::*;
        let root = self.random_choice(RIVER_ROOTS);
        let suffix = self.random_choice(&["", " River", " Stream", " Creek", " Rapids", " Falls"]);
        format!("{}{}", root, suffix)
    }

    fn generate_mountain_name(&mut self) -> String {
        use super::data::*;
        let root = self.random_choice(MOUNTAIN_ROOTS);
        let suffix = self.random_choice(&["", " Peak", " Mountain", " Ridge", " Summit", " Spire"]);
        format!("{}{}", root, suffix)
    }

    fn generate_ocean_name(&mut self) -> String {
        use super::data::*;
        let root = self.random_choice(OCEAN_ROOTS);
        let suffix = self.random_choice(&[" Ocean", " Sea", " Waters", " Depths", " Expanse"]);
        format!("{}{}", root, suffix)
    }

    fn generate_desert_name(&mut self) -> String {
        use super::data::*;
        let root = self.random_choice(DESERT_ROOTS);
        let suffix = self.random_choice(&[" Desert", " Wastes", " Sands", " Barrens", " Expanse"]);
        format!("{}{}", root, suffix)
    }

    fn generate_forest_name(&mut self) -> String {
        use super::data::*;
        let root = self.random_choice(FOREST_ROOTS);
        let suffix = self.random_choice(&[" Forest", " Woods", " Grove", " Wildwood", " Thicket"]);
        format!("{}{}", root, suffix)
    }

    // ========================================================================
    // ========================================================================

    /// Ensure a name is unique by appending Roman numerals if necessary
    fn ensure_unique(&mut self, name: String) -> String {
        if !self.used_names.contains(&name) {
            self.used_names.insert(name.clone());
            return name;
        }

        // Name already exists, append Roman numeral
        let mut counter = 2;
        loop {
            let unique_name = format!("{} {}", name, to_roman_numeral(counter));
            if !self.used_names.contains(&unique_name) {
                self.used_names.insert(unique_name.clone());
                return unique_name;
            }
            counter += 1;
            if counter > 50 {
                // Fallback to regular numbers if we exceed Roman numeral range
                let unique_name = format!("{} {}", name, counter);
                self.used_names.insert(unique_name.clone());
                return unique_name;
            }
        }
    }

    /// Random choice from a slice
    fn random_choice<'a, T>(&mut self, choices: &'a [T]) -> &'a T {
        let index = self.random_range(0, choices.len());
        &choices[index]
    }

    /// Generate random number in range [min, max)
    fn random_range(&mut self, min: usize, max: usize) -> usize {
        if let Some(ref mut rng) = self.rng {
            rng.gen_range(min..max)
        } else {
            rand::thread_rng().gen_range(min..max)
        }
    }

    /// Generate random boolean
    fn random_bool(&mut self) -> bool {
        if let Some(ref mut rng) = self.rng {
            rng.gen_bool(0.5)
        } else {
            rand::thread_rng().gen_bool(0.5)
        }
    }
}