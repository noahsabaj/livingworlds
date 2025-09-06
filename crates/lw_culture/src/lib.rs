//! Living Worlds - Culture Domain

use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct CulturePlugin;

impl Plugin for CulturePlugin {
    fn build(&self, app: &mut App) {
        // Register cultural systems here
    }
}
