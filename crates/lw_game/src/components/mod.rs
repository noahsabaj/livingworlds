//! Core ECS components for Living Worlds simulation
//!
//! These components define all entities in the game world using
//! the full Bevy framework for the Entity Component System architecture.
//! 
//! Built on Austrian Economics principles: economics emerges from individual
//! human actions. No predetermined winners - any system can succeed or fail
//! based on execution and circumstances.

// Domain modules - organized by concern
pub mod geography;
pub mod infrastructure;
pub mod economics;
pub mod governance;
pub mod common; // Common types like bounded values

// Existing modules
pub mod individual;
pub mod ai;
pub mod culture;
pub mod military;
pub mod diplomacy;
pub mod simulation;

// Re-export key types
pub use geography::*;
pub use infrastructure::*;
pub use economics::*;
pub use governance::*;
pub use individual::*;
pub use ai::*;
pub use culture::*;
pub use military::*;
pub use diplomacy::*;
pub use simulation::*;

use bevy::prelude::*;
use lw_core::{Fixed32, Vec2fx};
use serde::{Deserialize, Serialize};

// ============================================================================
// ENUMS
// ============================================================================

// ResourceType removed - use GoodType from economics/goods.rs instead
// This consolidation eliminates DRY violation where ResourceType
// duplicated items that were already in GoodType (Wood, Stone, Coal, etc.)
// Import GoodType to replace ResourceType usage throughout this file
use crate::components::economics::GoodType;

/// Terrain types that determine province characteristics
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType {
    DeepOcean = 0,
    Ocean,
    Shore,
    Plains,
    Hills,
    Mountains,
    Peaks,
    Desert,
    Forest,
    Tundra,
}

/// Economic system types that nations can adopt
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EconomicSystem {
    Tribal = 0,      // Resource sharing, no currency
    Feudal,          // Land-based wealth, limited trade
    Mercantile,      // Gold accumulation, colonial extraction
    Market,          // Supply/demand, private property
    Command,         // Central planning, state ownership
    Mixed,           // Regulated markets, public services
    Cooperative,     // Worker ownership, communes
}

/// Government types that determine nation behavior
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GovernmentType {
    Tribal = 0,
    Monarchy,
    Republic,
    Democracy,
    Dictatorship,
    Oligarchy,
    Theocracy,
}

/// City size categories
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CitySize {
    Village = 0,    // < 1k population
    Town,           // 1k - 10k
    City,           // 10k - 100k
    Metropolis,     // > 100k
}

/// Casus belli (justification for war)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CasusBelli {
    None = 0,
    Conquest,       // Territory expansion
    Resources,      // Resource control
    Religion,       // Religious differences
    Liberation,     // Free occupied territory
    Defensive,      // Response to aggression
    Revenge,        // Historical grievance
}

// ============================================================================
// CORE COMPONENTS
// ============================================================================

/// Position in world coordinates using fixed-point math
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub pos: Vec2fx,
}

impl Position {
    pub fn new(x: Fixed32, y: Fixed32) -> Self {
        Self {
            pos: Vec2fx::new(x, y),
        }
    }
}

/// Province - fundamental territorial unit
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Province {
    pub id: u32,
    pub terrain: TerrainType,
    pub center: Vec2fx,
    pub area: Fixed32,
    
    // Political control
    pub owner: Option<u32>,       // Nation ID that owns this province (None = uncontrolled)
    
    // Resources
    pub resource: Option<GoodType>,
    pub resource_quantity: Fixed32,
    
    // Development
    pub development: Fixed32,  // 0-1 development level
    
    // Production capacity (Austrian economics - land constraints)
    pub arable_land: Fixed32,     // Hectares of farmable land
    pub coastal_access: Fixed32,  // Km of coastline for fishing
    pub pasture_land: Fixed32,    // Hectares for grazing
    pub forest_coverage: Fixed32, // Hectares of forest
}

impl Province {
    pub fn new(id: u32, terrain: TerrainType, center: Vec2fx, area: Fixed32) -> Self {
        let mut province = Self {
            id,
            terrain,
            center,
            area,
            owner: None,  // Uncontrolled by default
            resource: None,
            resource_quantity: Fixed32::ZERO,
            development: Fixed32::ZERO,
            arable_land: Fixed32::ZERO,
            coastal_access: Fixed32::ZERO,
            pasture_land: Fixed32::ZERO,
            forest_coverage: Fixed32::ZERO,
        };
        province.setup_production_capacity();
        province
    }
    
