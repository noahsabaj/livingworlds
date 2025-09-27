//! Government-aware nation name builder
//!
//! This module provides a centralized, grammatically correct nation naming system
//! that generates names appropriate for each government type.

use crate::name_generator::{Culture, NameGenerator};
use crate::nations::governance::types::GovernmentType;
use rand::Rng;

/// Components for building nation names
#[derive(Debug, Clone)]
pub struct NameComponents {
    /// Place names without political connotations (Britannia, Bavaria, etc.)
    pub places: Vec<&'static str>,

    /// Descriptive adjectives appropriate for the government type
    pub descriptors: Vec<&'static str>,

    /// Political structures (only used when appropriate)
    pub structures: Vec<&'static str>,
}

impl NameComponents {
    /// Get components for a specific government and culture combination
    pub fn for_government(government: &GovernmentType, culture: Culture) -> Self {
        match government {
            // Authoritarian governments: strong, military descriptors
            GovernmentType::MilitaryJunta |
            GovernmentType::PoliceState |
            GovernmentType::FascistState |
            GovernmentType::TotalitarianRegime |
            GovernmentType::Stratocracy => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["Iron", "Steel", "Strong", "Unified", "Central", "National"],
                structures: vec![], // No structures - handled by formatter
            },

            // Democratic governments: freedom, unity descriptors
            GovernmentType::ParliamentaryDemocracy |
            GovernmentType::PresidentialRepublic |
            GovernmentType::DirectDemocracy |
            GovernmentType::FederalRepublic => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["United", "Free", "Allied", "Federal", "Independent", "Democratic"],
                structures: vec![], // No structures - handled by formatter
            },

            // Religious governments: holy, sacred descriptors
            GovernmentType::Theocracy |
            GovernmentType::DivineManadate |
            GovernmentType::FundamentalistState |
            GovernmentType::Caliphate |
            GovernmentType::MonasticState => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["Holy", "Sacred", "Blessed", "Divine", "Righteous", "Eternal"],
                structures: vec![], // No structures - handled by formatter
            },

            // Traditional monarchies: royal, noble descriptors
            GovernmentType::AbsoluteMonarchy |
            GovernmentType::ConstitutionalMonarchy |
            GovernmentType::Feudalism |
            GovernmentType::Empire => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["Royal", "Imperial", "Noble", "Grand", "Great", "Sovereign"],
                structures: vec![], // No structures - handled by formatter
            },

            // Socialist governments: people's, workers' descriptors
            GovernmentType::CouncilCommunism |
            GovernmentType::Syndicalism |
            GovernmentType::MarketSocialism |
            GovernmentType::DemocraticSocialism |
            GovernmentType::VanguardCommunism => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["People's", "Workers'", "Socialist", "Revolutionary", "Collective"],
                structures: vec![], // No structures - handled by formatter
            },

            // Corporate/Economic governments: trade, merchant descriptors
            GovernmentType::CorporateState |
            GovernmentType::MerchantRepublic |
            GovernmentType::Bankocracy |
            GovernmentType::GuildState => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["Trade", "Merchant", "Commercial", "Industrial", "Economic"],
                structures: vec![], // No structures - handled by formatter
            },

            // Default for other government types
            _ => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["Greater", "New", "Old", "Central", "United"],
                structures: vec![],
            },
        }
    }

    /// Get culture-appropriate place names
    fn get_culture_places(culture: Culture) -> Vec<&'static str> {
        match culture {
            Culture::Western => vec![
                "Britannia", "Germania", "Gallia", "Hispania", "Italia",
                "Helvetia", "Polonia", "Bohemia", "Hungaria", "Romania",
                "Bavaria", "Prussia", "Austria", "Normandy", "Burgundy",
                "Achaea", "Macedonia", "Thracia", "Dacia", "Iberia",
            ],
            Culture::Eastern => vec![
                "Yamato", "Zhongguo", "Joseon", "Nihon", "Tianxia",
                "Dongfang", "Beifang", "Nanfang", "Xifang", "Zhongyang",
                "Shenzhou", "Huaxia", "Dongying", "Goryeo", "Baekje",
            ],
            Culture::Northern => vec![
                "Nordheim", "Vinterland", "Frostgard", "Thornwall", "Ironhold",
                "Stormfjord", "Ravenpeak", "Wolfsburg", "Bjornholm", "Eriksgard",
                "Valhalla", "Asgard", "Midgard", "Jotunheim", "Niflheim",
            ],
            Culture::Southern => vec![
                "Solaria", "Mediterra", "Aurelia", "Lumina", "Florentina",
                "Valencia", "Cordova", "Sevilla", "Granada", "Catalonia",
                "Tuscania", "Venetia", "Sicilia", "Napoli", "Roma",
            ],
            Culture::Desert => vec![
                "Saharabad", "Qadesh", "Mirage", "Sirocco", "Dunehold",
                "Oasis", "Petra", "Babylon", "Assyria", "Phoenicia",
                "Arabia", "Maghreb", "Sahel", "Nubia", "Kush",
            ],
            Culture::Island => vec![
                "Archipelago", "Atoll", "Lagoon", "Coral", "Pearl",
                "Tidal", "Seafoam", "Driftwood", "Sunset", "Moonwater",
                "Polynesia", "Melanesia", "Micronesia", "Caribbean", "Bermuda",
            ],
            Culture::Ancient => vec![
                "Atlantis", "Lemuria", "Mu", "Hyperborea", "Thule",
                "Babylon", "Sumeria", "Akkadia", "Elam", "Ur",
                "Memphis", "Thebes", "Karnak", "Luxor", "Giza",
            ],
            Culture::Mystical => vec![
                "Avalon", "Shangri-La", "Eldorado", "Camelot", "Xanadu",
                "Arcadia", "Elysium", "Valhalla", "Aether", "Celestia",
                "Mystara", "Ethereal", "Astral", "Shadow", "Twilight",
            ],
        }
    }
}

