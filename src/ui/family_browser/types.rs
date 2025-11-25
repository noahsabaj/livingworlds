//! Data types for family browser

use bevy::prelude::*;
use crate::nations::NationId;
// NationId deleted - now using Entity directly

/// Prestige information for a house
#[derive(Debug, Clone)]
pub struct HousePrestige {
    pub house_entity: Entity,
    pub house_name: String,
    pub nation_id: Option<NationId>,
    pub nation_name: Option<String>,
    pub total_prestige: f32,
    pub wealth_score: f32,
    pub influence_score: f32,
    pub ruler_count: u32,
    pub years_in_power: u32,
    pub is_ruling: bool,
    pub is_extinct: bool,
}

impl HousePrestige {
    /// Get prestige tier based on total prestige score
    pub fn tier(&self) -> PrestigeTier {
        if self.total_prestige >= 1000.0 {
            PrestigeTier::Legendary
        } else if self.total_prestige >= 500.0 {
            PrestigeTier::Noble
        } else {
            PrestigeTier::Minor
        }
    }

    /// Get prestige star count (1-5 stars)
    pub fn star_count(&self) -> u8 {
        if self.total_prestige >= 1000.0 {
            5
        } else if self.total_prestige >= 750.0 {
            4
        } else if self.total_prestige >= 500.0 {
            3
        } else if self.total_prestige >= 250.0 {
            2
        } else {
            1
        }
    }
}

/// Prestige tiers for filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrestigeTier {
    #[default]
    All,
    Legendary, // 1000+
    Noble,     // 500-999
    Minor,     // 0-499
}

impl PrestigeTier {
    pub fn label(&self) -> &'static str {
        match self {
            PrestigeTier::All => "All Houses",
            PrestigeTier::Legendary => "Legendary",
            PrestigeTier::Noble => "Noble",
            PrestigeTier::Minor => "Minor",
        }
    }
}

/// Filter state for family browser
#[derive(Resource, Default, Debug, Clone)]
pub struct FamilyBrowserFilters {
    pub nation_filter: Option<NationId>,
    pub prestige_tier: PrestigeTier,
    pub show_extinct: bool,
    pub search_text: String,
}

impl FamilyBrowserFilters {
    /// Check if a house passes all active filters
    pub fn matches(&self, house: &HousePrestige) -> bool {
        // Nation filter
        if let Some(filter_nation) = self.nation_filter {
            if house.nation_id != Some(filter_nation) {
                return false;
            }
        }

        // Prestige tier filter
        match self.prestige_tier {
            PrestigeTier::All => {}
            tier => {
                if house.tier() != tier {
                    return false;
                }
            }
        }

        // Extinct filter
        if !self.show_extinct && house.is_extinct {
            return false;
        }

        // Search text filter
        if !self.search_text.is_empty() {
            let search_lower = self.search_text.to_lowercase();
            if !house.house_name.to_lowercase().contains(&search_lower) {
                return false;
            }
        }

        true
    }
}

/// Cached list of all houses with prestige scores, sorted by prestige
#[derive(Resource, Default)]
pub struct HousePrestigeCache {
    pub houses: Vec<HousePrestige>,
    pub last_update: f64,
}

impl HousePrestigeCache {
    /// Get filtered and sorted houses
    pub fn filtered(&self, filters: &FamilyBrowserFilters) -> Vec<HousePrestige> {
        self.houses
            .iter()
            .filter(|h| filters.matches(h))
            .cloned()
            .collect()
    }
}

/// Marker for the family browser panel root
#[derive(Component)]
pub struct FamilyBrowserPanel;

/// Marker for the house list container
#[derive(Component)]
pub struct HouseListContainer;

/// Marker for a house entry in the list
#[derive(Component)]
pub struct HouseEntry {
    pub house_entity: Entity,
}

/// Marker for "View Tree" button
#[derive(Component)]
pub struct ViewTreeButton {
    pub house_entity: Entity,
}

/// Resource tracking which house's tree is currently being viewed
#[derive(Resource, Default)]
pub struct SelectedHouseTree {
    pub house_entity: Option<Entity>,
}

/// Event to request opening a family tree
#[derive(Message)]
pub struct OpenFamilyTreeEvent {
    pub house_entity: Entity,
}

/// Event to request closing the family tree viewer
#[derive(Message)]
pub struct CloseFamilyTreeEvent;
