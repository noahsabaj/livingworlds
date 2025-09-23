//! Infrastructure Relationships - Physical connections and trade networks
//!
//! This module defines relationships for roads, trade routes, and other infrastructure
//! that connects provinces and enables economic activity.

use bevy::prelude::*;

// ================================================================================================
// ROAD NETWORK RELATIONSHIPS
// ================================================================================================

/// A province is connected to another by road
/// Infrastructure for trade, military movement, and communication
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = ConnectedRoads)]
pub struct ConnectedByRoad(pub Entity);

/// Reverse relationship: A province has road connections to other provinces
/// Automatically maintained by Bevy when `ConnectedByRoad` is added
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = ConnectedByRoad, linked_spawn)]
pub struct ConnectedRoads(Vec<Entity>); // Private for safety - Bevy handles internal access

impl ConnectedRoads {
    /// Get read-only access to connected provinces via roads
    pub fn connected_provinces(&self) -> &[Entity] {
        &self.0
    }

    /// Get the number of road connections
    pub fn connection_count(&self) -> usize {
        self.0.len()
    }

    /// Check if connected to a specific province by road
    pub fn connected_to(&self, province: Entity) -> bool {
        self.0.contains(&province)
    }

    /// Check if province has any road connections
    pub fn has_connections(&self) -> bool {
        !self.0.is_empty()
    }
}

// ================================================================================================
// TRADE ROUTE RELATIONSHIPS
// ================================================================================================

/// A province is connected to another by trade route
/// Economic connection for goods flow and commercial activity
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = ConnectedTradeRoutes)]
pub struct ConnectedByTrade(pub Entity);

/// Reverse relationship: A province has trade route connections to other provinces
/// Automatically maintained by Bevy when `ConnectedByTrade` is added
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = ConnectedByTrade, linked_spawn)]
pub struct ConnectedTradeRoutes(Vec<Entity>); // Private for safety - Bevy handles internal access

impl ConnectedTradeRoutes {
    /// Get read-only access to connected provinces via trade routes
    pub fn connected_provinces(&self) -> &[Entity] {
        &self.0
    }

    /// Get the number of trade route connections
    pub fn connection_count(&self) -> usize {
        self.0.len()
    }

    /// Check if connected to a specific province by trade route
    pub fn connected_to(&self, province: Entity) -> bool {
        self.0.contains(&province)
    }

    /// Check if province has any trade route connections
    pub fn has_trade_connections(&self) -> bool {
        !self.0.is_empty()
    }
}

// ================================================================================================
// INFRASTRUCTURE ENTITIES
// ================================================================================================

/// Marker component for road entities
#[derive(Component, Debug, Clone)]
pub struct Road {
    pub name: String,
    pub quality: RoadQuality,
    pub length: f32,            // Distance in world units
    pub maintenance_cost: f32,  // Ongoing cost to maintain
    pub construction_year: u32, // When it was built
}

