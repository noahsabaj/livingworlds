//! Economy system - handles trade, production, and markets

use bevy::prelude::*;
use crate::components::{Nation, Province, Territory, Economy, ResourceProduction, ResourceStockpile, TradeRoute, EconomicSystem};
use crate::components::GoodType;
use lw_core::{Fixed32, DeterministicRNG};

#[derive(Event, Debug)]
pub struct MarketUpdateEvent {
    pub nation_id: u32,
    pub resource: u8,
    pub old_price: Fixed32,
    pub new_price: Fixed32,
}

#[derive(Event, Debug)]
pub struct TradeCompletedEvent {
    pub route_id: u32,
    pub resource: u8,
    pub quantity: Fixed32,
    pub value: Fixed32,
}

/// Main economic update system
pub fn economy_system(
    mut nations: Query<(&mut Nation, &mut Economy, &mut ResourceStockpile)>,
    provinces: Query<(&Province, &ResourceProduction)>,
    trade_routes: Query<&TradeRoute>,
    mut market_events: EventWriter<MarketUpdateEvent>,
    mut trade_events: EventWriter<TradeCompletedEvent>,
    time: Res<Time>,
) {
    let dt = Fixed32::from_float(time.delta().as_secs_f32());
    
    // 1. Production phase
    for (mut nation, mut economy, mut stockpile) in nations.iter_mut() {
        // Calculate production from owned provinces
        let mut total_production = [Fixed32::ZERO; 8];
        
        for (province, production) in provinces.iter() {
            if province.owner == Some(nation.id) {
                // Use individual production fields instead of array
                total_production[0] += production.food * dt;
                total_production[1] += production.wood * dt;
                total_production[2] += production.stone * dt;
                total_production[3] += production.iron * dt;
                total_production[4] += production.gold * dt;
                // TODO: Add remaining resource types when implemented
            }
        }
        
        // Add to stockpile
        for i in 0..8 {
            stockpile.resources[i] = (stockpile.resources[i] + total_production[i])
                .min(stockpile.storage_capacity[i]);
        }
        
        // Update GDP
        economy.gdp = total_production.iter()
            .enumerate()
            .map(|(i, &amount)| amount * economy.resource_prices[i])
            .sum();
    }
    
    // 2. Consumption phase
    for (mut nation, mut economy, mut stockpile) in nations.iter_mut() {
        // Basic consumption based on economy type
        let consumption_rate = match nation.economy {
            EconomicSystem::Tribal => Fixed32::from_float(0.5), // Tribal - low consumption
            EconomicSystem::Feudal => Fixed32::from_float(0.7), // Feudal
            EconomicSystem::Mercantile => Fixed32::from_float(0.8), // Mercantile
            EconomicSystem::Market => Fixed32::from_float(1.0), // Market
            EconomicSystem::Command => Fixed32::from_float(0.9), // Command
            EconomicSystem::Mixed => Fixed32::from_float(0.95), // Mixed
            EconomicSystem::Cooperative => Fixed32::from_float(0.85), // Cooperative
        };
        
        // Consume resources
        for i in 0..8 {
            let consumption = stockpile.resources[i] * consumption_rate * dt;
            stockpile.resources[i] -= consumption;
            
            // Generate treasury from consumption (simplified)
            nation.treasury += consumption * economy.resource_prices[i] * Fixed32::from_float(0.1);
        }
    }
    
    // 3. Trade phase
    for route in trade_routes.iter() {
        if route.is_active {
            // Execute trade
            let trade_value = route.quantity * Fixed32::from_float(10.0); // Simplified pricing
            
            trade_events.send(TradeCompletedEvent {
                route_id: route.id,
                resource: match route.resource {
                    Some(GoodType::Food) => 0,
                    Some(GoodType::Wood) => 1,
                    Some(_) => 0, // TODO: Map remaining resource types
                    None => 0,
                },
                quantity: route.quantity,
                value: trade_value,
            });
        }
    }
    
    // 4. Price updates (supply/demand)
    for (nation, mut economy, stockpile) in nations.iter_mut() {
        for i in 0..8 {
            let old_price = economy.resource_prices[i];
            
            // Simple supply/demand pricing
            let supply_ratio = stockpile.resources[i] / stockpile.storage_capacity[i];
            
            // High supply = lower price, low supply = higher price
            let price_adjustment = if supply_ratio > Fixed32::from_float(0.8) {
                Fixed32::from_float(0.95) // 5% decrease
            } else if supply_ratio < Fixed32::from_float(0.2) {
                Fixed32::from_float(1.1) // 10% increase
            } else {
                Fixed32::ONE // No change
            };
            
            economy.resource_prices[i] = (old_price * price_adjustment)
                .clamp(Fixed32::from_float(0.1), Fixed32::from_num(100));
            
            if economy.resource_prices[i] != old_price {
                market_events.send(MarketUpdateEvent {
                    nation_id: nation.id,
                    resource: i as u8,
                    old_price,
                    new_price: economy.resource_prices[i],
                });
            }
        }
    }
}

/// Trade route creation system
pub fn trade_route_system(
    mut commands: Commands,
    nations: Query<(&Nation, &Economy, &ResourceStockpile)>,
    mut rng: Local<DeterministicRNG>,
) {
    if !rng.is_initialized() {
        *rng = DeterministicRNG::new(44444);
    }
    
    // Simplified trade route creation
    // In full implementation, would pathfind between cities
    let nations_vec: Vec<_> = nations.iter().collect();
    
    for i in 0..nations_vec.len() {
        for j in i+1..nations_vec.len() {
            let (nation1, economy1, stock1) = nations_vec[i];
            let (nation2, economy2, stock2) = nations_vec[j];
            
            // Check if trade would be beneficial
            for resource in 1..8 {
                let surplus1 = stock1.resources[resource] > stock1.storage_capacity[resource] * Fixed32::from_float(0.7);
                let deficit2 = stock2.resources[resource] < stock2.storage_capacity[resource] * Fixed32::from_float(0.3);
                
                if surplus1 && deficit2 && rng.next_bool(0.1) {
                    // Create trade route
                    commands.spawn(TradeRoute {
                        id: 0, // Would generate unique ID
                        from_city: 0, // Would be actual city ID
                        to_city: 0,   // Would be actual city ID
                        resource: Some(GoodType::Wood),
                        quantity: Fixed32::from_num(10),
                        flow_rate: Fixed32::from_num(10),
                        efficiency: Fixed32::from_float(0.8), // 80% efficiency
                        is_active: true,
                    });
                }
            }
        }
    }
}