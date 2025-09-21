//! Settlement system gateway - Cities, towns, villages, and infrastructure
//!
//! This module handles all settlement-related functionality including spawning,
//! growth, visual representation, and road networks between settlements.

// PRIVATE MODULES
mod types;
mod spawning;
mod growth;
mod roads;
mod rendering;

// PUBLIC EXPORTS
pub use types::{
    Settlement,
    SettlementType,
    SettlementBundle,
    ProductionFocus,
    SettlementGrowthFactors,
};

pub use spawning::{
    spawn_capital_city,
    spawn_settlement,
    calculate_settlement_position,
};

pub use growth::{
    update_settlement_growth,
    upgrade_settlement_type,
    calculate_growth_rate,
};

pub use roads::{
    Road,
    RoadQuality,
    RoadBundle,
    create_road_between,
    calculate_road_path,
};

pub use rendering::{
    render_settlements,
    render_roads,
    SettlementSprite,
    RoadMesh,
};