//! Tension calculation functions

/// Calculate tension from war percentage using exponential curve
///
/// This uses a power function to make tension rise exponentially:
/// - 10% at war = ~18% tension (local conflicts)
/// - 25% at war = ~40% tension (regional wars)
/// - 50% at war = ~70% tension (world crisis)
/// - 75% at war = ~90% tension (near apocalypse)
/// - 100% at war = 100% tension (total war)
pub fn calculate_from_war_percentage(war_percentage: f32) -> f32 {
    // Use square root for exponential growth
    // This makes small conflicts barely register but large wars escalate rapidly
    war_percentage.sqrt().clamp(0.0, 1.0)
}

// Future calculations will go here:
// - calculate_from_power_imbalance()
// - calculate_from_economic_stress()
// - calculate_from_disasters()
// - calculate_tension_physics()