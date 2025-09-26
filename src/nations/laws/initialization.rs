//! Law system initialization
//!
//! Handles registering all laws and setting up exclusive groups at startup.

use bevy::prelude::*;

use super::definitions;
use super::registry::LawRegistry;
use super::types::LawId;

/// Initialize the law registry with all defined laws
pub fn initialize_law_registry(mut registry: ResMut<LawRegistry>) {
    info!("Initializing law registry...");

    // Register all laws from the definitions module
    for law in definitions::get_all_laws() {
        registry.register_law(law.clone());
    }

    // Register exclusive groups (mutually exclusive laws)
    register_exclusive_groups(&mut registry);

    info!(
        "Law registry initialized with {} laws across {} categories",
        registry.law_count(),
        12
    );
}

/// Register groups of mutually exclusive laws
fn register_exclusive_groups(registry: &mut LawRegistry) {
    // Economic exclusive groups
    registry.register_exclusive_group(
        "tax_system".to_string(),
        vec![
            LawId::new(1000), // Flat Tax
            LawId::new(1001), // Progressive Tax
            LawId::new(1002), // No Income Tax
        ],
    );

    registry.register_exclusive_group(
        "trade_policy".to_string(),
        vec![
            LawId::new(1003), // Free Trade
            LawId::new(1004), // Protective Tariffs
            LawId::new(1005), // Trade Embargo
        ],
    );

    registry.register_exclusive_group(
        "labor_policy".to_string(),
        vec![
            LawId::new(1006), // Minimum Wage
            LawId::new(1007), // Unrestricted Labor Market
        ],
    );

    registry.register_exclusive_group(
        "currency_system".to_string(),
        vec![
            LawId::new(1010), // Gold Standard
            LawId::new(1011), // Fiat Currency
            LawId::new(1012), // Barter Economy
        ],
    );

    registry.register_exclusive_group(
        "market_system".to_string(),
        vec![
            LawId::new(1013), // Laissez-Faire
            LawId::new(1014), // Mixed Economy
            LawId::new(1015), // Planned Economy
        ],
    );

    registry.register_exclusive_group(
        "banking_system".to_string(),
        vec![
            LawId::new(1016), // Central Banking
            LawId::new(1017), // Free Banking
        ],
    );

    // Military exclusive groups
    registry.register_exclusive_group(
        "conscription_policy".to_string(),
        vec![
            LawId::new(2000), // Volunteer Army
            LawId::new(2001), // Limited Conscription
            LawId::new(2002), // Universal Conscription
            LawId::new(2003), // Pacifist Constitution
        ],
    );

    registry.register_exclusive_group(
        "officer_system".to_string(),
        vec![
            LawId::new(2004), // Professional Officer Corps
            LawId::new(2005), // Elected Officers
        ],
    );

    registry.register_exclusive_group(
        "war_doctrine".to_string(),
        vec![
            LawId::new(2006), // Total War Doctrine
            LawId::new(2007), // Laws of War
        ],
    );

    registry.register_exclusive_group(
        "weapons_policy".to_string(),
        vec![
            LawId::new(2008), // Chemical Weapons Ban
            LawId::new(2009), // Unrestricted Weapons Development
        ],
    );

    // Social exclusive groups
    registry.register_exclusive_group(
        "healthcare_system".to_string(),
        vec![
            LawId::new(3000), // Universal Healthcare
            LawId::new(3001), // Private Healthcare
        ],
    );

    registry.register_exclusive_group(
        "education_system".to_string(),
        vec![
            LawId::new(3002), // Public Education
            LawId::new(3003), // Religious Education
            LawId::new(3004), // No Formal Education
        ],
    );

    registry.register_exclusive_group(
        "gender_policy".to_string(),
        vec![
            LawId::new(3005), // Gender Equality
            LawId::new(3006), // Traditional Gender Roles
        ],
    );

    registry.register_exclusive_group(
        "marriage_system".to_string(),
        vec![
            LawId::new(3007), // Civil Marriage
            LawId::new(3008), // Free Union
        ],
    );
}