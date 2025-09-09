//! Camera control module for Living Worlds
//! 
//! Handles camera movement, zooming, and viewport management with keyboard,
//! mouse, and edge-panning controls similar to strategy games.
//! 
//! Features smooth interpolation, zoom-to-cursor, and middle mouse drag.

use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel, MouseMotion};
use bevy::window::{PrimaryWindow, CursorGrabMode};
use crate::constants::*;
use crate::resources::MapDimensions;

/// Camera control plugin for managing viewport and camera movement
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CameraBounds>()
            .insert_resource(CursorConfinementPreference {
                user_wants_confined: true,  // Start with confinement enabled
                was_focused: true,
            })
            .add_systems(Startup, (setup_camera, setup_cursor_confinement))
            .add_systems(Update, (
                // Input gathering systems
                handle_keyboard_input,
                handle_mouse_wheel_zoom,
                handle_mouse_drag,
                handle_edge_panning,
                handle_camera_reset,
                toggle_cursor_confinement,
                handle_window_focus,  // Auto-release on alt-tab
                
                // Movement and interpolation
                apply_smooth_movement,
                apply_camera_bounds,
            ).chain())  // Chain ensures proper ordering
            .add_systems(PostStartup, calculate_camera_bounds);
    }
}

/// Camera controller component storing camera state and settings
#[derive(Component)]
pub struct CameraController {
    /// Target position for smooth interpolation
    pub target_position: Vec3,
    /// Target zoom level for smooth zooming
    pub target_zoom: f32,
    /// Current zoom level (cached for performance)
    pub current_zoom: f32,
    
    /// Smoothing factor (0.0 = instant, 1.0 = very smooth)
    pub position_smoothing: f32,
    pub zoom_smoothing: f32,
    
    /// Mouse drag state
    pub is_dragging: bool,
    pub last_cursor_pos: Option<Vec2>,
    
    /// Movement speed multipliers
    pub pan_speed_base: f32,
    pub edge_pan_speed_base: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            target_position: Vec3::ZERO,
            target_zoom: 1.0,
            current_zoom: 1.0,
            position_smoothing: 8.0,  // Nice smooth feeling
            zoom_smoothing: 10.0,     // Slightly faster zoom response
            is_dragging: false,
            last_cursor_pos: None,
            pan_speed_base: CAMERA_PAN_SPEED_BASE,
            edge_pan_speed_base: CAMERA_EDGE_PAN_SPEED_BASE,
        }
    }
}

/// Cached camera bounds to avoid recalculation every frame
#[derive(Resource, Default)]
struct CameraBounds {
    min_zoom: f32,
    max_zoom: f32,
    max_y: f32,
    half_map_width: f32,
}

/// Tracks cursor confinement preference
#[derive(Resource)]
struct CursorConfinementPreference {
    /// Whether the user wants confinement enabled (via Tab key)
    user_wants_confined: bool,
    /// Whether window was focused last frame (for detecting focus changes)
    was_focused: bool,
}

/// Setup the main game camera with initial position and projection
pub fn setup_camera(mut commands: Commands) {
    // Add 2D camera with custom clear color and controller
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(COLOR_OCEAN_BACKGROUND),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        CameraController::default(),
        Name::new("Main Camera"),
    ));
}

/// Calculate camera bounds once after startup
fn calculate_camera_bounds(
    mut bounds: ResMut<CameraBounds>,
    map_dimensions: Res<MapDimensions>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.get_single() else { return; };
    
    // Calculate zoom bounds
    let min_zoom_x = map_dimensions.width_pixels / window.width();
    let min_zoom_y = map_dimensions.height_pixels / window.height();
    let min_zoom = min_zoom_x.max(min_zoom_y) * CAMERA_MAP_PADDING_FACTOR;
    
    bounds.min_zoom = CAMERA_MIN_ZOOM;
    bounds.max_zoom = min_zoom.max(CAMERA_MAX_ZOOM);
    bounds.half_map_width = map_dimensions.width_pixels / 2.0;
    bounds.max_y = map_dimensions.height_pixels / 2.0;
}

/// Handle keyboard input for camera panning
fn handle_keyboard_input(
    mut query: Query<&mut CameraController>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut exit: EventWriter<AppExit>,
) {
    // ESC to exit
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
        return;
    }
    
    // Early return if no movement keys pressed
    if !keyboard.any_pressed([
        KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD,
        KeyCode::ArrowUp, KeyCode::ArrowDown, KeyCode::ArrowLeft, KeyCode::ArrowRight
    ]) {
        return;
    }
    
    let Ok(mut controller) = query.get_single_mut() else { return; };
    
    // Calculate speed with shift modifier
    let speed_multiplier = if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
        CAMERA_SPEED_MULTIPLIER
    } else {
        1.0
    };
    
    let pan_speed = controller.pan_speed_base * controller.current_zoom * time.delta_secs() * speed_multiplier;
    
    // Update target position based on input
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        controller.target_position.y += pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        controller.target_position.y -= pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        controller.target_position.x -= pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        controller.target_position.x += pan_speed;
    }
}