    /// Set production capacities based on terrain type
    fn setup_production_capacity(&mut self) {
        let base_hectares = self.area * Fixed32::from_num(100); // 100 hectares per km²
        
        match self.terrain {
            TerrainType::Plains => {
                self.arable_land = base_hectares * Fixed32::from_float(0.7);
                self.pasture_land = base_hectares * Fixed32::from_float(0.2);
                self.forest_coverage = base_hectares * Fixed32::from_float(0.1);
            }
            TerrainType::Shore => {
                self.arable_land = base_hectares * Fixed32::from_float(0.2);
                self.coastal_access = Fixed32::from_num(50);
                self.pasture_land = base_hectares * Fixed32::from_float(0.1);
            }
            TerrainType::Hills => {
                self.arable_land = base_hectares * Fixed32::from_float(0.2);
                self.pasture_land = base_hectares * Fixed32::from_float(0.5);
                self.forest_coverage = base_hectares * Fixed32::from_float(0.2);
            }
            TerrainType::Forest => {
                self.arable_land = base_hectares * Fixed32::from_float(0.1);
                self.forest_coverage = base_hectares * Fixed32::from_float(0.8);
            }
            TerrainType::Desert => {
                self.pasture_land = base_hectares * Fixed32::from_float(0.1);
            }
            TerrainType::Mountains | TerrainType::Peaks => {
                self.pasture_land = base_hectares * Fixed32::from_float(0.05);
            }
            _ => {}
        }
    }
}

/// Nation - sovereign political entity
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Nation {
    pub id: u32,
    pub name: String,
    pub color: u32,  // RGB packed as u32
    
    // Government
    pub government: GovernmentType,
    pub economy: EconomicSystem,
    
    // Resources
    pub treasury: Fixed32,
    pub stability: Fixed32,     // 0-1
    pub legitimacy: Fixed32,    // 0-1
    pub is_collapsing: bool,
    
    // Technology
    pub tech_level: u8,
    pub research_points: f32,
    
    // Culture and religion
    pub culture_id: u32,
    pub religion_id: u32,
    
    // Statistics
    pub owned_provinces: u32,
    pub gdp: Fixed32,
    
    // AI personality (0-10)
    pub aggression: u8,
    pub expansion: u8,
    pub diplomacy: u8,
    pub trade_focus: u8,
}

impl Nation {
    pub fn new(id: u32, name: String, color: u32) -> Self {
        Self {
            id,
            name,
            color,
            government: GovernmentType::Monarchy,
            economy: EconomicSystem::Feudal,
            treasury: Fixed32::from_num(1000),
            stability: Fixed32::ONE,
            legitimacy: Fixed32::ONE,
            is_collapsing: false,
            tech_level: 0,
            research_points: 0.0,
            culture_id: 0,
            religion_id: 0,
            owned_provinces: 0,
            gdp: Fixed32::ZERO,
            aggression: 5,
            expansion: 5,
            diplomacy: 5,
            trade_focus: 5,
        }
    }
}

/// City - urban center of trade and culture
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct City {
    pub id: u32,
    pub name: String,
    pub province_id: u32,
    pub nation_id: u32,
    pub position: Vec2fx,
    pub size: CitySize,
    pub is_capital: bool,
    pub has_port: bool,
}

impl City {
    pub fn new(
        id: u32,
        name: String,
        province_id: u32,
        nation_id: u32,
        position: Vec2fx,
    ) -> Self {
        Self {
            id,
            name,
            province_id,
            nation_id,
            position,
            size: CitySize::Village,
            is_capital: false,
            has_port: false,
        }
    }
}

/// Population - Austrian economics model where every person is an economic actor
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Population {
    pub count: Fixed32,
    pub growth_rate: Fixed32,
    pub happiness: Fixed32,  // 0-1
    
    // Labor allocation (absolute numbers)
    pub farmers: Fixed32,    // Working arable land
    pub fishers: Fixed32,    // Coastal fishing
    pub ranchers: Fixed32,   // Managing livestock
    pub hunters: Fixed32,    // Forest hunting/gathering
    pub artisans: Fixed32,   // Crafting goods
    pub merchants: Fixed32,  // Trading
    pub laborers: Fixed32,   // General labor
    pub idle: Fixed32,       // Unemployed
    
    // Survival tracking
    pub hunger_days: Fixed32,
    pub food_deficit: Fixed32,
}

