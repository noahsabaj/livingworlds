//! Universal Name Generator System for Living Worlds
//! 
//! This module provides culturally diverse, contextually appropriate names
//! for all entities in the game - from ancient civilizations to individual
//! leaders, from mighty rivers to small villages.

use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashSet;

// ============================================================================
// CORE TYPES
// ============================================================================

/// The type of name to generate, with context-specific parameters
#[derive(Debug, Clone)]
pub enum NameType {
    World,
    Nation { culture: Culture },
    Province { region: Region, culture: Culture },
    City { size: CitySize, culture: Culture },
    Person { gender: Gender, culture: Culture, role: PersonRole },
    River,
    Mountain,
    Ocean,
    Desert,
    Forest,
}

/// Cultural/linguistic styles for name generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Culture {
    Western,      // European-inspired
    Eastern,      // Asian-inspired
    Northern,     // Nordic/Slavic-inspired
    Southern,     // Mediterranean/African-inspired
    Desert,       // Middle Eastern-inspired
    Island,       // Polynesian/Caribbean-inspired
    Ancient,      // Lost civilization style
    Mystical,     // Fantasy/magical names
}

/// Gender for person name generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
    Neutral,  // For cultures or roles without gender distinction
}

/// Role/occupation for person name generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersonRole {
    Ruler,
    General,
    Diplomat,
    Merchant,
    Scholar,
    Priest,
    Explorer,
    Commoner,
}

/// Geographical region types for place names
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region {
    Coastal,
    Mountain,
    Desert,
    Forest,
    Plains,
    River,
    Arctic,
    Tropical,
    Valley,
    Island,
}

/// City size categories affecting name generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitySize {
    Hamlet,     // < 100 people
    Village,    // 100-1000
    Town,       // 1000-10000
    City,       // 10000-100000
    Metropolis, // > 100000
}

/// Relationship between names for derivative generation
#[derive(Debug, Clone, Copy)]
pub enum NameRelation {
    NewSettlement,  // "New X"
    OldSettlement,  // "Old X"
    ChildCity,      // Derived from parent
    TwinCity,       // Sister city
    RivalCity,      // Competing settlement
}

