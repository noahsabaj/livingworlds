//! Diplomacy system based on trust through actions, not abstract points
//!
//! Trust builds slowly through consistent behavior but can be destroyed instantly.
//! Geography forces certain relationships - you can't ignore your neighbors.
//! Power dynamics shape what promises actually mean.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use lw_economics::components::GoodType;
use lw_core::shared_types::{
    GameTime, NationId, ProvinceId,
    WarGoal, ProxyConflict, NeutralityType, CooperationArea,
    AllianceType, Obligation, TreatyType, TreatyTerm, EnforcementType,
    Consequence, TreatyDuration, RenewalCondition, ComplianceRecord,
    TechArea, CrisisType, TradeTerms,
};

/// Diplomatic relation between two nations - trust earned through actions
#[derive(Component, Debug, Clone)]
pub struct DiplomaticRelation {
    pub nation_a: NationId,
    pub nation_b: NationId,
    
    // Core trust mechanics
    pub trust_level: Fixed32,           // -1.0 (enemies) to 1.0 (trusted allies)
    pub trust_history: Vec<TrustEvent>, // Actions that built/destroyed trust
    pub reputation_modifier: Fixed32,   // How much each nation's word is worth
    
    // Economic foundation of cooperation
    pub trade_history: Vec<TradeRecord>,
    pub trade_dependency: Fixed32,      // How much each side needs the other
    pub mutual_benefit: Fixed32,        // Sum of past beneficial interactions
    
    // Conflict creates lasting wounds
    pub conflict_memory: Vec<ConflictMemory>,
    pub historical_grievances: Fixed32, // Negative events fade slowly
    pub war_exhaustion: Fixed32,        // Recent wars reduce appetite for more
    
    // Power shapes everything
    pub power_balance: RelativePower,
    pub nuclear_balance: NuclearBalance,
    pub economic_leverage: Fixed32,
    
    // Geography forces interaction
    pub shared_borders: Vec<SharedBorder>,
    pub strategic_chokepoints: Vec<StrategicLocation>,
    pub resource_dependencies: Vec<ResourceDependency>,
    pub forced_interaction_level: Fixed32, // Can't ignore neighbors
    
    // Current diplomatic state
    pub formal_status: DiplomaticStatus,
    pub active_treaties: Vec<Treaty>,
    pub ongoing_negotiations: Vec<Negotiation>,
    
    // Communication and intelligence
    pub diplomatic_contact_level: Fixed32, // How well they can talk
    pub intelligence_on_other: IntelligenceLevel,
    pub cultural_understanding: Fixed32,   // Affects misunderstandings
}

/// Trust events that build or destroy relationships
#[derive(Debug, Clone)]
pub struct TrustEvent {
    pub timestamp: GameTime,
    pub event_type: TrustEventType,
    pub trust_change: Fixed32,      // Positive = trust gained, negative = lost
    pub power_weighted: bool,       // Whether power difference affected impact
    pub witnesses: Vec<NationId>,   // Other nations that observed this
    pub decay_rate: Fixed32,        // How quickly this event fades from memory
}

#[derive(Debug, Clone)]
pub enum TrustEventType {
    // Positive trust events
    PromiseKept {
        promise_type: PromiseType,
        difficulty: Fixed32,        // Harder promises = more trust when kept
        cost_to_keeper: Fixed32,   // Personal sacrifice increases trust
    },
    MutualDefense {
        threat_faced: ThreatLevel,
        response_speed: Fixed32,   // Quick response = more trust
        cost_incurred: Fixed32,
    },
    EconomicAid {
        aid_amount: Fixed32,
        recipient_need_level: Fixed32, // Helping in crisis = more trust
        strings_attached: bool,        // Conditional aid = less trust
    },
    TerritorialRespect {
        temptation_level: Fixed32, // Respecting borders when you could take them
        strategic_value: Fixed32,   // More valuable = more trust gained
    },
    CrisisCooperation {
        crisis_severity: Fixed32,
        cooperation_level: Fixed32,
        shared_sacrifice: Fixed32,
    },
    
