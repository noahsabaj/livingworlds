//! Core government type definitions
//!
//! This module contains the fundamental government types and categories
//! that form the basis of the governance system in Living Worlds.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// All government types in Living Worlds with distinct mechanics
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum GovernmentType {
    // ==================== ANARCHIST VARIANTS ====================
    /// CNT-FAI style - Trade unions run everything
    AnarchoSyndicalism,
    /// Stateless communes with gift economy
    AnarchoCommunism,
    /// Cooperative markets without state
    Mutualism,
    /// Rejection of technology and civilization
    AnarchoPrimitivism,
    /// Extreme individualism, no collective structures
    Egoism,

    // ==================== SOCIALIST/COMMUNIST ====================
    /// Worker councils make all decisions
    CouncilCommunism,
    /// Industrial unions control production
    Syndicalism,
    /// Worker cooperatives with market competition
    MarketSocialism,
    /// Elected socialist government with democratic reforms
    DemocraticSocialism,
    /// Party vanguard leads revolution
    VanguardCommunism,
    /// Peasant-based agrarian communes
    AgrarianSocialism,
    /// Decentralized planned economy
    LibertarianSocialism,
    /// State controls all means of production
    StateSocialism,

    // ==================== FASCIST/AUTHORITARIAN ====================
    /// Ultranationalist corporate state merger
    FascistState,
    /// Military generals rule directly
    MilitaryJunta,
    /// Total state control over all aspects
    TotalitarianRegime,
    /// Military service required for citizenship
    Stratocracy,
    /// Surveillance and secret police control
    PoliceState,
    /// Single party controls everything
    OnePartyState,
    /// Personal dictatorship cult
    Autocracy,

    // ==================== DEMOCRATIC VARIANTS ====================
    /// Parliamentary system with coalitions
    ParliamentaryDemocracy,
    /// Strong executive president
    PresidentialRepublic,
    /// Citizens vote on all issues directly
    DirectDemocracy,
    /// Delegated voting chains
    LiquidDemocracy,
    /// Random citizen selection for governance
    SortitionDemocracy,
    /// Constitutional monarchy with parliament
    ConstitutionalMonarchy,
    /// Swiss-style federal democracy
    FederalRepublic,

    // ==================== ECONOMIC/CORPORATE ====================
    /// Rule by the wealthy elite
    Plutocracy,
    /// Megacorporations as government
    CorporateState,
    /// Scientists and engineers rule
    Technocracy,
    /// Government exists to steal
    Kleptocracy,
    /// Trading families control city-states
    MerchantRepublic,
    /// Central bank controls everything
    Bankocracy,
    /// Guilds control their industries
    GuildState,

    // ==================== RELIGIOUS/IDEOLOGICAL ====================
    /// Religious leaders rule
    Theocracy,
    /// God-emperor with divine mandate
    DivineManadate,
    /// Personality cult worship
    CultState,
    /// Scripture-based unchangeable law
    FundamentalistState,
    /// Mage-priests rule with magic
    MysticOrder,
    /// Islamic religious state
    Caliphate,
    /// Monks/monasteries rule
    MonasticState,

    // ==================== TRADITIONAL/HISTORICAL ====================
    /// Absolute power monarch
    AbsoluteMonarchy,
    /// Decentralized feudal vassals
    Feudalism,
    /// Autonomous tribal confederation
    TribalFederation,
    /// Mobile horse nomads
    NomadicKhanate,
    /// Economy based on slavery
    SlaveState,
    /// Rigid social hierarchy castes
    CasteSystem,
    /// Traditional empire structure
    Empire,
    /// City controls surrounding area
    CityState,
    /// Noble families share power
    Oligarchy,

    // ==================== HYBRID/EXPERIMENTAL ====================
    /// Shared consciousness collective
    HiveMindCollective,
    /// Artificial intelligence governance
    AIGovernance,
    /// Elected pirate captains
    PirateRepublic,
    /// Environmental harmony focus
    EcoCommune,
    /// Secret society shadow rule
    Cryptocracy,
    /// Rule by the elderly
    Gerontocracy,
    /// Judge-based governance
    Kritarchy,
    /// Philosophers rule
    Noocracy,
    /// Temporary emergency powers
    ProvisionalGovernment,
    /// Post-apocalyptic warlords
    Warlordism,
    /// Cyborg collective democracy
    CyborgCollective,
    /// Time-loop based governance
    ChronocraticCouncil,
}

/// Government category for grouping similar types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum GovernmentCategory {
    Democratic,
    Autocratic,
    Theocratic,
    Monarchic,
    Socialist,
    Anarchist,
    Corporate,
    Technocratic,
    Tribal,
}

impl GovernmentType {
    /// Get the category this government belongs to
    pub fn category(&self) -> GovernmentCategory {
        match self {
            // Anarchist types
            Self::AnarchoSyndicalism | Self::AnarchoCommunism | Self::Mutualism
            | Self::AnarchoPrimitivism | Self::Egoism => GovernmentCategory::Anarchist,

            // Socialist types
            Self::CouncilCommunism | Self::Syndicalism | Self::MarketSocialism
            | Self::DemocraticSocialism | Self::VanguardCommunism | Self::AgrarianSocialism
            | Self::LibertarianSocialism | Self::StateSocialism => GovernmentCategory::Socialist,

            // Autocratic types (fascist/authoritarian)
            Self::FascistState | Self::MilitaryJunta | Self::TotalitarianRegime
            | Self::Stratocracy | Self::PoliceState | Self::OnePartyState
            | Self::Autocracy => GovernmentCategory::Autocratic,

            // Democratic types
            Self::ParliamentaryDemocracy | Self::PresidentialRepublic | Self::DirectDemocracy
            | Self::LiquidDemocracy | Self::SortitionDemocracy | Self::FederalRepublic => {
                GovernmentCategory::Democratic
            }

            // Corporate/Economic types
            Self::Plutocracy | Self::CorporateState | Self::Kleptocracy
            | Self::MerchantRepublic | Self::Bankocracy | Self::GuildState => {
                GovernmentCategory::Corporate
            }

            // Theocratic/Religious types
            Self::Theocracy | Self::DivineManadate | Self::CultState | Self::FundamentalistState
            | Self::MysticOrder | Self::Caliphate | Self::MonasticState => {
                GovernmentCategory::Theocratic
            }

            // Monarchic types
            Self::AbsoluteMonarchy | Self::ConstitutionalMonarchy => {
                GovernmentCategory::Monarchic
            }

            // Technocratic types
            Self::Technocracy | Self::Noocracy => GovernmentCategory::Technocratic,

            // Traditional/Tribal types
            Self::Feudalism | Self::TribalFederation | Self::NomadicKhanate | Self::SlaveState
            | Self::CasteSystem | Self::Empire | Self::CityState | Self::Oligarchy => {
                GovernmentCategory::Tribal
            }

            // Special/Experimental types default to their most similar category
            Self::HiveMindCollective | Self::CyborgCollective => GovernmentCategory::Technocratic,
            Self::AIGovernance | Self::ChronocraticCouncil => GovernmentCategory::Technocratic,
            Self::PirateRepublic => GovernmentCategory::Democratic,
            Self::EcoCommune => GovernmentCategory::Anarchist,
            Self::Cryptocracy => GovernmentCategory::Autocratic,
            Self::Gerontocracy | Self::Kritarchy => GovernmentCategory::Tribal,
            Self::ProvisionalGovernment => GovernmentCategory::Democratic,
            Self::Warlordism => GovernmentCategory::Autocratic,
        }
    }
}