/// Main builder for creating government-aware nation names
pub struct NationNameBuilder<'a> {
    generator: &'a mut NameGenerator,
    government: GovernmentType,
    culture: Culture,
    components: NameComponents,
}

impl<'a> NationNameBuilder<'a> {
    /// Create a new nation name builder
    pub fn new(
        generator: &'a mut NameGenerator,
        government: GovernmentType,
        culture: Culture,
    ) -> Self {
        let components = NameComponents::for_government(&government, culture);
        Self {
            generator,
            government,
            culture,
            components,
        }
    }

    /// Build a grammatically correct nation name
    pub fn build(&mut self) -> String {
        // Generate base name (place with optional descriptor)
        let base_name = self.generate_base_name();

        // Apply government-specific formatting
        self.format_for_government(&base_name)
    }

    /// Generate an appropriate base name
    fn generate_base_name(&mut self) -> String {
        let place = self.generator.random_choice(&self.components.places);

        // 60% chance to add a descriptor for variety (using random_range as workaround)
        let add_descriptor = self.generator.random_range(0, 10) < 6;
        if add_descriptor && !self.components.descriptors.is_empty() {
            let descriptor = self.generator.random_choice(&self.components.descriptors);
            format!("{} {}", descriptor, place)
        } else {
            place.to_string()
        }
    }