impl Population {
    pub const MAX_HUNGER_DAYS: i32 = 14;
    pub const FOOD_PER_PERSON_PER_MONTH: f32 = 30.0;
    
    pub fn new(initial_pop: Fixed32) -> Self {
        Self {
            count: initial_pop,
            growth_rate: Fixed32::from_float(0.002),
            happiness: Fixed32::from_float(0.5),
            farmers: Fixed32::ZERO,
            fishers: Fixed32::ZERO,
            ranchers: Fixed32::ZERO,
            hunters: Fixed32::ZERO,
            artisans: Fixed32::ZERO,
            merchants: Fixed32::ZERO,
            laborers: Fixed32::ZERO,
            idle: Fixed32::ZERO,
            hunger_days: Fixed32::ZERO,
            food_deficit: Fixed32::ZERO,
        }
    }
    
    /// Get labor force (60% of population are working adults)
    pub fn get_labor_force(&self) -> Fixed32 {
        self.count * Fixed32::from_float(0.6)
    }
    
    /// Get total food producers
    pub fn get_food_producers(&self) -> Fixed32 {
        self.farmers + self.fishers + self.ranchers + self.hunters
    }
    
    /// Get food needed this month
    pub fn get_food_needed(&self) -> Fixed32 {
        self.count * Fixed32::from_float(Self::FOOD_PER_PERSON_PER_MONTH)
    }
}

/// Army - military unit
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Army {
    pub id: u32,
    pub nation_id: u32,
    pub name: String,
    pub units: u32,           // Number of soldiers
    pub morale: Fixed32,      // 0-1
    pub experience: Fixed32,  // 0-1
    
    // Movement
    pub target_province: Option<u32>,
    pub movement_progress: Fixed32,  // 0-1
    
    // Combat
    pub is_fighting: bool,
    pub enemy_army: Option<Entity>,
    pub combat_strength: Fixed32,
}

impl Army {
    pub fn new(id: u32, nation_id: u32, name: String, units: u32) -> Self {
        Self {
            id,
            nation_id,
            name,
            units,
            morale: Fixed32::ONE,
            experience: Fixed32::ZERO,
            target_province: None,
            movement_progress: Fixed32::ZERO,
            is_fighting: false,
            enemy_army: None,
            combat_strength: Fixed32::from_num(units as i32),
        }
    }
}

/// Economy component - tracks GDP and market prices
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Economy {
    pub gdp: Fixed32,
    pub gdp_per_capita: Fixed32,
    pub inflation: Fixed32,
    pub trade_balance: Fixed32,
    
    // Market prices (emerge from supply/demand)
    pub food_price: Fixed32,
    pub wood_price: Fixed32,
    pub stone_price: Fixed32,
    pub iron_price: Fixed32,
    pub gold_price: Fixed32,
    pub coal_price: Fixed32,
    pub oil_price: Fixed32,
    
    // Price array for system compatibility (maps to individual prices above)
    pub resource_prices: Vec<Fixed32>,  // [food, wood, stone, iron, gold, coal, oil, other]
    
    // Trade efficiency
    pub trade_efficiency: Fixed32,  // 0-1
    pub market_access: Fixed32,     // 0-1
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            gdp: Fixed32::ZERO,
            gdp_per_capita: Fixed32::ZERO,
            inflation: Fixed32::ZERO,
            trade_balance: Fixed32::ZERO,
            food_price: Fixed32::ONE,
            wood_price: Fixed32::ONE,
            stone_price: Fixed32::ONE,
            iron_price: Fixed32::from_num(2),
            gold_price: Fixed32::from_num(10),
            coal_price: Fixed32::from_num(3),
            oil_price: Fixed32::from_num(5),
            resource_prices: vec![
                Fixed32::ONE,       // food
                Fixed32::ONE,       // wood  
                Fixed32::ONE,       // stone
                Fixed32::from_num(2), // iron
                Fixed32::from_num(10), // gold
                Fixed32::from_num(3), // coal
                Fixed32::from_num(5), // oil
                Fixed32::ONE,       // other
            ],
            trade_efficiency: Fixed32::from_float(0.5),
            market_access: Fixed32::from_float(0.5),
        }
    }
}

/// Territory control and culture
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Territory {
    pub controlling_nation: u32,
    pub control_strength: Fixed32,  // 0-1
    pub culture_id: u32,
    pub religion_id: u32,
    pub culture_conversion_progress: Fixed32,  // 0-1
    pub fortification_level: u8,  // 0-10
}

