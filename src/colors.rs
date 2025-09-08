//! Color functions for Living Worlds
//! 
//! This module centralizes all color generation functions for terrain,
//! minerals, and other visual elements.

use bevy::prelude::*;
use crate::components::MineralType;
use crate::terrain::TerrainType;
use crate::constants::*;

// ============================================================================
// TERRAIN COLORS
// ============================================================================

/// Get color for terrain based on type and elevation
pub fn get_terrain_color_gradient(terrain: TerrainType, elevation: f32) -> Color {
    // Define base colors with smoother transitions
    let color = match terrain {
        TerrainType::Ocean => {
            // Three distinct ocean depth colors based on elevation
            if elevation >= OCEAN_DEPTH_SHALLOW {
                // Shallow water (coastal)
                Color::srgb(0.15, 0.35, 0.55)
            } else if elevation >= OCEAN_DEPTH_MEDIUM {
                // Medium depth
                Color::srgb(0.08, 0.25, 0.45)
            } else {
                // Deep ocean (below OCEAN_DEPTH_DEEP)
                Color::srgb(0.02, 0.15, 0.35)
            }
        },
        TerrainType::Beach => {
            // Sandy beach with slight variation
            let sand_var = elevation * 2.0;
            Color::srgb(0.9 + sand_var * 0.05, 0.85 + sand_var * 0.05, 0.65 + sand_var * 0.1)
        },
        TerrainType::Plains => {
            // Lush green plains with elevation-based variation
            let green_factor = (elevation - 0.2) / 0.25;
            let r = 0.25 + green_factor * 0.1;
            let g = 0.55 + green_factor * 0.1;
            let b = 0.25 + green_factor * 0.05;
            Color::srgb(r, g, b)
        },
        TerrainType::Hills => {
            // Brown hills transitioning to grey at higher elevations
            let hill_factor = (elevation - 0.45) / 0.2;
            let r = 0.45 + hill_factor * 0.1;
            let g = 0.4 + hill_factor * 0.05;
            let b = 0.3 + hill_factor * 0.15;
            Color::srgb(r, g, b)
        },
        TerrainType::Mountains => {
            // Rocky grey to snow white based on height
            let snow_factor = ((elevation - 0.65) / 0.35).clamp(0.0, 1.0);
            let grey = 0.6 + snow_factor * 0.35;
            Color::srgb(grey, grey, grey + snow_factor * 0.05)
        },
        TerrainType::Ice => {
            // Polar ice - bright white with blue tint
            Color::srgb(0.92, 0.95, 1.0)
        },
        TerrainType::Tundra => {
            // Cold barren land - gray-brown
            Color::srgb(0.65, 0.6, 0.55)
        },
        TerrainType::Desert => {
            // Sandy desert - warm tan
            let variation = (elevation * 3.0).sin() * 0.05;
            Color::srgb(0.9 + variation, 0.8 + variation, 0.6)
        },
        TerrainType::Forest => {
            // Temperate forest - rich green with variation
            let forest_var = (elevation - 0.3) / 0.2;
            let r = 0.15 + forest_var * 0.05;
            let g = 0.35 + forest_var * 0.1;
            let b = 0.12 + forest_var * 0.03;
            Color::srgb(r, g, b)
        },
        TerrainType::Jungle => {
            // Tropical jungle - deep vibrant green
            let jungle_var = ((elevation * 5.0).sin() * 0.1).abs();
            let r = 0.05 + jungle_var;
            let g = 0.3 + jungle_var * 1.5;
            let b = 0.08 + jungle_var * 0.5;
            Color::srgb(r, g, b)
        },
        TerrainType::River => {
            // River - lighter freshwater blue-green, shallower than ocean
            Color::srgb(0.2, 0.45, 0.55)
        },
        TerrainType::Delta => {
            // Delta - rich fertile green-brown, mix of river and farmland
            Color::srgb(0.35, 0.5, 0.25)
        },
    };
    
    color
}

// ============================================================================
// MINERAL COLORS
// ============================================================================