// ============================================================================
// NAME GENERATOR
// ============================================================================

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
    /// Create a new name generator with random seed
    pub fn new() -> Self {
        Self {
            rng: None,
            used_names: HashSet::new(),
        }
    }
    
    /// Create a deterministic name generator with a specific seed
    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: Some(StdRng::seed_from_u64(seed)),
            used_names: HashSet::new(),
        }
    }
    
    /// Generate a name based on the specified type
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
        
        // Ensure uniqueness
        let mut final_name = name.clone();
        let mut counter = 2;
        while self.used_names.contains(&final_name) {
            final_name = format!("{} {}", name, to_roman_numeral(counter));
            counter += 1;
        }
        
        self.used_names.insert(final_name.clone());
        final_name
    }
    
    /// Generate a derivative name based on an existing one
    pub fn generate_related_name(&mut self, parent_name: &str, relation: NameRelation) -> String {
        match relation {
            NameRelation::NewSettlement => format!("New {}", parent_name),
            NameRelation::OldSettlement => format!("Old {}", parent_name),
            NameRelation::ChildCity => {
                let suffixes = ["ton", "ville", "burg", "shire", "ford"];
                let suffix = self.random_choice(&suffixes);
                format!("{}{}", parent_name, suffix)
            },
            NameRelation::TwinCity => {
                let prefixes = ["North", "South", "East", "West", "Upper", "Lower"];
                let prefix = self.random_choice(&prefixes);
                format!("{} {}", prefix, parent_name)
            },
            NameRelation::RivalCity => {
                let prefixes = ["Fort", "Port", "Mount", "Saint"];
                let prefix = self.random_choice(&prefixes);
                format!("{} {}", prefix, parent_name)
            },
        }
    }
    
    // ========================================================================
    // SPECIFIC GENERATORS
    // ========================================================================
    
    /// Generate world names with various patterns
    fn generate_world_name(&mut self) -> String {
        use name_data::world::*;
        
        let pattern = self.random_range(0, 5);
        match pattern {
            0 => {
                // Prefix + Root + Suffix pattern (classic)
                let prefix = self.random_choice(PREFIXES);
                let root = self.random_choice(ROOTS);
                let suffix = self.random_choice(SUFFIXES);
                format!("{} {}{}", prefix, root, suffix).trim().to_string()
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
                let number = self.random_range(1, 13);
                format!("{} {}", root, to_roman_numeral(number as u32))
            },
            _ => {
                // Compound name
                let first = self.random_choice(ROOTS);
                let second = self.random_choice(ROOTS);
                format!("{}{}", first, second.to_lowercase())
            },
        }
    }
    
    /// Generate culturally appropriate nation names
    fn generate_nation_name(&mut self, culture: Culture) -> String {
        match culture {
            Culture::Western => {
                use name_data::western::*;
                let patterns = [
                    "Kingdom of {}",
                    "{} Empire",
                    "Republic of {}",
                    "{} Federation",
                    "Commonwealth of {}",
                    "{} Union",
                    "Principality of {}",
                    "{} Dominion",
                ];
                let pattern = self.random_choice(&patterns);
                let root = self.random_choice(NATION_ROOTS);
                pattern.replace("{}", root)
            },
            Culture::Eastern => {
                use name_data::eastern::*;
                let dynasty = self.random_choice(DYNASTY_NAMES);
                let patterns = [
                    "{} Dynasty",
                    "{} Shogunate",
                    "Empire of the {} Sun",
                    "{} Kingdom",
                    "{} Celestial Empire",
                ];
                let pattern = self.random_choice(&patterns);
                pattern.replace("{}", dynasty)
            },
            Culture::Northern => {
                use name_data::northern::*;
                let clan = self.random_choice(CLAN_NAMES);
                let patterns = [
                    "{} Clans",
                    "Tribes of {}",
                    "{} Jarldom",
                    "{} Federation",
                    "United {}",
                ];
                let pattern = self.random_choice(&patterns);
                pattern.replace("{}", clan)
            },
            Culture::Southern => {
                use name_data::southern::*;
                let root = self.random_choice(NATION_ROOTS);
                let patterns = [
                    "{} Republic",
                    "{} Confederation",
                    "Free Cities of {}",
                    "{} League",
                    "{} Sultanate",
                ];
                let pattern = self.random_choice(&patterns);
                pattern.replace("{}", root)
            },
            Culture::Desert => {
                use name_data::desert::*;
                let root = self.random_choice(NATION_ROOTS);
                let patterns = [
                    "{} Caliphate",
                    "Emirate of {}",
                    "{} Sultanate",
                    "{} Khanate",
                    "Tribes of {}",
                ];
                let pattern = self.random_choice(&patterns);
                pattern.replace("{}", root)
            },
            Culture::Island => {
                use name_data::island::*;
                let root = self.random_choice(NATION_ROOTS);
                let patterns = [
                    "{} Islands",
                    "Kingdom of {}",
                    "{} Confederation",
                    "United Isles of {}",
                    "{} Archipelago",
                ];
                let pattern = self.random_choice(&patterns);
                pattern.replace("{}", root)
            },
            Culture::Ancient => {
                use name_data::ancient::*;
                let root = self.random_choice(NATION_ROOTS);
                let patterns = [
                    "Empire of {}",
                    "{} Hegemony",
                    "{} Dominion",
                    "The {} Imperium",
                    "Realm of {}",
                ];
                let pattern = self.random_choice(&patterns);
                pattern.replace("{}", root)
            },
            Culture::Mystical => {
                use name_data::mystical::*;
                let root = self.random_choice(NATION_ROOTS);
                let patterns = [
                    "{} Covenant",
                    "Circle of {}",
                    "{} Conclave",
                    "Order of {}",
                    "{} Sanctum",
                ];
                let pattern = self.random_choice(&patterns);
                pattern.replace("{}", root)
            },
        }
    }
    
    /// Generate province names based on region and culture
    fn generate_province_name(&mut self, region: Region, culture: Culture) -> String {
        let cultural_style = self.get_cultural_place_style(culture);
        let geographical_prefix = match region {
            Region::Coastal => self.random_choice(&["Port", "Bay", "Cape", "Harbor"]),
            Region::Mountain => self.random_choice(&["Mount", "Peak", "Highland", "Ridge"]),
            Region::Desert => self.random_choice(&["Oasis", "Dune", "Sand", "Mirage"]),
            Region::Forest => self.random_choice(&["Wood", "Grove", "Timber", "Sylvan"]),
            Region::Plains => self.random_choice(&["Field", "Prairie", "Meadow", "Steppe"]),
            Region::River => self.random_choice(&["River", "Ford", "Bridge", "Delta"]),
            Region::Arctic => self.random_choice(&["Frost", "Ice", "Snow", "Glacier"]),
            Region::Tropical => self.random_choice(&["Palm", "Jungle", "Tropic", "Rain"]),
            Region::Valley => self.random_choice(&["Vale", "Valley", "Glen", "Hollow"]),
            Region::Island => self.random_choice(&["Isle", "Atoll", "Key", "Cay"]),
        };
        
        format!("{} {}", geographical_prefix, cultural_style)
    }
    
    /// Generate city names based on size and culture
    fn generate_city_name(&mut self, size: CitySize, culture: Culture) -> String {
        let base_name = self.get_cultural_place_style(culture);
        
        match size {
            CitySize::Hamlet => {
                let suffix = self.random_choice(&["stead", "thorpe", "ham", "cot"]);
                format!("{}{}", base_name, suffix)
            },
            CitySize::Village => base_name,
            CitySize::Town => {
                let prefix = self.random_choice(&["", "Market ", "Old "]);
                format!("{}{}", prefix, base_name)
            },
            CitySize::City => {
                let suffix = self.random_choice(&["", " City", "polis", "grad"]);
                format!("{}{}", base_name, suffix)
            },
            CitySize::Metropolis => {
                let prefix = self.random_choice(&["Great ", "Grand ", "Imperial "]);
                format!("{}{}", prefix, base_name)
            },
        }
    }
    
    /// Generate person names with titles based on role and culture
    fn generate_person_name(&mut self, gender: Gender, culture: Culture, role: PersonRole) -> String {
        let (first_name, surname) = self.get_cultural_person_name(gender, culture);
        let title = self.get_role_title(role, gender, culture);
        
        if title.is_empty() {
            format!("{} {}", first_name, surname)
        } else {
            format!("{} {} {}", title, first_name, surname)
        }
    }
    
    /// Generate river names
    fn generate_river_name(&mut self) -> String {
        use name_data::geographical::*;
        let root = self.random_choice(RIVER_ROOTS);
        let suffix = self.random_choice(&["", " River", " Stream", " Creek", " Rapids"]);
        format!("{}{}", root, suffix)
    }
    
    /// Generate mountain names
    fn generate_mountain_name(&mut self) -> String {
        use name_data::geographical::*;
        let root = self.random_choice(MOUNTAIN_ROOTS);
        let suffix = self.random_choice(&["", " Peak", " Mountain", " Ridge", " Summit"]);
        format!("{}{}", root, suffix)
    }
    
    /// Generate ocean names
    fn generate_ocean_name(&mut self) -> String {
        use name_data::geographical::*;
        let root = self.random_choice(OCEAN_ROOTS);
        let suffix = self.random_choice(&[" Ocean", " Sea", " Waters", " Depths"]);
        format!("{}{}", root, suffix)
    }
    
    /// Generate desert names
    fn generate_desert_name(&mut self) -> String {
        use name_data::geographical::*;
        let root = self.random_choice(DESERT_ROOTS);
        let suffix = self.random_choice(&[" Desert", " Wastes", " Sands", " Barrens"]);
        format!("{}{}", root, suffix)
    }
    
    /// Generate forest names
    fn generate_forest_name(&mut self) -> String {
        use name_data::geographical::*;
        let root = self.random_choice(FOREST_ROOTS);
        let suffix = self.random_choice(&[" Forest", " Woods", " Grove", " Wildwood"]);
        format!("{}{}", root, suffix)
    }
    
    // ========================================================================
    // HELPER METHODS
    // ========================================================================
    
    /// Get a cultural place name style
    fn get_cultural_place_style(&mut self, culture: Culture) -> String {
        match culture {
            Culture::Western => {
                use name_data::western::*;
                let prefix = self.random_choice(PLACE_PREFIXES);
                let suffix = self.random_choice(PLACE_SUFFIXES);
                format!("{}{}", prefix, suffix)
            },
            Culture::Eastern => {
                use name_data::eastern::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
            Culture::Northern => {
                use name_data::northern::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
            Culture::Southern => {
                use name_data::southern::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
            Culture::Desert => {
                use name_data::desert::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
            Culture::Island => {
                use name_data::island::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
            Culture::Ancient => {
                use name_data::ancient::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
            Culture::Mystical => {
                use name_data::mystical::*;
                self.random_choice(PLACE_NAMES).to_string()
            },
        }
    }
    
    /// Get culturally appropriate person names
    fn get_cultural_person_name(&mut self, gender: Gender, culture: Culture) -> (String, String) {
        match culture {
            Culture::Western => {
                use name_data::western::*;
                let first = match gender {
                    Gender::Male => self.random_choice(MALE_NAMES),
                    Gender::Female => self.random_choice(FEMALE_NAMES),
                    Gender::Neutral => self.random_choice(NEUTRAL_NAMES),
                };
                let surname = self.random_choice(SURNAMES);
                (first.to_string(), surname.to_string())
            },
            Culture::Eastern => {
                use name_data::eastern::*;
                let first = match gender {
                    Gender::Male => self.random_choice(MALE_NAMES),
                    Gender::Female => self.random_choice(FEMALE_NAMES),
                    Gender::Neutral => self.random_choice(NEUTRAL_NAMES),
                };
                let surname = self.random_choice(SURNAMES);
                (first.to_string(), surname.to_string())
            },
            // ... similar for other cultures (simplified for brevity)
            _ => ("Unknown".to_string(), "Person".to_string()),
        }
    }
    
    /// Get appropriate title for a role
    fn get_role_title(&mut self, role: PersonRole, gender: Gender, _culture: Culture) -> String {
        match role {
            PersonRole::Ruler => match gender {
                Gender::Male => "King",
                Gender::Female => "Queen",
                Gender::Neutral => "Sovereign",
            },
            PersonRole::General => "General",
            PersonRole::Diplomat => "Ambassador",
            PersonRole::Merchant => "Merchant",
            PersonRole::Scholar => "Scholar",
            PersonRole::Priest => match gender {
                Gender::Male => "Priest",
                Gender::Female => "Priestess",
                Gender::Neutral => "Cleric",
            },
            PersonRole::Explorer => "Explorer",
            PersonRole::Commoner => "",
        }.to_string()
    }
    
    /// Random choice from slice
    fn random_choice<'a, T>(&mut self, choices: &'a [T]) -> &'a T {
        let index = self.random_range(0, choices.len());
        &choices[index]
    }
    
    /// Generate random number in range
    fn random_range(&mut self, min: usize, max: usize) -> usize {
        if let Some(ref mut rng) = self.rng {
            rng.gen_range(min..max)
        } else {
            rand::thread_rng().gen_range(min..max)
        }
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Convert number to Roman numerals (simplified, up to 20)
fn to_roman_numeral(n: u32) -> &'static str {
    match n {
        1 => "I", 2 => "II", 3 => "III", 4 => "IV", 5 => "V",
        6 => "VI", 7 => "VII", 8 => "VIII", 9 => "IX", 10 => "X",
        11 => "XI", 12 => "XII", 13 => "XIII", 14 => "XIV", 15 => "XV",
        16 => "XVI", 17 => "XVII", 18 => "XVIII", 19 => "XIX", 20 => "XX",
        _ => "",
    }
}

// ============================================================================
// NAME DATABASES
// ============================================================================

/// Name data organized by culture and type
mod name_data {
    /// World name components
    pub mod world {
        pub const PREFIXES: &[&str] = &[
            "New", "Ancient", "Lost", "Eternal", "Prime", "Nova", "Neo", "Crystal",
            "Golden", "Silver", "Mystic", "Shadow", "Dawn", "Twilight", "Astral",
            "Forgotten", "Hidden", "Sacred", "Divine", "Cosmic",
        ];
        
        pub const ROOTS: &[&str] = &[
            "Terra", "Gaia", "Eden", "Avalon", "Elysium", "Pangaea", "Atlantis",
            "Aetheria", "Celestia", "Arcadia", "Zephyr", "Olympus", "Valhalla",
            "Midgard", "Asgard", "Nibiru", "Xanadu", "Shangri", "Lemuria",
            "Hyperborea", "Thule", "Mu", "Eldoria", "Mythos", "Cosmos",
        ];
        
        pub const SUFFIXES: &[&str] = &[
            "", " Prime", " Nova", " Alpha", " Beta", " Omega", " Major", " Minor",
            " Reborn", " Ascendant", " Eternal", " Infinite", " Pristine",
        ];
        
        pub const EPIC_NAMES: &[&str] = &[
            "Genesis", "Revelation", "Paradox", "Eternity", "Infinity",
            "Chronos", "Nexus", "Apex", "Zenith", "Odyssey", "Legacy",
        ];
        
        pub const ADJECTIVES: &[&str] = &[
            "Endless", "Verdant", "Crimson", "Azure", "Golden", "Shattered",
            "Frozen", "Burning", "Living", "Dying", "Awakening", "Sleeping",
        ];
        
        pub const NOUNS: &[&str] = &[
            "Realm", "Sphere", "Domain", "Expanse", "Frontier", "Horizon",
            "Sanctuary", "Crucible", "Tapestry", "Symphony", "Echo",
        ];
    }
    
    /// Western culture names (European-inspired)
    pub mod western {
        pub const MALE_NAMES: &[&str] = &[
            "Alexander", "Marcus", "Julius", "Augustus", "Constantine", "Theodore",
            "William", "Richard", "Edward", "Charles", "Frederick", "Leopold",
            "Maximilian", "Sebastian", "Victor", "Arthur", "Roland", "Gregory",
        ];
        
        pub const FEMALE_NAMES: &[&str] = &[
            "Helena", "Victoria", "Isabella", "Catherine", "Elizabeth", "Margaret",
            "Sophia", "Eleanor", "Beatrice", "Anastasia", "Josephine", "Charlotte",
            "Amelia", "Rosalind", "Genevieve", "Matilda", "Adelaide", "Cordelia",
        ];
        
        pub const NEUTRAL_NAMES: &[&str] = &[
            "Morgan", "Alex", "Jordan", "Taylor", "Cameron", "Quinn", "Sage",
        ];
        
        pub const SURNAMES: &[&str] = &[
            "Blackwood", "Ironforge", "Goldstein", "Whitmore", "Greystone",
            "Northwind", "Eastwood", "Westbrook", "Southgate", "Brightblade",
            "Stormborn", "Ravencrest", "Lionheart", "Dragonsbane", "Starweaver",
        ];
        
        pub const PLACE_PREFIXES: &[&str] = &[
            "New", "Old", "North", "South", "East", "West", "Upper", "Lower",
            "Great", "Little", "Saint", "Fort", "Port", "Mount",
        ];
        
        pub const PLACE_SUFFIXES: &[&str] = &[
            "burg", "shire", "ford", "haven", "bridge", "gate", "wick", "thorpe",
            "ton", "ville", "chester", "field", "wood", "moor", "dale", "marsh",
        ];
        
        pub const NATION_ROOTS: &[&str] = &[
            "Britannia", "Germania", "Gallia", "Hispania", "Italia", "Helvetia",
            "Polonia", "Bohemia", "Hungaria", "Romania", "Bulgaria", "Serbia",
        ];
    }
    
    /// Eastern culture names (Asian-inspired)
    pub mod eastern {
        pub const MALE_NAMES: &[&str] = &[
            "Akira", "Chen", "Jin", "Kenji", "Liu", "Ming", "Ryu", "Shin",
            "Tao", "Wei", "Xian", "Yasuo", "Zhang", "Hiroshi", "Kazuki",
        ];
        
        pub const FEMALE_NAMES: &[&str] = &[
            "Mei", "Sakura", "Yuki", "Ling", "Xia", "Feng", "Ai", "Hana",
            "Jade", "Li", "Ming-Yue", "Nami", "Rei", "Suki", "Yui",
        ];
        
        pub const NEUTRAL_NAMES: &[&str] = &[
            "Ren", "Yu", "Aki", "Haru", "Kyo", "Sora", "Yuki",
        ];
        
        pub const SURNAMES: &[&str] = &[
            "Tanaka", "Wang", "Chen", "Li", "Zhang", "Yamamoto", "Suzuki",
            "Kim", "Park", "Lee", "Nakamura", "Fujiwara", "Minamoto",
        ];
        
        pub const DYNASTY_NAMES: &[&str] = &[
            "Ming", "Tang", "Song", "Han", "Jin", "Yuan", "Qing", "Zhou",
            "Yamato", "Heian", "Kamakura", "Ashikaga", "Tokugawa",
        ];
        
        pub const PLACE_NAMES: &[&str] = &[
            "Jade Harbor", "Dragon's Rest", "Phoenix Rise", "Lotus Valley",
            "Bamboo Grove", "Cherry Blossom Hill", "Moon Lake", "Sun Temple",
            "Cloud Mountain", "Silk Road", "Pearl River", "Golden Bridge",
        ];
    }
    
    /// Northern culture names (Nordic/Slavic-inspired)
    pub mod northern {
        pub const CLAN_NAMES: &[&str] = &[
            "Ironwolf", "Frostbeard", "Stormaxe", "Snowbear", "Icefang",
            "Thundershield", "Winterborn", "Ravenclaw", "Wolfsbane", "Dragonmaw",
        ];
        
        pub const PLACE_NAMES: &[&str] = &[
            "Winterhold", "Frostheim", "Snowfall", "Icemark", "Nordheim",
            "Volgagrad", "Novigrad", "Kievsk", "Moscovia", "Petrograd",
        ];
    }
    
    /// Southern culture names (Mediterranean/African-inspired)
    pub mod southern {
        pub const NATION_ROOTS: &[&str] = &[
            "Carthago", "Alexandria", "Memphis", "Thebes", "Athens", "Sparta",
            "Corinth", "Syracuse", "Massalia", "Tarentum", "Cyrene",
        ];
        
        pub const PLACE_NAMES: &[&str] = &[
            "Sol Marina", "Terra Vista", "Monte Alto", "Valle Verde",
            "Porto Bello", "Costa Dorada", "Isla Bonita", "Rio Grande",
        ];
    }
    
    /// Desert culture names (Middle Eastern-inspired)
    pub mod desert {
        pub const NATION_ROOTS: &[&str] = &[
            "Babylon", "Assyria", "Persia", "Arabia", "Mesopotamia",
            "Phoenicia", "Damascus", "Baghdad", "Cairo", "Mecca",
        ];
        
        pub const PLACE_NAMES: &[&str] = &[
            "Oasis Mirage", "Golden Dunes", "Sandstone Keep", "Desert Rose",
            "Scorpion Pass", "Camel's Rest", "Sultan's Pride", "Jewel of the Sands",
        ];
    }
    
    /// Island culture names (Polynesian/Caribbean-inspired)
    pub mod island {
        pub const NATION_ROOTS: &[&str] = &[
            "Moana", "Tiki", "Samoa", "Tahiti", "Fiji", "Maui",
            "Trinidad", "Jamaica", "Bermuda", "Nassau", "Havana",
        ];
        
        pub const PLACE_NAMES: &[&str] = &[
            "Coral Bay", "Pearl Harbor", "Turtle Beach", "Paradise Cove",
            "Sunset Shores", "Palm Grove", "Coconut Island", "Mermaid Lagoon",
        ];
    }
    
    /// Ancient culture names (Lost civilization style)
    pub mod ancient {
        pub const NATION_ROOTS: &[&str] = &[
            "Atlantis", "Lemuria", "Mu", "Hyperborea", "Thule", "Shambhala",
            "El Dorado", "Xibalba", "Agartha", "Avalon", "Lyonesse",
        ];
        
        pub const PLACE_NAMES: &[&str] = &[
            "Temple of Ages", "Eternal City", "Forgotten Realm", "Lost Valley",
            "Ancient Gates", "Timeless Spire", "Crystal Citadel", "Void Sanctum",
        ];
    }
    
    /// Mystical culture names (Fantasy/magical)
    pub mod mystical {
        pub const NATION_ROOTS: &[&str] = &[
            "Arcanum", "Mystara", "Ethereal", "Celestial", "Umbral",
            "Astral", "Elemental", "Primordial", "Transcendent", "Enigma",
        ];
        
        pub const PLACE_NAMES: &[&str] = &[
            "Mage Tower", "Wizard's Peak", "Enchanted Grove", "Mystic Falls",
            "Arcane Nexus", "Spell Forge", "Crystal Caverns", "Ethereal Plains",
        ];
    }
    
    /// Geographical feature names
    pub mod geographical {
        pub const RIVER_ROOTS: &[&str] = &[
            "Silver", "Golden", "Crystal", "Serpent", "Dragon", "Swift",
            "Lazy", "Mighty", "Ancient", "Young", "Wild", "Calm",
        ];
        
        pub const MOUNTAIN_ROOTS: &[&str] = &[
            "Thunder", "Storm", "Cloud", "Sky", "Eagle", "Dragon",
            "Giant", "Titan", "Gods", "Eternal", "Lonely", "Twin",
        ];
        
        pub const OCEAN_ROOTS: &[&str] = &[
            "Endless", "Sapphire", "Emerald", "Crimson", "Azure", "Dark",
            "Peaceful", "Raging", "Silent", "Singing", "Frozen", "Boiling",
        ];
        
        pub const DESERT_ROOTS: &[&str] = &[
            "Scorching", "Endless", "Golden", "Crimson", "Shifting", "Silent",
            "Whispering", "Forgotten", "Cursed", "Blessed", "Ancient", "Young",
        ];
        
        pub const FOREST_ROOTS: &[&str] = &[
            "Dark", "Light", "Ancient", "Young", "Sacred", "Cursed",
            "Whispering", "Silent", "Singing", "Dancing", "Sleeping", "Waking",
        ];
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_world_name_generation() {
        let mut gen = NameGenerator::new();
        for _ in 0..10 {
            let name = gen.generate(NameType::World);
            assert!(!name.is_empty());
            println!("World: {}", name);
        }
    }
    
    #[test]
    fn test_nation_name_generation() {
        let mut gen = NameGenerator::new();
        let cultures = [
            Culture::Western, Culture::Eastern, Culture::Northern,
            Culture::Southern, Culture::Desert, Culture::Island,
            Culture::Ancient, Culture::Mystical,
        ];
        
        for culture in cultures {
            let name = gen.generate(NameType::Nation { culture });
            assert!(!name.is_empty());
            println!("{:?} Nation: {}", culture, name);
        }
    }
    
    #[test]
    fn test_uniqueness() {
        let mut gen = NameGenerator::new();
        let mut names = HashSet::new();
        
        for _ in 0..100 {
            let name = gen.generate(NameType::World);
            assert!(!names.contains(&name), "Duplicate name generated: {}", name);
            names.insert(name);
        }
    }
    
    #[test]
    fn test_deterministic_generation() {
        let mut gen1 = NameGenerator::with_seed(12345);
        let mut gen2 = NameGenerator::with_seed(12345);
        
        for _ in 0..10 {
            let name1 = gen1.generate(NameType::World);
            let name2 = gen2.generate(NameType::World);
            assert_eq!(name1, name2, "Deterministic generation failed");
        }
    }
}