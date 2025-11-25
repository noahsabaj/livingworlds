//! Nation generation and territory assignment
//!
//! This module handles spawning nations with dynasties and assigning them
//! territory using a Voronoi-like growth algorithm.
//!
//! ## Module Structure
//!
//! - `capitals` - Capital province selection with spatial distribution
//! - `colors` - Nation color generation using HSL color space
//! - `creation` - Nation and house creation with governance
//! - `territory` - Territory assignment using parallel growth algorithms

mod capitals;
mod colors;
mod creation;
mod territory;

use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng as StdRng;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

use crate::diagnostics::{TimedOperation, log_memory_usage};
use crate::world::Province;
use super::house::House;
use super::types::*;

// Re-export public items
pub use capitals::{select_capital_provinces, calculate_min_capital_distance};
pub use colors::{generate_nation_color, hsl_to_rgb};
pub use creation::{
    create_nation_with_house_parallel, create_nation_with_house,
    ensure_unique_nation_names, pick_culture_for_location, generate_adjective,
};
pub use territory::{assign_territory_to_nations, build_territories_from_provinces};

/// Spawn nations into the world with territory, ruling houses, and governance
pub fn spawn_nations(
    settings: &NationGenerationSettings,
    provinces: &mut [Province],
    seed: u32,
) -> (
    Vec<(NationId, Nation)>,
    Vec<House>,
    Vec<crate::nations::governance::GovernmentType>,
    std::collections::HashMap<NationId, Vec<u32>>
) {
    let total_timer = TimedOperation::start_with_level("Nation Generation", crate::diagnostics::LogLevel::Info);

    info!("Starting nation generation - Target: {} nations, Density: {}, Seed: {}",
          settings.nation_count, settings.nation_density, seed);

    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut nations = Vec::new();
    let mut houses = Vec::new();
    let nation_registry = NationRegistry::default();

    // Find suitable capital locations
    let capital_timer = TimedOperation::start("Capital Selection");
    let capital_provinces = capitals::select_capital_provinces(provinces, settings.nation_count, &mut rng);
    let _capital_time = capital_timer.complete_with_context(format!("{} capitals selected", capital_provinces.len()));

    if capital_provinces.is_empty() {
        // Early exit if no provinces
        if provinces.is_empty() {
            return (nations, houses, Vec::new(), std::collections::HashMap::new());
        }
        warn!("No suitable provinces found for nation capitals!");
        return (nations, houses, Vec::new(), std::collections::HashMap::new());
    }

    debug!("Selected capital provinces: {:?}", capital_provinces.iter().take(10).collect::<Vec<_>>());
    info!("Spawning {} nations with capitals", capital_provinces.len());

    // Pre-generate seeds for parallel nation creation (avoids RNG contention)
    let nation_seeds: Vec<u64> = (0..capital_provinces.len()).map(|_| rng.r#gen()).collect();

    // Create nations with ruling houses and governance in parallel
    let nation_creation_timer = TimedOperation::start("Nation Creation");
    let nation_registry_arc = Arc::new(nation_registry);
    let mut nation_data: Vec<(NationId, Nation, House, super::governance::GovernmentType)> = capital_provinces
        .par_iter()
        .zip(nation_seeds.par_iter())
        .map(|(&capital_idx, &nation_seed)| {
            creation::create_nation_with_house_parallel(
                &nation_registry_arc,
                capital_idx,
                provinces[capital_idx].position,
                nation_seed,
                settings,
            )
        })
        .collect();
    let _creation_time = nation_creation_timer.complete_with_context(format!("{} nations created", nation_data.len()));

    // Ensure all nation names are unique
    creation::ensure_unique_nation_names(&mut nation_data, &capital_provinces, provinces, settings);

    let mut governments = Vec::new();
    for (idx, (nation_id, nation, house, government)) in nation_data.into_iter().enumerate() {
        debug!("Created nation: {} (index: {}) with house {} and {:?} government",
               nation.name, idx, house.name, government);
        nations.push((nation_id, nation));
        houses.push(house);
        governments.push(government);
    }

    // Assign territory using growth algorithm
    let territory_timer = TimedOperation::start("Territory Assignment");
    let province_ownership = territory::assign_territory_to_nations(&mut nations, provinces, settings.nation_density);
    let total_provinces_assigned: usize = province_ownership.values().map(|v| v.len()).sum();
    let _territory_time = territory_timer.complete_with_context(
        format!("{} provinces assigned to {} nations", total_provinces_assigned, province_ownership.len())
    );

    // Log memory usage
    let nation_memory = nations.len() * std::mem::size_of::<Nation>()
        + houses.len() * std::mem::size_of::<House>()
        + governments.len() * std::mem::size_of::<super::governance::GovernmentType>();
    log_memory_usage("Nation Data", nation_memory);

    let total_time = total_timer.complete();

    info!("Nation Generation Summary:");
    info!("  Nations created: {}", nations.len());
    info!("  Ruling houses: {}", houses.len());
    info!("  Governments: {}", governments.len());
    info!("  Average territory: {:.1} provinces/nation", total_provinces_assigned as f32 / nations.len() as f32);
    info!("  Total time: {:.2}ms", total_time);

    (nations, houses, governments, province_ownership)
}
