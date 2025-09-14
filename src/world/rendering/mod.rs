//! World rendering gateway module
//!
//! This module contains all GPU and rendering concerns for the world.
//! It manages the mega-mesh architecture, overlays, borders, and visual effects.

use bevy::prelude::*;

// PRIVATE MODULES - Rendering implementation details

mod mesh;
mod overlay;
mod borders;
mod clouds;

// SELECTIVE PUBLIC EXPORTS - Controlled rendering API

// Mesh building and management
pub use mesh::{
    build_world_mesh,
    ProvinceStorage,
    WorldMeshHandle,
    MeshBuilder,
    MeshBuildStats,
};

// Overlay system
pub use overlay::{
    update_province_colors,
    OverlayMode,
};

// Border rendering
pub use borders::{
    SelectionBorder,
    BorderSettings,
};

// Cloud rendering
pub use clouds::{
    CloudLayer as CloudRenderLayer,
    animate_clouds,
};


/// Plugin that aggregates all world rendering subsystems
///
/// This plugin doesn't add systems directly - it delegates to specialized
/// plugins in each rendering submodule following the gateway pattern.
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Global rendering resources
            .init_resource::<WorldMeshHandle>()
            .init_resource::<ProvinceStorage>()

            // Add specialized rendering plugins
            // Each plugin manages its own systems and resources
            .add_plugins(borders::BorderPlugin)     // Handles selection borders
            .add_plugins(overlay::OverlayPlugin)    // Handles map overlays
            .add_plugins(clouds::CloudPlugin);      // Handles cloud rendering

        // Note: Each submodule plugin registers its own systems:
        // - BorderPlugin adds update_selection_border
        // - OverlayPlugin adds update_province_colors
        // - CloudPlugin adds animate_clouds
    }
}

// INTERNAL COORDINATION - Plugin wiring only

// Note: All actual system implementations are in their respective files:
// - update_province_colors is in overlay.rs
// - update_selection_border is in borders.rs
// - animate_clouds is in clouds.rs
// This gateway file should NEVER contain implementation logic.


/// The rendering module handles all GPU and visual aspects of the world.
///
/// # Architecture
///
/// This module implements the mega-mesh architecture where the entire world
/// is rendered as a single mesh with millions of vertices. It provides:
///
/// - **Mesh Building**: Constructs the world mesh from province data
/// - **Dynamic Overlays**: Updates vertex colors for different map modes
/// - **Border Rendering**: Highlights selected provinces
/// - **Cloud Animation**: Atmospheric effects layer
///
/// # Performance
///
/// The mega-mesh architecture achieves:
/// - Single draw call for entire world (up to 9M provinces)
/// - 60+ FPS on modern GPUs
/// - Dynamic vertex color updates for overlays
/// - ~2.4GB GPU memory for maximum world size
///
/// # Usage
///
/// ```rust
/// use crate::world::rendering::{build_world_mesh, ProvinceStorage};
///
/// let (mesh, storage) = build_world_mesh(&provinces);
/// ```