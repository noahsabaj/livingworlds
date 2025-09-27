//! Drama event system for generating viral moments and emergent narratives
//!
//! This module creates the dramatic events that make Living Worlds shareable.
//! Events range from personal scandals to epic betrayals, creating stories
//! that players will want to share on social media.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use rand::Rng;

use super::characters::{
    Character, CharacterId, CharacterRole, RelationshipType,
    Secret, Scandal, LifeEvent, Quirk
};
use crate::nations::NationId;
use crate::simulation::GameTime;

/// A dramatic event that creates shareable moments
#[derive(Debug, Clone, Event, Serialize, Deserialize, Reflect)]
pub struct DramaEvent {
    pub id: DramaEventId,
    pub event_type: DramaEventType,
    pub participants: Vec<CharacterId>,
    pub importance: EventImportance, // How "viral" this is
    pub visibility: EventVisibility,
    pub consequences: Vec<EventConsequence>,
    pub timestamp: u32, // Game year when it happened
    pub resolved: bool,
}

/// Unique identifier for drama events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Component, Reflect)]
pub struct DramaEventId(pub u32);

/// How important/shareable an event is
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum EventImportance {
    Trivial,     // Normal gameplay
    Notable,     // Worth mentioning
    Significant, // Players might screenshot
    Major,       // Definitely shareable
    Legendary,   // Goes viral on TikTok
}

/// Who knows about this event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum EventVisibility {
    Secret,      // Only participants know
    Rumor,       // Starting to spread
    CourtGossip, // Known in the palace
    Public,      // Everyone knows
    Legendary,   // Will be remembered forever
}