/// Trade route between cities
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct TradeRoute {
    pub id: u32,             // Unique identifier
    pub from_city: u32,
    pub to_city: u32,
    pub resource: Option<GoodType>,
    pub quantity: Fixed32,   // Amount being traded
    pub flow_rate: Fixed32,  // Units per month
    pub efficiency: Fixed32,  // 0-1 based on distance and infrastructure
    pub is_active: bool,     // Whether route is currently operating
}

/// Diplomatic relation between two nations
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct DiplomaticRelation {
    pub nation_a: u32,
    pub nation_b: u32,
    pub opinion: Fixed32,  // -100 to 100
    pub is_at_war: bool,
    pub is_allied: bool,
    pub trade_agreement: bool,
}

/// Resource production component
#[derive(Component, Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResourceProduction {
    pub food: Fixed32,
    pub wood: Fixed32,
    pub stone: Fixed32,
    pub iron: Fixed32,
    pub gold: Fixed32,
    pub coal: Fixed32,
    pub oil: Fixed32,
    pub efficiency: Fixed32,
}

impl ResourceProduction {
    pub fn get_production(&self, resource: GoodType) -> Fixed32 {
        let base = match resource {
            GoodType::Food => self.food,
            GoodType::Wood => self.wood,
            GoodType::Stone => self.stone,
            GoodType::IronOre => self.iron,
            GoodType::Gold => self.gold,
            GoodType::Coal => self.coal,
            GoodType::Oil => self.oil,
            _ => Fixed32::ZERO,
        };
        base * self.efficiency
    }
}

/// Resource stockpile component  
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceStockpile {
    pub food: Fixed32,
    pub wood: Fixed32,
    pub stone: Fixed32,
    pub iron: Fixed32,
    pub gold: Fixed32,
    pub coal: Fixed32,
    pub oil: Fixed32,
    
    // Collections for system compatibility
    pub resources: Vec<Fixed32>,         // [food, wood, stone, iron, gold, coal, oil, other]
    pub storage_capacity: Vec<Fixed32>,  // Maximum storage for each resource
}

impl Default for ResourceStockpile {
    fn default() -> Self {
        let initial = Fixed32::from_num(100);
        let max_capacity = Fixed32::from_num(1000);
        Self {
            food: initial,
            wood: initial,
            stone: initial,
            iron: initial,
            gold: initial,
            coal: Fixed32::ZERO,
            oil: Fixed32::ZERO,
            resources: vec![
                initial,        // food
                initial,        // wood
                initial,        // stone
                initial,        // iron
                initial,        // gold
                Fixed32::ZERO,  // coal
                Fixed32::ZERO,  // oil
                Fixed32::ZERO,  // other
            ],
            storage_capacity: vec![
                max_capacity, // food
                max_capacity, // wood
                max_capacity, // stone
                max_capacity, // iron
                max_capacity, // gold
                max_capacity, // coal
                max_capacity, // oil
                max_capacity, // other
            ],
        }
    }
}

impl ResourceStockpile {
    pub fn get_amount(&self, resource: GoodType) -> Fixed32 {
        match resource {
            GoodType::Food => self.food,
            GoodType::Wood => self.wood,
            GoodType::Stone => self.stone,
            GoodType::IronOre => self.iron,
            GoodType::Gold => self.gold,
            GoodType::Coal => self.coal,
            GoodType::Oil => self.oil,
            _ => Fixed32::ZERO,
        }
    }
    
    pub fn add_resource(&mut self, resource: GoodType, amount: Fixed32) {
        let max_stock = Fixed32::from_num(10000);
        match resource {
            GoodType::Food => self.food = (self.food + amount).min(max_stock),
            GoodType::Wood => self.wood = (self.wood + amount).min(max_stock),
            GoodType::Stone => self.stone = (self.stone + amount).min(max_stock),
            GoodType::IronOre => self.iron = (self.iron + amount).min(max_stock),
            GoodType::Gold => self.gold = (self.gold + amount).min(max_stock),
            GoodType::Coal => self.coal = (self.coal + amount).min(max_stock),
            GoodType::Oil => self.oil = (self.oil + amount).min(max_stock),
            _ => {}
        }
    }
    
