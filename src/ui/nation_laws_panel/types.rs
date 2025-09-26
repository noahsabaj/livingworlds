//! Types for nation laws panel
//!
//! Markers and state for the nation laws UI.

use bevy::prelude::*;

/// Root marker for nation laws panel
#[derive(Component)]
pub struct NationLawsPanel;

/// State tracking whether panel is open
#[derive(Resource, Default)]
pub struct NationLawsPanelState {
    pub is_open: bool,
}

/// Container for active laws list
#[derive(Component)]
pub struct ActiveLawsContainer;

/// Container for proposed laws list
#[derive(Component)]
pub struct ProposedLawsContainer;

/// Marker for an active law item
#[derive(Component)]
pub struct ActiveLawItem {
    pub law_id: crate::nations::laws::LawId,
}

/// Marker for a proposed law item
#[derive(Component)]
pub struct ProposedLawItem {
    pub index: usize,
}

/// Marker for panel title text
#[derive(Component)]
pub struct PanelTitleText;

/// Marker for active laws header
#[derive(Component)]
pub struct ActiveLawsHeader;

/// Marker for proposed laws header
#[derive(Component)]
pub struct ProposedLawsHeader;

/// Marker for combined effects text
#[derive(Component)]
pub struct CombinedEffectsText;

/// Marker for close button
#[derive(Component)]
pub struct ClosePanelButton;

/// Marker for repeal button
#[derive(Component)]
pub struct RepealLawButton {
    pub law_id: crate::nations::laws::LawId,
}

/// Marker for support button
#[derive(Component)]
pub struct SupportLawButton {
    pub proposal_index: usize,
}

/// Marker for oppose button
#[derive(Component)]
pub struct OpposeLawButton {
    pub proposal_index: usize,
}