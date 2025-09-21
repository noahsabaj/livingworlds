//! Consolidated motto variations for all traits and cultures
//!
//! This module contains ALL motto content organized by trait type and culture.
//! This replaces the arbitrary split between motto_data.rs and motto_data_extended.rs
//! with a single, logical source of truth.

use super::super::super::traits::DominantTrait;
use super::super::types::MottoVariation;
use crate::name_generator::Culture;

/// Get all motto variations for a specific trait and culture
///
/// This is the main entry point for accessing motto data. It routes to the appropriate
/// trait-specific function and returns all available variations for selection.
pub fn get_variations_for_trait(
    trait_type: DominantTrait,
    culture: &Culture,
) -> Vec<MottoVariation> {
    match trait_type {
        DominantTrait::Martial => martial_variations(culture),
        DominantTrait::Stewardship => stewardship_variations(culture),
        DominantTrait::Diplomacy => diplomacy_variations(culture),
        DominantTrait::Learning => learning_variations(culture),
        DominantTrait::Intrigue => intrigue_variations(culture),
        DominantTrait::Piety => piety_variations(culture),
    }
}

/// Get a fallback motto if no variations qualify
///
/// These are simple, safe mottos that work for any house regardless of prestige or exact trait values.
/// Used as a safety net when no other variations meet the requirements.
pub fn get_fallback_motto(trait_type: DominantTrait, _culture: &Culture) -> &'static str {
    match trait_type {
        DominantTrait::Martial => "Strength and Honor",
        DominantTrait::Stewardship => "Prosperity and Power",
        DominantTrait::Diplomacy => "Unity Through Peace",
        DominantTrait::Learning => "Knowledge Is Power",
        DominantTrait::Intrigue => "Shadows Serve",
        DominantTrait::Piety => "Faith Guides Us",
    }
}

// ========================================================================
// MARTIAL MOTTO VARIATIONS
// ========================================================================

/// Martial trait motto variations - focused on warfare, honor, and military prowess
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

// ========================================================================
// STEWARDSHIP MOTTO VARIATIONS
// ========================================================================

/// Stewardship trait motto variations - focused on wealth, trade, and economic management
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

// ========================================================================
// DIPLOMACY MOTTO VARIATIONS
// ========================================================================

/// Diplomacy trait motto variations - focused on peace, alliance, and negotiation
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

// ========================================================================
// LEARNING MOTTO VARIATIONS
// ========================================================================

/// Learning trait motto variations - focused on knowledge, wisdom, and scholarship
fn learning_variations(culture: &Culture) -> Vec<MottoVariation> {
    match culture {
        Culture::Western => vec![
            MottoVariation::common("Knowledge Is Power"),
            MottoVariation::common("The Scholar's Path"),
            MottoVariation::common("Wisdom Through Study"),
            MottoVariation::common("The Learned Hand"),
            MottoVariation::common("Books Before Blades"),
            MottoVariation::uncommon("The Eternal Student"),
            MottoVariation::uncommon("Truth Illuminates All"),
            MottoVariation::uncommon("The Ink Is Mightier"),
            MottoVariation::rare("Masters of All Knowledge"),
            MottoVariation::rare("The Living Library"),
            MottoVariation::legendary("Keepers of All Secrets", 0.85, 0.75),
            MottoVariation::legendary("The Omniscient Eye", 0.9, 0.85),
        ],
        Culture::Eastern => vec![
            MottoVariation::common("Wisdom of Ages"),
            MottoVariation::common("The Scroll Keepers"),
            MottoVariation::common("Ten Thousand Texts"),
            MottoVariation::common("The Brush and Mind"),
            MottoVariation::uncommon("The Celestial Library"),
            MottoVariation::uncommon("Masters of the Four Arts"),
            MottoVariation::rare("The Dragon's Wisdom"),
            MottoVariation::legendary("Heaven's Own Scholars", 0.85, 0.8),
        ],
        Culture::Northern => vec![
            MottoVariation::common("Truth in the Runes"),
            MottoVariation::common("The Sage's Stone"),
            MottoVariation::common("Winter's Wisdom"),
            MottoVariation::common("The Elder's Knowledge"),
            MottoVariation::uncommon("Keepers of the Old Ways"),
            MottoVariation::uncommon("The Runestone Legacy"),
            MottoVariation::rare("Before Memory, We Knew"),
            MottoVariation::legendary("The All-Father's Wisdom", 0.85, 0.8),
        ],
        Culture::Southern => vec![
            MottoVariation::common("Light of Learning"),
            MottoVariation::common("The Sun's Wisdom"),
            MottoVariation::common("Enlightenment and Glory"),
            MottoVariation::uncommon("The Golden Academy"),
            MottoVariation::rare("Illuminated Forever"),
            MottoVariation::legendary("The Solar Library", 0.85, 0.75),
        ],
        Culture::Desert => vec![
            MottoVariation::common("Stars Guide the Wise"),
            MottoVariation::common("The Desert Scholars"),
            MottoVariation::common("Wisdom of the Sands"),
            MottoVariation::uncommon("The Astrolabe Masters"),
            MottoVariation::rare("Readers of the Heavens"),
            MottoVariation::legendary("The Cosmic Calculators", 0.85, 0.8),
        ],
        Culture::Island => vec![
            MottoVariation::common("Depths of Understanding"),
            MottoVariation::common("The Pearl of Wisdom"),
            MottoVariation::common("Knowledge Like the Tide"),
            MottoVariation::uncommon("The Navigators' Lore"),
            MottoVariation::rare("Masters of Wind and Star"),
            MottoVariation::legendary("The Ocean's Memory", 0.85, 0.75),
        ],
        Culture::Ancient => vec![
            MottoVariation::common("Keepers of the Old Ways"),
            MottoVariation::common("Before Writing Was"),
            MottoVariation::uncommon("The First Words"),
            MottoVariation::rare("Memory of the World"),
            MottoVariation::legendary("The Primordial Truth", 0.9, 0.9),
        ],
        Culture::Mystical => vec![
            MottoVariation::common("Secrets of the Cosmos"),
            MottoVariation::common("The Arcane Path"),
            MottoVariation::uncommon("Masters of the Hidden"),
            MottoVariation::rare("The Void's Knowledge"),
            MottoVariation::legendary("Beyond Mortal Understanding", 0.85, 0.85),
        ],
    }
}

