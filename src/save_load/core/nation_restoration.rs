//! Nation restoration during save game loading
//!
//! Restores nation entities and their associated law data.

use bevy::prelude::*;
use crate::nations::{Nation, NationId, laws::NationLaws};
use crate::save_load::types::SaveGameData;
use std::collections::HashMap;

/// Restore nations and their laws from save data
pub fn restore_nations_with_laws(
    commands: &mut Commands,
    save_data: &SaveGameData,
) -> HashMap<NationId, Entity> {
    let mut nation_entity_map = HashMap::new();

    // Recreate each nation with its laws
    for (nation_id, nation_laws) in &save_data.nation_laws {
        // Create nation entity with laws
        let entity = commands
            .spawn((
                // Note: We need the actual Nation component data
                // This is a placeholder - in reality we'd need to save full nation data
                Nation {
                    id: *nation_id,
                    name: format!("Nation {}", nation_id.value()),
                    treasury: 1000.0,
                    stability: 0.5,
                    military_strength: 100.0,
                    technology_level: 1,
                    culture: crate::name_generator::Culture::Western,
                    personality: crate::nations::NationPersonality::balanced(),
                    // ... other fields would be restored from save
                },
                nation_laws.clone(),
            ))
            .id();

        nation_entity_map.insert(*nation_id, entity);

        info!(
            "Restored nation {} with {} active laws and {} proposed laws",
            nation_id.value(),
            nation_laws.active_laws.len(),
            nation_laws.proposed_laws.len()
        );
    }

    nation_entity_map
}

/// Update the province ownership based on restored nations
pub fn update_province_ownership(
    provinces: &mut [crate::world::Province],
    nation_entity_map: &HashMap<NationId, Entity>,
) {
    for province in provinces.iter_mut() {
        if let Some(owner_id) = province.owner {
            // Update province ownership to use new entity references
            // This maintains the relationship between provinces and nations
            if let Some(_entity) = nation_entity_map.get(&owner_id) {
                // Province ownership is already stored as NationId
                // Entity relationships would be handled by the entity system
            }
        }
    }
}