    // Negative trust events (instant trust destruction)
    PromiseBroken {
        promise_type: PromiseType,
        broken_severity: Fixed32,   // Complete betrayal vs minor failure
        beneficiary: Option<NationId>, // Who benefited from the betrayal
    },
    TerritorialViolation {
        violation_type: ViolationType,
        strategic_gain: Fixed32,    // What was gained by the violation
        civilian_impact: Fixed32,   // Civilian casualties amplify betrayal
    },
    EconomicBetrayal {
        betrayal_type: EconomicBetrayalType,
        economic_damage: Fixed32,
        timing: BetrayalTiming,     // Betrayal during crisis = worse
    },
    InformationBetrayal {
        secrets_revealed: Vec<SecretType>,
        damage_caused: Fixed32,
        third_party_benefit: Option<NationId>,
    },
}

/// Trade records show mutual benefit over time
#[derive(Debug, Clone)]
pub struct TradeRecord {
    pub timestamp: GameTime,
    pub trade_volume: Fixed32,
    pub mutual_benefit: Fixed32,    // Positive sum vs zero sum
    pub fairness_rating: Fixed32,   // Exploitative vs mutually beneficial
    pub strategic_importance: Fixed32, // Critical resources vs luxury goods
}

/// Military conflicts leave lasting scars
#[derive(Debug, Clone)]
pub struct ConflictMemory {
    pub conflict_id: Entity,        // Reference to historical war
    pub timeline: GameTime,
    pub aggressor: NationId,        // Who started it
    pub outcome: ConflictOutcome,
    pub casualties: CasualtyRecord,
    pub territorial_changes: Vec<TerritorialChange>,
    pub civilian_impact: Fixed32,   // War crimes leave permanent scars
    pub memory_intensity: Fixed32,  // How strongly this is remembered
    pub generational_decay: Fixed32, // Memories fade as generations pass
}

/// Power determines what promises actually mean
#[derive(Debug, Clone)]
pub struct RelativePower {
    pub military_ratio: Fixed32,    // Nation A vs Nation B military strength
    pub economic_ratio: Fixed32,    // Economic capacity comparison  
    pub technological_ratio: Fixed32, // Tech level comparison
    pub alliance_strength: Fixed32, // Combined alliance power
    pub geographic_advantage: Fixed32, // Home field advantage
    
    // Power affects diplomacy:
    pub guarantee_credibility: Fixed32, // Can weak nation actually help?
    pub threat_credibility: Fixed32,    // Can they actually hurt you?
    pub negotiation_leverage: Fixed32,  // Who has more to offer/threaten?
}

/// Geography creates unavoidable relationships
#[derive(Debug, Clone)]
pub struct SharedBorder {
    pub border_provinces: Vec<(ProvinceId, ProvinceId)>,
    pub border_length: Fixed32,
    pub terrain_type: BorderTerrain,
    pub defensibility: Fixed32,     // Mountains vs open plains
    pub trade_routes: Vec<TradeRoute>,
    pub migration_pressure: Fixed32, // Population movement across border
    pub cultural_mixing: Fixed32,   // How much cultures blend at border
}

#[derive(Debug, Clone)]
pub struct ResourceDependency {
    pub resource_type: GoodType,
    pub dependency_level: Fixed32,  // How much nation needs this resource
    pub alternative_sources: Vec<NationId>, // Other potential suppliers
    pub strategic_importance: Fixed32, // Oil vs luxury goods
    pub supply_vulnerability: Fixed32, // How easily supply can be cut
}

/// Current formal diplomatic state
#[derive(Debug, Clone)]
pub enum DiplomaticStatus {
    War {
        war_goals: Vec<WarGoal>,
        war_exhaustion: Fixed32,
        escalation_ladder: EscalationLevel,
    },
    ColdWar {
        proxy_conflicts: Vec<ProxyConflict>,
        espionage_level: Fixed32,
        economic_warfare: Fixed32,
    },
    Neutral {
        neutrality_type: NeutralityType, // Armed neutrality vs friendly neutral
        trade_level: Fixed32,
        diplomatic_contact: Fixed32,
    },
    Friendly {
        cooperation_areas: Vec<CooperationArea>,
        regular_contact: Fixed32,
        mutual_benefit: Fixed32,
    },
    Allied {
        alliance_type: AllianceType,
        mutual_obligations: Vec<Obligation>,
        integration_level: Fixed32, // How closely integrated
    },
}