// ========================================================================
// INTRIGUE MOTTO VARIATIONS
// ========================================================================

/// Intrigue trait motto variations - focused on secrets, espionage, and manipulation
fn intrigue_variations(culture: &Culture) -> Vec<MottoVariation> {
    match culture {
        Culture::Western => vec![
            MottoVariation::common("Shadows Serve the Crown"),
            MottoVariation::common("The Hidden Hand"),
            MottoVariation::common("Daggers in the Dark"),
            MottoVariation::common("Secrets and Schemes"),
            MottoVariation::common("The Spider's Web"),
            MottoVariation::uncommon("Every Wall Has Ears"),
            MottoVariation::uncommon("Trust Is a Luxury"),
            MottoVariation::uncommon("The Poisoned Cup"),
            MottoVariation::rare("Masters of All Secrets"),
            MottoVariation::rare("The Invisible Throne"),
            MottoVariation::legendary("The Shadow Behind All Thrones", 0.85, 0.75),
            MottoVariation::legendary("Puppet Masters Supreme", 0.9, 0.85),
        ],
        Culture::Eastern => vec![
            MottoVariation::common("The Silent Path"),
            MottoVariation::common("Shadows and Silk"),
            MottoVariation::common("The Hidden Dragon"),
            MottoVariation::common("Whispers in the Garden"),
            MottoVariation::uncommon("The Jade Serpent"),
            MottoVariation::uncommon("Ten Thousand Masks"),
            MottoVariation::rare("The Emperor's Shadow"),
            MottoVariation::legendary("The Celestial Conspiracy", 0.85, 0.8),
        ],
        Culture::Northern => vec![
            MottoVariation::common("Whispers in Winter"),
            MottoVariation::common("The Raven's Secret"),
            MottoVariation::common("Ice Hides All"),
            MottoVariation::common("The Wolf in Sheep's Fur"),
            MottoVariation::uncommon("The Frozen Conspiracy"),
            MottoVariation::uncommon("Blades in the Blizzard"),
            MottoVariation::rare("The Winter Court's Secret"),
            MottoVariation::legendary("The Eternal Deception", 0.85, 0.75),
        ],
        Culture::Southern => vec![
            MottoVariation::common("Silk Hides Steel"),
            MottoVariation::common("The Smiling Serpent"),
            MottoVariation::common("Honey and Hemlock"),
            MottoVariation::uncommon("The Golden Mask"),
            MottoVariation::rare("Sunlight Casts Shadows"),
            MottoVariation::legendary("The Solar Eclipse", 0.85, 0.8),
        ],
        Culture::Desert => vec![
            MottoVariation::common("Serpent's Wisdom"),
            MottoVariation::common("Mirages and Lies"),
            MottoVariation::common("The Scorpion's Dance"),
            MottoVariation::uncommon("Poison in the Oasis"),
            MottoVariation::rare("The Desert's Hidden Heart"),
            MottoVariation::legendary("The Sandstorm Conspiracy", 0.85, 0.75),
        ],
        Culture::Island => vec![
            MottoVariation::common("Hidden Currents"),
            MottoVariation::common("The Siren's Call"),
            MottoVariation::common("Depths Unknown"),
            MottoVariation::uncommon("The Coral Conspiracy"),
            MottoVariation::rare("Masters of the Undertow"),
            MottoVariation::legendary("The Abyss Gazes Back", 0.85, 0.8),
        ],
        Culture::Ancient => vec![
            MottoVariation::common("The Unseen Hand"),
            MottoVariation::common("Before Truth, Lies"),
            MottoVariation::uncommon("The First Deception"),
            MottoVariation::rare("Shadows Since the Dawn"),
            MottoVariation::legendary("The Primordial Lie", 0.9, 0.85),
        ],
        Culture::Mystical => vec![
            MottoVariation::common("Weaver of Fates"),
            MottoVariation::common("The Illusionist's Path"),
            MottoVariation::uncommon("Reality Is Optional"),
            MottoVariation::rare("The Void's Deception"),
            MottoVariation::legendary("Masters of All Illusions", 0.85, 0.8),
        ],
    }
}

