//! Governance domain module - supranational entities and power structures
//!
//! Governance above the nation-state level, including customs unions,
//! currency unions, and political federations.

pub mod supranational;
pub mod policy_incentives;

// Re-export key types
pub use supranational::*;
pub use policy_incentives::*;

use bevy::prelude::*;
use lw_core::{Fixed32, Vec2fx};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::components::individual::SocialClass;
use super::policy_incentives::PolicyType;

/// Core government types - how decisions get made
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub enum Government {
    Monarchy {
        ruler: Entity,                    // The monarch
        succession: SuccessionLaw,        // How power transfers
        court: Vec<Entity>,              // Advisors (filter information)
        legitimacy: Fixed32,             // 0-1, popular acceptance
        centralization: Fixed32,         // 0-1, royal control vs local nobles
    },
    
    Democracy {
        voters: Vec<Entity>,             // Citizens who vote
        parties: Vec<PoliticalParty>,    // Competing factions
        institutions: DemocraticInstitutions,
        voter_information: Fixed32,      // 0-1, how well informed are voters?
        participation_rate: Fixed32,     // 0-1, who actually votes?
    },
    
    Oligarchy {
        elite: Vec<Entity>,              // The ruling class
        selection_method: EliteSelection, // How elites are chosen
        coordination: Fixed32,           // 0-1, can elites cooperate?
        elite_competence: Fixed32,       // 0-1, are they capable?
        popular_support: Fixed32,        // 0-1, do people accept this?
    },
    
    Theocracy {
        clergy: ReligiousHierarchy,      // Religious authorities
        doctrine: BeliefSystem,          // What they believe
        faith_level: Fixed32,            // 0-1, population's religious belief
        interpretation_flexibility: Fixed32, // 0-1, can doctrine adapt?
    },
    
    Republic {
        representatives: Vec<Entity>,     // Elected officials
        constitution: Constitution,       // Rules and constraints
        checks_balances: Fixed32,        // 0-1, power distribution
        corruption_resistance: Fixed32,   // 0-1, institutional integrity
    },
    
    Anarchy {
        local_authorities: Vec<Entity>,   // Ad hoc leaders
        voluntary_cooperation: Fixed32,   // 0-1, do people self-organize?
        conflict_resolution: ConflictSystem, // How disputes are handled
        external_threats: Fixed32,       // 0-1, pressure from neighbors
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SuccessionLaw {
    Hereditary { gender_rules: GenderRules },
    Elective { electors: Vec<Entity> },
    Meritocratic { selection_criteria: Vec<MeritCriterion> },
    Military { coup_probability: Fixed32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GenderRules {
    MaleOnly,
    FemaleOnly,
    Primogeniture,  // Eldest child
    Equal,          // Gender irrelevant
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MeritCriterion {
    Intelligence,
    Military,
    Economic,
    Popular,
    Religious,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PoliticalParty {
    pub name: String,
    pub ideology: Ideology,
    pub support_base: Vec<SocialClass>,  // Who votes for them
    pub platform: Vec<PolicyProposal>,   // What they promise
    pub competence: Fixed32,             // 0-1, ability to govern
    pub corruption: Fixed32,             // 0-1, self-serving behavior
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Ideology {
    Conservative { tradition_strength: Fixed32 },
    Liberal { reform_enthusiasm: Fixed32 },
    Socialist { redistribution_level: Fixed32 },
    Nationalist { expansionism: Fixed32 },
    Religious { doctrine_strictness: Fixed32 },
    Technocratic { expertise_focus: Fixed32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyProposal {
    pub policy_type: PolicyType,
    pub popularity: HashMap<SocialClass, Fixed32>, // Who likes it
    pub cost: Fixed32,                             // Implementation cost
    pub effectiveness: Fixed32,                    // How well it works
    pub time_horizon: Fixed32,                     // When benefits appear
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DemocraticInstitutions {
    pub legislature: Legislature,
    pub judiciary: Judiciary,
    pub bureaucracy: Bureaucracy,
    pub media: Media,
    pub civil_society: Vec<CivicOrganization>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Legislature {
    pub representatives: Vec<Entity>,
    pub election_method: ElectionMethod,
    pub term_length: Fixed32,
    pub professionalization: Fixed32,  // 0-1, career politicians vs amateurs
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Judiciary {
    pub judges: Vec<Entity>,
    pub independence: Fixed32,         // 0-1, free from political pressure
    pub competence: Fixed32,          // 0-1, legal expertise
    pub corruption: Fixed32,          // 0-1, bought judges
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bureaucracy {
    pub civil_servants: Vec<Entity>,
    pub meritocracy: Fixed32,         // 0-1, hired on merit vs connections
    pub efficiency: Fixed32,          // 0-1, how well they execute policy
    pub politicization: Fixed32,      // 0-1, partisan vs neutral
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Media {
    pub outlets: Vec<MediaOutlet>,
    pub press_freedom: Fixed32,       // 0-1, can they criticize government?
    pub accuracy: Fixed32,            // 0-1, do they report truth?
    pub bias: Fixed32,               // 0-1, partisan slant
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaOutlet {
    pub name: String,
    pub reach: Fixed32,              // 0-1, what % of population
    pub credibility: Fixed32,        // 0-1, do people trust it?
    pub funding: FundingSource,      // Who pays for it?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FundingSource {
    Government,
    Private,
    Advertising,
    Subscription,
    Donations,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CivicOrganization {
    pub name: String,
    pub purpose: CivicPurpose,
    pub membership: Vec<Entity>,
    pub influence: Fixed32,          // 0-1, political clout
    pub independence: Fixed32,       // 0-1, autonomous vs captured
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CivicPurpose {
    Professional,    // Guilds, unions
    Religious,       // Churches, temples
    Educational,     // Schools, universities
    Charitable,      // Helping the poor
    Advocacy,        // Promoting causes
    Social,          // Clubs, fraternities
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ElectionMethod {
    FirstPastPost,      // Winner takes all
    Proportional,       // Seats by vote share
    TwoRound,          // Runoff elections
    RankedChoice,      // Preferential voting
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EliteSelection {
    Wealth { minimum_fortune: Fixed32 },
    Birth { noble_bloodlines: Vec<Entity> },
    Merit { achievements: Vec<MeritCriterion> },
    Military { officer_ranks: Vec<Entity> },
    Religious { clergy_hierarchy: Vec<Entity> },
    Party { political_membership: Entity },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReligiousHierarchy {
    pub high_priest: Entity,
    pub clergy_levels: Vec<ClergyLevel>,
    pub theological_schools: Vec<Entity>,
    pub sacred_texts: Vec<SacredText>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClergyLevel {
    pub rank: String,
    pub members: Vec<Entity>,
    pub authority: Fixed32,      // 0-1, decision-making power
    pub popular_respect: Fixed32, // 0-1, how much people listen
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeliefSystem {
    pub core_doctrines: Vec<Doctrine>,
    pub moral_codes: Vec<MoralRule>,
    pub rituals: Vec<Ritual>,
    pub adaptability: Fixed32,   // 0-1, can beliefs evolve?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Doctrine {
    pub principle: String,
    pub interpretation: Vec<String>, // Different views possible
    pub adherence: Fixed32,         // 0-1, how strictly followed
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoralRule {
    pub behavior: String,          // What's required/forbidden
    pub severity: Fixed32,         // 0-1, how important
    pub enforcement: Fixed32,      // 0-1, actually punished?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ritual {
    pub occasion: String,
    pub participants: ParticipantType,
    pub social_function: SocialFunction,
    pub participation_rate: Fixed32, // 0-1, who actually does it
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ParticipantType {
    All,
    Clergy,
    Elite,
    Adults,
    Specific(SocialClass),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SocialFunction {
    Bonding,        // Builds community
    Status,         // Shows hierarchy
    Teaching,       // Transmits values
    Healing,        // Psychological comfort
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Constitution {
    pub articles: Vec<ConstitutionalArticle>,
    pub amendment_difficulty: Fixed32,    // 0-1, how hard to change
    pub enforcement_strength: Fixed32,    // 0-1, actually followed?
    pub popular_legitimacy: Fixed32,      // 0-1, accepted by people
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstitutionalArticle {
    pub subject: ConstitutionalSubject,
    pub rules: Vec<Rule>,
    pub exceptions: Vec<Exception>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConstitutionalSubject {
    ExecutivePower,
    LegislativePower,
    JudicialPower,
    CitizenRights,
    Property,
    Religion,
    Military,
    Taxation,
    Trade,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    pub requirement: String,
    pub enforcement: EnforcementMechanism,
    pub penalties: Vec<PenaltyType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Exception {
    pub condition: String,
    pub alternative_rule: String,
    pub duration: Option<Fixed32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EnforcementMechanism {
    Courts,
    Legislature,
    PopularVote,
    Military,
    Tradition,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PenaltyType {
    Removal,
    Fine(Fixed32),
    Imprisonment(Fixed32),
    PublicShame,
    LossOfRights,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConflictSystem {
    Mediation { mediators: Vec<Entity> },
    Arbitration { arbitrators: Vec<Entity> },
    Combat { rules: CombatRules },
    Ostracism { community_decision: bool },
    Compensation { payment_scales: Vec<CompensationScale> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombatRules {
    pub weapons_allowed: Vec<WeaponType>,
    pub victory_conditions: VictoryCondition,
    pub third_party_intervention: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WeaponType {
    None,      // Fists only
    Blunt,     // Clubs, sticks
    Bladed,    // Swords, knives
    Ranged,    // Bows, guns
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VictoryCondition {
    FirstBlood,
    Submission,
    Death,
    Judges,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompensationScale {
    pub offense: String,
    pub payment: Fixed32,
    pub payment_form: PaymentForm,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PaymentForm {
    Money,
    Labor,
    Goods,
    Service,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SacredText {
    pub title: String,
    pub authority: Fixed32,        // 0-1, how binding
    pub interpretation_schools: Vec<String>, // Different readings
    pub literacy_requirement: bool, // Must read to understand?
}