//! Systems for family tree interaction and management

use bevy::prelude::*;
use crate::nations::{Character, HasRelationship, RelationshipMetadata};
use crate::ui::family_browser::{OpenFamilyTreeEvent, CloseFamilyTreeEvent, SelectedHouseTree};
use super::types::*;
use super::layout::*;
use super::ui::CloseTreeButton;

/// Insert tree relationship filters resource
pub fn insert_tree_filters(mut commands: Commands) {
    commands.insert_resource(TreeRelationshipFilters::all_visible());
}

/// Handle opening family tree requests
pub fn handle_open_tree(
    mut events: MessageReader<OpenFamilyTreeEvent>,
    mut selected: ResMut<SelectedHouseTree>,
    mut layout: ResMut<FamilyTreeLayout>,
    mut panel: Query<&mut Visibility, With<FamilyTreePanel>>,
    characters: Query<(Entity, &Character)>,
    relationships: Query<(&HasRelationship, &RelationshipMetadata)>,
) {
    for event in events.read() {
        selected.house_entity = Some(event.house_entity);

        // Build tree layout
        let config = TreeLayoutConfig::default();
        *layout = build_tree_layout(
            event.house_entity,
            None, // Use default root (oldest/ruler)
            &characters,
            &relationships,
            &config,
        );

        // Show panel
        if let Ok(mut visibility) = panel.single_mut() {
            *visibility = Visibility::Visible;
        }
    }
}

/// Handle closing family tree
pub fn handle_close_tree(
    mut events: MessageReader<CloseFamilyTreeEvent>,
    mut selected: ResMut<SelectedHouseTree>,
    mut layout: ResMut<FamilyTreeLayout>,
    mut panel: Query<&mut Visibility, With<FamilyTreePanel>>,
    mut highlight: ResMut<BloodlineHighlight>,
) {
    for _event in events.read() {
        selected.house_entity = None;
        *layout = FamilyTreeLayout::default();
        highlight.clear();

        // Hide panel
        if let Ok(mut visibility) = panel.single_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}

/// Handle close button click
pub fn handle_close_button(
    buttons: Query<&Interaction, (Changed<Interaction>, With<CloseTreeButton>)>,
    mut close_events: MessageWriter<CloseFamilyTreeEvent>,
) {
    for interaction in &buttons {
        if *interaction == Interaction::Pressed {
            close_events.write(CloseFamilyTreeEvent);
        }
    }
}

/// Handle character node clicks for recentering
pub fn handle_node_click(
    nodes: Query<(&Interaction, &TreeNodeUI), Changed<Interaction>>,
    layout: Res<FamilyTreeLayout>,
    mut highlight: ResMut<BloodlineHighlight>,
    selected: Res<SelectedHouseTree>,
    mut open_events: MessageWriter<OpenFamilyTreeEvent>,
) {
    for (interaction, node_ui) in &nodes {
        if *interaction == Interaction::Pressed {
            // Toggle bloodline highlight
            if highlight.focused_character == Some(node_ui.character_id) {
                highlight.clear();
            } else {
                let (ancestors, descendants) = trace_bloodline(node_ui.character_id, &layout);
                highlight.focused_character = Some(node_ui.character_id);
                highlight.ancestors = ancestors;
                highlight.descendants = descendants;
            }

            // Alternatively, we could recenter the tree on this character
            // by rebuilding the layout with this character as root.
            // For now, just highlighting the bloodline.
        }
    }
}

/// Apply relationship filters to edge visibility
pub fn apply_relationship_filters(
    mut layout: ResMut<FamilyTreeLayout>,
    filters: Res<TreeRelationshipFilters>,
) {
    if !filters.is_changed() {
        return;
    }

    for edge in &mut layout.edges {
        edge.visible = filters.should_show(&edge.relationship);
    }
}
