//! Tile info panel for displaying province information

use super::super::{ChildBuilder, LabelBuilder, PanelBuilder, PanelStyle};
use crate::resources::SelectedProvinceInfo;
use crate::world::{ProvinceId, ProvinceStorage};
use bevy::log::{debug, error};
use bevy::prelude::*;

/// Marker component for the tile info panel
#[derive(Component)]
pub struct TileInfoPanel;

/// Marker component for the tile info text
#[derive(Component)]
pub struct TileInfoText;

/// Spawn the tile info panel UI
pub fn spawn_tile_info_panel(parent: &mut ChildBuilder) {
    // Create panel using PanelBuilder
    PanelBuilder::new()
        .style(PanelStyle::Default)
        .custom_background(crate::constants::COLOR_TILE_INFO_BACKGROUND)
        .padding(UiRect::all(Val::Px(10.0)))
        .build_with_children(parent, |panel| {
            let entity = LabelBuilder::new("Click a tile to see info")
                .font_size(16.0)
                .color(Color::WHITE)
                .build(panel);

            // Add our marker to the text entity
            panel.commands().entity(entity).insert(TileInfoText);
        });
}

/// Update UI panel showing selected tile info
pub fn update_tile_info_ui(
    selected_info: Res<SelectedProvinceInfo>,
    province_storage: Res<ProvinceStorage>,
    mut text_query: Query<&mut Text, With<TileInfoText>>,
) {
    if let Ok(mut text) = text_query.single_mut() {
        if let Some(province_id) = selected_info.province_id {
            // Use HashMap for O(1) lookup instead of O(n) linear search
            if let Some(&idx) = province_storage
                .province_by_id
                .get(&ProvinceId::new(province_id))
            {
                // Bounds check to prevent panic on invalid index
                if let Some(province) = province_storage.provinces.get(idx) {
                    *text = Text::new(format!(
                        "Province #{}
Terrain: {:?}
Elevation: {:.2}
Population: {:.0}
Agriculture: {:.1}
Water Distance: {:.1} hex
Position: ({:.0}, {:.0})",
                        province.id,
                        province.terrain,
                        province.elevation,
                        province.population,
                        province.agriculture,
                        province.fresh_water_distance,
                        province.position.x,
                        province.position.y,
                    ));
                } else {
                    // Handle invalid index gracefully with error reporting
                    error!(
                        "Invalid province index {} for province ID {}. Data structures may be out of sync.",
                        idx, province_id
                    );
                    *text = Text::new(format!(
                        "Error: Province #{} data corrupted\nPlease reload the world",
                        province_id
                    ));
                }
            } else {
                // Handle missing province ID gracefully
                debug!("Province ID {} not found in storage", province_id);
                *text = Text::new("Province data not available");
            }
        } else {
            *text = Text::new("Click a tile to see info");
        }
    }
}
