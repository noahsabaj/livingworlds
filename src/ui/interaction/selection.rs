//! Province selection tracking
//!
//! This module handles tracking which province is currently selected by the user.

use bevy::prelude::*;

/// Tracks information about the currently selected province
/// In mega-mesh architecture, provinces are data (not entities) stored in ProvinceStorage
#[derive(Resource, Default)]
pub struct SelectedProvinceInfo {
    pub province_id: Option<u32>,
}