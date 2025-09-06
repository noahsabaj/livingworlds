//! Individual Human Components - Foundation of All Systems
//! 
//! Every person in Living Worlds is simulated as an individual making decisions
//! based on their needs, skills, information, and incentives.

use bevy::prelude::*;
use lw_core::{Fixed32, Vec2fx};
use serde::{Deserialize, Serialize};
use lw_core::bounded_types::Percentage;
use lw_core::shared_types::SocialClass;
use lw_economics::components::GoodType;
use lw_governance::components::policy_incentives::PolicyType;

/// Core needs that drive human behavior
#[derive(Clone, Debug)]
pub enum Need {
    Food { satisfaction: Percentage },      // how well fed
    Shelter { quality: Percentage },        // housing quality
    Safety { threat_level: Percentage },    // perceived danger
    Status { social_rank: Percentage },     // social standing
    Purpose { fulfillment: Percentage },    // meaningful work
}

/// Skills that determine what individuals can do
#[derive(Clone, Debug)]
pub enum Skill {
    Farming { proficiency: Fixed32 },
    Crafting { specialty: CraftType, proficiency: Fixed32 },
    Trading { reputation: Fixed32 },
    Fighting { experience: Fixed32 },
    Leading { charisma: Fixed32 },
    Learning { intelligence: Fixed32 },
}

#[derive(Clone, Debug)]
pub enum CraftType {
    Metalworking,
    Textiles,
    Pottery,
    Woodworking,
    Stonework,
    Engineering,
}

/// What an individual knows about the world (limited, local)
#[derive(Component, Clone, Debug)]
pub struct LocalKnowledge {
    pub known_provinces: Vec<Entity>,     // Places they've been/heard about
    pub price_memory: Vec<PriceMemory>,   // What goods cost where
    pub job_opportunities: Vec<JobInfo>,  // Work they know about
    pub social_connections: Vec<Entity>,  // People they know
    pub rumors: Vec<Rumor>,              // Information (may be false!)
}

#[derive(Clone, Debug)]
pub struct PriceMemory {
    pub good: GoodType,
    pub location: Entity,    // Province entity
    pub price: Fixed32,
    pub age: Fixed32,        // How old is this information?
}

#[derive(Clone, Debug)]
pub struct JobInfo {
    pub employer: Entity,    // Nation, Noble, or Merchant
    pub job_type: JobType,
    pub wage: Fixed32,
    pub location: Entity,    // Province
    pub requirements: Vec<Skill>,
}

#[derive(Clone, Debug)]
pub struct Rumor {
    pub content: RumorType,
    pub reliability: Percentage,  // how trustworthy
    pub age: Fixed32,
}

#[derive(Clone, Debug)]
pub enum RumorType {
    War { attacker: Entity, defender: Entity },
    Trade { opportunity: String, location: Entity },
    Disease { plague_type: String, severity: Fixed32 },
    Politics { event: String, nation: Entity },
    Weather { forecast: String, region: Entity },
}

#[derive(Clone, Debug)]
pub enum JobType {
    Farmer,
    Soldier,
    Merchant,
    Craftsman { specialty: CraftType },
    Administrator,
    Priest,
    Scholar,
}

// GoodType moved to economics/mod.rs to avoid duplication
// RawMaterial removed as part of GoodType consolidation

/// What motivates this individual to take action
#[derive(Clone, Debug)]
pub struct Incentive {
    pub source: IncentiveSource,
    pub strength: Fixed32,     // How motivating (0-1)
    pub duration: Fixed32,     // How long it lasts
}

#[derive(Clone, Debug)]
pub enum IncentiveSource {
    Government { policy: PolicyType, compliance: Fixed32 },
    Market { opportunity: String, profit: Fixed32 },
    Social { expectation: String, pressure: Fixed32 },
    Personal { goal: String, importance: Fixed32 },
    Religious { doctrine: String, faith: Fixed32 },
}

