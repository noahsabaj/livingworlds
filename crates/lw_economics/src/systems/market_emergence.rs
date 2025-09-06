//! Market emergence systems - prices emerge from individual actions
//!
//! Following Austrian economics principles, prices are discovered through 
//! the market process as individuals make buy/sell decisions based on 
//! their subjective valuations.

use bevy::prelude::*;
use lw_core::Fixed32;
use crate::components::*;
use crate::components::individual::*;
use std::collections::HashMap;

/// Price discovery through order matching
/// This system matches buy and sell orders to discover market-clearing prices
pub fn price_discovery_system(
    mut orders: Query<(Entity, &mut MarketOrder)>,
    mut order_books: Query<&mut OrderBook>,
    mut transactions: Commands,
    time: Res<Time>,
) {
    for mut order_book in &mut order_books {
        let mut buy_orders: Vec<(Entity, MarketOrder)> = Vec::new();
        let mut sell_orders: Vec<(Entity, MarketOrder)> = Vec::new();
        
        // Collect and sort orders
        for buy_entity in &order_book.buy_orders {
            if let Ok((entity, order)) = orders.get(*buy_entity) {
                if order.order_type == OrderType::Buy {
                    buy_orders.push((entity, order.clone()));
                }
            }
        }
        
        for sell_entity in &order_book.sell_orders {
            if let Ok((entity, order)) = orders.get(*sell_entity) {
                if order.order_type == OrderType::Sell {
                    sell_orders.push((entity, order.clone()));
                }
            }
        }
        
        // Sort buy orders by price (highest first) and sell orders by price (lowest first)
        buy_orders.sort_by(|a, b| {
            b.1.limit_price.unwrap_or(Fixed32::MAX)
                .cmp(&a.1.limit_price.unwrap_or(Fixed32::MAX))
        });
        
        sell_orders.sort_by(|a, b| {
            a.1.limit_price.unwrap_or(Fixed32::ZERO)
                .cmp(&b.1.limit_price.unwrap_or(Fixed32::ZERO))
        });
        
        // Match orders
        let mut buy_idx = 0;
        let mut sell_idx = 0;
        
        while buy_idx < buy_orders.len() && sell_idx < sell_orders.len() {
            let (buy_entity, buy_order) = &buy_orders[buy_idx];
            let (sell_entity, sell_order) = &sell_orders[sell_idx];
            
            let buy_price = buy_order.limit_price.unwrap_or(Fixed32::MAX);
            let sell_price = sell_order.limit_price.unwrap_or(Fixed32::ZERO);
            
            // Check if orders can match
            if buy_price >= sell_price {
                // Determine transaction price (midpoint)
                let transaction_price = (buy_price + sell_price) / Fixed32::from_num(2);
                
                // Determine quantity
                let quantity = buy_order.quantity.min(sell_order.quantity);
                
                // Create transaction record
                transactions.spawn(CompletedTransaction {
                    buyer: buy_order.issuer,
                    seller: sell_order.issuer,
                    good: buy_order.good,
                    quantity,
                    price: transaction_price,
                    location: buy_order.location,
                    timestamp: time.elapsed_seconds() as u64,
                    transaction_costs: Fixed32::from_float(0.01), // Small transaction cost
                });
                
                // Update or remove orders based on remaining quantity
                if let Ok((_, mut buy_order_mut)) = orders.get_mut(*buy_entity) {
                    buy_order_mut.quantity -= quantity;
                    if buy_order_mut.quantity <= Fixed32::ZERO {
                        buy_idx += 1;
                    }
                }
                
                if let Ok((_, mut sell_order_mut)) = orders.get_mut(*sell_entity) {
                    sell_order_mut.quantity -= quantity;
                    if sell_order_mut.quantity <= Fixed32::ZERO {
                        sell_idx += 1;
                    }
                }
            } else {
                // No more matches possible
                break;
            }
        }
        
        order_book.last_cleared = time.elapsed_seconds() as u64;
    }
}

