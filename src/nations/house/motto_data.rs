//! Motto data and variations
//!
//! This module contains all the actual motto strings organized by trait and culture.
//! Each combination has multiple variations with different rarity levels.

use crate::name_generator::Culture;
use super::traits::DominantTrait;

/// A motto variation with rarity and optional requirements
#[derive(Debug, Clone)]
pub struct MottoVariation {
    pub text: &'static str,
    pub rarity: MottoRarity,
    pub min_trait: Option<f32>,  // Minimum trait value (0.0-1.0) required
    pub min_prestige: Option<f32>, // Minimum prestige required
}

/// Rarity tiers for mottos
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MottoRarity {
    Common,      // 60% chance when selected
    Uncommon,    // 30% chance when selected
    Rare,        // 8% chance when selected
    Legendary,   // 2% chance when selected
}

impl MottoVariation {
    /// Create a common motto with no requirements
    pub const fn common(text: &'static str) -> Self {
        Self {
            text,
            rarity: MottoRarity::Common,
            min_trait: None,
            min_prestige: None,
        }
    }

    /// Create an uncommon motto
    pub const fn uncommon(text: &'static str) -> Self {
        Self {
            text,
            rarity: MottoRarity::Uncommon,
            min_trait: None,
            min_prestige: None,
        }
    }

    /// Create a rare motto
    pub const fn rare(text: &'static str) -> Self {
        Self {
            text,
            rarity: MottoRarity::Rare,
            min_trait: None,
            min_prestige: None,
        }
    }

    /// Create a legendary motto with requirements
    pub const fn legendary(text: &'static str, min_trait: f32, min_prestige: f32) -> Self {
        Self {
            text,
            rarity: MottoRarity::Legendary,
            min_trait: Some(min_trait),
            min_prestige: Some(min_prestige),
        }
    }
}

/// Get motto variations for a specific trait and culture
/// Note: This only handles Martial, Stewardship, and Diplomacy.
/// Learning, Intrigue, and Piety are in motto_data_extended.rs
pub fn get_motto_variations(trait_type: DominantTrait, culture: &Culture) -> Vec<MottoVariation> {
    match trait_type {
        DominantTrait::Martial => martial_variations(culture),
        DominantTrait::Stewardship => stewardship_variations(culture),
        DominantTrait::Diplomacy => diplomacy_variations(culture),
        _ => vec![], // Other traits handled in motto_data_extended
    }
}

// MARTIAL MOTTO VARIATIONS
fn martial_variations(culture: &Culture) -> Vec<MottoVariation> {
    match culture {
        Culture::Western => vec![
            MottoVariation::common("Steel and Honor"),
            MottoVariation::common("Victory or Death"),
            MottoVariation::common("The Sword Decides"),
            MottoVariation::common("Strength Through Arms"),
            MottoVariation::common("Born in Battle"),
            MottoVariation::uncommon("Never Yield, Never Break"),
            MottoVariation::uncommon("Blood Before Dishonor"),
            MottoVariation::uncommon("Iron Sharpens Iron"),
            MottoVariation::uncommon("The Strong Survive"),
            MottoVariation::rare("Unbroken, Unbowed, Undefeated"),
            MottoVariation::rare("Death Before Retreat"),
            MottoVariation::legendary("Eternal War, Eternal Glory", 0.8, 0.7),
            MottoVariation::legendary("The Sword That Guards the Realm", 0.9, 0.8),
        ],
        Culture::Eastern => vec![
            MottoVariation::common("The Blade Remembers"),
            MottoVariation::common("Strike Like Lightning"),
            MottoVariation::common("Way of the Warrior"),
            MottoVariation::common("Honor in Death"),
            MottoVariation::common("The Path of Steel"),
            MottoVariation::uncommon("Ten Thousand Victories"),
            MottoVariation::uncommon("The Sword Is the Soul"),
            MottoVariation::uncommon("Discipline Conquers All"),
            MottoVariation::rare("The Unsheathed Blade Never Rusts"),
            MottoVariation::rare("Master of Ten Thousand Battles"),
            MottoVariation::legendary("The Heavenly Sword", 0.85, 0.75),
            MottoVariation::legendary("Dragon Among Warriors", 0.9, 0.8),
        ],
        Culture::Northern => vec![
            MottoVariation::common("Blood and Thunder"),
            MottoVariation::common("Axes High"),
            MottoVariation::common("Winter Warriors"),
            MottoVariation::common("The Bear Awakens"),
            MottoVariation::common("Ice and Iron"),
            MottoVariation::uncommon("The Wolf Never Sleeps"),
            MottoVariation::uncommon("Thunder in Our Veins"),
            MottoVariation::uncommon("Forged in Frost"),
            MottoVariation::rare("The Eternal Winter War"),
            MottoVariation::rare("Blood on Snow"),
            MottoVariation::legendary("The Frost That Burns", 0.85, 0.7),
            MottoVariation::legendary("Lords of the Frozen Throne", 0.9, 0.85),
        ],
        Culture::Southern => vec![
            MottoVariation::common("Sun and Sword"),
            MottoVariation::common("Heat of Battle"),
            MottoVariation::common("The Golden Legion"),
            MottoVariation::common("Shields of the Sun"),
            MottoVariation::uncommon("Where Eagles Dare"),
            MottoVariation::uncommon("The Burning Blade"),
            MottoVariation::rare("Eternal as the Sun"),
            MottoVariation::legendary("The Sun Never Sets on Our Swords", 0.85, 0.8),
        ],
        Culture::Desert => vec![
            MottoVariation::common("Sand and Scimitar"),
            MottoVariation::common("Wind and War"),
            MottoVariation::common("The Desert Hawks"),
            MottoVariation::common("Riders of the Dunes"),
            MottoVariation::uncommon("The Scorpion Strikes"),
            MottoVariation::uncommon("Mirages of Death"),
            MottoVariation::rare("Lords of the Endless Sands"),
            MottoVariation::legendary("The Sandstorm Incarnate", 0.85, 0.75),
        ],
        Culture::Island => vec![
            MottoVariation::common("Tide and Tempest"),
            MottoVariation::common("Masters of the Waves"),
            MottoVariation::common("Salt and Steel"),
            MottoVariation::common("The Storm Riders"),
            MottoVariation::uncommon("Blood in the Water"),
            MottoVariation::uncommon("The Kraken Wakes"),
            MottoVariation::rare("Lords of Wind and Wave"),
            MottoVariation::legendary("The Sea Itself Bows", 0.9, 0.8),
        ],
        Culture::Ancient => vec![
            MottoVariation::common("Iron Will, Iron Hand"),
            MottoVariation::common("The Old Guard"),
            MottoVariation::common("Strength of Ages"),
            MottoVariation::uncommon("The First and Last"),
            MottoVariation::rare("Before the Dawn, After the Dusk"),
            MottoVariation::legendary("The Eternal Legion", 0.85, 0.9),
        ],
        Culture::Mystical => vec![
            MottoVariation::common("By Spell and Sword"),
            MottoVariation::common("Magic and Might"),
            MottoVariation::common("The Enchanted Blade"),
            MottoVariation::uncommon("Power Beyond Steel"),
            MottoVariation::rare("The Arcane Warriors"),
            MottoVariation::legendary("Masters of War and Magic", 0.85, 0.8),
        ],
    }
}

