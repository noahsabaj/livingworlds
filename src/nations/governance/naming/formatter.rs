//! Government-specific name formatting
//!
//! This module contains the logic for formatting nation names based on their
//! government type and cultural context.

use crate::name_generator::Culture;
use crate::nations::governance::types::GovernmentType;

/// Format the final nation name based on government type
pub fn format_nation_name(
    government: &GovernmentType,
    _structure: &str,
    base_name: &str,
    culture: Culture,
) -> String {
    use GovernmentType::*;

    match government {
        // ========== ANARCHIST - No formal state names ==========
        AnarchoSyndicalism => match culture {
            Culture::Western => format!("Free Territory of {}", base_name),
            Culture::Eastern => format!("{} Autonomous Zone", base_name),
            _ => format!("{} Free Territory", base_name),
        },
        AnarchoCommunism => format!("{} Commune", base_name),
        Mutualism => format!("{} Mutual Society", base_name),
        AnarchoPrimitivism => format!("The {} Wilds", base_name),
        Egoism => format!("Union of {}", base_name),

        // ========== SOCIALIST - People's/Worker's names ==========
        CouncilCommunism => format!("{} Council Republic", base_name),
        Syndicalism => format!("Syndicate of {}", base_name),
        MarketSocialism => format!("Socialist Republic of {}", base_name),
        DemocraticSocialism => format!("Democratic Republic of {}", base_name),
        VanguardCommunism => format!("People's Republic of {}", base_name),
        AgrarianSocialism => format!("Peasant Republic of {}", base_name),
        LibertarianSocialism => format!("Free Socialist Territory of {}", base_name),
        StateSocialism => format!("Socialist State of {}", base_name),

        // ========== FASCIST - State/National names ==========
        FascistState => format!("{} State", base_name),
        MilitaryJunta => format!("Military State of {}", base_name),
        TotalitarianRegime => format!("Totalitarian State of {}", base_name),
        Stratocracy => format!("Military Republic of {}", base_name),
        PoliceState => format!("Security State of {}", base_name),
        OnePartyState => format!("{} Party State", base_name),
        Autocracy => format!("Autocracy of {}", base_name),

        // ========== DEMOCRATIC - Republic/Federation names ==========
        ParliamentaryDemocracy => format!("Parliamentary Republic of {}", base_name),
        PresidentialRepublic => format!("Republic of {}", base_name),
        DirectDemocracy => format!("Democratic Assembly of {}", base_name),
        LiquidDemocracy => format!("Delegative Democracy of {}", base_name),
        SortitionDemocracy => format!("Citizens' Assembly of {}", base_name),
        ConstitutionalMonarchy => format!("Kingdom of {}", base_name),
        FederalRepublic => format!("Federation of {}", base_name),

        // ========== ECONOMIC - Corporate/Trade names ==========
        Plutocracy => format!("Plutocracy of {}", base_name),
        CorporateState => match culture {
            Culture::Western | Culture::Eastern => format!("{} Incorporated", base_name),
            _ => format!("Corporate State of {}", base_name),
        },
        Technocracy => format!("Technocracy of {}", base_name),
        Kleptocracy => format!("{} Territory", base_name), // Don't advertise theft
        MerchantRepublic => match culture {
            Culture::Island | Culture::Southern => format!("Free Port of {}", base_name),
            _ => format!("Merchant Republic of {}", base_name),
        },
        Bankocracy => format!("Banking State of {}", base_name),
        GuildState => format!("Guild Federation of {}", base_name),

        // ========== RELIGIOUS - Holy/Sacred names ==========
        Theocracy => format!("Holy State of {}", base_name),
        DivineManadate => format!("Divine Empire of {}", base_name),
        CultState => format!("Cult of {}", base_name),
        FundamentalistState => format!("Fundamentalist State of {}", base_name),
        MysticOrder => format!("Mystic Order of {}", base_name),
        Caliphate => format!("Caliphate of {}", base_name),
        MonasticState => format!("Monastic Order of {}", base_name),

        // ========== TRADITIONAL - Classic kingdom names ==========
        AbsoluteMonarchy => format!("Kingdom of {}", base_name),
        Feudalism => format!("Feudal Kingdom of {}", base_name),
        TribalFederation => format!("{} Tribes", base_name),
        NomadicKhanate => format!("{} Khanate", base_name),
        SlaveState => format!("{} Dominion", base_name), // Euphemistic
        CasteSystem => format!("{} Hierarchy", base_name),
        Empire => format!("{} Empire", base_name),
        CityState => format!("Free City of {}", base_name),
        Oligarchy => format!("Oligarchy of {}", base_name),

        // ========== SPECIAL - Unique names ==========
        HiveMindCollective => format!("{} Collective", base_name),
        AIGovernance => format!("Algorithm State {}", base_name),
        PirateRepublic => format!("{} Haven", base_name),
        EcoCommune => format!("{} Eco-Commune", base_name),
        Cryptocracy => format!("{}", base_name), // Hidden government
        Gerontocracy => format!("Elder Council of {}", base_name),
        Kritarchy => format!("Judicial State of {}", base_name),
        Noocracy => format!("Philosopher State of {}", base_name),
        ProvisionalGovernment => format!("Provisional Government of {}", base_name),
        Warlordism => format!("{} Warlord Territory", base_name),
        CyborgCollective => format!("Cybernetic Union of {}", base_name),
        ChronocraticCouncil => format!("Temporal Authority of {}", base_name),
    }
}