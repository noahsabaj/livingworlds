//! Camera control module for Living Worlds
//! 
//! Handles camera movement, zooming, and viewport management with keyboard,
//! mouse, and edge-panning controls similar to strategy games.

use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::window::PrimaryWindow;
use crate::constants::*;

/// Camera control plugin for managing viewport and camera movement
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .add_systems(Update, camera_control_system);
    }
}

/// Setup the main game camera with initial position and projection
pub fn setup_camera(mut commands: Commands) {
    // Add 2D camera with custom clear color
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(COLOR_OCEAN_BACKGROUND),
            ..default()
        },
        Name::new("Main Camera"),
    ));
    
    // Camera initialized with orthographic projection
}

/// Camera control system handling zoom and pan with multiple input methods
pub fn camera_control_system(
    mut query: Query<(&mut Projection, &mut Transform), With<Camera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel: EventReader<MouseWheel>,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    // Get window for mouse position and dimensions
    let Ok(window) = windows.single() else { return; };
    
    // Calculate map dimensions - MASSIVE world
    let provinces_per_row = PROVINCES_PER_ROW as usize;
    let provinces_per_col = PROVINCES_PER_COL as usize;
    // POINTY-TOP hexagon dimensions (correct spacing)
    let map_width_pixels = provinces_per_row as f32 * HEX_SIZE_PIXELS * SQRT3; // sqrt(3) horizontal
    let map_height_pixels = provinces_per_col as f32 * HEX_SIZE_PIXELS * 1.5; // 3/2 vertical
    
    for (mut projection, mut transform) in query.iter_mut() {
        // Handle zoom only for orthographic projections
        if let Projection::Orthographic(ref mut ortho) = projection.as_mut() {
            // Zoom with mouse wheel
            for event in mouse_wheel.read() {
                let zoom_speed = CAMERA_ZOOM_SPEED;
                let zoom_delta = match event.unit {
                    MouseScrollUnit::Line => event.y,
                    MouseScrollUnit::Pixel => event.y * 0.01,
                };
                
                // Apply zoom (inverted so scrolling up zooms in)
                let _old_scale = ortho.scale;
                ortho.scale *= 1.0 - zoom_delta * zoom_speed;
                
                // Calculate minimum zoom to show entire map
                // The scale should fit the map within the window
                let min_zoom_x = map_width_pixels / window.width();
                let min_zoom_y = map_height_pixels / window.height();
                let min_zoom = min_zoom_x.max(min_zoom_y) * CAMERA_MAP_PADDING_FACTOR;
                
                // Clamp zoom levels - min zoom shows entire map
                ortho.scale = ortho.scale.clamp(CAMERA_MIN_ZOOM, min_zoom.max(CAMERA_MAX_ZOOM));
                
                // Zoom applied
            }
        }
        
        // Get current scale for pan speed calculation
        let current_scale = if let Projection::Orthographic(ref ortho) = projection.as_ref() {
            ortho.scale
        } else {
            1.0
        };
        
        // Pan with WASD or arrow keys
        // SHIFT modifier for 3x faster movement
        let speed_multiplier = if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            CAMERA_SPEED_MULTIPLIER
        } else {
            1.0
        };
        let pan_speed = CAMERA_PAN_SPEED_BASE * current_scale * time.delta_secs() * speed_multiplier;
        
        // Keyboard panning
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            transform.translation.y += pan_speed;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= pan_speed;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= pan_speed;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            transform.translation.x += pan_speed;
        }
        
        // Mouse edge panning (like strategy games)
        if let Some(cursor_pos) = window.cursor_position() {
            let edge_threshold = CAMERA_EDGE_PAN_THRESHOLD;
            let edge_speed = CAMERA_EDGE_PAN_SPEED_BASE * current_scale * time.delta_secs();
            
            // Check each edge
            if cursor_pos.x <= edge_threshold {
                transform.translation.x -= edge_speed; // Pan left
            }
            if cursor_pos.x >= window.width() - edge_threshold {
                transform.translation.x += edge_speed; // Pan right
            }
            if cursor_pos.y <= edge_threshold {
                transform.translation.y += edge_speed; // Pan up (Y is inverted in screen space)
            }
            if cursor_pos.y >= window.height() - edge_threshold {
                transform.translation.y -= edge_speed; // Pan down
            }
        }
        
        // Handle Y-axis clamping (no wrapping on Y)
        let max_y = (map_height_pixels / 2.0 - window.height() * current_scale / 2.0).max(0.0);
        if max_y <= 0.0 {
            // Map fits vertically, center it
            transform.translation.y = 0.0;
        } else {
            // Clamp Y position to map bounds
            transform.translation.y = transform.translation.y.clamp(-max_y, max_y);
        }
        
        // Handle X-axis wrapping (the world wraps horizontally)
        let half_map_width = map_width_pixels / 2.0;
        
        // Wrap camera X position for seamless horizontal scrolling
        if transform.translation.x > half_map_width {
            transform.translation.x -= map_width_pixels;
            // Camera wrapped from right to left edge
        } else if transform.translation.x < -half_map_width {
            transform.translation.x += map_width_pixels;
            // Camera wrapped from left to right edge
        }
        
        // Reset camera to origin with Home key
        if keyboard.just_pressed(KeyCode::Home) {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            if let Projection::Orthographic(ref mut ortho) = projection.as_mut() {
                ortho.scale = 1.0;
            }
            // Camera reset to origin
        }
    }
}