// ========================================================================
// PIETY MOTTO VARIATIONS
// ========================================================================

/// Piety trait motto variations - focused on faith, divine favor, and religious devotion
fn piety_variations(culture: &Culture) -> Vec<MottoVariation> {
    match culture {
        Culture::Western => vec![
            MottoVariation::common("Faith Before Fear"),
            MottoVariation::common("The Righteous Path"),
            MottoVariation::common("By Divine Right"),
            MottoVariation::common("Heaven's Will"),
            MottoVariation::common("The Sacred Trust"),
            MottoVariation::uncommon("Blessed and Beloved"),
            MottoVariation::uncommon("The Holy Covenant"),
            MottoVariation::uncommon("Angels Guide Us"),
            MottoVariation::rare("The Divine Mandate"),
            MottoVariation::rare("Heaven's Own Children"),
            MottoVariation::legendary("The Living Saints", 0.85, 0.75),
            MottoVariation::legendary("God's Own Hand", 0.9, 0.85),
        ],
        Culture::Eastern => vec![
            MottoVariation::common("Heaven's Mandate"),
            MottoVariation::common("The Celestial Path"),
            MottoVariation::common("Balance and Blessing"),
            MottoVariation::common("The Dragon's Blessing"),
            MottoVariation::uncommon("Ten Thousand Prayers"),
            MottoVariation::uncommon("The Jade Emperor's Chosen"),
            MottoVariation::rare("Heaven and Earth United"),
            MottoVariation::legendary("The Celestial Dynasty", 0.85, 0.8),
        ],
        Culture::Northern => vec![
            MottoVariation::common("Gods Watch Over Us"),
            MottoVariation::common("The All-Father's Children"),
            MottoVariation::common("By Hammer and Prayer"),
            MottoVariation::common("The Sacred Grove"),
            MottoVariation::uncommon("Chosen of the Old Gods"),
            MottoVariation::uncommon("The Runepriests"),
            MottoVariation::rare("The Gods Walk With Us"),
            MottoVariation::legendary("Blessed by All Nine", 0.85, 0.8),
        ],
        Culture::Southern => vec![
            MottoVariation::common("Divine Light Guides"),
            MottoVariation::common("The Sun's Blessing"),
            MottoVariation::common("Holy and High"),
            MottoVariation::uncommon("The Golden Prayer"),
            MottoVariation::rare("Eternal in His Light"),
            MottoVariation::legendary("The Solar Prophets", 0.85, 0.75),
        ],
        Culture::Desert => vec![
            MottoVariation::common("Written in the Stars"),
            MottoVariation::common("The Prophet's Path"),
            MottoVariation::common("Sacred Sands"),
            MottoVariation::uncommon("The Oasis of Faith"),
            MottoVariation::rare("Chosen of the Desert God"),
            MottoVariation::legendary("The Living Prophecy", 0.85, 0.8),
        ],
        Culture::Island => vec![
            MottoVariation::common("Spirits of the Deep"),
            MottoVariation::common("The Ocean's Blessing"),
            MottoVariation::common("Tides of Faith"),
            MottoVariation::uncommon("The Coral Altar"),
            MottoVariation::rare("Blessed by Sea and Sky"),
            MottoVariation::legendary("The Ocean's Chosen", 0.85, 0.75),
        ],
        Culture::Ancient => vec![
            MottoVariation::common("Blessed by the Ancestors"),
            MottoVariation::common("The First Faith"),
            MottoVariation::uncommon("Before Gods, We Prayed"),
            MottoVariation::rare("The Original Covenant"),
            MottoVariation::legendary("The Primordial Faith", 0.9, 0.9),
        ],
        Culture::Mystical => vec![
            MottoVariation::common("Chosen of the Beyond"),
            MottoVariation::common("The Ethereal Path"),
            MottoVariation::uncommon("Between Worlds We Walk"),
            MottoVariation::rare("The Void's Blessing"),
            MottoVariation::legendary("Transcendent and Eternal", 0.85, 0.85),
        ],
    }
}