/// Calculate color for mineral abundance (0-100)
pub fn mineral_abundance_color(abundance: u8) -> Color {
    if abundance == 0 {
        // No resources - dark grey
        Color::srgb(0.15, 0.15, 0.15)
    } else {
        // Heat map: black -> red -> yellow -> white
        let normalized = abundance as f32 / 100.0;
        
        if normalized < 0.33 {
            // Black to red
            let t = normalized * 3.0;
            Color::srgb(t, 0.0, 0.0)
        } else if normalized < 0.67 {
            // Red to yellow
            let t = (normalized - 0.33) * 3.0;
            Color::srgb(1.0, t, 0.0)
        } else {
            // Yellow to white
            let t = (normalized - 0.67) * 3.0;
            Color::srgb(1.0, 1.0, t)
        }
    }
}

/// Special color scale for stone (which ranges from 20-80, not 0-100)
pub fn stone_abundance_color(abundance: u8) -> Color {
    if abundance == 0 {
        // Ocean - show as dark blue
        Color::srgb(0.0, 0.1, 0.2)
    } else {
        // Normalize stone's 20-80 range to 0-1
        let normalized = ((abundance as f32 - 20.0) / 60.0).clamp(0.0, 1.0);
        
        // Use earth tones: dark brown -> tan -> light yellow
        if normalized < 0.5 {
            // Dark brown to medium brown
            let t = normalized * 2.0;
            Color::srgb(0.3 + t * 0.2, 0.2 + t * 0.2, 0.1 + t * 0.1)
        } else {
            // Medium brown to light tan
            let t = (normalized - 0.5) * 2.0;
            Color::srgb(0.5 + t * 0.3, 0.4 + t * 0.3, 0.2 + t * 0.3)
        }
    }
}

/// Get mineral-specific color based on type
pub fn get_mineral_color(mineral_type: MineralType) -> Color {
    match mineral_type {
        MineralType::Iron => Color::srgb(0.5, 0.3, 0.2),      // Rusty brown
        MineralType::Copper => Color::srgb(0.7, 0.4, 0.2),    // Copper orange
        MineralType::Tin => Color::srgb(0.7, 0.7, 0.8),       // Silvery grey
        MineralType::Gold => Color::srgb(1.0, 0.84, 0.0),     // Gold
        MineralType::Coal => Color::srgb(0.2, 0.2, 0.2),      // Dark grey
        MineralType::Stone => Color::srgb(0.6, 0.6, 0.6),     // Medium grey
        MineralType::Gems => Color::srgb(0.6, 0.2, 0.9),      // Purple
        MineralType::Bronze => Color::srgb(0.8, 0.5, 0.3),    // Bronze alloy
        MineralType::Steel => Color::srgb(0.7, 0.75, 0.8),    // Steel grey-blue
    }
}

/// Color for combined mineral richness
pub fn combined_richness_color(richness: f32) -> Color {
    let clamped = richness.clamp(0.0, 10.0) / 10.0;
    
    if clamped < 0.2 {
        // Poor - brown
        Color::srgb(0.4, 0.2, 0.1)
    } else if clamped < 0.4 {
        // Below average - dark orange
        Color::srgb(0.6, 0.3, 0.1)
    } else if clamped < 0.6 {
        // Average - orange
        Color::srgb(0.8, 0.5, 0.2)
    } else if clamped < 0.8 {
        // Rich - gold
        Color::srgb(1.0, 0.8, 0.3)
    } else {
        // Extremely rich - bright gold
        Color::srgb(1.0, 0.95, 0.5)
    }
}

/// Color for mining infrastructure level
pub fn infrastructure_level_color(level: u8) -> Color {
    match level {
        0 => Color::srgb(0.2, 0.2, 0.2),  // No infrastructure - dark grey
        1 => Color::srgb(0.4, 0.3, 0.2),  // Basic mine - brown
        2 => Color::srgb(0.5, 0.4, 0.3),  // Improved mine - light brown
        3 => Color::srgb(0.6, 0.5, 0.4),  // Advanced mine - tan
        4 => Color::srgb(0.7, 0.6, 0.5),  // Industrial mine - light tan
        _ => Color::srgb(0.9, 0.8, 0.6),  // Max level - almost white
    }
}