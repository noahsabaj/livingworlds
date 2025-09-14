//! Loading tips and hints system
//!
//! This module provides a collection of helpful tips that can be displayed
//! during loading screens and other waiting periods.

use rand::prelude::*;

/// Categories of tips for filtering
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TipCategory {
    Controls,    // Keyboard and mouse controls
    Gameplay,    // Game mechanics and features
    Interface,   // UI tips and shortcuts
    Observation, // Tips about watching civilizations
    WorldGen,    // World generation insights
    Performance, // Performance and settings tips
}

/// A loading tip with its category
pub struct LoadingTip {
    pub text: &'static str,
    pub category: TipCategory,
}

/// Collection of all loading tips
pub const LOADING_TIPS: &[LoadingTip] = &[
    // Controls
    LoadingTip {
        text: "Press Space to pause the simulation and observe your world",
        category: TipCategory::Controls,
    },
    LoadingTip {
        text: "Use WASD or arrow keys to pan the camera across the world",
        category: TipCategory::Controls,
    },
    LoadingTip {
        text: "Mouse wheel zooms in and out. Hold Shift for faster zooming",
        category: TipCategory::Controls,
    },
    LoadingTip {
        text: "Press 1-5 to control simulation speed, from pause to 9x speed",
        category: TipCategory::Controls,
    },
    LoadingTip {
        text: "Press M to cycle through different map overlay modes",
        category: TipCategory::Controls,
    },
    LoadingTip {
        text: "Press B to toggle province borders on and off",
        category: TipCategory::Controls,
    },
    LoadingTip {
        text: "Press ESC to open the pause menu during gameplay",
        category: TipCategory::Controls,
    },
    LoadingTip {
        text: "Click on any province to see detailed information about it",
        category: TipCategory::Controls,
    },
    LoadingTip {
        text: "Press Home to reset the camera to the world center",
        category: TipCategory::Controls,
    },
    // Gameplay
    LoadingTip {
        text: "Living Worlds is an observer game - watch civilizations rise and fall naturally",
        category: TipCategory::Gameplay,
    },
    LoadingTip {
        text: "River deltas are the cradles of civilization with 3x population growth",
        category: TipCategory::Gameplay,
    },
    LoadingTip {
        text: "Mountains provide natural borders but limit population growth",
        category: TipCategory::Gameplay,
    },
    LoadingTip {
        text: "Coastal provinces benefit from trade and fishing resources",
        category: TipCategory::Gameplay,
    },
    LoadingTip {
        text: "Desert civilizations develop unique adaptations and technologies",
        category: TipCategory::Gameplay,
    },
    LoadingTip {
        text: "Watch for emergent stories as nations interact over centuries",
        category: TipCategory::Gameplay,
    },
    LoadingTip {
        text: "Climate affects agriculture, which drives population growth",
        category: TipCategory::Gameplay,
    },
    LoadingTip {
        text: "Natural resources shape the development of civilizations",
        category: TipCategory::Gameplay,
    },
    LoadingTip {
        text: "Wars and alliances form naturally based on resources and borders",
        category: TipCategory::Gameplay,
    },
    // Interface
    LoadingTip {
        text: "The overlay system shows political, mineral, and infrastructure maps",
        category: TipCategory::Interface,
    },
    LoadingTip {
        text: "Province information appears when you click on any territory",
        category: TipCategory::Interface,
    },
    LoadingTip {
        text: "The mineral legend shows resource distribution across the world",
        category: TipCategory::Interface,
    },
    LoadingTip {
        text: "Game speed indicator shows current simulation rate in the top-right",
        category: TipCategory::Interface,
    },
    LoadingTip {
        text: "The year display tracks the passage of time in your world",
        category: TipCategory::Interface,
    },
    // Observation
    LoadingTip {
        text: "You cannot control nations - enjoy watching their stories unfold",
        category: TipCategory::Observation,
    },
    LoadingTip {
        text: "Every simulation creates unique histories and narratives",
        category: TipCategory::Observation,
    },
    LoadingTip {
        text: "Pay attention to trade routes forming between civilizations",
        category: TipCategory::Observation,
    },
    LoadingTip {
        text: "Technology spreads naturally through trade and conquest",
        category: TipCategory::Observation,
    },
    LoadingTip {
        text: "Watch how geography shapes the destiny of nations",
        category: TipCategory::Observation,
    },
    LoadingTip {
        text: "Civilizations rise and fall in cycles - no empire lasts forever",
        category: TipCategory::Observation,
    },
    // World Generation
    LoadingTip {
        text: "Larger worlds support more diverse civilizations and longer histories",
        category: TipCategory::WorldGen,
    },
    LoadingTip {
        text: "World seeds determine the entire geography and resource distribution",
        category: TipCategory::WorldGen,
    },
    LoadingTip {
        text: "Continental drift affects long-term geographic patterns",
        category: TipCategory::WorldGen,
    },
    LoadingTip {
        text: "River systems are generated using realistic flow accumulation",
        category: TipCategory::WorldGen,
    },
    LoadingTip {
        text: "Climate zones determine vegetation and agricultural potential",
        category: TipCategory::WorldGen,
    },
    LoadingTip {
        text: "Erosion simulation creates realistic mountain and valley formations",
        category: TipCategory::WorldGen,
    },
    // Performance
    LoadingTip {
        text: "Large worlds with 3 million provinces run at 60+ FPS on modern GPUs",
        category: TipCategory::Performance,
    },
    LoadingTip {
        text: "The mega-mesh architecture renders the entire world in one draw call",
        category: TipCategory::Performance,
    },
    LoadingTip {
        text: "Disable borders with B key for better performance when zoomed out",
        category: TipCategory::Performance,
    },
    LoadingTip {
        text: "Save your game regularly - autosave is not yet implemented",
        category: TipCategory::Performance,
    },
];

