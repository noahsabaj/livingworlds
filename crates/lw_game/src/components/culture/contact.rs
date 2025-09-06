//! Cultural contact and transmission components
//!
//! Culture spreads through actual human contact, not abstract diffusion.
//! People adopt ideas based on utility, prestige, or coercion.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};

/// Contact event between individuals
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ContactEvent {
    pub participants: Vec<Entity>,
    pub location: Entity,
    pub contact_type: ContactType,
    pub duration: u64,
    pub intensity: Fixed32,          // How meaningful was the contact
    pub language_barrier: Fixed32,   // 0 = same language, 1 = no common language
    pub cultural_exchange: Vec<CulturalExchange>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContactType {
    Trade,
    Migration,
    War,
    Diplomacy,
    Marriage,
    Education,
    Religious,
    Tourism,
    Neighborly,
    Occupational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalExchange {
    pub from: Entity,
    pub to: Entity,
    pub idea_type: IdeaType,
    pub idea_content: String,
    pub transmission_success: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdeaType {
    Technology,
    Practice,
    Belief,
    Value,
    Language,
    Art,
    Food,
    Fashion,
    Custom,
    Knowledge,
}

/// Idea adoption tracking for individuals
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdeaAdoption {
    pub individual: Entity,
    pub adopted_ideas: Vec<AdoptedIdea>,
    pub resistance_to_change: Fixed32,
    pub openness: Fixed32,
    pub adoption_criteria: AdoptionCriteria,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdoptedIdea {
    pub idea: IdeaType,
    pub content: String,
    pub source: Entity,              // Who they learned it from
    pub adoption_date: u64,
    pub integration_level: Fixed32,  // How fully adopted
    pub utility_gained: Fixed32,     // Practical benefit
    pub social_pressure: Fixed32,    // Peer influence to adopt
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdoptionCriteria {
    pub utility_weight: Fixed32,     // Care about practical benefit
    pub prestige_weight: Fixed32,    // Care about status
    pub conformity_weight: Fixed32,  // Care about fitting in
    pub tradition_weight: Fixed32,   // Resist change
}

/// Cultural transmission mechanism
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TransmissionMechanism {
    pub mechanism_type: TransmissionType,
    pub effectiveness: Fixed32,
    pub required_contact_time: u64,
    pub prerequisites: Vec<TransmissionPrerequisite>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransmissionType {
    OralTradition,
    WrittenText,
    Demonstration,
    FormalEducation,
    Apprenticeship,
    MassMedia,
    SocialMedia,
    Imitation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransmissionPrerequisite {
    pub prerequisite_type: PrerequisiteType,
    pub requirement: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrerequisiteType {
    Literacy,
    SharedLanguage,
    SocialStatus,
    Technology,
    Trust,
    Authority,
}

/// Cultural barrier component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CulturalBarrier {
    pub barrier_type: BarrierType,
    pub strength: Fixed32,
    pub permeability: Fixed32,       // Can some ideas get through?
    pub selective_filter: Vec<IdeaType>, // What gets blocked
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BarrierType {
    Language,
    Religious,
    Political,
    Geographic,
    Economic,
    Social,
    Technological,
}

/// Cultural convergence tracking
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CulturalConvergence {
    pub cultures: Vec<Entity>,
    pub convergence_rate: Fixed32,
    pub common_elements: Vec<String>,
    pub divergent_elements: Vec<String>,
    pub hybridization: bool,         // Creating new culture?
}

/// Cultural innovation component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CulturalInnovation {
    pub innovator: Entity,
    pub innovation_type: IdeaType,
    pub novelty: Fixed32,            // How different from existing
    pub utility: Fixed32,            // How useful
    pub adoptability: Fixed32,       // How easy to adopt
    pub spread_potential: Fixed32,
}

/// Cultural prestige component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CulturalPrestige {
    pub culture: Entity,
    pub prestige_level: Fixed32,
    pub sources: Vec<PrestigeSource>,
    pub influence_radius: Fixed32,   // How far prestige reaches
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrestigeSource {
    pub source_type: PrestigeType,
    pub contribution: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrestigeType {
    Military,
    Economic,
    Technological,
    Artistic,
    Religious,
    Political,
    Educational,
}

/// Language learning component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct LanguageLearning {
    pub learner: Entity,
    pub languages: Vec<LanguageKnowledge>,
    pub learning_ability: Fixed32,
    pub motivation: LearningMotivation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageKnowledge {
    pub language: String,
    pub proficiency: Fixed32,        // 0 = none, 1 = native
    pub literacy: bool,
    pub accent: Fixed32,             // 0 = native, 1 = heavy
    pub vocabulary_size: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LearningMotivation {
    Trade,
    Migration,
    Education,
    Marriage,
    Conquest,
    Religion,
    Prestige,
}

/// Cultural resistance component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CulturalResistance {
    pub defending_culture: Entity,
    pub resistance_strength: Fixed32,
    pub methods: Vec<ResistanceMethod>,
    pub success_rate: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResistanceMethod {
    Isolation,
    Persecution,
    CounterPropaganda,
    Legislation,
    Violence,
    Education,
    Economic,
}

/// Syncretism component - blending of cultures
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Syncretism {
    pub parent_cultures: Vec<Entity>,
    pub blend_type: SyncretismType,
    pub new_practices: Vec<String>,
    pub stability: Fixed32,          // Will it last?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncretismType {
    Religious,       // Blended beliefs
    Linguistic,      // Creole/pidgin
    Culinary,        // Fusion cuisine
    Artistic,        // Mixed styles
    Political,       // Hybrid systems
    Complete,        // New culture
}

/// Cultural memory component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CulturalMemory {
    pub culture: Entity,
    pub collective_memories: Vec<CollectiveMemory>,
    pub forgetting_rate: Fixed32,
    pub reinforcement_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveMemory {
    pub event: String,
    pub interpretation: String,
    pub emotional_valence: Fixed32,  // -1 = trauma, 1 = triumph
    pub identity_importance: Fixed32,
    pub accuracy: Fixed32,           // How true to actual events
}