/// Types of dramatic events
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum DramaEventType {
    // ===== RELATIONSHIP DRAMA =====

    /// Secret affair discovered
    AffairRevealed {
        lovers: (String, String), // Names for narrative
        discovered_by: String,
        spouse_reaction: SpouseReaction,
    },

    /// Unexpected marriage
    SurpriseMarriage {
        bride: String,
        groom: String,
        reason: MarriageReason,
    },

    /// Love triangle drama
    LoveTriangle {
        person_a: String,
        person_b: String,
        person_c: String,
        outcome: LoveTriangleOutcome,
    },

    // ===== SUCCESSION DRAMA =====

    /// Bastard claims throne
    BastardClaim {
        claimant: String,
        current_ruler: String,
        proof_revealed: bool,
    },

    /// Succession crisis
    SuccessionCrisis {
        claimants: Vec<String>,
        cause: SuccessionCrisisCause,
    },

    /// Unexpected heir
    UnexpectedHeir {
        new_heir: String,
        reason: String, // "all others died of plague"
        age: u32, // Creates drama if very young/old
    },

    // ===== BETRAYAL & CONSPIRACY =====

    /// Coup attempt
    CoupAttempt {
        conspirator: String,
        target: String,
        success: bool,
        method: CoupMethod,
    },

    /// Betrayal revealed
    Betrayal {
        betrayer: String,
        betrayed: String,
        nature: BetrayalType,
    },

    /// Secret exposed
    SecretExposed {
        character: String,
        secret: Secret,
        exposed_by: ExposureMethod,
    },

    // ===== PERSONAL DRAMA =====

    /// Character goes mad
    DescentIntoMadness {
        character: String,
        trigger: MadnessTrigger,
        first_sign: String, // "declared war on the ocean"
    },

    /// Quirk causes incident
    QuirkIncident {
        character: String,
        quirk: Quirk,
        incident: String, // "Refused to sign treaty because afraid of water"
    },

    /// Duel for honor
    Duel {
        challenger: String,
        challenged: String,
        reason: DuelReason,
        winner: Option<String>,
        consequence: DuelConsequence,
    },

    // ===== FAMILY DRAMA =====

    /// Parent-child conflict
    FamilyFeud {
        parent: String,
        child: String,
        cause: FeudCause,
    },

    /// Sibling rivalry
    SiblingRivalry {
        sibling_a: String,
        sibling_b: String,
        escalation: RivalryEscalation,
    },

    /// Inheritance dispute
    InheritanceDispute {
        claimants: Vec<String>,
        disputed_item: String, // "the throne", "grandfather's sword"
    },

    // ===== SCANDAL & REPUTATION =====

    /// Public scandal
    PublicScandal {
        character: String,
        scandal: Scandal,
        public_reaction: PublicReaction,
    },

    /// Drunk incident
    DrunkenIncident {
        character: String,
        incident: String, // "Declared moon independence"
        witnesses: Vec<String>,
    },

    /// Religious controversy
    ReligiousControversy {
        character: String,
        action: String, // "married a horse to prove divine right"
        church_response: ChurchResponse,
    },

    // ===== UNEXPECTED EVENTS =====

    /// Child prodigy
    ChildProdigy {
        child: String,
        age: u32,
        achievement: String, // "Negotiated peace at age 7"
    },

    /// Miraculous recovery
    MiraculousRecovery {
        character: String,
        from: String, // "death's door", "madness"
    },

    /// Bizarre coincidence
    BizarreCoincidence {
        description: String, // "Three rulers died on same day"
        affected: Vec<String>,
    },

    // ===== VIRAL MOMENT EVENTS =====

    /// Baby does something unexpected
    BabyRuler {
        baby: String,
        age_months: u32,
        action: String, // "signed peace treaty with drool"
    },

    /// Animal becomes important
    AnimalIncident {
        animal_type: String, // "cat", "horse", "parrot"
        incident: String, // "inherited the throne"
        character_reaction: String,
    },

    /// Completely absurd event
    AbsurdEvent {
        description: String, // "Declared war on concept of Tuesday"
        perpetrator: String,
        reasoning: String, // Their "logical" explanation
    },

    // ===== ADDITIONAL DRAMA EVENTS =====

    /// Romantic proposal event
    RomanticProposal {
        proposer: String,
        recipient: String,
        accepted: bool,
        context: String, // "during battle", "at funeral", etc.
    },

    /// Former enemies become allies
    EnemiesAlly {
        former_enemy_a: String,
        former_enemy_b: String,
        reason: String, // What brought them together
    },

    /// Accidental death
    AccidentalDeath {
        deceased: String,
        cause: String, // "fell off horse", "poisoned by own plot"
        witnesses: Vec<String>,
    },

    /// Deathbed confession
    DeathbedConfession {
        confessor: String,
        confession: String, // What they revealed
        impact: String, // How it affects others
    },

    /// Personal duel between characters
    PersonalDuel {
        challenger: String,
        challenged: String,
        reason: String,
        winner: Option<String>,
    },

    /// Leader becomes disgraced
    DisgracedLeader {
        leader: String,
        disgrace_reason: String,
        public_reaction: String,
    },

    /// Heroic sacrifice
    HeroicSacrifice {
        hero: String,
        saved: Vec<String>,
        sacrifice_type: String, // What they gave up
    },
}

/// Consequences of dramatic events
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum EventConsequence {
    // Relationship changes
    RelationshipChange {
        character_a: CharacterId,
        character_b: CharacterId,
        new_relationship: RelationshipType,
    },

    // Status changes
    ReputationChange {
        character: CharacterId,
        change: f32,
    },

    StressIncrease {
        character: CharacterId,
        amount: f32,
    },

    // Role changes
    RoleChange {
        character: CharacterId,
        new_role: CharacterRole,
    },

    Exile {
        character: CharacterId,
        duration_years: Option<u32>,
    },

    // New events triggered
    TriggerEvent {
        event_type: DramaEventType,
        delay_years: u32,
    },

    // War and conflict
    WarDeclared {
        nation_a: NationId,
        nation_b: NationId,
        casus_belli: String,
    },

    // Death (the ultimate consequence)
    Death {
        character: CharacterId,
        cause: DeathCause,
    },
}

