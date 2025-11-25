//! Nation selection system
//!
//! Handles clicking on provinces to select their owning nation

use crate::nations::Nation;
use crate::relationships::ControlledBy;
use crate::resources::MapMode;
use crate::ui::nation_info::SelectedNation;
use crate::ui::SelectedProvinceInfo;
use crate::world::ProvinceEntityOrder;
use bevy::prelude::*;

/// System to handle nation selection when clicking on provinces
pub fn handle_nation_selection(
    mouse_button: Res<ButtonInput<MouseButton>>,
    selected_province_info: Res<SelectedProvinceInfo>,
    province_entity_order: Option<Res<ProvinceEntityOrder>>,
    controlled_by_query: Query<&ControlledBy>,
    nations_query: Query<(Entity, &Nation, &crate::nations::NationId)>,
    mut selected_nation: ResMut<SelectedNation>,
    map_mode: Res<MapMode>,
) {
    // Only process selection on actual mouse click
    if mouse_button.just_pressed(MouseButton::Left) {
        // Only select nations in political mode
        if *map_mode != MapMode::Political {
            // Clear selection when not in political mode
            if selected_nation.entity.is_some() {
                selected_nation.entity = None;
                selected_nation.nation_id = None;
                info!("Cleared nation selection (not in political mode)");
            }
            return;
        }

        // Get selected province
        let Some(province_id) = selected_province_info.province_id else {
            // Clicked on empty space - clear selection
            if selected_nation.entity.is_some() {
                selected_nation.entity = None;
                selected_nation.nation_id = None;
                info!("Cleared nation selection");
            }
            return;
        };

        // Get province entity from entity order
        let Some(entity_order) = province_entity_order.as_ref() else {
            warn!("ProvinceEntityOrder not available");
            return;
        };

        let Some(province_entity) = entity_order.get(province_id as usize) else {
            warn!("Province ID {} not found in entity order", province_id);
            return;
        };

        // Check if province has an owner via ControlledBy relationship
        if let Ok(controlled_by) = controlled_by_query.get(province_entity) {
            let owner_entity = controlled_by.0;
            // Find nation entity with this ID
            if let Ok((entity, nation, nation_id)) = nations_query.get(owner_entity) {
                // Select this nation
                if selected_nation.entity != Some(entity) {
                    selected_nation.entity = Some(entity);
                    selected_nation.nation_id = Some(*nation_id);
                    info!("Selected nation: {} (Entity: {:?})", nation.name, entity);
                }
                return;
            }

            // Owner not found in entities (shouldn't happen)
            warn!(
                "Province owned by entity {:?} but entity not found",
                owner_entity
            );
        } else {
            // Clicked on unclaimed province - clear selection
            if selected_nation.entity.is_some() {
                selected_nation.entity = None;
                selected_nation.nation_id = None;
                info!("Cleared nation selection (unclaimed province)");
            }
        }
    }
}

/// System to highlight selected nation's territory
pub fn highlight_selected_nation_territory(
    selected_nation: Res<SelectedNation>,
    controls_query: Query<&crate::relationships::Controls>,
    map_mode: Res<MapMode>,
) {
    // Only highlight in political mode
    if *map_mode != MapMode::Political {
        return;
    }

    // Check if selection changed
    if !selected_nation.is_changed() {
        return;
    }

    if let Some(nation_entity) = selected_nation.entity {
        // Get province count owned by selected nation using Controls relationship (O(1))
        let province_count = crate::nations::get_nation_province_count(&controls_query, nation_entity);
        debug!("Selected nation owns {} provinces", province_count);
        // Future: Add highlight border or brightness boost
    }
}
