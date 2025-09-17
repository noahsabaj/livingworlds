//! House motto generation system
//!
//! This module contains the motto generation logic with weighted random selection,
//! compound mottos, and rarity tiers. Can generate thousands of unique variations.

use rand::Rng;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use crate::name_generator::Culture;
use super::traits::{HouseTraits, DominantTrait};

// Import from sibling data modules
use super::motto_data::{MottoVariation, MottoRarity, get_motto_variations};
use super::motto_data_extended::{learning_variations, intrigue_variations, piety_variations};

/// Generate a motto based on house traits, culture, prestige, and RNG
pub fn generate_motto(traits: &HouseTraits, culture: &Culture) -> String {
    // Use a seeded RNG for consistency within the generation
    let mut rng = rand::thread_rng();

    // Get prestige (will be passed in when the house system is more developed)
    // For now, use a random value
    let house_prestige = rng.gen_range(0.0..1.0);

    // Check if we should generate a compound motto (for houses with multiple strong traits)
    if should_generate_compound_motto(traits, &mut rng) {
        generate_compound_motto(traits, culture, house_prestige, &mut rng)
    } else {
        generate_single_motto(traits, culture, house_prestige, &mut rng)
    }
}

/// Generate a motto for the dominant trait with variations
fn generate_single_motto(
    traits: &HouseTraits,
    culture: &Culture,
    prestige: f32,
    rng: &mut impl Rng
) -> String {
    let dominant = traits.dominant_trait();
    let dominant_value = get_trait_value(traits, dominant);

    // Get all variations for this trait/culture combo
    let all_variations = get_all_variations(dominant, culture);

    // Filter variations based on requirements
    let eligible_variations: Vec<&MottoVariation> = all_variations
        .iter()
        .filter(|v| {
            let trait_ok = v.min_trait.map_or(true, |min| dominant_value >= min);
            let prestige_ok = v.min_prestige.map_or(true, |min| prestige >= min);
            trait_ok && prestige_ok
        })
        .collect();

    if eligible_variations.is_empty() {
        // Fallback to a basic motto if no variations qualify
        return get_fallback_motto(dominant, culture).to_string();
    }

    // Select based on rarity weights
    let selected = select_by_rarity(&eligible_variations, rng);
    selected.text.to_string()
}

/// Check if we should generate a compound motto
fn should_generate_compound_motto(traits: &HouseTraits, rng: &mut impl Rng) -> bool {
    // Count how many traits are above 0.7 (strong)
    let strong_traits = [
        traits.martial,
        traits.stewardship,
        traits.diplomacy,
        traits.learning,
        traits.intrigue,
        traits.piety,
    ].iter().filter(|&&t| t > 0.7).count();

    // If house has 2+ strong traits, chance for compound motto
    if strong_traits >= 2 {
        rng.gen_bool(0.3) // 30% chance for compound motto
    } else {
        false
    }
}

/// Generate a compound motto combining two traits
fn generate_compound_motto(
    traits: &HouseTraits,
    culture: &Culture,
    prestige: f32,
    rng: &mut impl Rng
) -> String {
    // Find the two strongest traits
    let mut trait_pairs: Vec<(DominantTrait, f32)> = vec![
        (DominantTrait::Martial, traits.martial),
        (DominantTrait::Stewardship, traits.stewardship),
        (DominantTrait::Diplomacy, traits.diplomacy),
        (DominantTrait::Learning, traits.learning),
        (DominantTrait::Intrigue, traits.intrigue),
        (DominantTrait::Piety, traits.piety),
    ];

    // Sort by strength
    trait_pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let primary = trait_pairs[0].0;
    let secondary = trait_pairs[1].0;

    // Generate compound based on the combination
    generate_compound_text(primary, secondary, culture, rng)
}