/// Handle mouse wheel zoom with zoom-to-cursor
fn handle_mouse_wheel_zoom(
    mut query: Query<(&mut CameraController, &Transform, &Projection)>,
    mut mouse_wheel: EventReader<MouseWheel>,
    windows: Query<&Window, With<PrimaryWindow>>,
    bounds: Res<CameraBounds>,
) {
    if mouse_wheel.is_empty() {
        return;
    }
    
    let Ok(window) = windows.get_single() else { return; };
    let Ok((mut controller, transform, projection)) = query.get_single_mut() else { return; };
    
    let Projection::Orthographic(ortho) = projection else { return; };
    
    for event in mouse_wheel.read() {
        let zoom_delta = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y * 0.01,
        };
        
        // Calculate new zoom level
        let zoom_factor = 1.0 - zoom_delta * CAMERA_ZOOM_SPEED;
        let new_zoom = (controller.target_zoom * zoom_factor).clamp(bounds.min_zoom, bounds.max_zoom);
        
        // Only zoom toward cursor when zooming IN (not when zooming out)
        // This prevents the disorienting "tugging" feeling when trying to see more of the map
        let is_zooming_in = new_zoom < controller.target_zoom;
        
        if is_zooming_in {
            // Zoom toward cursor position if cursor is over window
            if let Some(cursor_pos) = window.cursor_position() {
                // Convert cursor position to world coordinates before zoom
                let cursor_ndc = Vec2::new(
                    (cursor_pos.x / window.width()) * 2.0 - 1.0,
                    -((cursor_pos.y / window.height()) * 2.0 - 1.0),
                );
                
                let world_pos_before = Vec2::new(
                    cursor_ndc.x * ortho.scale * window.width() / 2.0 + transform.translation.x,
                    cursor_ndc.y * ortho.scale * window.height() / 2.0 + transform.translation.y,
                );
                
                // Calculate world position after zoom
                let world_pos_after = Vec2::new(
                    cursor_ndc.x * new_zoom * window.width() / 2.0 + transform.translation.x,
                    cursor_ndc.y * new_zoom * window.height() / 2.0 + transform.translation.y,
                );
                
                // Adjust target position to keep cursor at same world position
                // Use a factor to make it less aggressive (0.5 = 50% of the movement)
                let zoom_to_cursor_strength = 0.7;  // Adjust this to taste
                controller.target_position.x += (world_pos_before.x - world_pos_after.x) * zoom_to_cursor_strength;
                controller.target_position.y += (world_pos_before.y - world_pos_after.y) * zoom_to_cursor_strength;
            }
        }
        // When zooming OUT, just zoom from current center (no camera movement)
        
        controller.target_zoom = new_zoom;
    }
}

/// Handle middle mouse button drag for panning
fn handle_mouse_drag(
    mut query: Query<&mut CameraController>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut controller) = query.get_single_mut() else { return; };
    let Ok(window) = windows.get_single() else { return; };
    
    // Start/stop dragging
    if mouse_button.just_pressed(MouseButton::Middle) {
        controller.is_dragging = true;
        controller.last_cursor_pos = window.cursor_position();
    } else if mouse_button.just_released(MouseButton::Middle) {
        controller.is_dragging = false;
    }
    
    // Apply drag movement
    if controller.is_dragging && !mouse_motion.is_empty() {
        let mut total_delta = Vec2::ZERO;
        for event in mouse_motion.read() {
            total_delta += event.delta;
        }
        
        // Convert screen delta to world delta (accounting for zoom)
        let world_delta = total_delta * controller.current_zoom;
        controller.target_position.x -= world_delta.x;
        controller.target_position.y += world_delta.y;  // Y is inverted
    }
}