// PolicyType moved to governance/policy_incentives.rs to avoid duplication

/// A complete individual person in the simulation
#[derive(Component, Clone, Debug)]
pub struct Individual {
    pub age: Fixed32,
    pub health: Percentage,           // affects productivity
    pub education: Percentage,        // affects learning
    
    // Core drivers of behavior
    pub needs: Vec<Need>,
    pub skills: Vec<Skill>,
    pub knowledge: LocalKnowledge,
    pub incentives: Vec<Incentive>,
    pub personality: Personality,     // Behavioral traits and preferences
    
    // Current state
    pub current_job: Option<JobType>,
    pub employer: Option<Entity>,
    pub wealth: Fixed32,
    pub location: Entity,          // Province where they live
    
    // Social identity
    pub culture: Entity,           // Cultural group
    pub religion: Option<Entity>,  // Religious affiliation
    pub loyalty: Vec<Loyalty>,     // Who they're loyal to
}

#[derive(Clone, Debug)]
pub struct Loyalty {
    pub target: LoyaltyTarget,
    pub strength: Percentage,         // how loyal
    pub reason: String,           // Why they're loyal
}

#[derive(Clone, Debug)]
pub enum LoyaltyTarget {
    Nation(Entity),
    Ruler(Entity),
    Religion(Entity),
    Family(Entity),
    Employer(Entity),
}

/// Bundle for spawning a complete individual
#[derive(Bundle)]
pub struct IndividualBundle {
    pub individual: Individual,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

// All Individual logic moved to systems/individual_logic.rs
// Components should be pure data - no methods!

/// Possible decisions an individual can make
#[derive(Clone, Debug)]
pub enum Decision {
    Continue,                    // Stay the course
    ChangeJob(JobInfo),         // Switch to new employment
    Migrate(Entity),            // Move to new province
    StartBusiness(BusinessPlan), // Entrepreneurship
    JoinArmy(Entity),           // Military service
    Revolt(Entity),             // Rebellion against authority
    Convert(Entity),            // Change religion
    Learn(Skill),              // Acquire new skills
}

#[derive(Clone, Debug)]
pub struct BusinessPlan {
    pub business_type: GoodType,
    pub location: Entity,
    pub initial_investment: Fixed32,
    pub expected_profit: Fixed32,
}

/// Event fired when an individual migrates to a new location
#[derive(Event, Debug, Clone)]
pub struct MigrationEvent {
    pub individual: Entity,
    pub from_province: Entity,
    pub to_province: Entity,
    pub reason: MigrationReason,
}

#[derive(Debug, Clone)]
pub enum MigrationReason {
    Economic,       // Better job opportunities
    War,           // Fleeing conflict
    Famine,        // Food shortage
    Religious,     // Religious persecution or pilgrimage
    Family,        // Joining family members
    Political,     // Political oppression
}

/// Personality traits that affect decision-making
#[derive(Component, Debug, Clone)]
pub struct Personality {
    pub traits: PersonalityTraits,
    pub preferences: Vec<Preference>,
    pub risk_tolerance: Percentage,
    pub time_preference: Percentage,    // 0=present focused, 1=future focused
    pub social_orientation: Percentage, // 0=individualist, 1=collectivist
    pub materialism: Percentage,        // 0=spiritual focused, 1=material focused
    pub spirituality: Fixed32,          // Spiritual inclination (for cultural systems)
    pub sociability: Fixed32,           // Social connection needs (for cultural systems)
}

#[derive(Debug, Clone)]
pub struct PersonalityTraits {
    pub openness: Percentage,          // To new experiences
    pub conscientiousness: Percentage, // Organized and dutiful
    pub extraversion: Percentage,      // Social and energetic
    pub agreeableness: Percentage,     // Cooperative and trusting
    pub neuroticism: Percentage,       // Emotional stability (inverted)
}

#[derive(Debug, Clone)]
pub struct Preference {
    pub good: GoodType,
    pub intensity: Fixed32,            // How much they want it
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            traits: PersonalityTraits::default(),
            preferences: Vec::new(),
            risk_tolerance: Percentage::new(Fixed32::from_float(0.5)), // Moderate risk tolerance
            time_preference: Percentage::new(Fixed32::from_float(0.5)), // Balanced time preference
            social_orientation: Percentage::new(Fixed32::from_float(0.5)), // Balanced social orientation
            materialism: Percentage::new(Fixed32::from_float(0.5)), // Balanced material/spiritual
            spirituality: Fixed32::from_float(0.5),  // Moderate spirituality
            sociability: Fixed32::from_float(0.6),   // Slightly social
        }
    }
}

