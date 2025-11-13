//! Core governance-aware name generation
//!
//! This module provides the main entry point for generating nation names
//! that match their government type.

use crate::name_generator::{Culture, NameGenerator};
use crate::nations::governance::types::{Gender, GovernmentType};


/// Generate a nation name and ruler title based on government type and culture
///
/// This now uses the new NationNameBuilder for grammatically correct names
pub fn generate_governance_aware_name(
    generator: &mut NameGenerator,
    culture: Culture,
    government: &GovernmentType,
) -> (String, String) {
    // Use the new builder system for grammatically correct names
    super::builder::build_nation_name(generator, culture, *government)
}

/// Get the appropriate ruler title for a government type
pub fn get_ruler_title(government: &GovernmentType, _gender: Gender) -> &'static str {
    use GovernmentType::*;

    match government {
        // Anarchist types
        AnarchoSyndicalism | AnarchoCommunism | Mutualism | AnarchoPrimitivism | Egoism => "Speaker",

        // Socialist types
        CouncilCommunism => "Council Chair",
        Syndicalism => "General Secretary",
        MarketSocialism | DemocraticSocialism => "Premier",
        VanguardCommunism | StateSocialism => "Chairman",
        AgrarianSocialism => "Agrarian Leader",
        LibertarianSocialism => "Coordinator",

        // Fascist/Authoritarian
        FascistState | TotalitarianRegime => "Supreme Leader",
        MilitaryJunta | Stratocracy => "General",
        PoliceState => "Director",
        OnePartyState => "Party Secretary",
        Autocracy => "Autocrat",

        // Democratic
        ParliamentaryDemocracy => "Prime Minister",
        PresidentialRepublic => "President",
        DirectDemocracy | LiquidDemocracy | SortitionDemocracy => "Moderator",
        ConstitutionalMonarchy => "King",
        FederalRepublic => "Chancellor",

        // Economic/Corporate
        Plutocracy => "Oligarch",
        CorporateState => "CEO",
        Technocracy => "Chief Scientist",
        Kleptocracy => "Boss",
        MerchantRepublic => "Doge",
        Bankocracy => "Governor",
        GuildState => "Guildmaster",

        // Religious
        Theocracy => "High Priest",
        DivineManadate => "God-Emperor",
        CultState => "Cult Leader",
        FundamentalistState => "Supreme Cleric",
        MysticOrder => "Archmagus",
        Caliphate => "Caliph",
        MonasticState => "Abbot",

        // Traditional
        AbsoluteMonarchy => "King",
        Feudalism => "Lord",
        TribalFederation => "Chief",
        NomadicKhanate => "Khan",
        SlaveState => "Master",
        CasteSystem => "Raja",
        Empire => "Emperor",
        CityState => "Mayor",
        Oligarchy => "Consul",

        // Special
        HiveMindCollective => "Overmind",
        AIGovernance => "Core Process",
        PirateRepublic => "Pirate King",
        EcoCommune => "Elder",
        Cryptocracy => "Unknown",
        Gerontocracy => "Elder",
        Kritarchy => "Chief Judge",
        Noocracy => "Philosopher King",
        ProvisionalGovernment => "Provisional Leader",
        Warlordism => "Warlord",
        CyborgCollective => "Primary Node",
        ChronocraticCouncil => "Timekeeper",
    }
}

/// Get the structure name for a government type
pub fn get_structure_name(government: &GovernmentType) -> &'static str {
    use GovernmentType::*;

    match government {
        // Anarchist types
        AnarchoSyndicalism => "Syndicate",
        AnarchoCommunism => "Commune",
        Mutualism => "Mutual Society",
        AnarchoPrimitivism => "Tribe",
        Egoism => "Union of Egoists",

        // Socialist types
        CouncilCommunism => "Council Republic",
        Syndicalism => "Trade Union Federation",
        MarketSocialism => "Socialist Republic",
        DemocraticSocialism => "Democratic Republic",
        VanguardCommunism => "People's Republic",
        AgrarianSocialism => "Peasant Republic",
        LibertarianSocialism => "Free Territory",
        StateSocialism => "Socialist State",

        // Fascist/Authoritarian
        FascistState => "Fascist State",
        MilitaryJunta => "Military Government",
        TotalitarianRegime => "Totalitarian State",
        Stratocracy => "Military Republic",
        PoliceState => "Security State",
        OnePartyState => "Party State",
        Autocracy => "Autocracy",

        // Democratic
        ParliamentaryDemocracy => "Parliamentary Republic",
        PresidentialRepublic => "Republic",
        DirectDemocracy => "Assembly",
        LiquidDemocracy => "Delegative Democracy",
        SortitionDemocracy => "Citizens' Assembly",
        ConstitutionalMonarchy => "Kingdom",
        FederalRepublic => "Federation",

        // Economic/Corporate
        Plutocracy => "Plutocracy",
        CorporateState => "Corporate State",
        Technocracy => "Technocracy",
        Kleptocracy => "Territory",
        MerchantRepublic => "Merchant Republic",
        Bankocracy => "Banking State",
        GuildState => "Guild Federation",

        // Religious
        Theocracy => "Theocracy",
        DivineManadate => "Divine Empire",
        CultState => "Cult",
        FundamentalistState => "Fundamentalist State",
        MysticOrder => "Mystic Order",
        Caliphate => "Caliphate",
        MonasticState => "Monastic Order",

        // Traditional
        AbsoluteMonarchy => "Kingdom",
        Feudalism => "Feudal Kingdom",
        TribalFederation => "Tribal Federation",
        NomadicKhanate => "Khanate",
        SlaveState => "Dominion",
        CasteSystem => "Hierarchy",
        Empire => "Empire",
        CityState => "City-State",
        Oligarchy => "Oligarchy",

        // Special
        HiveMindCollective => "Collective",
        AIGovernance => "Algorithm State",
        PirateRepublic => "Pirate Haven",
        EcoCommune => "Eco-Commune",
        Cryptocracy => "Shadow State",
        Gerontocracy => "Elder Council",
        Kritarchy => "Judicial State",
        Noocracy => "Philosopher State",
        ProvisionalGovernment => "Provisional Government",
        Warlordism => "Warlord Territory",
        CyborgCollective => "Cybernetic Union",
        ChronocraticCouncil => "Temporal Authority",
    }
}