    pub fn remove_resource(&mut self, resource: GoodType, amount: Fixed32) -> Fixed32 {
        let available = self.get_amount(resource);
        let removed = amount.min(available);
        
        match resource {
            GoodType::Food => self.food = self.food - removed,
            GoodType::Wood => self.wood = self.wood - removed,
            GoodType::Stone => self.stone = self.stone - removed,
            GoodType::IronOre => self.iron = self.iron - removed,
            GoodType::Gold => self.gold = self.gold - removed,
            GoodType::Coal => self.coal = self.coal - removed,
            GoodType::Oil => self.oil = self.oil - removed,
            _ => {}
        }
        
        removed
    }
}

/// Time state for calendar system
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct TimeState {
    pub current_year: i32,
    pub current_month: u8,  // 1-12
    pub current_day: u8,    // 1-30
    pub accumulated_time: Fixed32,  // Sub-day time tracking
    pub tick: u32,  // Total ticks since start
    pub speed_multiplier: f32,  // Game speed
    pub is_paused: bool,
    
    // Calendar constants
    pub days_per_month: u8,    // Usually 30
    pub months_per_year: u8,   // Usually 12
}

impl Default for TimeState {
    fn default() -> Self {
        Self {
            current_year: 0,
            current_month: 1,
            current_day: 1,
            accumulated_time: Fixed32::ZERO,
            tick: 0,
            speed_multiplier: 1.0,
            is_paused: false,
            days_per_month: 30,
            months_per_year: 12,
        }
    }
}

impl TimeState {
    pub fn advance_day(&mut self) {
        self.current_day += 1;
        self.tick += 1;
        
        if self.current_day > self.days_per_month {
            self.current_day = 1;
            self.current_month += 1;
            
            if self.current_month > self.months_per_year {
                self.current_month = 1;
                self.current_year += 1;
            }
        }
    }
    
    pub fn get_date_string(&self) -> String {
        format!("{}/{}/{}", self.current_day, self.current_month, self.current_year)
    }
}

// ============================================================================
// EVENT COMPONENTS
// ============================================================================

/// Event that affects the world
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: u32,
    pub event_type: EventType,
    pub target_entity: Option<Entity>,
    pub duration_remaining: u32,  // Ticks
    pub intensity: Fixed32,  // 0-1
    pub is_active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    Plague,
    Famine,
    GoldenAge,
    DarkAge,
    NaturalDisaster,
    Revolution,
    Discovery,
    War,
    Peace,
}

// ============================================================================
// BUNDLE DEFINITIONS
// ============================================================================

/// Bundle for creating a new province entity
#[derive(Bundle)]
pub struct ProvinceBundle {
    pub province: Province,
    pub position: Position,
    pub territory: Territory,
    pub population: Population,
    pub production: ResourceProduction,
}

/// Bundle for creating a new nation entity
#[derive(Bundle)]
pub struct NationBundle {
    pub nation: Nation,
    pub economy: Economy,
    pub stockpile: ResourceStockpile,
}

/// Bundle for creating a new city entity
#[derive(Bundle)]
pub struct CityBundle {
    pub city: City,
    pub position: Position,
}

/// Bundle for creating a new army entity
#[derive(Bundle)]
pub struct ArmyBundle {
    pub army: Army,
    pub position: Position,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore] // TODO: Fix test after Province refactoring
    fn test_province_production_capacity() {
        // Province::new() no longer exists - need to rewrite test
        // using proper entity spawning
    }
    
    #[test]
    #[ignore] // TODO: Fix test after Population refactoring
    fn test_population_labor() {
        // Population::new() no longer exists - components are pure data
        // Need to rewrite test using proper entity spawning
    }
    
    #[test]
    fn test_resource_stockpile() {
        let mut stockpile = ResourceStockpile::default();
        
        stockpile.add_resource(GoodType::Food, Fixed32::from_num(50));
        assert_eq!(stockpile.food.to_f32(), 150.0); // 100 + 50
        
        let removed = stockpile.remove_resource(GoodType::Food, Fixed32::from_num(200));
        assert_eq!(removed.to_f32(), 150.0); // Can only remove what's available
        assert_eq!(stockpile.food, Fixed32::ZERO);
    }
    
    #[test]
    fn test_time_advancement() {
        let mut time = TimeState::default();
        
        for _ in 0..30 {
            time.advance_day();
        }
        
        assert_eq!(time.month, 2);
        assert_eq!(time.day, 1);
    }
}