// Enums for event variations
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum SpouseReaction {
    Rage,
    Forgiveness,
    JoinIn, // The truly unexpected!
    Revenge,
    Divorce,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum MarriageReason {
    Love,
    Political,
    Shotgun, // Had to marry quickly
    Drunk,   // Married while drunk
    Blackmail,
    ProphecyFulfillment,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum LoveTriangleOutcome {
    ChooseOne,
    ChooseNeither,
    Polyamory, // Modern solution!
    Duel,
    Murder,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum SuccessionCrisisCause {
    NoHeir,
    TooManyHeirs,
    DisputedLegitimacy,
    ForeignClaimant,
    ElectiveMonarchy,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum CoupMethod {
    Military,
    Assassination,
    PopularRevolt,
    PalaceIntrigue,
    ReligiousBacking,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum BetrayalType {
    Political,
    Military,
    Personal,
    Financial,
    Religious,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum ExposureMethod {
    InvestigationA,
    Confession,
    AccidentalReveal,
    DeathbedConfession,
    ChildBlurtedOut, // Kids say the darndest things!
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum MadnessTrigger {
    Grief,
    Stress,
    Disease,
    CurseLegacy,
    TooMuchPower,
    AlwaysWasMad, // Just hid it well
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum DuelReason {
    Honor,
    Love,
    Insult,
    Inheritance,
    JustBecause, // Some people just like dueling
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum DuelConsequence {
    Death,
    Injury,
    Humiliation,
    Respect,
    MarriageProposal, // Impressed by skills!
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum FeudCause {
    Succession,
    Marriage,
    Religion,
    Lifestyle,
    Favoritism,
    JustDislike,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum RivalryEscalation {
    PettyPranks,
    PublicHumiliation,
    Sabotage,
    Violence,
    KinslayingAttempt,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum PublicReaction {
    Outrage,
    Support,
    Amusement,
    Indifference,
    CopyBehavior, // It becomes trendy!
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum ChurchResponse {
    Excommunication,
    Blessing, // They like it actually
    Inquisition,
    MakeSaint, // Somehow becomes holy
    SchismCreated,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum DeathCause {
    Natural,
    Battle,
    Assassination,
    Duel,
    Accident(String), // "choked on grape", "fell off horse"
    Heartbreak,
    Madness,
    Mystery, // Creates speculation
}

/// System for generating drama events based on character states
pub fn generate_drama_events(
    characters: Query<(Entity, &Character, &FamilyMember)>,
    relationships: Query<&HasRelationship>,
    mut events: EventWriter<DramaEvent>,
    time: Res<GameTime>,
    mut rng: ResMut<GlobalRng>,
) {
    use rand::Rng;
    use rand::seq::SliceRandom;

    // Check for potential drama situations
    for (entity, character, family) in &characters {
        // Mad characters create incidents
        if character.personality.madness > 0.7 && rng.gen_bool(0.1) {
            generate_madness_event(character, &mut events, &time, &mut rng);
        }

        // Stressed characters might snap
        if character.stress > 0.9 && rng.gen_bool(0.05) {
            generate_stress_event(character, &mut events, &time, &mut rng);
        }

        // Quirks cause problems
        if !character.quirks.is_empty() && rng.gen_bool(0.02) {
            generate_quirk_incident(character, &mut events, &time, &mut rng);
        }

        // Secrets might be revealed
        if !character.secrets.is_empty() && rng.gen_bool(0.01) {
            generate_secret_reveal(character, &mut events, &time, &mut rng);
        }

        // Young rulers create viral moments
        if character.age < 10 && character.role == CharacterRole::Ruler && rng.gen_bool(0.2) {
            generate_baby_ruler_event(character, &mut events, &time, &mut rng);
        }
    }

    // Check relationships for drama
    check_relationship_drama(&relationships, &characters, &mut events, &time, &mut rng);
}

// Helper functions for specific event generation
fn generate_madness_event(
    character: &Character,
    events: &mut EventWriter<DramaEvent>,
    time: &Res<GameTime>,
    rng: &mut ResMut<GlobalRng>,
) {
    use rand::seq::SliceRandom;

    let mad_actions = vec![
        "declared war on the concept of Wednesday",
        "appointed their horse as chancellor",
        "banned the color blue from the kingdom",
        "insisted everyone walk backwards on Tuesdays",
        "declared themselves emperor of the moon",
        "mandated all citizens must speak in rhyme",
        "ordered the execution of all mirrors",
    ];

    let action = mad_actions.choose(&mut rng.0).unwrap();

    events.send(DramaEvent {
        id: DramaEventId(rng.gen()),
        event_type: DramaEventType::DescentIntoMadness {
            character: character.name.clone(),
            trigger: MadnessTrigger::TooMuchPower,
            first_sign: action.to_string(),
        },
        participants: vec![character.id],
        importance: EventImportance::Major,
        visibility: EventVisibility::Public,
        consequences: vec![
            EventConsequence::ReputationChange {
                character: character.id,
                change: -0.3,
            },
            EventConsequence::StressIncrease {
                character: character.id,
                amount: 0.2,
            },
        ],
        timestamp: time.current_year(),
        resolved: false,
    });
}

fn generate_baby_ruler_event(
    character: &Character,
    events: &mut EventWriter<DramaEvent>,
    time: &Res<GameTime>,
    rng: &mut ResMut<GlobalRng>,
) {
    use rand::seq::SliceRandom;

    let baby_actions = vec![
        "signed a peace treaty with drool",
        "appointed their teddy bear as general",
        "declared naptime a national holiday",
        "babbled and everyone pretended it was wisdom",
        "threw food at foreign ambassador, starting a war",
        "crawled away during coronation ceremony",
    ];

    let action = baby_actions.choose(&mut rng.0).unwrap();

    events.send(DramaEvent {
        id: DramaEventId(rng.gen()),
        event_type: DramaEventType::BabyRuler {
            baby: character.name.clone(),
            age_months: character.age * 12,
            action: action.to_string(),
        },
        participants: vec![character.id],
        importance: EventImportance::Legendary, // Always goes viral!
        visibility: EventVisibility::Legendary,
        consequences: vec![],
        timestamp: time.current_year(),
        resolved: true,
    });
}

// Global RNG resource for drama generation
#[derive(Resource)]
pub struct GlobalRng(pub rand::rngs::StdRng);

impl std::ops::Deref for GlobalRng {
    type Target = rand::rngs::StdRng;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for GlobalRng {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<rand::rngs::StdRng> for GlobalRng {
    fn as_ref(&self) -> &rand::rngs::StdRng {
        &self.0
    }
}

impl AsMut<rand::rngs::StdRng> for GlobalRng {
    fn as_mut(&mut self) -> &mut rand::rngs::StdRng {
        &mut self.0
    }
}

impl bevy::ecs::world::FromWorld for GlobalRng {
    fn from_world(_world: &mut bevy::ecs::world::World) -> Self {
        use rand::SeedableRng;
        GlobalRng(rand::rngs::StdRng::from_entropy())
    }
}

// Additional helper functions would go here...
fn generate_stress_event(
    character: &Character,
    events: &mut EventWriter<DramaEvent>,
    time: &Res<GameTime>,
    rng: &mut ResMut<GlobalRng>,
) {
    // Implementation for stress-induced events
}

fn generate_quirk_incident(
    character: &Character,
    events: &mut EventWriter<DramaEvent>,
    time: &Res<GameTime>,
    rng: &mut ResMut<GlobalRng>,
) {
    // Implementation for quirk-based incidents
}

fn generate_secret_reveal(
    character: &Character,
    events: &mut EventWriter<DramaEvent>,
    time: &Res<GameTime>,
    rng: &mut ResMut<GlobalRng>,
) {
    // Implementation for secret revelations
}

fn check_relationship_drama(
    relationships: &Query<&HasRelationship>,
    characters: &Query<(Entity, &Character, &FamilyMember)>,
    events: &mut EventWriter<DramaEvent>,
    time: &Res<GameTime>,
    rng: &mut ResMut<GlobalRng>,
) {
    // Implementation for relationship-based drama
}

use super::characters::{FamilyMember, HasRelationship};