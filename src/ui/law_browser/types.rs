//! Law browser UI types and markers

use bevy::prelude::*;
use crate::nations::laws::types::{LawId, LawCategory};

/// Resource tracking law browser state
#[derive(Resource, Default)]
pub struct LawBrowserState {
    pub is_open: bool,
    pub selected_category: Option<LawCategory>,
    pub selected_law: Option<LawId>,
    pub search_text: String,
}

/// Resource tracking selected law category
#[derive(Resource, Default)]
pub struct SelectedLawCategory(pub Option<LawCategory>);

/// Resource tracking selected law for details view
#[derive(Resource, Default)]
pub struct SelectedLawId(pub Option<LawId>);

/// Marker for the law browser root
#[derive(Component)]
pub struct LawBrowserRoot;

/// Marker for the category tabs container
#[derive(Component)]
pub struct CategoryTabsContainer;

/// Marker for individual category tab
#[derive(Component)]
pub struct CategoryTab {
    pub category: LawCategory,
}

/// Marker for the laws list container
#[derive(Component)]
pub struct LawsListContainer;

/// Marker for individual law item in the list
#[derive(Component)]
pub struct LawListItem {
    pub law_id: LawId,
}

/// Marker for the law details panel
#[derive(Component)]
pub struct LawDetailsPanel;

/// Marker for law name text
#[derive(Component)]
pub struct LawNameText;

/// Marker for law description text
#[derive(Component)]
pub struct LawDescriptionText;

/// Marker for law effects container
#[derive(Component)]
pub struct LawEffectsContainer;

/// Marker for law prerequisites container
#[derive(Component)]
pub struct LawPrerequisitesContainer;

/// Marker for law conflicts container
#[derive(Component)]
pub struct LawConflictsContainer;

/// Marker for search input field
#[derive(Component)]
pub struct LawSearchInput;

/// Marker for the close button
#[derive(Component)]
pub struct LawBrowserCloseButton;