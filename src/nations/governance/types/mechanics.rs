//! Government mechanics and restrictions
//!
//! This module contains the functional mechanics that make each government
//! type play differently, including economic, military, social, and political modifiers.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::government::GovernmentType;
use super::succession::SuccessionType;

/// Functional mechanics for each government type
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct GovernmentMechanics {
    // ========== ECONOMIC MODIFIERS ==========
    /// Tax collection efficiency (0.5 = 50% taxes lost to inefficiency)
    pub tax_efficiency: f32,
    /// Trade income and merchant effectiveness
    pub trade_modifier: f32,
    /// Industrial/manufacturing output
    pub industrial_output: f32,
    /// Food production efficiency
    pub agricultural_output: f32,
    /// Wealth inequality level (0.0 = equal, 1.0 = extreme inequality)
    pub inequality: f32,
    /// Can accumulate treasury (false for gift economies)
    pub can_accumulate_wealth: bool,

    // ========== MILITARY MODIFIERS ==========
    /// Days needed to mobilize armies
    pub mobilization_speed: f32,
    /// Base morale and fighting effectiveness
    pub army_morale: f32,
    /// Naval combat and exploration bonuses
    pub naval_tradition: f32,
    /// Can maintain professional armies
    pub can_maintain_standing_army: bool,
    /// Defensive bonuses when invaded
    pub defensive_bonus: f32,
    /// Aggressive expansion modifier
    pub expansion_desire: f32,

    // ========== SOCIAL MODIFIERS ==========
    /// Speed of cultural conversion
    pub cultural_conversion: f32,
    /// Technology research rate
    pub technology_rate: f32,
    /// Base stability level
    pub stability_base: f32,
    /// How fast legitimacy decreases
    pub legitimacy_decay: f32,
    /// Population growth rate
    pub population_growth: f32,
    /// Happiness/unrest modifier
    pub citizen_happiness: f32,

    // ========== POLITICAL MODIFIERS ==========
    /// Resistance to government reform
    pub reform_resistance: f32,
    /// How succession works
    pub succession_type: SuccessionType,
    /// Speed of policy implementation
    pub decision_speed: f32,
    /// Central vs local power (0.0 = local, 1.0 = central)
    pub centralization: f32,
    /// Diplomatic reputation
    pub diplomatic_weight: f32,
    /// Corruption level
    pub corruption: f32,

    // ========== SPECIAL MECHANICS ==========
    /// Unique mechanics only this government has
    pub unique_mechanics: Vec<UniqueMechanic>,
    /// Forbidden actions this government cannot take
    pub restrictions: Vec<GovernmentRestriction>,
}

/// Unique mechanics certain governments possess
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum UniqueMechanic {
    // ========== ANARCHIST MECHANICS ==========
    GeneralStrike,           // Can paralyze enemies with strikes
    CollectivizedProduction, // Worker-run factories
    VoluntaryMilitias,       // No conscription but high morale
    GiftEconomy,            // No money system
    ConsensusDecisions,     // All decisions need agreement

    // ========== SOCIALIST MECHANICS ==========
    PlannedEconomy,         // Central economic planning
    WorkerCouncils,         // Direct worker democracy
    RedTerror,              // Purge counter-revolutionaries
    CollectiveFarms,        // Agricultural communes
    InternationalSolidarity, // Support foreign revolutions

    // ========== FASCIST MECHANICS ==========
    TotalWar,               // Everything serves the war
    SecretPolice,           // Surveillance state
    YouthIndoctrination,    // Children serve the state
    RacialPurity,           // Ethnic cleansing mechanics
    LeaderWorship,          // Cult of personality

    // ========== DEMOCRATIC MECHANICS ==========
    FreePress,              // Information spreads faster
    CivilRights,            // Protected minorities
    PeacefulTransitions,    // Government can change without violence
    Referendums,            // Direct democracy on issues
    TermLimits,             // Leaders must rotate

    // ========== ECONOMIC MECHANICS ==========
    MarketManipulation,     // Control prices artificially
    CorporateLobbying,      // Buy political influence
    TaxHavens,              // Hide wealth offshore
    IndustrialComplex,      // Military-industrial merger
    TradeMonopolies,        // Control specific goods

    // ========== RELIGIOUS MECHANICS ==========
    DivineMandate,          // God approves all actions
    Excommunication,        // Cast out heretics
    HolyWar,                // Religious justification for war
    Miracles,               // Random positive events
    Martyrdom,              // Death strengthens cause

    // ========== TRADITIONAL MECHANICS ==========
    NoblePrivilege,         // Aristocracy exempt from laws
    Serfdom,                // Peasants tied to land
    TrialByCombat,          // Disputes settled by fighting
    BloodFeud,              // Family vengeance cycles
    SacredTradition,        // Ancient ways are unchangeable

    // ========== SPECIAL MECHANICS ==========
    HiveMind,               // Shared consciousness
    TimeLoop,               // Can retry decisions
    TechnoProphecy,         // Predict future with AI
    EnvironmentalHarmony,   // Nature provides bonuses
    ShadowRule,             // Hidden true government
}

/// Restrictions certain governments face
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum GovernmentRestriction {
    // Economic restrictions
    NoPrivateProperty,      // Cannot have private ownership
    NoTradeWithEnemies,     // Cannot trade during war
    NoUsury,                // Cannot charge interest
    NoForeignInvestment,    // Cannot accept foreign money

    // Military restrictions
    NoStandingArmy,         // Cannot maintain permanent military
    NoOffensiveWars,        // Cannot declare war
    NoMercenaries,          // Cannot hire foreign fighters
    NoChemicalWeapons,      // Cannot use certain weapons

    // Social restrictions
    NoCulturalChange,       // Must maintain traditions
    NoEducationForPeasants, // Lower classes stay ignorant
    NoReligiousFreedom,     // One faith only
    NoEmigration,           // Citizens cannot leave

    // Political restrictions
    NoElections,            // Leaders not elected
    NoDiplomacy,            // Cannot negotiate
    NoAlliances,            // Must stand alone
    NoReforms,              // Government cannot change
}

// Implementation of mechanics() for each government type
// NOTE: Full implementations would go here, but are omitted for brevity
// They follow the pattern of creating a GovernmentMechanics struct with
// appropriate values for each government type.

impl GovernmentType {
    /// Get the functional mechanics for this government type
    pub fn mechanics(&self) -> GovernmentMechanics {
        // Placeholder - actual implementations are extensive
        // Each government type returns a customized GovernmentMechanics struct
        GovernmentMechanics {
            tax_efficiency: 0.5,
            trade_modifier: 1.0,
            industrial_output: 1.0,
            agricultural_output: 1.0,
            inequality: 0.5,
            can_accumulate_wealth: true,
            mobilization_speed: 1.0,
            army_morale: 1.0,
            naval_tradition: 1.0,
            can_maintain_standing_army: true,
            defensive_bonus: 1.0,
            expansion_desire: 0.5,
            cultural_conversion: 1.0,
            technology_rate: 1.0,
            stability_base: 0.5,
            legitimacy_decay: 0.01,
            population_growth: 1.0,
            citizen_happiness: 0.5,
            reform_resistance: 0.5,
            succession_type: SuccessionType::Democratic,
            decision_speed: 1.0,
            centralization: 0.5,
            diplomatic_weight: 1.0,
            corruption: 0.2,
            unique_mechanics: Vec::new(),
            restrictions: Vec::new(),
        }
    }
}