    /// Format the name according to government type
    fn format_for_government(&self, base_name: &str) -> String {
        use GovernmentType::*;

        // Clean the base name of any conflicting terms
        let clean_base = self.clean_base_name(base_name);

        match self.government {
            // Anarchist - minimal state references
            AnarchoSyndicalism | AnarchoCommunism => format!("{} Free Territory", clean_base),
            Mutualism => format!("{} Mutual Society", clean_base),
            AnarchoPrimitivism => format!("The {} Wilds", clean_base),

            // Socialist - people's/worker's names
            VanguardCommunism => format!("People's Republic of {}", clean_base),
            DemocraticSocialism => format!("Democratic Republic of {}", clean_base),
            MarketSocialism => format!("Socialist Republic of {}", clean_base),
            Syndicalism => format!("Syndicate of {}", clean_base),

            // Authoritarian - state names
            MilitaryJunta => format!("Military State of {}", clean_base),
            PoliceState => format!("Security State of {}", clean_base),
            FascistState => format!("{} State", clean_base),
            TotalitarianRegime => format!("Totalitarian State of {}", clean_base),

            // Democratic - republic/federation names
            ParliamentaryDemocracy => format!("Parliamentary Republic of {}", clean_base),
            PresidentialRepublic => format!("Republic of {}", clean_base),
            FederalRepublic => format!("Federation of {}", clean_base),

            // Monarchical - kingdom/empire names
            AbsoluteMonarchy | ConstitutionalMonarchy => format!("Kingdom of {}", clean_base),
            Empire => format!("{} Empire", clean_base),
            Feudalism => format!("Feudal Kingdom of {}", clean_base),

            // Religious - holy/divine names
            Theocracy => format!("Holy State of {}", clean_base),
            DivineManadate => format!("Divine Empire of {}", clean_base),
            Caliphate => format!("Caliphate of {}", clean_base),

            // Corporate - company/trade names
            CorporateState => format!("{} Incorporated", clean_base),
            MerchantRepublic => format!("Merchant Republic of {}", clean_base),

            // Default pattern
            _ => format!("{}", clean_base),
        }
    }

    /// Remove any conflicting political terms from the base name
    fn clean_base_name(&self, name: &str) -> String {
        // Terms that should never appear in base names when using formatters
        let conflict_terms = [
            "Republic", "Kingdom", "Empire", "State", "Federation",
            "Senate", "Parliament", "Congress", "Assembly",
            "Democracy", "Monarchy", "Oligarchy", "Theocracy",
            "Commonwealth", "Union", "Alliance", "Coalition",
        ];

        let mut result = name.to_string();
        for term in &conflict_terms {
            result = result.replace(term, "");
        }

        // Clean up any double spaces
        while result.contains("  ") {
            result = result.replace("  ", " ");
        }

        result.trim().to_string()
    }
}

/// Generate a nation name and ruler title using the new builder system
pub fn build_nation_name(
    generator: &mut NameGenerator,
    culture: Culture,
    government: GovernmentType,
) -> (String, String) {
    let mut builder = NationNameBuilder::new(generator, government, culture);
    let nation_name = builder.build();

    // Get ruler title from existing system
    let ruler_title = super::generator::get_ruler_title(&government, crate::nations::governance::types::Gender::Neutral);

    (nation_name, ruler_title.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_double_political_structures() {
        let mut gen = NameGenerator::new();

        // Test that we don't get "Security State of Republic" type names
        for _ in 0..100 {
            let (name, _) = build_nation_name(
                &mut gen,
                Culture::Western,
                GovernmentType::PoliceState,
            );

            // Should not contain conflicting terms
            assert!(!name.contains("Republic of"));
            assert!(!name.contains("Senate of"));
            assert!(!name.contains("Parliament of"));
            assert!(!name.contains("Empire of"));
        }
    }

    #[test]
    fn test_government_appropriate_names() {
        let mut gen = NameGenerator::new();

        // Theocracy should have religious terms
        let (name, _) = build_nation_name(
            &mut gen,
            Culture::Western,
            GovernmentType::Theocracy,
        );
        assert!(name.contains("Holy State"));

        // Military junta should have military terms
        let (name2, _) = build_nation_name(
            &mut gen,
            Culture::Western,
            GovernmentType::MilitaryJunta,
        );
        assert!(name2.contains("Military State"));
    }
}