/// Treaties are actual commitments, not just diplomacy points
#[derive(Debug, Clone)]
pub struct Treaty {
    pub treaty_id: Entity,
    pub treaty_type: TreatyType,
    pub signatories: Vec<NationId>,
    pub terms: Vec<TreatyTerm>,
    pub enforcement_mechanism: EnforcementType,
    pub violation_consequences: Vec<Consequence>,
    pub duration: TreatyDuration,
    pub renewal_conditions: Vec<RenewalCondition>,
    
    // Treaty reputation
    pub compliance_history: Vec<ComplianceRecord>,
    pub treaty_effectiveness: Fixed32, // How well it's actually followed
    pub international_support: Fixed32, // Other nations' backing
}

/// Diplomatic systems for managing relationships
#[derive(Debug, Clone)]
pub struct DiplomaticCapability {
    pub diplomatic_corps: Vec<Diplomat>,
    pub embassy_network: Vec<Embassy>,
    pub intelligence_services: IntelligenceServices,
    pub cultural_exchange: CulturalExchangePrograms,
    pub economic_diplomacy: EconomicDiplomacyTools,
    pub information_warfare: InformationWarfareCapability,
}

/// Diplomat entities with actual skills and relationships
#[derive(Debug, Clone)]
pub struct Diplomat {
    pub person_id: Entity,          // Reference to Individual component
    pub diplomatic_skill: Fixed32,  // Negotiation ability
    pub cultural_knowledge: Vec<(NationId, Fixed32)>, // Understanding of other cultures
    pub language_skills: Vec<LanguageSkill>,
    pub personal_relationships: Vec<PersonalRelationship>, // Diplomat-to-diplomat bonds
    pub negotiation_style: NegotiationStyle,
    pub reputation: DiplomaticReputation,
    pub specializations: Vec<DiplomaticSpecialty>,
}

/// Intelligence affects diplomatic understanding
#[derive(Debug, Clone)]
pub enum IntelligenceLevel {
    Blind,          // No intelligence on other nation
    Basic,          // Public information only
    Limited,        // Some insider knowledge
    Comprehensive,  // Deep understanding of other nation
    Intimate,       // Know their secrets
}

/// Systems for processing diplomatic relationships
impl DiplomaticRelation {
    /// Update trust based on a new action
    pub fn process_trust_event(&mut self, event: TrustEvent, current_time: GameTime) {
        // Power affects impact of trust events
        let power_modifier = match event.power_weighted {
            true => {
                if self.power_balance.military_ratio > Fixed32::from_num(2) {
                    Fixed32::from_float(1.5) // Strong nations' actions matter more
                } else if self.power_balance.military_ratio < Fixed32::from_float(0.5) {
                    Fixed32::from_float(0.5) // Weak nations' promises mean less
                } else {
                    Fixed32::ONE
                }
            },
            false => Fixed32::ONE,
        };
        
        let weighted_change = event.trust_change * power_modifier;
        
        // Trust destruction is instant and complete
        if weighted_change < Fixed32::ZERO {
            match event.event_type {
                TrustEventType::PromiseBroken { broken_severity, .. } => {
                    self.trust_level = (self.trust_level + weighted_change * broken_severity)
                        .clamp(Fixed32::from_num(-1), Fixed32::ONE);
                },
                TrustEventType::TerritorialViolation { .. } => {
                    // Territory violations destroy trust immediately
                    self.trust_level = Fixed32::from_float(-0.8);
                },
                _ => {
                    self.trust_level = (self.trust_level + weighted_change)
                        .clamp(Fixed32::from_num(-1), Fixed32::ONE);
                }
            }
        } else {
            // Trust builds slowly
            let trust_building_rate = Fixed32::from_float(0.1); // Trust builds slowly
            self.trust_level = (self.trust_level + weighted_change * trust_building_rate)
                .clamp(Fixed32::from_num(-1), Fixed32::ONE);
        }
        
        // Add to history
        self.trust_history.push(event);
        
        // Update reputation based on consistency
        self.update_reputation_modifier();
    }
    
