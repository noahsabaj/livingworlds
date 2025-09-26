//! Government selection based on culture and development level
//!
//! This module provides logic for selecting appropriate government types
//! based on cultural context and technological development.

use rand::Rng;
use crate::name_generator::Culture;
use crate::nations::governance::types::GovernmentType;
use super::development::DevelopmentLevel;

/// Suggest an appropriate government type based on culture and development level
pub fn suggest_government_for_culture(
    culture: Culture,
    development: DevelopmentLevel,
    rng: &mut impl Rng,
) -> GovernmentType {
    use GovernmentType::*;

    // Each culture/era combination has weighted pools of appropriate governments
    let government_pool = match (culture, development) {
        // Western cultures
        (Culture::Western, DevelopmentLevel::Primitive) => vec![
            TribalFederation,
            CityState,
        ],
        (Culture::Western, DevelopmentLevel::Medieval) => vec![
            Feudalism,
            AbsoluteMonarchy,
            CityState,
            Oligarchy,
            Theocracy,
        ],
        (Culture::Western, DevelopmentLevel::Renaissance) => vec![
            AbsoluteMonarchy,
            ConstitutionalMonarchy,
            MerchantRepublic,
            Oligarchy,
            CityState,
        ],
        (Culture::Western, DevelopmentLevel::Modern) => vec![
            ParliamentaryDemocracy,
            PresidentialRepublic,
            ConstitutionalMonarchy,
            FederalRepublic,
            CorporateState,
            DemocraticSocialism,
        ],

        // Eastern cultures
        (Culture::Eastern, DevelopmentLevel::Primitive) => vec![
            TribalFederation,
            CasteSystem,
        ],
        (Culture::Eastern, DevelopmentLevel::Medieval) => vec![
            Empire,
            Feudalism,
            DivineManadate,
            CasteSystem,
        ],
        (Culture::Eastern, DevelopmentLevel::Renaissance) => vec![
            Empire,
            AbsoluteMonarchy,
            DivineManadate,
        ],
        (Culture::Eastern, DevelopmentLevel::Modern) => vec![
            VanguardCommunism,
            OnePartyState,
            Technocracy,
            CorporateState,
        ],

        // Northern cultures
        (Culture::Northern, DevelopmentLevel::Primitive) => vec![
            TribalFederation,
            NomadicKhanate,
        ],
        (Culture::Northern, DevelopmentLevel::Medieval) => vec![
            Feudalism,
            TribalFederation,
            NomadicKhanate,
            AbsoluteMonarchy,
        ],
        (Culture::Northern, DevelopmentLevel::Renaissance) => vec![
            AbsoluteMonarchy,
            Empire,
            Oligarchy,
        ],
        (Culture::Northern, DevelopmentLevel::Modern) => vec![
            FederalRepublic,
            DemocraticSocialism,
            ParliamentaryDemocracy,
        ],

        // Desert cultures
        (Culture::Desert, DevelopmentLevel::Primitive) => vec![
            TribalFederation,
            NomadicKhanate,
        ],
        (Culture::Desert, DevelopmentLevel::Medieval) => vec![
            Caliphate,
            NomadicKhanate,
            TribalFederation,
            Theocracy,
        ],
        (Culture::Desert, DevelopmentLevel::Renaissance) => vec![
            Caliphate,
            Empire,
            AbsoluteMonarchy,
        ],
        (Culture::Desert, DevelopmentLevel::Modern) => vec![
            PresidentialRepublic,
            MilitaryJunta,
            Theocracy,
            OnePartyState,
        ],

        // Island cultures
        (Culture::Island, DevelopmentLevel::Primitive) => vec![
            TribalFederation,
            CityState,
        ],
        (Culture::Island, DevelopmentLevel::Medieval) => vec![
            CityState,
            MerchantRepublic,
            PirateRepublic,
            TribalFederation,
        ],
        (Culture::Island, DevelopmentLevel::Renaissance) => vec![
            MerchantRepublic,
            PirateRepublic,
            CityState,
            Oligarchy,
        ],
        (Culture::Island, DevelopmentLevel::Modern) => vec![
            ParliamentaryDemocracy,
            FederalRepublic,
            CorporateState,
            EcoCommune,
        ],

        // Mystical cultures
        (Culture::Mystical, _) => vec![
            MysticOrder,
            Theocracy,
            DivineManadate,
            CultState,
            Noocracy,
        ],

        // Ancient cultures
        (Culture::Ancient, _) => vec![
            Empire,
            DivineManadate,
            Theocracy,
            Oligarchy,
            CityState,
        ],

        // Southern cultures
        (Culture::Southern, DevelopmentLevel::Primitive) => vec![
            TribalFederation,
            CityState,
        ],
        (Culture::Southern, DevelopmentLevel::Medieval) => vec![
            MerchantRepublic,
            CityState,
            AbsoluteMonarchy,
            Theocracy,
        ],
        (Culture::Southern, DevelopmentLevel::Renaissance) => vec![
            MerchantRepublic,
            CityState,
            Empire,
            Oligarchy,
        ],
        (Culture::Southern, DevelopmentLevel::Modern) => vec![
            PresidentialRepublic,
            ParliamentaryDemocracy,
            CorporateState,
            DemocraticSocialism,
        ],
    };

    // Randomly select from the appropriate pool
    let index = rng.gen_range(0..government_pool.len());
    government_pool[index]
}