/// Market clearing system - executes matched trades
pub fn market_clearing_system(
    mut commands: Commands,
    transactions: Query<(Entity, &CompletedTransaction), Added<CompletedTransaction>>,
    mut individuals: Query<&mut Individual>,
    mut markets: Query<&mut Market>,
) {
    for (transaction_entity, transaction) in &transactions {
        // Transfer goods from seller to buyer
        if let Ok(mut seller) = individuals.get_mut(transaction.seller) {
            // Seller loses the goods
            // This would update the seller's inventory
        }
        
        if let Ok(mut buyer) = individuals.get_mut(transaction.buyer) {
            // Buyer gains the goods
            // This would update the buyer's inventory
        }
        
        // Update market information
        for mut market in &mut markets {
            if market.location == transaction.location {
                // Add to transaction history for price discovery
                market.transaction_history.push(Transaction {
                    buyer: transaction.buyer,
                    seller: transaction.seller,
                    good: transaction.good,
                    quantity: transaction.quantity,
                    price: transaction.price,
                    timestamp: Fixed32::from_num(transaction.timestamp),
                });
                
                // Keep history manageable
                if market.transaction_history.len() > 1000 {
                    market.transaction_history.drain(0..100);
                }
                
                // Update supply/demand tracking
                if let Some(good_info) = market.goods.get_mut(&transaction.good) {
                    good_info.supply = good_info.supply.saturating_sub(transaction.quantity);
                }
            }
        }
        
        // Mark transaction as processed
        commands.entity(transaction_entity).despawn();
    }
}

/// Individual valuation system - how individuals value goods
pub fn individual_valuation_system(
    mut individuals: Query<(&Individual, &mut IndividualValuation, &Needs)>,
    market_knowledge: Query<&LocalMarketKnowledge>,
) {
    for (individual, mut valuation, needs) in &mut individuals {
        // Update valuations based on current needs
        valuation.valuations.clear();
        
        // Basic necessities are valued highly when scarce
        let food_need = Fixed32::from_num(1) - needs.food_satisfaction;
        valuation.valuations.push(GoodValuation {
            good: GoodType::Food,
            subjective_value: food_need * Fixed32::from_num(10), // High value when hungry
            urgency: food_need,
            quantity_desired: food_need * Fixed32::from_num(5),
        });
        
        // Shelter needs
        let shelter_need = Fixed32::from_num(1) - needs.shelter_satisfaction;
        valuation.valuations.push(GoodValuation {
            good: GoodType::Shelter,
            subjective_value: shelter_need * Fixed32::from_num(8),
            urgency: shelter_need * Fixed32::from_float(0.8),
            quantity_desired: if shelter_need > Fixed32::from_float(0.5) {
                Fixed32::from_num(1)
            } else {
                Fixed32::ZERO
            },
        });
        
        // Luxury goods valued only when basic needs met
        if needs.food_satisfaction > Fixed32::from_float(0.8) &&
           needs.shelter_satisfaction > Fixed32::from_float(0.8) {
            valuation.valuations.push(GoodValuation {
                good: GoodType::Art,
                subjective_value: individual.wealth * Fixed32::from_float(0.1),
                urgency: Fixed32::from_float(0.1),
                quantity_desired: Fixed32::from_float(0.5),
            });
        }
        
        // Tools valued based on profession
        if individual.skills.iter().any(|s| matches!(s.skill_type, SkillType::Crafting)) {
            valuation.valuations.push(GoodValuation {
                good: GoodType::Tools,
                subjective_value: Fixed32::from_num(5),
                urgency: Fixed32::from_float(0.5),
                quantity_desired: Fixed32::from_num(2),
            });
        }
    }
}

/// Price history tracking system
pub fn price_history_tracking_system(
    transactions: Query<&CompletedTransaction, Added<CompletedTransaction>>,
    mut price_events: EventWriter<PriceDiscoveryEvent>,
    mut market_knowledge: Query<&mut LocalMarketKnowledge>,
) {
    let mut price_aggregates: HashMap<(Entity, GoodType), Vec<(Fixed32, Fixed32)>> = HashMap::new();
    
    // Aggregate transactions by market and good
    for transaction in &transactions {
        price_aggregates
            .entry((transaction.location, transaction.good))
            .or_insert_with(Vec::new)
            .push((transaction.price, transaction.quantity));
    }
    
    // Calculate volume-weighted average prices
    for ((market, good), prices) in price_aggregates {
        let total_value: Fixed32 = prices.iter().map(|(p, q)| *p * *q).sum();
        let total_quantity: Fixed32 = prices.iter().map(|(_, q)| *q).sum();
        
        if total_quantity > Fixed32::ZERO {
            let discovered_price = total_value / total_quantity;
            
            // Emit price discovery event
            price_events.send(PriceDiscoveryEvent {
                good,
                discovered_price,
                volume: total_quantity,
                market,
                participants: Vec::new(), // Could track actual participants
                timestamp: 0, // Would use actual game time
            });
            
            // Update local market knowledge for nearby individuals
            for mut knowledge in &mut market_knowledge {
                // Add or update known price
                if let Some(known) = knowledge.known_prices.iter_mut()
                    .find(|k| k.good == good && k.location == market) {
                    known.observed_price = discovered_price;
                    known.observation_time = 0; // Would use actual game time
                    known.confidence = Fixed32::from_float(0.9);
                } else {
                    knowledge.known_prices.push(KnownPrice {
                        good,
                        observed_price: discovered_price,
                        location: market,
                        observation_time: 0,
                        confidence: Fixed32::from_float(0.8),
                    });
                }
                
                // Keep knowledge limited
                if knowledge.known_prices.len() > 50 {
                    knowledge.known_prices.drain(0..10);
                }
            }
        }
    }
}