    /// Update how much this nation's word is worth
    fn update_reputation_modifier(&mut self) {
        let kept_promises = self.trust_history.iter()
            .filter(|e| matches!(e.event_type, TrustEventType::PromiseKept { .. }))
            .count();
            
        let broken_promises = self.trust_history.iter()
            .filter(|e| matches!(e.event_type, TrustEventType::PromiseBroken { .. }))
            .count();
            
        if broken_promises == 0 && kept_promises > 0 {
            self.reputation_modifier = Fixed32::from_float(1.2); // Trustworthy bonus
        } else if broken_promises > 0 {
            let reliability_ratio = kept_promises as f32 / (kept_promises + broken_promises) as f32;
            self.reputation_modifier = Fixed32::from_float(reliability_ratio * 0.8); // Broken trust lingers
        } else {
            self.reputation_modifier = Fixed32::ONE; // No history
        }
    }
    
    /// Calculate negotiation leverage based on multiple factors
    pub fn calculate_negotiation_leverage(&self) -> Fixed32 {
        let mut leverage = Fixed32::ZERO;
        
        // Military leverage
        if self.power_balance.military_ratio > Fixed32::ONE {
            leverage += (self.power_balance.military_ratio - Fixed32::ONE) * Fixed32::from_float(0.3);
        }
        
        // Economic leverage (especially through dependencies)
        leverage += self.economic_leverage * Fixed32::from_float(0.4);
        
        // Geographic leverage (chokepoints, strategic locations)
        leverage += Fixed32::from_num(self.strategic_chokepoints.len() as i32) * Fixed32::from_float(0.1);
        
        // Alliance leverage
        leverage += self.power_balance.alliance_strength * Fixed32::from_float(0.2);
        
        leverage.clamp(Fixed32::from_num(-2), Fixed32::from_num(2))
    }
    
    /// Geography determines minimum interaction level
    pub fn calculate_forced_interaction(&self) -> Fixed32 {
        let mut interaction_level = Fixed32::ZERO;
        
        // Shared borders force interaction
        let border_interaction = Fixed32::from_float(self.shared_borders.len() as f32 * 0.3);
        interaction_level += border_interaction;
        
        // Resource dependencies create unavoidable relationships
        let resource_interaction = self.resource_dependencies.iter()
            .map(|dep| dep.dependency_level * dep.strategic_importance)
            .fold(Fixed32::ZERO, |acc, val| acc + val) * Fixed32::from_float(0.4);
        interaction_level += resource_interaction;
        
        // Strategic locations (straits, mountain passes, etc.)
        let strategic_interaction = Fixed32::from_float(self.strategic_chokepoints.len() as f32 * 0.2);
        interaction_level += strategic_interaction;
        
        interaction_level.clamp(Fixed32::ZERO, Fixed32::from_num(2))
    }
}

// Additional supporting types
#[derive(Debug, Clone)]
pub enum PromiseType {
    MilitarySupport { threat: ThreatLevel },
    TerritorialRespect { disputed_areas: Vec<ProvinceId> },
    TradeAgreement { terms: TradeTerms },
    NonAggression { duration: GameTime },
    TechnicalCooperation { areas: Vec<TechArea> },
    HumanitarianAid { crisis_type: CrisisType },
}

#[derive(Debug, Clone)]
pub enum BorderTerrain {
    Mountain { passes: Vec<MountainPass> },
    River { crossings: Vec<RiverCrossing> },
    Desert { oases: Vec<Oasis> },
    Forest { roads: Vec<ForestRoad> },
    Plains { natural_barriers: Vec<NaturalBarrier> },
    Coast { ports: Vec<Port> },
}

