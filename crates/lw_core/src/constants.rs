//! Central constants and configuration values for Living Worlds
//!
//! All magic numbers and tunable parameters in one place.

/// World generation size configurations
pub mod world {
    /// Small world dimensions
    pub const SMALL_WIDTH: usize = 256;
    pub const SMALL_HEIGHT: usize = 256;
    pub const SMALL_PROVINCES: usize = 1000;
    
    /// Medium world dimensions
    pub const MEDIUM_WIDTH: usize = 512;
    pub const MEDIUM_HEIGHT: usize = 512;
    pub const MEDIUM_PROVINCES: usize = 2000;
    
    /// Large world dimensions
    pub const LARGE_WIDTH: usize = 1024;
    pub const LARGE_HEIGHT: usize = 1024;
    pub const LARGE_PROVINCES: usize = 5000;
    
    /// Default counts
    pub const DEFAULT_PROVINCE_COUNT: usize = 100;
    pub const DEFAULT_NATION_COUNT: usize = 20;
    pub const DEFAULT_CITY_COUNT: usize = 50;
    
    /// Limits
    pub const MAX_PROVINCES: usize = 10000;
    pub const MAX_NATIONS: usize = 500;
    pub const MAX_CITIES: usize = 5000;
    pub const MAX_ARMIES: usize = 2000;
    
    /// Province constraints
    pub const MIN_PROVINCE_SIZE: f32 = 10.0;
    pub const MAX_PROVINCE_SIZE: f32 = 1000.0;
    pub const MIN_PROVINCE_NEIGHBORS: usize = 3;
    pub const MAX_PROVINCE_NEIGHBORS: usize = 12;
}

/// Game simulation constants
pub mod simulation {
    /// Time constants
    pub const TICKS_PER_MONTH: u32 = 30;
    pub const MONTHS_PER_YEAR: u32 = 12;
    pub const TICKS_PER_YEAR: u32 = TICKS_PER_MONTH * MONTHS_PER_YEAR;
    pub const START_YEAR: i32 = 0;
    
    /// Stability ranges
    pub const MIN_STABILITY: f32 = 0.0;
    pub const MAX_STABILITY: f32 = 1.0;
    pub const CRITICAL_STABILITY: f32 = 0.2;
    pub const COLLAPSE_THRESHOLD: f32 = 0.1;
    
    /// Population dynamics
    pub const MIN_POPULATION: u32 = 100;
    pub const MAX_POPULATION_PER_PROVINCE: u32 = 1_000_000;
    pub const POPULATION_GROWTH_RATE: f32 = 0.001; // Per month
    pub const STARVATION_RATE: f32 = 0.1; // Population loss per month when starving
    
    /// Economy
    pub const STARTING_TREASURY: f32 = 1000.0;
    pub const MIN_TRADE_DISTANCE: f32 = 50.0;
    pub const MAX_TRADE_DISTANCE: f32 = 500.0;
    pub const TRADE_EFFICIENCY_FALLOFF: f32 = 0.002; // Per unit distance
    
    /// Military
    pub const ARMY_MAINTENANCE_COST: f32 = 10.0; // Per unit per month
    pub const ARMY_MOVEMENT_SPEED: f32 = 50.0; // Units per month
    pub const SIEGE_DURATION: u32 = 3; // Months
    pub const ATTRITION_RATE: f32 = 0.01; // Per month in enemy territory
}

/// Resource production rates per terrain type (monthly)
pub mod resources {
    /// Food production
    pub const FOOD_PLAINS: f32 = 10.0;
    pub const FOOD_SHORE: f32 = 8.0;
    pub const FOOD_FOREST: f32 = 5.0;
    pub const FOOD_HILLS: f32 = 3.0;
    pub const FOOD_DESERT: f32 = 1.0;
    pub const FOOD_MOUNTAINS: f32 = 0.5;
    pub const FOOD_TUNDRA: f32 = 0.2;
    
    /// Wood production
    pub const WOOD_FOREST: f32 = 15.0;
    pub const WOOD_PLAINS: f32 = 3.0;
    pub const WOOD_HILLS: f32 = 5.0;
    pub const WOOD_SHORE: f32 = 2.0;
    
