//! Character system for house members with personalities, quirks, and drama potential
//!
//! This module adds individual characters to houses, creating the foundation for
//! emergent storytelling and viral social media moments. Each character has
//! relationships, secrets, quirks, and personal events that drive narrative drama.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::name_generator::{Culture, Gender, NameGenerator, NameType, PersonRole};

// Re-export relationship types from the relationships module for convenience
pub use crate::relationships::{
    CharacterRelationshipBundle, HasRelationship, RelatedTo, RelationshipMetadata, RelationshipType,
};

/// A full character with personality, relationships, and drama potential
#[derive(Debug, Clone, Component, Serialize, Deserialize, Reflect)]
pub struct Character {
    pub id: CharacterId,
    pub house_id: Entity, // The house this character belongs to
    // NOTE: Nation can be determined via house_id → House → RulesOver relationship

    // Basic info
    pub name: String,
    pub title: Option<String>, // King, Queen, Duke, etc.
    pub age: u32,
    pub gender: Gender,
    pub culture: Culture,

    // Personality and traits
    pub personality: DetailedPersonality,
    pub quirks: Vec<Quirk>,
    pub secrets: Vec<Secret>,

    // Life events and history
    pub life_events: Vec<LifeEvent>,
    pub achievements: Vec<Achievement>,
    pub scandals: Vec<Scandal>,

    // Stats that affect behavior
    pub health: f32,        // 0.0 to 1.0
    pub stress: f32,        // 0.0 to 1.0, affects decision-making
    pub happiness: f32,     // 0.0 to 1.0, affects loyalty
    pub influence: f32,     // Political power within the house
    pub reputation: f32,    // Public perception

    // Role in the house
    pub role: CharacterRole,
    pub succession_order: Option<u32>, // Position in line for throne
}

/// Unique identifier for a character
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Component, Reflect)]
pub struct CharacterId(pub u32);

/// Role within a house/dynasty
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum CharacterRole {
    Ruler,
    Heir,
    Spouse,
    Child,
    Sibling,
    Cousin,
    Advisor,
    General,
    Courtier,
    Bastard, // Illegitimate children create drama!
}

/// Detailed personality beyond basic traits
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct DetailedPersonality {
    // Core traits (existing)
    pub competence: f32,    // How effective they are
    pub ambition: f32,      // Drive for power
    pub temperament: f32,   // Calm vs volatile
    pub honor: f32,         // Values oaths and reputation

    // Social traits (new for drama)
    pub charisma: f32,      // Social influence
    pub loyalty: f32,       // To family/nation
    pub cruelty: f32,       // Kindness vs cruelty
    pub intelligence: f32,  // Problem-solving ability
    pub courage: f32,       // Bravery vs cowardice

    // Hidden traits (create surprises)
    pub madness: f32,       // Mental stability (can increase over time)
    pub paranoia: f32,      // Suspicion of others
    pub hedonism: f32,      // Pursuit of pleasure
    pub zealotry: f32,      // Religious/ideological fervor
}

/// Quirks that make characters memorable and create viral moments
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum Quirk {
    // Phobias
    AfraidOfWater,
    AfraidOfHeights,
    AfraidOfDarkness,
    AfraidOfCrowds,

    // Obsessions
    ObsessedWithCats,
    ObsessedWithBirds,
    ObsessedWithBooks,
    ObsessedWithGold,
    ObsessedWithCleanliness,
    CollectsWeirdThings(String), // "skulls", "butterflies", "ancient coins"

    // Behavioral
    TalksToThemselves,
    SleepsWithEyesOpen,
    NeverSleeps,
    LaughsAtInappropriateTimes,
    CriesWhenAngry,
    DancesWhenNervous,

    // Physical
    MissingLimb(String), // "left hand", "right eye"
    UnusualAppearance(String), // "purple eyes", "seven fingers"
    AlwaysWearsSomething(String), // "a mask", "gloves", "their mother's ring"

    // Social
    CantKeepSecrets,
    CompulsiveLiar,
    BrutallyHonest,
    FallsInLoveEasily,
    HatesBeingTouched,
    NeverForgets,
    NeverForgives,

    // Abilities
    UncannyMemory,
    PerfectPitch,
    AnimalWhisperer,
    LuckyGambler,
    TerribleAtSomething(String), // "dancing", "lying", "fighting"
}

