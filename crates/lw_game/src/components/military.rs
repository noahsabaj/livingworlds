//! Military System - Logistics & Morale Reality
//! 
//! War is about supply lines, morale, terrain, and leadership - not just numbers.
//! Napoleon in Russia, Vietnam War, Afghan Wars show technology isn't everything.

use bevy::prelude::*;
use bevy::math::Vec2;
use lw_core::{Fixed32, Vec2fx};
use serde::{Deserialize, Serialize};
use super::individual::*;
use std::collections::HashMap;

/// Military unit composition for combat calculations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArmyComposition {
    pub infantry: Fixed32,    // Number of infantry units
    pub cavalry: Fixed32,     // Number of cavalry units  
    pub artillery: Fixed32,   // Number of artillery pieces
}

impl ArmyComposition {
    pub fn total(&self) -> Fixed32 {
        self.infantry + self.cavalry + self.artillery
    }
}

/// Army movement state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MovementState {
    pub target_position: Option<Vec2>,  // Where the army is moving to
    pub is_moving: bool,                // Currently in movement
    pub movement_speed: Fixed32,        // Movement rate
    pub movement_cost: Fixed32,         // Supply cost of movement
}

/// Army supply details (different from SupplyState enum)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArmySupplyDetails {
    pub food_days: Fixed32,            // Days of food remaining
    pub ammunition: Fixed32,           // Ammunition available
    pub equipment_quality: Fixed32,    // Overall equipment condition
    pub medical_supplies: Fixed32,     // Medical supplies available
}

/// Army logistics management
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Logistics {
    pub supply_wagons: Fixed32,        // Number of supply wagons
    pub supply_efficiency: Fixed32,    // How well supplies are managed
    pub foraging_ability: Fixed32,     // Ability to live off land
}