/// Marker component for trade route entities
#[derive(Component, Debug, Clone)]
pub struct TradeRoute {
    pub name: String,
    pub route_type: TradeRouteType,
    pub volume: f32,        // Trade volume (goods per year)
    pub profit_margin: f32, // Profitability
    pub security: f32,      // 0.0 = dangerous, 1.0 = completely safe
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoadQuality {
    Dirt,        // Basic dirt path
    Cobblestone, // Improved stone road
    Paved,       // Advanced paved road
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeRouteType {
    Local,         // Between neighboring provinces
    Regional,      // Across cultural regions
    International, // Between different nations
    Maritime,      // Sea-based trade
}

// ================================================================================================
// INFRASTRUCTURE DATA
// ================================================================================================

/// Provincial infrastructure status
#[derive(Component, Debug, Clone)]
pub struct InfrastructureStatus {
    /// Number of roads connected to this province
    pub road_connections: u32,
    /// Number of trade routes passing through
    pub trade_route_count: u32,
    /// Overall connectivity (0.0 = isolated, 1.0 = major hub)
    pub connectivity: f32,
    /// Infrastructure maintenance cost
    pub maintenance_cost: f32,
}

// ================================================================================================
// QUERY SYSTEMS - Infrastructure queries
// ================================================================================================

/// System for querying road connections
pub fn query_road_network_system(roads_query: Query<(Entity, &Road, &ConnectedByRoad)>) {
    for (road_entity, road, connected_by_road) in roads_query.iter() {
        debug!(
            "Road {:?} ({}) connects to province {:?}",
            road_entity, road.name, connected_by_road.0
        );
    }
}

/// System for querying trade route connections
pub fn query_trade_network_system(routes_query: Query<(Entity, &TradeRoute, &ConnectedByTrade)>) {
    for (route_entity, route, connected_by_trade) in routes_query.iter() {
        debug!(
            "Trade route {:?} ({}) connects to province {:?}",
            route_entity, route.name, connected_by_trade.0
        );
    }
}

/// Find all roads connected to a province
/// Using entity relationships for O(1) lookup instead of O(n) filtering
pub fn find_province_roads(
    province_entity: Entity,
    provinces_query: &Query<&ConnectedRoads>,
) -> Option<Vec<Entity>> {
    provinces_query
        .get(province_entity)
        .ok()
        .map(|connected_roads| connected_roads.connected_provinces().to_vec())
}

/// Find all trade routes connected to a province
/// Using entity relationships for O(1) lookup instead of O(n) filtering
pub fn find_province_trade_routes(
    province_entity: Entity,
    provinces_query: &Query<&ConnectedTradeRoutes>,
) -> Option<Vec<Entity>> {
    provinces_query
        .get(province_entity)
        .ok()
        .map(|connected_trade_routes| connected_trade_routes.connected_provinces().to_vec())
}

// ================================================================================================
// INFRASTRUCTURE SYSTEMS
// ================================================================================================

/// Updates infrastructure status for all provinces
/// Using entity relationships for O(1) lookups instead of O(n) filtering
pub fn update_infrastructure_status(
    mut provinces_query: Query<(
        Entity,
        &mut InfrastructureStatus,
        Option<&ConnectedRoads>,
        Option<&ConnectedTradeRoutes>,
    )>,
) {
    // NOTE: Bevy queries should not be manually parallelized with Rayon
    // Bevy has its own parallel scheduling system
    for (province_entity, mut infrastructure, connected_roads, connected_trade_routes) in &mut provinces_query {
        // Count road connections using entity relationships
        infrastructure.road_connections = connected_roads
            .map(|roads| roads.connection_count() as u32)
            .unwrap_or(0);

        // Count trade route connections using entity relationships
        infrastructure.trade_route_count = connected_trade_routes
            .map(|trade_routes| trade_routes.connection_count() as u32)
            .unwrap_or(0);

        // Calculate connectivity score
        let road_score = (infrastructure.road_connections as f32 * 0.3).min(1.0);
        let trade_score = (infrastructure.trade_route_count as f32 * 0.2).min(1.0);
        infrastructure.connectivity = (road_score + trade_score).min(1.0);

        // Note: Maintenance cost calculation would require accessing Road components
        // which would need a separate query. For now, we'll set a placeholder.
        infrastructure.maintenance_cost = infrastructure.road_connections as f32 * 10.0;
    }
}

/// Calculates trade efficiency between provinces
pub fn calculate_trade_efficiency(
    province_a: Entity,
    province_b: Entity,
    routes_query: &Query<(&TradeRoute, &ConnectedByTrade)>,
) -> f32 {
    // Check for direct trade route
    let direct_route = routes_query.iter().find(|(_, connected_by_trade)| {
        connected_by_trade.0 == province_a || connected_by_trade.0 == province_b
    });

    if let Some((trade_route, _)) = direct_route {
        trade_route.security * (1.0 + trade_route.profit_margin)
    } else {
        // No direct route - lower efficiency
        0.3
    }
}

// ================================================================================================
// INFRASTRUCTURE EVENTS
// ================================================================================================

/// Event fired when a new road is constructed
#[derive(Event, Debug, Clone)]
pub struct RoadConstructedEvent {
    pub road: Entity,
    pub province_a: Entity,
    pub province_b: Entity,
    pub constructor: Entity, // Nation that built it
}

/// Event fired when a trade route is established
#[derive(Event, Debug, Clone)]
pub struct TradeRouteEstablishedEvent {
    pub route: Entity,
    pub province_a: Entity,
    pub province_b: Entity,
}

/// Event fired when infrastructure needs maintenance
#[derive(Event, Debug, Clone)]
pub struct InfrastructureMaintenanceEvent {
    pub infrastructure: Entity,
    pub province: Entity,
    pub maintenance_cost: f32,
    pub urgency: MaintenanceUrgency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceUrgency {
    Routine,  // Regular maintenance
    Urgent,   // Needs immediate attention
    Critical, // Infrastructure failing
}

// ================================================================================================
// VALIDATION SYSTEMS
// ================================================================================================

/// Validates infrastructure connections
pub fn validate_infrastructure_connections(
    roads_query: Query<(Entity, &ConnectedByRoad)>,
    routes_query: Query<(Entity, &ConnectedByTrade)>,
    provinces_query: Query<Entity>,
) {
    let valid_provinces: std::collections::HashSet<Entity> = provinces_query.iter().collect();

    // Validate road connections
    for (road_entity, connected_by_road) in roads_query.iter() {
        if !valid_provinces.contains(&connected_by_road.0) {
            warn!(
                "Road {:?} connects to invalid province {:?}",
                road_entity, connected_by_road.0
            );
        }
    }

    // Validate trade route connections
    for (route_entity, connected_by_trade) in routes_query.iter() {
        if !valid_provinces.contains(&connected_by_trade.0) {
            warn!(
                "Trade route {:?} connects to invalid province {:?}",
                route_entity, connected_by_trade.0
            );
        }
    }
}