#[derive(Debug, Clone)]
pub enum EscalationLevel {
    LimitedConflict,
    RegionalWar,
    FullScale,
    ExistentialThreat,
}

// Component for managing all diplomatic relationships
#[derive(Component, Debug, Default)]
pub struct DiplomaticNetwork {
    pub relations: Vec<Entity>, // References to DiplomaticRelation entities
    pub overall_reputation: Fixed32,
    pub diplomatic_stance: DiplomaticStance,
    pub active_mediations: Vec<MediationAttempt>,
}

#[derive(Debug, Clone, Default)]
pub enum DiplomaticStance {
    Isolationist,
    Defensive,
    #[default]
    Balanced,
    Aggressive,
    Hegemonic,
}

// Additional type definitions for diplomatic system

#[derive(Debug, Clone)]
pub struct NuclearBalance {
    pub nation_a_warheads: u32,
    pub nation_b_warheads: u32,
    pub mutually_assured_destruction: bool,
    pub first_strike_capability: Fixed32,
    pub second_strike_capability: Fixed32,
}

#[derive(Debug, Clone)]
pub struct StrategicLocation {
    pub location_type: StrategicLocationType,
    pub control_value: Fixed32,
    pub contested: bool,
}

#[derive(Debug, Clone)]
pub enum StrategicLocationType {
    Strait,          // Maritime chokepoint
    MountainPass,    // Land chokepoint
    RiverCrossing,   // Bridge or ford
    NaturalHarbor,   // Naval base potential
    HighGround,      // Military advantage
}

#[derive(Debug, Clone)]
pub struct Negotiation {
    pub negotiation_id: Entity,
    pub topic: NegotiationTopic,
    pub progress: Fixed32,
    pub deadline: Option<GameTime>,
}

#[derive(Debug, Clone)]
pub enum NegotiationTopic {
    PeaceTreaty,
    TradeAgreement,
    TerritorialExchange,
    MilitaryAlliance,
    CulturalExchange,
}

#[derive(Debug, Clone)]
pub enum ThreatLevel {
    Minimal,
    Low,
    Moderate,
    High,
    Extreme,
    Existential,
}

#[derive(Debug, Clone)]
pub enum EconomicBetrayalType {
    DefaultOnDebt,
    CurrencyManipulation,
    TradeEmbargo,
    AssetSeizure,
    IntellectualPropertyTheft,
}

#[derive(Debug, Clone)]
pub enum BetrayalTiming {
    DuringCrisis,    // Betrayal when they needed you most
    AfterCooperation, // After receiving help
    Preemptive,      // Before they could betray you
    Opportunistic,   // Taking advantage of weakness
}

/// Types of territorial violations that destroy trust
#[derive(Debug, Clone)]
pub enum ViolationType {
    BorderIncursion,      // Minor border crossing
    AirspaceViolation,    // Unauthorized overflight
    TerritorialSeizure,   // Taking control of territory
    NavalBlockade,        // Blocking sea access
    MilitaryOccupation,   // Full military control
    ResourceExtraction,   // Taking resources without permission
}

#[derive(Debug, Clone)]
pub enum SecretType {
    MilitaryPlans,
    EconomicData,
    TechnologicalAdvance,
    DiplomaticCorrespondence,
    InternalWeakness,
}

#[derive(Debug, Clone)]
pub enum ConflictOutcome {
    VictoryDecisive,
    VictoryPyrrhic,
    Stalemate,
    DefeatTactical,
    DefeatStrategic,
}

#[derive(Debug, Clone)]
pub struct CasualtyRecord {
    pub military_casualties: u32,
    pub civilian_casualties: u32,
    pub infrastructure_damage: Fixed32,
    pub economic_cost: Fixed32,
}

#[derive(Debug, Clone)]
pub struct TerritorialChange {
    pub province_id: ProvinceId,
    pub previous_owner: NationId,
    pub new_owner: NationId,
    pub transfer_type: TerritorialTransferType,
}

#[derive(Debug, Clone)]
pub enum TerritorialTransferType {
    Conquest,
    Cession,
    Purchase,
    Liberation,
    Partition,
}