/// Handle edge panning when cursor is near screen edges
fn handle_edge_panning(
    mut query: Query<&mut CameraController>,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let Ok(window) = windows.get_single() else { return; };
    let Ok(mut controller) = query.get_single_mut() else { return; };
    
    // Only edge pan if not dragging
    if controller.is_dragging {
        return;
    }
    
    let Some(cursor_pos) = window.cursor_position() else { return; };
    
    let edge_threshold = CAMERA_EDGE_PAN_THRESHOLD;
    let edge_speed = controller.edge_pan_speed_base * controller.current_zoom * time.delta_secs();
    
    // Check each edge
    if cursor_pos.x <= edge_threshold {
        controller.target_position.x -= edge_speed;
    }
    if cursor_pos.x >= window.width() - edge_threshold {
        controller.target_position.x += edge_speed;
    }
    if cursor_pos.y <= edge_threshold {
        controller.target_position.y += edge_speed;
    }
    if cursor_pos.y >= window.height() - edge_threshold {
        controller.target_position.y -= edge_speed;
    }
}

/// Handle camera reset with Home key
fn handle_camera_reset(
    mut query: Query<&mut CameraController>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Home) {
        if let Ok(mut controller) = query.get_single_mut() {
            controller.target_position = Vec3::ZERO;
            controller.target_zoom = 1.0;
        }
    }
}

/// Apply smooth interpolation to camera movement and zoom
fn apply_smooth_movement(
    mut query: Query<(&mut Transform, &mut Projection, &mut CameraController)>,
    time: Res<Time>,
) {
    for (mut transform, mut projection, mut controller) in query.iter_mut() {
        // Smooth position interpolation
        let position_lerp = 1.0 - (0.01_f32.powf(controller.position_smoothing * time.delta_secs()));
        transform.translation = transform.translation.lerp(
            controller.target_position,
            position_lerp
        );
        
        // Smooth zoom interpolation
        if let Projection::Orthographic(ref mut ortho) = projection.as_mut() {
            let zoom_lerp = 1.0 - (0.01_f32.powf(controller.zoom_smoothing * time.delta_secs()));
            ortho.scale = ortho.scale.lerp(controller.target_zoom, zoom_lerp);
            controller.current_zoom = ortho.scale;  // Cache for other systems
        }
    }
}

/// Apply camera bounds and handle wrapping
fn apply_camera_bounds(
    mut query: Query<(&mut Transform, &mut CameraController, &Projection)>,
    bounds: Res<CameraBounds>,
    windows: Query<&Window, With<PrimaryWindow>>,
    map_dimensions: Res<MapDimensions>,
) {
    let Ok(window) = windows.get_single() else { return; };
    
    for (mut transform, mut controller, projection) in query.iter_mut() {
        let Projection::Orthographic(ortho) = projection else { continue; };
        
        // Calculate visible area
        let visible_height = window.height() * ortho.scale;
        
        // Y-axis clamping with margin
        let margin_factor = 0.3;
        let max_y = (bounds.max_y - visible_height / 2.0 + map_dimensions.height_pixels * margin_factor).max(0.0);
        
        transform.translation.y = transform.translation.y.clamp(-max_y, max_y);
        controller.target_position.y = controller.target_position.y.clamp(-max_y, max_y);
        
        // X-axis wrapping for seamless horizontal scrolling
        if transform.translation.x > bounds.half_map_width {
            transform.translation.x -= map_dimensions.width_pixels;
            controller.target_position.x -= map_dimensions.width_pixels;
        } else if transform.translation.x < -bounds.half_map_width {
            transform.translation.x += map_dimensions.width_pixels;
            controller.target_position.x += map_dimensions.width_pixels;
        }
    }
}

/// Setup cursor confinement on startup to keep mouse within window for edge panning
fn setup_cursor_confinement(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        // Confine cursor to window bounds - perfect for strategy games
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
    }
}

/// Toggle cursor confinement with Tab key for windowed/fullscreen flexibility
fn toggle_cursor_confinement(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut preference: ResMut<CursorConfinementPreference>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        if let Ok(mut window) = windows.get_single_mut() {
            // Toggle user preference
            preference.user_wants_confined = !preference.user_wants_confined;
            
            // Apply the preference (only if window is focused)
            if window.focused {
                window.cursor_options.grab_mode = if preference.user_wants_confined {
                    CursorGrabMode::Confined
                } else {
                    CursorGrabMode::None
                };
            }
        }
    }
}

/// Automatically handle cursor confinement based on window focus
/// This allows alt-tab and window switching to work properly
fn handle_window_focus(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut preference: ResMut<CursorConfinementPreference>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        let is_focused = window.focused;
        
        // Detect focus state changes
        if is_focused != preference.was_focused {
            preference.was_focused = is_focused;
            
            if is_focused {
                // Window regained focus - restore user's preference
                if preference.user_wants_confined {
                    window.cursor_options.grab_mode = CursorGrabMode::Confined;
                }
            } else {
                // Window lost focus - ALWAYS release cursor for alt-tab/window switching
                window.cursor_options.grab_mode = CursorGrabMode::None;
            }
        }
    }
}