/// Get a random tip from all categories
pub fn get_random_tip() -> &'static str {
    let mut rng = thread_rng();
    LOADING_TIPS
        .choose(&mut rng)
        .map(|tip| tip.text)
        .unwrap_or("Living Worlds - A Civilization Observer")
}

/// Get a random tip from a specific category
pub fn get_random_tip_from_category(category: TipCategory) -> Option<&'static str> {
    let mut rng = thread_rng();
    let filtered: Vec<&LoadingTip> = LOADING_TIPS
        .iter()
        .filter(|tip| tip.category == category)
        .collect();

    filtered.choose(&mut rng).map(|tip| tip.text)
}

/// Get tips in sequence (for tutorials or guided experiences)
pub struct TipSequence {
    tips: Vec<&'static str>,
    current_index: usize,
}

impl TipSequence {
    pub fn from_category(category: TipCategory) -> Self {
        let tips: Vec<&'static str> = LOADING_TIPS
            .iter()
            .filter(|tip| tip.category == category)
            .map(|tip| tip.text)
            .collect();

        Self {
            tips,
            current_index: 0,
        }
    }

    pub fn next(&mut self) -> Option<&'static str> {
        if self.current_index < self.tips.len() {
            let tip = self.tips[self.current_index];
            self.current_index += 1;
            Some(tip)
        } else {
            None
        }
    }

    /// Reset to the beginning
    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    /// Get current tip without advancing
    pub fn current(&self) -> Option<&'static str> {
        self.tips.get(self.current_index).copied()
    }
}

/// Tips with weights for more intelligent selection
pub struct WeightedTipSelector {
    last_category: Option<TipCategory>,
    shown_tips: Vec<usize>,
    max_history: usize,
}

impl Default for WeightedTipSelector {
    fn default() -> Self {
        Self {
            last_category: None,
            shown_tips: Vec::with_capacity(20),
            max_history: 20,
        }
    }
}

impl WeightedTipSelector {
    /// Get a tip that hasn't been shown recently and varies categories
    pub fn get_next_tip(&mut self) -> &'static str {
        let mut rng = thread_rng();

        // Filter out recently shown tips
        let available: Vec<(usize, &LoadingTip)> = LOADING_TIPS
            .iter()
            .enumerate()
            .filter(|(idx, _)| !self.shown_tips.contains(idx))
            .collect();

        // If we've shown everything, reset
        if available.is_empty() {
            self.shown_tips.clear();
            return get_random_tip();
        }

        // Try to pick from a different category than last time
        let tip = if let Some(last_cat) = self.last_category {
            available
                .iter()
                .filter(|(_, tip)| tip.category != last_cat)
                .choose(&mut rng)
                .or_else(|| available.choose(&mut rng))
        } else {
            available.choose(&mut rng)
        };

        if let Some((idx, selected_tip)) = tip {
            // Track this tip as shown
            self.shown_tips.push(*idx);
            if self.shown_tips.len() > self.max_history {
                self.shown_tips.remove(0);
            }

            // Remember the category
            self.last_category = Some(selected_tip.category);

            selected_tip.text
        } else {
            get_random_tip()
        }
    }
}