#[derive(Debug, Clone)]
pub struct TradeRoute {
    pub origin: ProvinceId,
    pub destination: ProvinceId,
    pub goods_transported: Vec<GoodType>,
    pub volume: Fixed32,
    pub profitability: Fixed32,
}

#[derive(Debug, Clone)]
pub struct MediationAttempt {
    pub mediator: NationId,
    pub party_a: NationId,
    pub party_b: NationId,
    pub success_probability: Fixed32,
}

// Supporting types for specific diplomatic concepts

#[derive(Debug, Clone)]
pub struct Embassy {
    pub host_nation: NationId,
    pub guest_nation: NationId,
    pub embassy_staff: u32,
    pub influence_level: Fixed32,
}

#[derive(Debug, Clone)]
pub struct IntelligenceServices {
    pub agency_size: u32,
    pub capability_level: Fixed32,
    pub counter_intelligence: Fixed32,
}

#[derive(Debug, Clone)]
pub struct CulturalExchangePrograms {
    pub student_exchanges: u32,
    pub cultural_events: u32,
    pub soft_power_influence: Fixed32,
}

#[derive(Debug, Clone)]
pub struct EconomicDiplomacyTools {
    pub foreign_aid_budget: Fixed32,
    pub sanctions_capability: Fixed32,
    pub investment_leverage: Fixed32,
}

#[derive(Debug, Clone)]
pub struct InformationWarfareCapability {
    pub propaganda_effectiveness: Fixed32,
    pub cyber_capability: Fixed32,
    pub narrative_control: Fixed32,
}

#[derive(Debug, Clone)]
pub struct LanguageSkill {
    pub language: String,
    pub proficiency: Fixed32,
}

#[derive(Debug, Clone)]
pub struct PersonalRelationship {
    pub other_diplomat: Entity,
    pub trust_level: Fixed32,
    pub shared_experiences: u32,
}

#[derive(Debug, Clone)]
pub enum NegotiationStyle {
    Aggressive,
    Collaborative,
    Compromising,
    Avoiding,
    Accommodating,
}

#[derive(Debug, Clone)]
pub struct DiplomaticReputation {
    pub reliability: Fixed32,
    pub negotiation_success_rate: Fixed32,
    pub cultural_sensitivity: Fixed32,
}

#[derive(Debug, Clone)]
pub enum DiplomaticSpecialty {
    TradeNegotiation,
    ConflictResolution,
    CulturalLiaison,
    IntelligenceGathering,
    TreatyDrafting,
}

// Pass-through types for mountain passes, river crossings, etc.
#[derive(Debug, Clone)]
pub struct MountainPass {
    pub name: String,
    pub elevation: Fixed32,
    pub width: Fixed32,
    pub seasonal_accessibility: Fixed32,
}

#[derive(Debug, Clone)]
pub struct RiverCrossing {
    pub crossing_type: CrossingType,
    pub capacity: Fixed32,
    pub strategic_value: Fixed32,
}

#[derive(Debug, Clone)]
pub enum CrossingType {
    Bridge,
    Ford,
    Ferry,
}

#[derive(Debug, Clone)]
pub struct Oasis {
    pub water_capacity: Fixed32,
    pub vegetation: Fixed32,
}

#[derive(Debug, Clone)]
pub struct ForestRoad {
    pub width: Fixed32,
    pub maintenance_level: Fixed32,
}

#[derive(Debug, Clone)]
pub struct NaturalBarrier {
    pub barrier_type: BarrierType,
    pub impassability: Fixed32,
}

#[derive(Debug, Clone)]
pub enum BarrierType {
    Mountain,
    River,
    Desert,
    Swamp,
    Ocean,
}

#[derive(Debug, Clone)]
pub struct Port {
    pub capacity: Fixed32,
    pub depth: Fixed32,
    pub facilities: PortFacilities,
}

#[derive(Debug, Clone)]
pub struct PortFacilities {
    pub docks: u32,
    pub warehouses: u32,
    pub shipyards: u32,
}