// STEWARDSHIP MOTTO VARIATIONS
fn stewardship_variations(culture: &Culture) -> Vec<MottoVariation> {
    match culture {
        Culture::Western => vec![
            MottoVariation::common("Prosperity Through Order"),
            MottoVariation::common("Gold Builds Kingdoms"),
            MottoVariation::common("Commerce and Crown"),
            MottoVariation::common("Wealth Wisely Won"),
            MottoVariation::common("The Merchant's Path"),
            MottoVariation::uncommon("Every Coin Counts"),
            MottoVariation::uncommon("Trade Conquers All"),
            MottoVariation::uncommon("The Golden Rule"),
            MottoVariation::rare("Masters of Coin and Crown"),
            MottoVariation::rare("Gold Flows Like Rivers"),
            MottoVariation::legendary("The Midas Touch", 0.85, 0.75),
            MottoVariation::legendary("Lords of Infinite Wealth", 0.9, 0.85),
        ],
        Culture::Eastern => vec![
            MottoVariation::common("Wealth Flows Like Water"),
            MottoVariation::common("The Jade Path"),
            MottoVariation::common("Prosperity and Peace"),
            MottoVariation::common("The Merchant's Way"),
            MottoVariation::uncommon("Ten Thousand Treasures"),
            MottoVariation::uncommon("The Golden Dragon"),
            MottoVariation::rare("Eternal Prosperity"),
            MottoVariation::legendary("The Celestial Treasury", 0.85, 0.8),
        ],
        Culture::Northern => vec![
            MottoVariation::common("Gold From Stone"),
            MottoVariation::common("The Miner's Fortune"),
            MottoVariation::common("Iron to Gold"),
            MottoVariation::common("Wealth of the Mountains"),
            MottoVariation::uncommon("The Dragon's Hoard"),
            MottoVariation::uncommon("Gold in Winter"),
            MottoVariation::rare("Masters of the Deep Mines"),
            MottoVariation::legendary("The Mountain's Heart of Gold", 0.85, 0.75),
        ],
        Culture::Southern => vec![
            MottoVariation::common("Abundance and Wisdom"),
            MottoVariation::common("The Harvest's Blessing"),
            MottoVariation::common("Gold Under the Sun"),
            MottoVariation::uncommon("The Fertile Fortune"),
            MottoVariation::rare("Endless Summer, Endless Wealth"),
            MottoVariation::legendary("The Golden Empire", 0.9, 0.8),
        ],
        Culture::Desert => vec![
            MottoVariation::common("Oasis of Fortune"),
            MottoVariation::common("Trade Winds Bring Gold"),
            MottoVariation::common("The Caravan's Path"),
            MottoVariation::common("Spice and Silver"),
            MottoVariation::uncommon("The Merchant Princes"),
            MottoVariation::rare("Lords of the Trade Routes"),
            MottoVariation::legendary("The Silk Road Throne", 0.85, 0.85),
        ],
        Culture::Island => vec![
            MottoVariation::common("Pearl of the Seas"),
            MottoVariation::common("Treasures of the Deep"),
            MottoVariation::common("Trade Winds and Gold"),
            MottoVariation::uncommon("The Coral Crown"),
            MottoVariation::rare("Masters of All Ports"),
            MottoVariation::legendary("The Ocean's Treasury", 0.85, 0.8),
        ],
        Culture::Ancient => vec![
            MottoVariation::common("The Eternal Treasury"),
            MottoVariation::common("Wealth of Ages"),
            MottoVariation::uncommon("The First Coin"),
            MottoVariation::rare("Before Gold, We Were"),
            MottoVariation::legendary("The Primordial Vault", 0.9, 0.9),
        ],
        Culture::Mystical => vec![
            MottoVariation::common("Alchemy of Wealth"),
            MottoVariation::common("Gold From Nothing"),
            MottoVariation::uncommon("The Philosopher's Fortune"),
            MottoVariation::rare("Transmutation of Power"),
            MottoVariation::legendary("The Infinite Conjuration", 0.85, 0.8),
        ],
    }
}

