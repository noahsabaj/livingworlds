//! Nation selection system
//!
//! Handles clicking on provinces to select their owning nation

use crate::nations::Nation;
use crate::resources::MapMode;
use crate::ui::nation_info::SelectedNation;
use crate::ui::SelectedProvinceInfo;
use crate::world::ProvinceStorage;
use bevy::prelude::*;

/// System to handle nation selection when clicking on provinces
pub fn handle_nation_selection(
    mouse_button: Res<ButtonInput<MouseButton>>,
    selected_province_info: Res<SelectedProvinceInfo>,
    provinces: Res<ProvinceStorage>,
    nations_query: Query<(Entity, &Nation)>,
    mut selected_nation: ResMut<SelectedNation>,
    map_mode: Res<MapMode>,
) {
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

    // Check for new click
    if !mouse_button.just_pressed(MouseButton::Left) {
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

    // Get province data
    let province_id_typed = crate::world::ProvinceId::new(province_id);
    let Some(&province_idx) = provinces.province_by_id.get(&province_id_typed) else {
        warn!("Province ID {} not found in storage", province_id);
        return;
    };
    let province = &provinces.provinces[province_idx];

    // Check if province has an owner
    if let Some(owner_id) = province.owner {
        // Find nation entity with this ID
        for (entity, nation) in nations_query.iter() {
            if nation.id == owner_id {
                // Select this nation
                if selected_nation.nation_id != Some(owner_id) {
                    selected_nation.entity = Some(entity);
                    selected_nation.nation_id = Some(owner_id);
                    info!("Selected nation: {} (ID: {:?})", nation.name, owner_id);
                }
                return;
            }
        }

        // Owner not found in entities (shouldn't happen)
        warn!(
            "Province owned by nation {:?} but entity not found",
            owner_id
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

/// System to highlight selected nation's territory
pub fn highlight_selected_nation_territory(
    selected_nation: Res<SelectedNation>,
    provinces: Res<ProvinceStorage>,
    ownership_cache: Res<crate::nations::ProvinceOwnershipCache>,
    cached_colors: ResMut<crate::resources::CachedOverlayColors>,
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

    if let Some(nation_id) = selected_nation.nation_id {
        // Get provinces owned by selected nation
        if let Some(owned_provinces) = ownership_cache.get_nation_provinces(nation_id) {
            debug!("Selected nation owns {} provinces", owned_provinces.len());
            // Future: Add highlight border or brightness boost
        }
    }
}