/// Secrets that can be revealed for drama
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum Secret {
    // Parentage
    Bastard { real_parent: CharacterId },
    SecretlyAdopted,
    RoyalBlood { true_house: String },

    // Relationships
    SecretLover { character: CharacterId },
    SecretMarriage { spouse: CharacterId },
    HiddenChild { child: CharacterId },

    // Crimes
    MurderedSomeone { victim: String, when: u32 },
    Traitor { allied_with: Entity },
    Embezzler { amount: u32 },

    // Personal
    SecretIdentity(String), // "actually a woman", "foreign spy"
    HiddenIllness(String), // "going blind", "has plague"
    DarkPast(String), // "was a pirate", "killed their sibling"
    ForbiddenKnowledge(String), // "knows about the ancient weapon"

    // Conspiracies
    PlottingCoup,
    PlottingAssassination { target: CharacterId },
    PartOfCult { cult_name: String },
}

/// Life events that shape a character
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum LifeEvent {
    // Birth and childhood
    BornDuringEvent(String), // "eclipse", "great battle", "plague"
    ChildhoodTrauma(String), // "witnessed parent's death", "kidnapped by raiders"
    ProdigiousYouth(String), // "youngest general ever", "spoke five languages at age 8"

    // Relationships
    MarriedInto { house: String, political: bool },
    HadAffairWith { character: String, discovered: bool },
    BestFriendsWith { character: String },
    BitterRivalsWith { character: String, reason: String },

    // Achievements
    WonBattle { battle_name: String, against: String },
    NegotiatedPeace { with: String },
    DiscoveredSomething(String), // "new trade route", "ancient ruins"

    // Tragedies
    LostChild { name: String, how: String },
    Betrayed { by: String, how: String },
    Exiled { reason: String, years: u32 },

    // Changes
    ConvertedReligion { from: String, to: String, why: String },
    GainedQuirk(Quirk),
    LostSomething(String), // "left hand in battle", "fortune to gambling"
}

/// Achievements that boost reputation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum Achievement {
    MilitaryVictory(String),
    DiplomaticSuccess(String),
    EconomicProsperity,
    CulturalPatron,
    BuiltWonder(String),
    SavedNationFrom(String), // "plague", "invasion", "bankruptcy"
    LegendaryDuel { defeated: String },
}

/// Scandals that damage reputation and create drama
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum Scandal {
    PublicAffair { with: String },
    BastardRevealed { child: String },
    CaughtInLie { about: String },
    DrunkenIncident(String), // "insulted the pope", "declared war on the moon"
    FinancialRuin { how: String },
    ReligiousHeresy(String),
    CowardiceInBattle,
    BetrayalExposed { betrayed: String },
}

// NOTE: HasRelationship, RelationshipMetadata, RelatedTo, and RelationshipType
// are now defined in crate::relationships::familial and re-exported above.
// They use Bevy's #[relationship] attribute for automatic bidirectional tracking.

/// Component tracking family tree membership
#[derive(Component, Debug, Clone)]
pub struct FamilyMember {
    pub house: Entity,
    pub generation: u32,
    pub branch: FamilyBranch,
}

/// Branch of the family tree
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum FamilyBranch {
    MainLine,      // Direct descendants of founders
    CadetBranch,   // Younger sons' lines
    BastardLine,   // Illegitimate but recognized
    MarriedIn,     // Spouses from other houses
}