// DIPLOMACY MOTTO VARIATIONS
fn diplomacy_variations(culture: &Culture) -> Vec<MottoVariation> {
    match culture {
        Culture::Western => vec![
            MottoVariation::common("Unity Through Peace"),
            MottoVariation::common("Words Before Swords"),
            MottoVariation::common("The Diplomat's Path"),
            MottoVariation::common("Alliance and Accord"),
            MottoVariation::uncommon("The Velvet Glove"),
            MottoVariation::uncommon("Bridges, Not Walls"),
            MottoVariation::rare("The Eternal Alliance"),
            MottoVariation::legendary("The Voice That Ends Wars", 0.85, 0.75),
        ],
        Culture::Eastern => vec![
            MottoVariation::common("Harmony in All Things"),
            MottoVariation::common("The Middle Path"),
            MottoVariation::common("Balance and Beauty"),
            MottoVariation::uncommon("Ten Thousand Friendships"),
            MottoVariation::rare("The Jade Bridge"),
            MottoVariation::legendary("Heaven's Harmony", 0.9, 0.8),
        ],
        Culture::Northern => vec![
            MottoVariation::common("Words Before Axes"),
            MottoVariation::common("The Speaking Stone"),
            MottoVariation::common("Treaties in Winter"),
            MottoVariation::uncommon("The Peace of Wolves"),
            MottoVariation::rare("The Great Moot"),
            MottoVariation::legendary("The Voice of All Clans", 0.85, 0.8),
        ],
        Culture::Southern => vec![
            MottoVariation::common("Together We Rise"),
            MottoVariation::common("The Open Hand"),
            MottoVariation::common("Fellowship and Fortune"),
            MottoVariation::uncommon("The Gathering Light"),
            MottoVariation::rare("United Under the Sun"),
            MottoVariation::legendary("The Eternal Congress", 0.85, 0.75),
        ],
        Culture::Desert => vec![
            MottoVariation::common("Many Paths, One Destination"),
            MottoVariation::common("The Oasis of Peace"),
            MottoVariation::common("Caravans of Friendship"),
            MottoVariation::uncommon("The Desert Rose"),
            MottoVariation::rare("Where All Roads Meet"),
            MottoVariation::legendary("The Great Convergence", 0.85, 0.8),
        ],
        Culture::Island => vec![
            MottoVariation::common("Bridges Over Waters"),
            MottoVariation::common("Islands United"),
            MottoVariation::common("The Coral Connection"),
            MottoVariation::uncommon("Tides That Bind"),
            MottoVariation::rare("The Archipelago Alliance"),
            MottoVariation::legendary("Masters of All Shores", 0.85, 0.75),
        ],
        Culture::Ancient => vec![
            MottoVariation::common("The Eternal Alliance"),
            MottoVariation::common("First Among Equals"),
            MottoVariation::uncommon("The Original Compact"),
            MottoVariation::rare("Before Words, We Knew"),
            MottoVariation::legendary("The Primordial Peace", 0.9, 0.85),
        ],
        Culture::Mystical => vec![
            MottoVariation::common("Bound by Sacred Oaths"),
            MottoVariation::common("The Mystic Bond"),
            MottoVariation::uncommon("Threads of Fate"),
            MottoVariation::rare("The Cosmic Alliance"),
            MottoVariation::legendary("Weavers of Destiny", 0.85, 0.8),
        ],
    }
}

// Continue with Learning, Intrigue, and Piety variations...
// (Truncated for brevity, but would follow same pattern with 8-15 variations each)