/// Army with realistic logistics and morale
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Army {
    pub id: u32,
    pub nation_id: u32,
    pub name: String,
    
    // Human component - each soldier is a person
    pub soldiers: Vec<Entity>,           // Individual entities
    pub officers: Vec<Entity>,           // Leadership hierarchy
    pub commander: Entity,               // Top commander
    
    // Military composition for combat
    pub composition: ArmyComposition,
    
    // Logistics - the foundation of military power
    pub supply_chain: SupplyChain,
    pub equipment: Equipment,
    pub supply_state: ArmySupplyDetails,  // Current supply status
    pub logistics: Logistics,           // Logistics management
    
    // Morale - the decisive factor
    pub morale: MoraleState,
    
    // Leadership
    pub leadership: Leadership,          // Army leadership
    
    // Combat factors
    pub experience: Fixed32,             // 0-1, veteran vs green troops
    pub discipline: Fixed32,             // 0-1, training and unit cohesion
    pub doctrine: MilitaryDoctrine,      // How they fight
    
    // Operational state
    pub current_location: Entity,        // Province
    pub movement_target: Option<Entity>, // Where they're going
    pub movement: MovementState,         // Movement tracking
    pub combat_status: CombatStatus,
    pub readiness: Fixed32,             // 0-1, ready for combat?
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct SupplyChain {
    pub supply_base: Entity,            // Home base/depot
    pub supply_lines: Vec<SupplyLine>,  // Routes to the army
    pub local_foraging: Fixed32,        // 0-1, can live off the land?
    pub stockpile: MilitaryStockpile,   // What they have with them
    pub consumption_rate: ConsumptionRate, // How fast they use supplies
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupplyLine {
    pub route: Vec<Entity>,             // Provinces from base to army
    pub capacity: Fixed32,              // Tons per day
    pub security: Fixed32,              // 0-1, protected from raids?
    pub reliability: Fixed32,           // 0-1, gets through consistently?
    pub cost: Fixed32,                  // Resources needed to maintain
    pub vulnerability_points: Vec<Entity>, // Chokepoints, bridges, passes
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MilitaryStockpile {
    pub food_days: Fixed32,             // Days of rations
    pub ammunition: Fixed32,            // Combat rounds available
    pub medical_supplies: Fixed32,      // Treatment capacity
    pub spare_equipment: Fixed32,       // Replacement gear
    pub fuel: Fixed32,                  // For vehicles (if applicable)
    pub pay_months: Fixed32,            // Soldier wages in reserve
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsumptionRate {
    pub food_per_day: Fixed32,          // Per soldier
    pub ammo_per_battle: Fixed32,       // Per engagement
    pub medical_per_casualty: Fixed32,  // Per wounded
    pub equipment_degradation: Fixed32, // Daily wear rate
    pub fuel_per_movement: Fixed32,     // Per distance unit
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Equipment {
    pub weapons: Vec<WeaponSystem>,
    pub armor: ArmorSystem,
    pub transport: Vec<TransportSystem>,
    pub communication: CommunicationSystem,
    pub engineering: EngineeringEquipment,
    pub quality: Fixed32,               // 0-1, overall equipment state
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeaponSystem {
    pub weapon_type: WeaponType,
    pub quantity: u32,
    pub condition: Fixed32,             // 0-1, maintenance state
    pub effectiveness: CombatEffectiveness,
    pub crew_requirements: u32,         // People needed to operate
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WeaponType {
    Melee { weapon_class: MeleeWeaponClass },
    Ranged { weapon_class: RangedWeaponClass, range: Fixed32 },
    Siege { weapon_class: SiegeWeaponClass, crew_size: u32 },
    Artillery { caliber: Fixed32, mobility: Mobility },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MeleeWeaponClass {
    Sword, Spear, Axe, Mace, Pike,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RangedWeaponClass {
    Bow, Crossbow, Sling, Javelin, Firearm,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SiegeWeaponClass {
    Catapult, Trebuchet, Ballista, Ram, Tower,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Mobility {
    Fixed,      // Cannot move once deployed
    Towed,      // Requires animals/vehicles to move
    SelfPropelled, // Can move on its own
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombatEffectiveness {
    pub vs_infantry: Fixed32,           // Effectiveness against foot soldiers
    pub vs_cavalry: Fixed32,            // Effectiveness against mounted troops
    pub vs_armor: Fixed32,              // Penetration vs armored targets
    pub vs_fortifications: Fixed32,     // Siege capability
    pub terrain_modifiers: HashMap<Terrain, Fixed32>, // How terrain affects it
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArmorSystem {
    pub protection_level: Fixed32,      // 0-1, damage reduction
    pub coverage: Fixed32,              // 0-1, how much body is protected
    pub mobility_impact: Fixed32,       // 0-1, how much it slows movement
    pub maintenance_cost: Fixed32,      // Resource cost to keep functional
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransportSystem {
    pub transport_type: TransportType,
    pub capacity: Fixed32,              // Load carrying capacity
    pub speed: Fixed32,                 // Movement rate
    pub terrain_capability: Vec<Terrain>, // Where it can go
    pub maintenance_requirements: MaintenanceRequirements,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransportType {
    Human,      // Soldiers carrying their own gear
    Pack_Animal { animal_type: String },
    Cart { wheel_type: WheelType },
    Ship { ship_class: ShipClass },
    Vehicle { vehicle_type: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WheelType {
    Solid, Spoked, Pneumatic,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ShipClass {
    Raft, Boat, Galley, Sailing_Ship, Warship,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaintenanceRequirements {
    pub daily_cost: Fixed32,
    pub specialist_skills: Vec<String>,
    pub replacement_parts: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommunicationSystem {
    pub system_type: CommunicationType,
    pub range: Fixed32,                 // How far messages can go
    pub reliability: Fixed32,           // 0-1, messages get through
    pub speed: Fixed32,                 // Time to transmit
    pub security: Fixed32,              // 0-1, encrypted/secure
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CommunicationType {
    Verbal,     // Shouting, horns, drums
    Visual,     // Flags, smoke, mirrors
    Messenger,  // Riders, runners
    Telegraph,  // Electrical communication
    Radio,      // Wireless communication
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EngineeringEquipment {
    pub fortification_tools: Vec<String>,
    pub bridge_building: Fixed32,       // Capability to build bridges
    pub road_building: Fixed32,         // Capability to build roads
    pub siege_engineering: Fixed32,     // Capability to breach walls
    pub field_works: Fixed32,           // Capability to build defensive positions
}

/// Morale - often the decisive factor in warfare
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct MoraleState {
    pub base_morale: Fixed32,           // 0-1, starting morale level
    pub current: Fixed32,               // 0-1, current state (renamed for system compatibility)
    pub morale_factors: Vec<MoraleFactor>,
    pub breaking_point: Fixed32,        // When army routs/deserts
    pub cohesion: Fixed32,              // 0-1, unit sticks together
    
    // Recent battle outcomes (flags for morale calculations)
    pub recent_victory: bool,           // Recent battle victory
    pub recent_defeat: bool,            // Recent battle defeat
    pub defending_homeland: bool,       // Fighting on home territory
    pub war_exhaustion: Fixed32,        // 0-1, accumulated fatigue
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoraleFactor {
    pub factor_type: MoraleFactorType,
    pub impact: Fixed32,                // -1 to 1, effect on morale
    pub duration: MoraleFactorDuration,
    pub affected_units: Vec<Entity>,    // Which soldiers affected
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MoraleFactorType {
    // Positive factors
    Victory { recent_wins: u32 },
    Pay { months_current: Fixed32 },
    Food { quality: Fixed32, quantity: Fixed32 },
    Leadership { commander_charisma: Fixed32 },
    Cause { belief_in_cause: Fixed32 },
    Home_Defense { defending_homeland: bool },
    Veteran_Status { battles_survived: u32 },
    Elite_Unit { reputation: Fixed32 },
    
    // Negative factors
    Defeat { recent_losses: u32 },
    Unpaid { months_overdue: Fixed32 },
    Hunger { severity: Fixed32 },
    Disease { infection_rate: Fixed32 },
    Casualties { friendly_losses: Fixed32 },
    Weather { severity: Fixed32 },
    Desertion { desertion_rate: Fixed32 },
    Poor_Leadership { incompetence_level: Fixed32 },
    Unclear_Objectives { confusion_level: Fixed32 },
    Foreign_War { distance_from_home: Fixed32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MoraleFactorDuration {
    Temporary { days_remaining: Fixed32 },
    Permanent,
    Conditional { condition: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MilitaryDoctrine {
    pub name: String,
    pub tactical_emphasis: TacticalEmphasis,
    pub formation_preferences: Vec<Formation>,
    pub engagement_philosophy: EngagementPhilosophy,
    pub logistics_approach: LogisticsApproach,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TacticalEmphasis {
    Offensive { aggression_level: Fixed32 },
    Defensive { fortification_focus: Fixed32 },
    Mobile { speed_emphasis: Fixed32 },
    Siege { siege_specialization: Fixed32 },
    Combined_Arms { coordination_level: Fixed32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Formation {
    pub name: String,
    pub unit_types: Vec<UnitType>,
    pub effectiveness_vs: HashMap<String, Fixed32>,  // Use formation name as key
    pub terrain_suitability: HashMap<Terrain, Fixed32>,
    pub command_complexity: Fixed32,    // How hard to coordinate
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UnitType {
    Infantry { specialization: InfantryType },
    Cavalry { specialization: CavalryType },
    Artillery { specialization: ArtilleryType },
    Engineers,
    Logistics,
    Command,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InfantryType {
    Light, Heavy, Missile, Pike, Elite,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CavalryType {
    Light, Heavy, Mounted_Archers, Shock,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ArtilleryType {
    Field, Siege, Anti_Personnel, Anti_Armor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EngagementPhilosophy {
    Decisive_Battle,    // Seek one big engagement
    Attrition,         // Wear down enemy over time
    Maneuver,          // Outposition rather than fight
    Guerrilla,         // Hit and run tactics
    Siege,             // Focus on fortified positions
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LogisticsApproach {
    Centralized { depot_system: bool },
    Distributed { cache_network: bool },
    Foraging { local_support_required: bool },
    Minimal { self_sufficiency_emphasis: bool },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CombatStatus {
    Peaceful,
    Alert,
    Engaged { enemy_armies: Vec<Entity> },
    Retreating { destination: Entity },
    Besieging { target: Entity },
    Besieged { by_armies: Vec<Entity> },
    Routing,            // Complete breakdown of unit discipline
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Terrain {
    Plains, Hills, Mountains, Forest, Desert, 
    Swamp, River, Coast, Urban, Fortified,
}

// All Army logic moved to systems/military_logic.rs
// Components should be pure data - no methods!

// All combat resolution and military system logic moved to systems/military_logic.rs
// This includes resolve_combat, military_system, and all helper functions

/// Supply state of a military unit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupplyState {
    WellSupplied,       // Full supplies, high morale
    Adequate,           // Sufficient for operations
    Low,                // Starting to affect combat effectiveness
    Critical,           // Severe penalties, risk of desertion
    Exhausted,          // Cannot fight effectively
}

/// Leadership quality and characteristics
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Leadership {
    pub commander: Entity,           // The individual leading
    pub leadership_style: LeadershipStyle,
    pub experience: Fixed32,         // 0-1, battles commanded
    pub charisma: Fixed32,          // 0-1, ability to inspire
    pub tactical_skill: Fixed32,    // 0-1, battlefield competence
    pub strategic_skill: Fixed32,   // 0-1, campaign planning
    pub loyalty: Fixed32,           // 0-1, to their nation/cause
    pub commander_skill: Fixed32,  // Overall commander effectiveness
    pub officer_quality: Fixed32,  // Quality of officer corps
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LeadershipStyle {
    Aggressive,         // Seeks decisive battles
    Defensive,          // Focuses on fortifications
    Maneuver,          // Emphasizes mobility
    Attrition,         // Wears down enemies
    Inspirational,     // Leads from the front
    Administrative,    // Focuses on logistics
}

/// Result of a combat engagement
#[derive(Event, Debug, Clone)]
pub struct CombatResult {
    pub attacker: Entity,
    pub defender: Entity,
    pub location: Entity,           // Province where combat occurred
    pub outcome: CombatOutcome,
    pub casualties: CasualtiesReport,
    pub duration: Fixed32,          // How long the battle lasted
    pub decisive: bool,             // Was this a major victory?
}

#[derive(Debug, Clone)]
pub enum CombatOutcome {
    AttackerVictory { rout: bool },
    DefenderVictory { rout: bool },
    Stalemate,
    MutualWithdrawal,
}

#[derive(Debug, Clone)]
pub struct CasualtiesReport {
    pub attacker_killed: u32,
    pub attacker_wounded: u32,
    pub attacker_captured: u32,
    pub defender_killed: u32,
    pub defender_wounded: u32,
    pub defender_captured: u32,
    pub civilian_casualties: u32,
}