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
    ///
    /// CRITICAL: Descriptors must be NON-POLITICAL to avoid redundancy with government structure terms.
    /// Use geographic, historical, and size-based descriptors only.
    pub fn for_government(government: &GovernmentType, culture: Culture) -> Self {
        match government {
            // Authoritarian governments: strong, geographic descriptors (NOT political terms)
            GovernmentType::MilitaryJunta |
            GovernmentType::PoliceState |
            GovernmentType::FascistState |
            GovernmentType::TotalitarianRegime |
            GovernmentType::Stratocracy => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["Iron", "Steel", "United", "Central", "Greater", "Northern"],
                structures: vec![], // No structures - handled by formatter
            },

            // Democratic governments: geographic and size-based (NOT "Democratic", "Federal")
            GovernmentType::ParliamentaryDemocracy |
            GovernmentType::PresidentialRepublic |
            GovernmentType::DirectDemocracy |
            GovernmentType::FederalRepublic => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["United", "Allied", "Northern", "Southern", "Greater", "Central"],
                structures: vec![], // No structures - handled by formatter
            },

            // Religious governments: historical/temporal (NOT "Holy", "Divine", "Sacred")
            GovernmentType::Theocracy |
            GovernmentType::DivineManadate |
            GovernmentType::FundamentalistState |
            GovernmentType::Caliphate |
            GovernmentType::MonasticState => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["Ancient", "Eternal", "Golden", "Old", "Greater", "Eastern"],
                structures: vec![], // No structures - handled by formatter
            },

            // Traditional monarchies: historical/temporal (NOT "Royal", "Imperial")
            GovernmentType::AbsoluteMonarchy |
            GovernmentType::ConstitutionalMonarchy |
            GovernmentType::Feudalism |
            GovernmentType::Empire => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["Grand", "Great", "Old", "New", "Greater", "Ancient"],
                structures: vec![], // No structures - handled by formatter
            },

            // Socialist governments: geographic/directional (NOT "Socialist", "People's", "Workers'")
            GovernmentType::CouncilCommunism |
            GovernmentType::Syndicalism |
            GovernmentType::MarketSocialism |
            GovernmentType::DemocraticSocialism |
            GovernmentType::VanguardCommunism => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["Red", "Eastern", "United", "Greater", "Northern", "Central"],
                structures: vec![], // No structures - handled by formatter
            },

            // Corporate/Economic governments: geographic/maritime (NOT "Trade", "Merchant", "Commercial")
            GovernmentType::CorporateState |
            GovernmentType::MerchantRepublic |
            GovernmentType::Bankocracy |
            GovernmentType::GuildState => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["Coastal", "Maritime", "Prosperous", "Greater", "Ancient", "Golden"],
                structures: vec![], // No structures - handled by formatter
            },

            // Default for other government types
            _ => Self {
                places: Self::get_culture_places(culture),
                descriptors: vec!["Greater", "New", "Old", "Central", "United", "Northern"],
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

        // 40% chance to add a descriptor for variety (reduced from 60% to minimize redundancy)
        let add_descriptor = self.generator.random_range(0, 10) < 4;
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
            // Anarchist - minimal state references with culture variants
            AnarchoSyndicalism => match self.culture {
                Culture::Western => format!("Free Territory of {}", clean_base),
                Culture::Eastern => format!("{} Autonomous Zone", clean_base),
                _ => format!("{} Free Territory", clean_base),
            },
            AnarchoCommunism => format!("{} Commune", clean_base),
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

            // Corporate - company/trade names with culture variants
            CorporateState => match self.culture {
                Culture::Western | Culture::Eastern => format!("{} Incorporated", clean_base),
                _ => format!("Corporate State of {}", clean_base),
            },
            MerchantRepublic => match self.culture {
                Culture::Island | Culture::Southern => format!("Free Port of {}", clean_base),
                _ => format!("Merchant Republic of {}", clean_base),
            },

            // Default pattern
            _ => format!("{}", clean_base),
        }
    }

    /// Remove any conflicting political terms from the base name
    ///
    /// This includes both exact matches AND semantic equivalents to prevent
    /// redundancy like "Merchant Republic of Trade Valencia"
    fn clean_base_name(&self, name: &str) -> String {
        // Exact political structure terms that must be removed
        let conflict_terms = [
            "Republic", "Kingdom", "Empire", "State", "Federation",
            "Senate", "Parliament", "Congress", "Assembly",
            "Democracy", "Monarchy", "Oligarchy", "Theocracy",
            "Commonwealth", "Union", "Alliance", "Coalition",
        ];

        // Semantic equivalents - these imply political meanings
        let semantic_conflicts = [
            ("Merchant", vec!["Trade", "Trading", "Commercial"]),
            ("Holy", vec!["Sacred", "Divine", "Blessed"]),
            ("Imperial", vec!["Empire", "Emperor"]),
            ("Royal", vec!["King", "Queen", "Regal"]),
            ("Socialist", vec!["Workers'", "People's"]),
            ("Democratic", vec!["Free", "Liberal"]),
            ("Military", vec!["Martial", "Warrior"]),
        ];

        let mut result = name.to_string();

        // Remove exact conflict terms
        for term in &conflict_terms {
            result = result.replace(term, "");
        }

        // Remove semantic equivalents based on government type
        // This prevents "Merchant Republic of Trade X" type redundancy
        for (_base_term, equivalents) in &semantic_conflicts {
            for equivalent in equivalents {
                // Remove as standalone word with spaces around it
                result = result.replace(&format!(" {} ", equivalent), " ");
                // Remove at start
                if result.starts_with(&format!("{} ", equivalent)) {
                    result = result[equivalent.len() + 1..].to_string();
                }
                // Remove at end
                if result.ends_with(&format!(" {}", equivalent)) {
                    result = result[..result.len() - equivalent.len() - 1].to_string();
                }
            }
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
        let mut rng_gen = NameGenerator::new();

        // Test that we don't get "Security State of Republic" type names
        for _ in 0..100 {
            let (name, _) = build_nation_name(
                &mut rng_gen,
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
        let mut rng_gen = NameGenerator::new();

        // Theocracy should have religious terms
        let (name, _) = build_nation_name(
            &mut rng_gen,
            Culture::Western,
            GovernmentType::Theocracy,
        );
        assert!(name.contains("Holy State"));

        // Military junta should have military terms
        let (name2, _) = build_nation_name(
            &mut rng_gen,
            Culture::Western,
            GovernmentType::MilitaryJunta,
        );
        assert!(name2.contains("Military State"));
    }

    #[test]
    fn test_no_merchant_trade_redundancy() {
        let mut rng_gen = NameGenerator::new();

        // Test 1000 generations to catch probabilistic issues (40% descriptor chance)
        for _ in 0..1000 {
            let (name, _) = build_nation_name(
                &mut rng_gen,
                Culture::Southern,
                GovernmentType::MerchantRepublic,
            );

            // Should NEVER contain these redundant patterns
            assert!(!name.contains("Merchant Republic of Trade"),
                    "Found redundant 'Merchant Republic of Trade' in: {}", name);
            assert!(!name.contains("Merchant Republic of Merchant"),
                    "Found redundant 'Merchant Republic of Merchant' in: {}", name);
            assert!(!name.contains("Merchant Republic of Commercial"),
                    "Found redundant 'Merchant Republic of Commercial' in: {}", name);
            assert!(!name.contains("Free Port of Trade"),
                    "Found redundant 'Free Port of Trade' in: {}", name);
            assert!(!name.contains("Free Port of Merchant"),
                    "Found redundant 'Free Port of Merchant' in: {}", name);
        }
    }

    #[test]
    fn test_no_socialist_redundancy() {
        let mut rng_gen = NameGenerator::new();

        for _ in 0..1000 {
            let (name, _) = build_nation_name(
                &mut rng_gen,
                Culture::Eastern,
                GovernmentType::DemocraticSocialism,
            );

            // Should NOT contain "Socialist Republic of Socialist"
            assert!(!name.contains("Socialist Republic of Socialist"),
                    "Found redundant 'Socialist' in: {}", name);
            assert!(!name.contains("Socialist Republic of Workers'"),
                    "Found redundant 'Workers'' in: {}", name);
            assert!(!name.contains("Socialist Republic of People's"),
                    "Found redundant 'People's' in: {}", name);
        }
    }

    #[test]
    fn test_no_holy_divine_redundancy() {
        let mut rng_gen = NameGenerator::new();

        for _ in 0..1000 {
            let (name, _) = build_nation_name(
                &mut rng_gen,
                Culture::Ancient,
                GovernmentType::Theocracy,
            );

            // Should NOT contain "Holy State of Holy"
            assert!(!name.contains("Holy State of Holy"),
                    "Found redundant 'Holy' in: {}", name);
            assert!(!name.contains("Holy State of Sacred"),
                    "Found redundant 'Sacred' in: {}", name);
            assert!(!name.contains("Holy State of Divine"),
                    "Found redundant 'Divine' in: {}", name);
        }
    }

    #[test]
    fn test_no_democratic_redundancy() {
        let mut rng_gen = NameGenerator::new();

        for _ in 0..1000 {
            let (name, _) = build_nation_name(
                &mut rng_gen,
                Culture::Western,
                GovernmentType::ParliamentaryDemocracy,
            );

            // Should NOT contain "Democratic Republic of Democratic"
            assert!(!name.contains("Democratic") || name.matches("Democratic").count() <= 1,
                    "Found redundant 'Democratic' in: {}", name);
        }
    }

    #[test]
    fn test_culture_aware_variants() {
        let mut rng_gen = NameGenerator::new();

        // Island/Southern cultures should get "Free Port" variant
        let (name, _) = build_nation_name(
            &mut rng_gen,
            Culture::Island,
            GovernmentType::MerchantRepublic,
        );
        // Either Free Port or Merchant Republic is acceptable
        assert!(name.contains("Free Port") || name.contains("Merchant Republic"));

        // Western/Eastern cultures should get "Incorporated" variant
        let (name2, _) = build_nation_name(
            &mut rng_gen,
            Culture::Western,
            GovernmentType::CorporateState,
        );
        assert!(name2.contains("Incorporated") || name2.contains("Corporate State"));

        // Eastern culture should get "Autonomous Zone" variant
        let (name3, _) = build_nation_name(
            &mut rng_gen,
            Culture::Eastern,
            GovernmentType::AnarchoSyndicalism,
        );
        assert!(name3.contains("Autonomous Zone") || name3.contains("Free Territory"));
    }

    #[test]
    fn test_descriptors_are_non_political() {
        let mut rng_gen = NameGenerator::new();

        // Test multiple government types to ensure descriptors don't conflict
        let governments = vec![
            GovernmentType::MerchantRepublic,
            GovernmentType::DemocraticSocialism,
            GovernmentType::Theocracy,
            GovernmentType::Empire,
        ];

        for gov in governments {
            for _ in 0..100 {
                let (name, _) = build_nation_name(&mut rng_gen, Culture::Western, gov);

                // Descriptors should be geographic, not political
                assert!(!name.contains("Trade") || !name.contains("Merchant"),
                        "Political descriptor conflict in: {}", name);
                assert!(!name.contains("Socialist") || name.matches("Socialist").count() <= 1,
                        "Political descriptor conflict in: {}", name);
                assert!(!name.contains("Holy") || name.matches("Holy").count() <= 1,
                        "Political descriptor conflict in: {}", name);
            }
        }
    }
}