/// Generate compound motto text based on trait combination
fn generate_compound_text(
    primary: DominantTrait,
    secondary: DominantTrait,
    culture: &Culture,
    rng: &mut impl Rng
) -> String {
    use DominantTrait::*;

    // Special combinations with unique mottos
    match (primary, secondary) {
        (Martial, Piety) | (Piety, Martial) => {
            match culture {
                Culture::Western => select_from(&[
                    "Faith and Steel United",
                    "Holy Warriors Eternal",
                    "God's Own Sword",
                ], rng).to_string(),
                Culture::Eastern => select_from(&[
                    "Heaven's Blade",
                    "The Divine Warrior",
                    "Celestial and Martial",
                ], rng).to_string(),
                Culture::Northern => select_from(&[
                    "By Axe and Altar",
                    "Warriors of the Old Gods",
                    "Sacred Berserkers",
                ], rng).to_string(),
                _ => "Sacred War Eternal".to_string(),
            }
        },
        (Stewardship, Intrigue) | (Intrigue, Stewardship) => {
            match culture {
                Culture::Western => select_from(&[
                    "Gold in Shadow",
                    "The Merchant Spies",
                    "Wealth Through Secrets",
                ], rng).to_string(),
                Culture::Eastern => select_from(&[
                    "Silk and Shadows",
                    "The Jade Conspiracy",
                    "Hidden Wealth",
                ], rng).to_string(),
                _ => "Secrets Buy Kingdoms".to_string(),
            }
        },
        (Learning, Piety) | (Piety, Learning) => {
            match culture {
                Culture::Western => select_from(&[
                    "Knowledge of the Divine",
                    "Sacred Wisdom",
                    "The Scholar Priests",
                ], rng).to_string(),
                Culture::Eastern => select_from(&[
                    "Celestial Knowledge",
                    "Heaven's Library",
                    "The Enlightened Path",
                ], rng).to_string(),
                _ => "Truth Through Faith".to_string(),
            }
        },
        (Martial, Stewardship) | (Stewardship, Martial) => {
            select_from(&[
                "War Feeds the Treasury",
                "Gold and Iron",
                "Conquest and Commerce",
                "The Merchant Knights",
            ], rng).to_string()
        },
        (Diplomacy, Intrigue) | (Intrigue, Diplomacy) => {
            select_from(&[
                "Smiles Hide Daggers",
                "The Velvet Conspiracy",
                "Peace Through Deception",
                "Friends and Spies",
            ], rng).to_string()
        },
        _ => {
            // Generic compound for other combinations
            format!("{} and {}",
                get_trait_word(primary, culture),
                get_trait_word(secondary, culture)
            )
        }
    }
}

/// Select a random element from a slice
fn select_from<T: Clone>(options: &[T], rng: &mut impl Rng) -> T {
    options[rng.gen_range(0..options.len())].clone()
}

/// Select a motto based on rarity weights
fn select_by_rarity<'a>(
    variations: &[&'a MottoVariation],
    rng: &mut impl Rng
) -> &'a MottoVariation {
    // Calculate weights based on rarity
    let weights: Vec<f32> = variations.iter().map(|v| {
        match v.rarity {
            MottoRarity::Common => 60.0,
            MottoRarity::Uncommon => 30.0,
            MottoRarity::Rare => 8.0,
            MottoRarity::Legendary => 2.0,
        }
    }).collect();

    // Use weighted selection
    let dist = WeightedIndex::new(&weights).unwrap();
    variations[dist.sample(rng)]
}

/// Get the value of a specific trait
fn get_trait_value(traits: &HouseTraits, trait_type: DominantTrait) -> f32 {
    match trait_type {
        DominantTrait::Martial => traits.martial,
        DominantTrait::Stewardship => traits.stewardship,
        DominantTrait::Diplomacy => traits.diplomacy,
        DominantTrait::Learning => traits.learning,
        DominantTrait::Intrigue => traits.intrigue,
        DominantTrait::Piety => traits.piety,
    }
}

/// Get a single word representing a trait for compound mottos
fn get_trait_word(trait_type: DominantTrait, culture: &Culture) -> &'static str {
    match trait_type {
        DominantTrait::Martial => match culture {
            Culture::Western => "Steel",
            Culture::Eastern => "Blade",
            Culture::Northern => "Iron",
            _ => "War",
        },
        DominantTrait::Stewardship => match culture {
            Culture::Western => "Gold",
            Culture::Eastern => "Jade",
            Culture::Northern => "Wealth",
            _ => "Fortune",
        },
        DominantTrait::Diplomacy => match culture {
            Culture::Western => "Peace",
            Culture::Eastern => "Harmony",
            Culture::Northern => "Accord",
            _ => "Unity",
        },
        DominantTrait::Learning => match culture {
            Culture::Western => "Knowledge",
            Culture::Eastern => "Wisdom",
            Culture::Northern => "Truth",
            _ => "Learning",
        },
        DominantTrait::Intrigue => match culture {
            Culture::Western => "Shadow",
            Culture::Eastern => "Silk",
            Culture::Northern => "Whisper",
            _ => "Secret",
        },
        DominantTrait::Piety => match culture {
            Culture::Western => "Faith",
            Culture::Eastern => "Heaven",
            Culture::Northern => "Gods",
            _ => "Divine",
        },
    }
}

/// Get all variations including those from extended module
fn get_all_variations(trait_type: DominantTrait, culture: &Culture) -> Vec<MottoVariation> {
    match trait_type {
        DominantTrait::Learning => learning_variations(culture),
        DominantTrait::Intrigue => intrigue_variations(culture),
        DominantTrait::Piety => piety_variations(culture),
        _ => get_motto_variations(trait_type, culture),
    }
}

/// Fallback motto if no variations qualify
fn get_fallback_motto(dominant: DominantTrait, culture: &Culture) -> &'static str {
    match dominant {
        DominantTrait::Martial => "Strength and Honor",
        DominantTrait::Stewardship => "Prosperity and Power",
        DominantTrait::Diplomacy => "Unity Through Peace",
        DominantTrait::Learning => "Knowledge Is Power",
        DominantTrait::Intrigue => "Shadows Serve",
        DominantTrait::Piety => "Faith Guides Us",
    }
}