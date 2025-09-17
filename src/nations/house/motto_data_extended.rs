//! Extended motto data - Learning, Intrigue, and Piety variations
//!
//! This module continues the motto variations for the remaining traits.

use crate::name_generator::Culture;
use super::motto_data::MottoVariation;

// LEARNING MOTTO VARIATIONS
pub fn learning_variations(culture: &Culture) -> Vec<MottoVariation> {
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

// INTRIGUE MOTTO VARIATIONS
pub fn intrigue_variations(culture: &Culture) -> Vec<MottoVariation> {
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

// PIETY MOTTO VARIATIONS
pub fn piety_variations(culture: &Culture) -> Vec<MottoVariation> {
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