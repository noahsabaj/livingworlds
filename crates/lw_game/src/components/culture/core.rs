//! Culture System - Ideas Through Human Contact
//! 
//! NO abstract "culture points" - ideas spread through actual human interaction.
//! Adoption based on utility, not random diffusion.

use bevy::prelude::*;
use lw_core::{Fixed32, Vec2fx};
use serde::{Deserialize, Serialize};
use super::super::individual::*;
use super::super::common::bounded_types::Percentage;
use super::contact::ContactType;
use crate::types::GameTime;
use crate::components::economics::GoodType;
use std::collections::HashMap;

/// Cultural group with shared beliefs, practices, and identity
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Culture {
    pub name: String,
    pub language: Language,
    pub beliefs: BeliefSystem,
    pub practices: Vec<CulturalPractice>,
    pub values: Vec<CulturalValue>,
    pub identity_markers: Vec<IdentityMarker>,
    pub adaptability: Fixed32,        // 0-1, how easily culture changes
    pub cohesion: Fixed32,           // 0-1, how unified the culture is
    pub average_wealth: Fixed32,     // Economic prosperity of culture members
    pub technology_level: Fixed32,   // Technological advancement of the culture
    pub prestige: Fixed32,           // Cultural influence and attractiveness
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub family: LanguageFamily,
    pub speakers: Vec<Entity>,       // Individuals who speak it
    pub literacy_rate: Fixed32,      // 0-1, who can read/write
    pub trade_utility: Fixed32,      // 0-1, useful for commerce
    pub prestige: Fixed32,           // 0-1, social status from knowing it
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LanguageFamily {
    IndoEuropean,
    SinoTibetan,
    Niger_Congo,
    Afroasiatic,
    Austronesian,
    Trans_NewGuinea,
    Isolated,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeliefSystem {
    pub cosmology: Cosmology,        // How the world works
    pub morality: MoralSystem,       // Right and wrong
    pub afterlife: AfterlifeBelief,  // What happens after death
    pub supernatural: Vec<SupernaturalBeing>, // Gods, spirits, etc.
    pub authority_source: AuthoritySource, // Who has the right to lead
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cosmology {
    pub creation_story: String,
    pub natural_laws: Vec<String>,   // How they think nature works
    pub human_place: String,         // Role of humans in cosmos
    pub predictive_power: Fixed32,   // 0-1, does it help predict reality?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoralSystem {
    pub virtues: Vec<Virtue>,
    pub taboos: Vec<Taboo>,
    pub reciprocity_rules: Vec<ReciprocityRule>,
    pub punishment_system: PunishmentSystem,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Virtue {
    pub name: String,
    pub description: String,
    pub social_benefit: Fixed32,     // 0-1, how much it helps society
    pub individual_cost: Fixed32,    // 0-1, personal sacrifice required
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Taboo {
    pub forbidden_action: String,
    pub severity: Fixed32,           // 0-1, how serious the violation
    pub enforcement: Fixed32,        // 0-1, actually punished?
    pub practical_reason: Option<String>, // Real-world benefit of taboo
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReciprocityRule {
    pub relationship: String,        // Parent-child, friend, stranger
    pub obligation: String,          // What you owe them
    pub expectation: String,         // What they owe you
    pub enforcement: EnforcementType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EnforcementType {
    Social,      // Shame, ostracism
    Legal,       // Courts, fines
    Religious,   // Divine punishment
    Personal,    // Individual retaliation
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AfterlifeBelief {
    None,        // Death is the end
    Reincarnation { karma_system: bool },
    Heaven { entry_requirements: Vec<String> },
    Ancestor { continuing_influence: Fixed32 },
    Unknown,     // Agnostic
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupernaturalBeing {
    pub name: String,
    pub domain: String,              // What they control
    pub disposition: Fixed32,        // -1 to 1, hostile to friendly
    pub power_level: Fixed32,        // 0-1, how mighty
    pub intervention_frequency: Fixed32, // 0-1, how often they act
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AuthoritySource {
    Divine,      // God-given right
    Ancestral,   // Traditional leadership
    Merit,       // Best qualified should lead
    Consent,     // Popular will
    Force,       // Might makes right
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PunishmentSystem {
    Restorative { compensation_scales: Vec<CompensationRule> },
    Retributive { punishment_levels: Vec<PunishmentLevel> },
    Exile { offense_thresholds: Vec<Fixed32> },
    Divine { supernatural_enforcement: bool },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompensationRule {
    pub offense: String,
    pub victim_compensation: Fixed32,
    pub community_service: Fixed32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PunishmentLevel {
    pub offense_type: String,
    pub severity: Fixed32,
    pub deterrent_effect: Fixed32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PracticeType {
    Economic { efficiency_bonus: Fixed32 },
    Social { cohesion_bonus: Fixed32 },
    Religious { spiritual_fulfillment: Fixed32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CulturalPractice {
    pub name: String,
    pub practice_type: PracticeType,  // Type and benefits of this practice
    pub occasion: PracticeOccasion,
    pub participants: ParticipationType,
    pub social_function: SocialFunction,
    pub resource_cost: Fixed32,      // How expensive to maintain
    pub practical_benefit: Fixed32,  // Real-world utility
    pub participation_rate: Fixed32, // 0-1, who actually does it
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PracticeOccasion {
    Daily,
    Weekly,
    Seasonal,
    LifeEvent { event_type: String },
    Crisis { crisis_type: String },
    Celebration { celebration_type: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ParticipationType {
    Everyone,
    Adults,
    Elders,
    Specialists { specialist_type: String },
    Class { social_class: SocialClass },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SocialFunction {
    Bonding,          // Builds community solidarity
    Status,           // Reinforces hierarchy
    Teaching,         // Transmits knowledge/values
    Healing,          // Psychological/physical health
    Coordination,     // Organizes group action
    Identity,         // Marks group membership
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CulturalValue {
    pub name: String,
    pub description: String,
    pub behavioral_implications: Vec<String>, // How it affects actions
    pub strength: Fixed32,                   // 0-1, how strongly held
    pub universality: Fixed32,               // 0-1, shared across all members
}

/// Collection of cultural values held by a group or individual
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct CulturalValues {
    pub values: Vec<CulturalValue>,
    pub dominant_value: Option<String>,  // Name of most important value
    pub flexibility: Fixed32,             // 0-1, how open to change
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentityMarker {
    pub marker_type: IdentityMarkerType,
    pub visibility: Fixed32,         // 0-1, how obvious to outsiders
    pub cost: Fixed32,              // Resource investment required
    pub exclusivity: Fixed32,       // 0-1, unique to this culture
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IdentityMarkerType {
    Clothing { style: String },
    Food { cuisine: String },
    Architecture { building_style: String },
    Art { artistic_tradition: String },
    Music { musical_style: String },
    Body_Modification { modification_type: String },
    Behavior { behavioral_pattern: String },
}

/// How culture spreads through human contact
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct CultureTransmission {
    pub transmission_vectors: Vec<TransmissionVector>,
    pub resistance_factors: Vec<ResistanceFactor>,
    pub adoption_patterns: Vec<AdoptionPattern>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransmissionVector {
    pub vector_type: VectorType,
    pub strength: Fixed32,           // 0-1, how effective
    pub selectivity: Fixed32,        // 0-1, transmits everything vs specific ideas
    pub bidirectionality: Fixed32,   // 0-1, ideas flow both ways
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VectorType {
    Trade {
        route: Entity,              // Trade route entity
        volume: Fixed32,            // Amount of contact
        merchant_influence: Fixed32, // How much merchants shape culture
    },
    Migration {
        source_culture: Entity,
        destination: Entity,
        migrant_count: Fixed32,
        integration_success: Fixed32, // How well they fit in
    },
    Conquest {
        conquering_culture: Entity,
        conquered_culture: Entity,
        military_dominance: Fixed32,
        cultural_policy: ConquestPolicy, // How conquerors treat local culture
    },
    Intermarriage {
        culture_a: Entity,
        culture_b: Entity,
        marriage_rate: Fixed32,
        child_cultural_outcome: CulturalOutcome,
    },
    Pilgrimage {
        destination: Entity,        // Holy site
        pilgrim_cultures: Vec<Entity>,
        religious_intensity: Fixed32,
    },
    Education {
        teacher_culture: Entity,
        student_cultures: Vec<Entity>,
        curriculum_bias: Fixed32,   // How much cultural content
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConquestPolicy {
    Assimilation { forced: bool },  // Make them like us
    Tolerance,                      // Let them be different
    Exploitation,                   // Use them, don't change them
    Genocide,                       // Eliminate their culture
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CulturalOutcome {
    ParentCulture(Entity),          // Child follows one parent's culture
    Hybrid,                         // New mixed culture
    Dominant,                       // Locally dominant culture wins
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResistanceFactor {
    pub factor_type: ResistanceType,
    pub strength: Fixed32,           // 0-1, how much it blocks transmission
    pub scope: Vec<CulturalAspect>,  // What aspects it protects
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ResistanceType {
    Geographic { barrier_type: String }, // Mountains, oceans, deserts
    Linguistic { language_barrier: Fixed32 }, // Can't communicate
    Religious { doctrinal_conflict: Fixed32 }, // Incompatible beliefs
    Economic { competition: Fixed32 },    // Cultural practices compete
    Political { state_policy: String },  // Government enforces culture
    Social { elite_resistance: Fixed32 }, // Leaders oppose change
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CulturalAspect {
    Language,
    Religion,
    Customs,
    Technology,
    Art,
    Values,
    Identity,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdoptionPattern {
    pub cultural_element: CulturalElement,
    pub adoption_rate: Fixed32,      // How fast it spreads
    pub adoption_threshold: Fixed32, // Critical mass needed
    pub utility_factor: Fixed32,    // How useful it is
    pub prestige_factor: Fixed32,   // Social status from adoption
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CulturalElement {
    pub name: String,
    pub element_type: CulturalAspect,
    pub complexity: Fixed32,         // 0-1, how hard to learn
    pub prerequisites: Vec<String>,  // What you need first
    pub benefits: Vec<CulturalBenefit>,
    pub costs: Vec<CulturalCost>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CulturalBenefit {
    pub benefit_type: BenefitType,
    pub magnitude: Fixed32,          // How much benefit
    pub beneficiaries: Vec<SocialClass>, // Who gets the benefit
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BenefitType {
    Economic,    // Makes money, saves time
    Social,      // Status, acceptance, connections
    Practical,   // Solves real problems
    Spiritual,   // Meaning, purpose, comfort
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CulturalCost {
    pub cost_type: CostType,
    pub magnitude: Fixed32,          // How much it costs
    pub bearers: Vec<SocialClass>,   // Who pays the cost
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CostType {
    Economic,    // Money, time, resources
    Social,      // Relationships, status
    Psychological, // Stress, identity confusion
    Opportunity, // What you give up
}

// All Culture logic moved to systems/culture_logic.rs
// Components should be pure data - no methods!

#[derive(Clone, Debug)]
pub enum AdoptionDecision {
    Adopt { 
        enthusiasm: Fixed32,
        adaptation: CulturalPractice,
    },
    PartialAdopt {
        modified_practice: CulturalPractice,
    },
    Reject {
        reasons: Vec<String>,
    },
}

/// System for culture transmission through human contact
pub fn culture_transmission_system(
    mut individuals: Query<(&mut Individual, &Transform)>,
    cultures: Query<&Culture>,
    transmission_vectors: Query<&CultureTransmission>,
    time: Res<Time>,
) {
    let dt = time.delta().as_secs_f32();
    
    // Process each transmission vector
    for transmission in transmission_vectors.iter() {
        for vector in &transmission.transmission_vectors {
            match &vector.vector_type {
                VectorType::Trade { route: _, volume, merchant_influence } => {
                    // Find individuals along trade routes
                    process_trade_cultural_exchange(&mut individuals, &cultures, 
                                                  *volume, *merchant_influence, dt);
                },
                
                VectorType::Migration { source_culture, destination, migrant_count, integration_success } => {
                    process_migration_cultural_impact(&mut individuals, &cultures,
                                                    *source_culture, *destination, 
                                                    *migrant_count, *integration_success, dt);
                },
                
                VectorType::Intermarriage { culture_a, culture_b, marriage_rate, child_cultural_outcome } => {
                    process_intermarriage_effects(&mut individuals, &cultures,
                                                *culture_a, *culture_b, 
                                                *marriage_rate, child_cultural_outcome, dt);
                },
                
                // Handle other vector types...
                _ => {},
            }
        }
    }
}

fn process_trade_cultural_exchange(
    individuals: &mut Query<(&mut Individual, &Transform)>,
    cultures: &Query<&Culture>,
    volume: Fixed32,
    merchant_influence: Fixed32,
    dt: f32,
) {
    // Cultural exchange along trade routes
    let exchange_rate = volume * merchant_influence * Fixed32::from_float(dt);
    
    // Find merchants and people near trade centers
    for (mut individual, _transform) in individuals.iter_mut() {
        if matches!(individual.current_job, Some(JobType::Merchant)) {
            // Merchants are cultural vectors - they pick up and spread ideas
            if exchange_rate > Fixed32::from_float(0.1) {
                // Merchant learns new practices/languages
                let location = individual.location; // Store location to avoid borrowing conflict
                individual.knowledge.rumors.push(Rumor {
                    content: RumorType::Trade { 
                        opportunity: "New cultural practice observed".to_string(),
                        location 
                    },
                    reliability: Percentage::new(Fixed32::from_float(0.7)),
                    age: Fixed32::ZERO,
                });
            }
        }
    }
}

fn process_migration_cultural_impact(
    individuals: &mut Query<(&mut Individual, &Transform)>,
    _cultures: &Query<&Culture>,
    _source_culture: Entity,
    _destination: Entity,
    migrant_count: Fixed32,
    integration_success: Fixed32,
    _dt: f32,
) {
    // Migrants bring their culture and adapt to new one
    let cultural_change_rate = migrant_count * integration_success;
    
    // Find migrants and locals in destination
    for (mut individual, _transform) in individuals.iter_mut() {
        // If this person is in contact with migrants
        if cultural_change_rate > Fixed32::from_float(0.2) {
            // Cultural mixing occurs
            // TODO: Implement actual cultural change mechanics
        }
    }
}

fn process_intermarriage_effects(
    _individuals: &mut Query<(&mut Individual, &Transform)>,
    _cultures: &Query<&Culture>,
    _culture_a: Entity,
    _culture_b: Entity,
    marriage_rate: Fixed32,
    _child_cultural_outcome: &CulturalOutcome,
    _dt: f32,
) {
    // Intermarriage creates cultural bridges and hybrid practices
    let cultural_blending = marriage_rate * Fixed32::from_float(0.5);
    
    if cultural_blending > Fixed32::from_float(0.3) {
        // Significant cultural mixing happening
        // TODO: Create new hybrid cultural practices
    }
}

/// Contact network for cultural transmission through human interaction
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ContactNetwork {
    pub active_contacts: Vec<CulturalContact>,
    pub contact_frequency: Fixed32,
    pub network_reach: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalContact {
    pub other_culture: Entity,
    pub contact_intensity: Fixed32,
    pub transmission_rate: Fixed32,
    pub contact_type: ContactType,
}

// ContactType moved to contact.rs to avoid duplication