    /// Stone production
    pub const STONE_MOUNTAINS: f32 = 20.0;
    pub const STONE_HILLS: f32 = 10.0;
    pub const STONE_PLAINS: f32 = 2.0;
    pub const STONE_DESERT: f32 = 5.0;
    
    /// Iron production
    pub const IRON_MOUNTAINS: f32 = 10.0;
    pub const IRON_HILLS: f32 = 5.0;
    pub const IRON_PLAINS: f32 = 1.0;
    
    /// Gold production (rare)
    pub const GOLD_MOUNTAINS: f32 = 2.0;
    pub const GOLD_HILLS: f32 = 0.5;
    pub const GOLD_DESERT: f32 = 0.3;
    
    /// Coal production (industrial)
    pub const COAL_MOUNTAINS: f32 = 8.0;
    pub const COAL_HILLS: f32 = 4.0;
    pub const COAL_FOREST: f32 = 1.0;
    
    /// Oil production (modern)
    pub const OIL_DESERT: f32 = 5.0;
    pub const OIL_SHORE: f32 = 3.0;
    pub const OIL_PLAINS: f32 = 1.0;
}

/// Procedural generation parameters
pub mod procedural {
    /// Name generation
    pub const NAME_MIN_LENGTH: usize = 3;
    pub const NAME_MAX_LENGTH: usize = 15;
    pub const NAME_MAX_ATTEMPTS: usize = 100;
    pub const CONSONANT_CLUSTER_MAX: usize = 3;
    pub const VOWEL_CLUSTER_MAX: usize = 2;
    pub const CULTURE_SUFFIX_CHANCE: f32 = 0.3;
    pub const NATION_SUFFIX_CHANCE: f32 = 0.5;
    
    /// Terrain generation
    pub const TERRAIN_OCTAVES: usize = 6;
    pub const TERRAIN_FREQUENCY: f32 = 0.01;
    pub const TERRAIN_PERSISTENCE: f32 = 0.5;
    pub const TERRAIN_LACUNARITY: f32 = 2.0;
    pub const EROSION_ITERATIONS: usize = 10;
    pub const EROSION_STRENGTH: f32 = 0.1;
    
    /// Province generation (Voronoi)
    pub const LLOYD_RELAXATION_ITERATIONS: usize = 3;
    pub const MIN_PROVINCE_DISTANCE: f32 = 20.0;
    pub const BORDER_THICKNESS: f32 = 2.0;
    
    /// City placement (Poisson disc)
    pub const CITY_MIN_DISTANCE: f32 = 30.0;
    pub const CITY_MAX_ATTEMPTS: usize = 30;
    pub const CAPITAL_MIN_DISTANCE: f32 = 100.0;
    
    /// Color palette
    pub const PALETTE_ATTEMPTS: usize = 100;
    pub const MIN_COLOR_DISTANCE: f32 = 0.2; // In HSV space
    pub const NATION_HUE_VARIANCE: f32 = 0.1;
}

/// Rendering parameters
pub mod render {
    /// Clear color (dark blue-grey)
    pub const CLEAR_COLOR: [f32; 4] = [0.1, 0.1, 0.15, 1.0];
    
    /// Camera
    pub const MIN_ZOOM: f32 = 0.1;
    pub const MAX_ZOOM: f32 = 10.0;
    pub const DEFAULT_ZOOM: f32 = 1.0;
    pub const ZOOM_SPEED: f32 = 1.1;
    pub const PAN_SPEED: f32 = 500.0; // Pixels per second
    
    /// Map rendering
    pub const PROVINCE_ALPHA: f32 = 0.8;
    pub const BORDER_ALPHA: f32 = 1.0;
    pub const RIVER_WIDTH: f32 = 2.0;
    pub const ROAD_WIDTH: f32 = 1.5;
    pub const CITY_MIN_RADIUS: f32 = 3.0;
    pub const CITY_MAX_RADIUS: f32 = 10.0;
    pub const CAPITAL_STAR_POINTS: usize = 5;
    