impl Default for PersonalityTraits {
    fn default() -> Self {
        Self {
            openness: Percentage::new(Fixed32::from_float(0.5)),
            conscientiousness: Percentage::new(Fixed32::from_float(0.5)),
            extraversion: Percentage::new(Fixed32::from_float(0.5)),
            agreeableness: Percentage::new(Fixed32::from_float(0.5)),
            neuroticism: Percentage::new(Fixed32::from_float(0.3)), // Lower neuroticism by default
        }
    }
}

/// State of an individual's decision-making process
#[derive(Component, Debug, Clone)]
pub struct DecisionState {
    pub current_goal: Goal,
    pub opportunities: Vec<Opportunity>,
    pub constraints: Vec<Constraint>,
    pub time_horizon: u32,             // Days ahead they're planning
    pub decision_urgency: Percentage,
    pub current_action: Option<Decision>,  // Current action being taken
    pub last_decision_time: Fixed32,       // Game time of last decision
}

#[derive(Debug, Clone)]
pub struct Goal {
    pub goal_type: GoalType,
    pub priority: Fixed32,
    pub progress: Percentage,
}

#[derive(Debug, Clone)]
pub enum GoalType {
    Survival,           // Basic needs
    Security,           // Safety and stability
    Social,             // Relationships and belonging
    Esteem,             // Recognition and achievement
    SelfActualization,  // Personal growth
}

/// An opportunity available to an individual
#[derive(Debug, Clone)]
pub struct Opportunity {
    pub opportunity_type: OpportunityType,
    pub location: Entity,
    pub expected_value: Fixed32,
    pub risk_level: Percentage,
    pub time_window: u32,              // Days until opportunity expires
    pub requirements: Vec<Requirement>,
}

#[derive(Debug, Clone)]
pub enum OpportunityType {
    Employment(JobInfo),
    Business(BusinessPlan),
    Education(Skill),
    Migration(Entity),
    Marriage(Entity),
    Military(Entity),
}

#[derive(Debug, Clone)]
pub struct Requirement {
    pub requirement_type: RequirementType,
    pub met: bool,
}

#[derive(Debug, Clone)]
pub enum RequirementType {
    Skill(Skill),
    Capital(Fixed32),
    Social(SocialClass),
    Location(Entity),
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    pub severity: Fixed32,
}

#[derive(Debug, Clone)]
pub enum ConstraintType {
    Financial,          // Lack of money
    Geographic,         // Can't move
    Social,            // Social barriers
    Legal,             // Laws prevent it
    Cultural,          // Cultural norms forbid
}

/// An action an individual can take
#[derive(Event, Debug, Clone)]
pub struct Action {
    pub actor: Entity,
    pub action_type: ActionType,
    pub target: Option<Entity>,
    pub location: Entity,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum ActionType {
    Work { job: JobInfo },
    Trade { good: GoodType, quantity: Fixed32 },
    Move { destination: Entity },
    Learn { skill: Skill },
    Socialize { with: Entity },
    Marry { partner: Entity },
    Reproduce,
    Fight { enemy: Entity },
    Flee { from: Entity },
    Build { structure: StructureType },
}

#[derive(Debug, Clone)]
pub enum StructureType {
    House,
    Workshop,
    Market,
    Temple,
    Fort,
}