// Helper functions for character generation
impl Character {
    /// Generate a new character with randomized traits
    pub fn generate(
        house_id: Entity,
        culture: Culture,
        role: CharacterRole,
        name_gen: &mut NameGenerator,
        rng: &mut impl rand::Rng,
    ) -> Self {
        use rand::Rng;

        let gender = if rng.gen_bool(0.5) { Gender::Male } else { Gender::Female };
        let person_role = match role {
            CharacterRole::Ruler => PersonRole::Noble,
            CharacterRole::General => PersonRole::General,
            CharacterRole::Advisor => PersonRole::Advisor,
            _ => PersonRole::Noble,
        };

        let name = name_gen.generate(NameType::Person {
            gender: gender.clone(),
            culture,
            role: person_role
        });

        // Generate random age based on role
        let age = match role {
            CharacterRole::Child => rng.gen_range(0..16),
            CharacterRole::Heir => rng.gen_range(16..35),
            CharacterRole::Ruler => rng.gen_range(25..70),
            CharacterRole::Advisor | CharacterRole::General => rng.gen_range(30..65),
            _ => rng.gen_range(16..60),
        };

        // Generate personality
        let personality = DetailedPersonality::generate(rng);

        // Generate 1-3 random quirks
        let num_quirks = rng.gen_range(1..=3);
        let quirks = (0..num_quirks)
            .map(|_| Quirk::random(rng))
            .collect();

        // Maybe generate a secret (30% chance)
        let secrets = if rng.gen_bool(0.3) {
            vec![Secret::random(rng)]
        } else {
            vec![]
        };

        Self {
            id: CharacterId(rng.r#gen()),
            house_id,
            name,
            title: None,
            age,
            gender,
            culture,
            personality,
            quirks,
            secrets,
            life_events: vec![],
            achievements: vec![],
            scandals: vec![],
            health: rng.gen_range(0.7..1.0),
            stress: rng.gen_range(0.0..0.3),
            happiness: rng.gen_range(0.3..0.8),
            influence: match role {
                CharacterRole::Ruler => rng.gen_range(0.7..1.0),
                CharacterRole::Heir => rng.gen_range(0.4..0.7),
                _ => rng.gen_range(0.0..0.4),
            },
            reputation: rng.gen_range(0.3..0.7),
            role,
            succession_order: None,
        }
    }
}

impl DetailedPersonality {
    fn generate(rng: &mut impl rand::Rng) -> Self {
        use rand::Rng;
        Self {
            competence: rng.gen_range(0.2..1.0),
            ambition: rng.gen_range(0.0..1.0),
            temperament: rng.gen_range(-1.0..1.0),
            honor: rng.gen_range(0.0..1.0),
            charisma: rng.gen_range(0.0..1.0),
            loyalty: rng.gen_range(0.2..1.0),
            cruelty: rng.gen_range(-0.5..0.5),
            intelligence: rng.gen_range(0.2..1.0),
            courage: rng.gen_range(0.0..1.0),
            madness: rng.gen_range(0.0..0.2), // Usually starts low
            paranoia: rng.gen_range(0.0..0.3),
            hedonism: rng.gen_range(0.0..0.7),
            zealotry: rng.gen_range(0.0..0.5),
        }
    }
}

impl Quirk {
    fn random(rng: &mut impl rand::Rng) -> Self {
        use rand::seq::SliceRandom;
        

        let quirks = vec![
            Quirk::AfraidOfWater,
            Quirk::ObsessedWithCats,
            Quirk::TalksToThemselves,
            Quirk::CollectsWeirdThings("ancient coins".to_string()),
            Quirk::LaughsAtInappropriateTimes,
            Quirk::AlwaysWearsSomething("their mother's ring".to_string()),
            Quirk::CantKeepSecrets,
            Quirk::FallsInLoveEasily,
            Quirk::NeverForgets,
            Quirk::TerribleAtSomething("dancing".to_string()),
            Quirk::AnimalWhisperer,
            Quirk::DancesWhenNervous,
            Quirk::BrutallyHonest,
        ];

        quirks.choose(rng).unwrap().clone()
    }
}

impl Secret {
    fn random(rng: &mut impl rand::Rng) -> Self {
        use rand::seq::SliceRandom;
        use rand::Rng;

        let secrets = vec![
            Secret::Bastard { real_parent: CharacterId(rng.r#gen()) },
            Secret::SecretLover { character: CharacterId(rng.r#gen()) },
            Secret::MurderedSomeone { victim: "a rival".to_string(), when: rng.gen_range(1..10) },
            Secret::HiddenIllness("slowly going blind".to_string()),
            Secret::DarkPast("was once a pirate".to_string()),
            Secret::PlottingCoup,
            Secret::PartOfCult { cult_name: "The Silent Order".to_string() },
            Secret::ForbiddenKnowledge("location of ancient weapon".to_string()),
        ];

        secrets.choose(rng).unwrap().clone()
    }
}