    /// UI
    pub const UI_PANEL_WIDTH: f32 = 300.0;
    pub const UI_PANEL_ALPHA: f32 = 0.9;
    pub const TOOLTIP_DELAY_MS: u32 = 500;
}

/// Window configuration
pub mod window {
    pub const DEFAULT_WIDTH: u32 = 1280;
    pub const DEFAULT_HEIGHT: u32 = 720;
    pub const MIN_WIDTH: u32 = 800;
    pub const MIN_HEIGHT: u32 = 600;
    pub const TITLE: &str = "Living Worlds";
}

/// Diplomacy constants
pub mod diplomacy {
    pub const INITIAL_OPINION: i32 = 0;
    pub const MIN_OPINION: i32 = -100;
    pub const MAX_OPINION: i32 = 100;
    pub const WAR_THRESHOLD: i32 = -50;
    pub const ALLIANCE_THRESHOLD: i32 = 50;
    pub const OPINION_DECAY_RATE: i32 = 1; // Per month
    pub const BORDER_FRICTION: i32 = -5; // Opinion penalty for sharing borders
    pub const TRADE_BONUS: i32 = 10; // Opinion bonus for trade
    pub const WAR_WEARINESS_GROWTH: f32 = 0.01; // Per month at war
}

/// Technology constants
pub mod technology {
    pub const BASE_RESEARCH_RATE: f32 = 1.0;
    pub const TECH_SPREAD_CHANCE: f32 = 0.01; // Per neighbor per month
    pub const TECH_LOSS_CHANCE: f32 = 0.001; // During dark ages
    pub const INNOVATION_CHANCE: f32 = 0.0001; // Random discovery
    pub const MAX_TECH_LEVEL: u32 = 100;
}

/// Event system constants
pub mod events {
    pub const PLAGUE_BASE_CHANCE: f32 = 0.001;
    pub const PLAGUE_DURATION: u32 = 12; // Months
    pub const PLAGUE_MORTALITY: f32 = 0.3;
    pub const PLAGUE_SPREAD_CHANCE: f32 = 0.1;
    
    pub const FAMINE_BASE_CHANCE: f32 = 0.002;
    pub const FAMINE_DURATION: u32 = 6;
    pub const FAMINE_MORTALITY: f32 = 0.2;
    
    pub const GOLDEN_AGE_CHANCE: f32 = 0.0005;
    pub const GOLDEN_AGE_DURATION: u32 = 24;
    pub const GOLDEN_AGE_BONUS: f32 = 1.5; // Multiplier for production
    
    pub const DISASTER_BASE_CHANCE: f32 = 0.0001;
    pub const DISASTER_DAMAGE: f32 = 0.5; // Infrastructure damage
}

/// Audio synthesis parameters
pub mod audio {
    pub const SAMPLE_RATE: u32 = 44100;
    pub const CHANNELS: u16 = 2;
    pub const BASE_FREQUENCY: f32 = 440.0; // A4
    pub const VOLUME: f32 = 0.3;
    
    /// Wavetable sizes
    pub const WAVETABLE_SIZE: usize = 2048;
    pub const ENVELOPE_ATTACK_MS: u32 = 10;
    pub const ENVELOPE_DECAY_MS: u32 = 100;
    pub const ENVELOPE_SUSTAIN: f32 = 0.7;
    pub const ENVELOPE_RELEASE_MS: u32 = 200;
    
    /// Ambient parameters
    pub const AMBIENT_LAYERS: usize = 4;
    pub const AMBIENT_FREQ_MIN: f32 = 100.0;
    pub const AMBIENT_FREQ_MAX: f32 = 2000.0;
}

/// File paths and save game
pub mod paths {
    pub const SAVES_DIR: &str = "saves";
    pub const SETTINGS_FILE: &str = "settings.json";
    pub const LOG_FILE: &str = "living_worlds.log";
    pub const AUTOSAVE_INTERVAL_MINUTES: u32 = 5;
    pub const MAX_SAVE_SLOTS: usize = 10;
}