/// System to create market orders based on individual needs and valuations
pub fn create_market_orders_system(
    mut commands: Commands,
    individuals: Query<(Entity, &Individual, &IndividualValuation, &LocalMarketKnowledge)>,
    markets: Query<(Entity, &Market)>,
    time: Res<Time>,
) {
    for (individual_entity, individual, valuation, knowledge) in &individuals {
        for good_valuation in &valuation.valuations {
            // Check if individual wants to buy this good
            if good_valuation.urgency > Fixed32::from_float(0.3) {
                // Find best known price from local knowledge
                let best_known_price = knowledge.known_prices
                    .iter()
                    .filter(|k| k.good == good_valuation.good)
                    .min_by_key(|k| k.observed_price)
                    .map(|k| k.observed_price);
                
                // Determine maximum willing to pay based on subjective value
                let max_price = if let Some(known_price) = best_known_price {
                    // Willing to pay a bit more than known price if urgent
                    known_price * (Fixed32::from_num(1) + good_valuation.urgency * Fixed32::from_float(0.2))
                } else {
                    // No price knowledge - use subjective value
                    good_valuation.subjective_value
                };
                
                // Find nearest market
                if let Some((market_entity, _)) = markets.iter().next() {
                    // Create buy order
                    commands.spawn(MarketOrder {
                        order_type: OrderType::Buy,
                        issuer: individual_entity,
                        good: good_valuation.good,
                        quantity: good_valuation.quantity_desired,
                        limit_price: Some(max_price),
                        time_limit: Some(time.elapsed_seconds() as u64 + 3600), // 1 hour
                        location: market_entity,
                        created_at: time.elapsed_seconds() as u64,
                        urgency: good_valuation.urgency,
                    });
                }
            }
            
            // Check if individual wants to sell (has surplus)
            // This would check inventory and create sell orders
        }
    }
}

/// System to update order books with new orders
pub fn update_order_books_system(
    new_orders: Query<(Entity, &MarketOrder), Added<MarketOrder>>,
    mut order_books: Query<&mut OrderBook>,
) {
    for (order_entity, order) in &new_orders {
        // Find the order book for this market
        for mut order_book in &mut order_books {
            if order_book.market == order.location {
                match order.order_type {
                    OrderType::Buy => order_book.buy_orders.push(order_entity),
                    OrderType::Sell => order_book.sell_orders.push(order_entity),
                }
                
                // Update liquidity measure
                order_book.liquidity = Fixed32::from_num(order_book.buy_orders.len() + 
                                                         order_book.sell_orders.len()) / 
                                      Fixed32::from_num(100);
            }
        }
    }
}

/// System to remove expired orders
pub fn remove_expired_orders_system(
    mut commands: Commands,
    orders: Query<(Entity, &MarketOrder)>,
    mut order_books: Query<&mut OrderBook>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_seconds() as u64;
    
    for (order_entity, order) in &orders {
        // Check if order has expired
        if let Some(time_limit) = order.time_limit {
            if current_time > time_limit {
                // Remove from order books
                for mut order_book in &mut order_books {
                    order_book.buy_orders.retain(|&e| e != order_entity);
                    order_book.sell_orders.retain(|&e| e != order_entity);
                }
                
                // Despawn the order
                commands.entity(order_entity).despawn();
            }
        }
        
        // Also remove orders with zero quantity
        if order.quantity <= Fixed32::ZERO {
            for mut order_book in &mut order_books {
                order_book.buy_orders.retain(|&e| e != order_entity);
                order_book.sell_orders.retain(|&e| e != order_entity);
            }
            commands.entity(order_entity).despawn();
        }
    }
}