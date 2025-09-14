//! Event handlers for settings menu interactions

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode, MonitorSelection, VideoModeSelection, PresentMode};
use bevy_pkv::PkvStore;
use crate::states::CurrentSettingsTab;
use super::types::*;
use super::components::*;
use super::persistence::save_settings;
use super::settings_ui::spawn_settings_menu;

// Settings button handling is done in menus.rs (handle_button_interactions and handle_pause_button_interactions)

/// Handle tab button clicks
pub fn handle_tab_buttons(
    mut interactions: Query<(&Interaction, &TabButton), (Changed<Interaction>, With<Button>)>,
    mut current_tab: ResMut<CurrentSettingsTab>,
    mut commands: Commands,
    settings_root: Query<Entity, With<SettingsMenuRoot>>,
    settings: Res<GameSettings>,
    temp_settings: ResMut<TempGameSettings>,
    dirty_state: ResMut<SettingsDirtyState>,
) {
    for (interaction, tab_button) in &mut interactions {
        if *interaction == Interaction::Pressed {
            println!("Switching to tab: {:?}", tab_button.tab);
            current_tab.0 = tab_button.tab;
            
            // Respawn settings menu with new tab
            if let Ok(entity) = settings_root.single() {
                commands.entity(entity).despawn();
            }
            // Respawn the settings menu with the new tab selected
            spawn_settings_menu(commands, settings, temp_settings, current_tab.into(), dirty_state);
            return; // Exit after handling the pressed button
        }
    }
}

/// Handle cycle button clicks
pub fn handle_cycle_buttons(
    mut interactions: Query<(&Interaction, &CycleButton, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    mut temp_settings: ResMut<TempGameSettings>,
) {
    for (interaction, cycle_button, children) in &mut interactions {
        if *interaction == Interaction::Pressed {
            match cycle_button.setting_type {
                SettingType::WindowMode => {
                    temp_settings.0.graphics.window_mode = temp_settings.0.graphics.window_mode.cycle();
                    for child in children {
                        if let Ok(mut text) = text_query.get_mut(*child) {
                            **text = format!("< {} >", temp_settings.0.graphics.window_mode.as_str());
                        }
                    }
                }
                SettingType::Resolution => {
                    temp_settings.0.graphics.resolution = temp_settings.0.graphics.resolution.cycle();
                    for child in children {
                        if let Ok(mut text) = text_query.get_mut(*child) {
                            **text = format!("< {} >", temp_settings.0.graphics.resolution.as_str());
                        }
                    }
                }
                SettingType::ShadowQuality => {
                    temp_settings.0.graphics.shadow_quality = temp_settings.0.graphics.shadow_quality.cycle();
                    for child in children {
                        if let Ok(mut text) = text_query.get_mut(*child) {
                            **text = format!("< {} >", temp_settings.0.graphics.shadow_quality.as_str());
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// Handle toggle button interactions
pub fn handle_toggle_buttons(
    mut interactions: Query<(Entity, &Interaction, &mut ToggleButton, &mut BackgroundColor, &Children), (Changed<Interaction>, With<Button>)>,
    mut temp_settings: ResMut<TempGameSettings>,
    mut commands: Commands,
    mut text_query: Query<&mut Text>,
) {
    for (entity, interaction, mut toggle, mut bg_color, children) in &mut interactions {
        if *interaction == Interaction::Pressed {
            toggle.enabled = !toggle.enabled;
            
            match toggle.setting_type {
                SettingType::VSync => temp_settings.0.graphics.vsync = toggle.enabled,
                SettingType::MuteWhenUnfocused => temp_settings.0.audio.mute_when_unfocused = toggle.enabled,
                SettingType::ShowFPS | SettingType::ShowFps => temp_settings.0.interface.show_fps = toggle.enabled,
                SettingType::ShowProvinceInfo => temp_settings.0.interface.show_province_info = toggle.enabled,
                SettingType::ShowTooltips => temp_settings.0.interface.show_tooltips = toggle.enabled,
                SettingType::InvertZoom => temp_settings.0.controls.invert_zoom = toggle.enabled,
                _ => {}
            }
            
            *bg_color = BackgroundColor(if toggle.enabled {
                Color::srgb(0.2, 0.4, 0.2)
            } else {
                Color::srgb(0.15, 0.15, 0.18)
            });
            
            let mut found_text = false;
            for &child in children {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.0 = if toggle.enabled { "X".to_string() } else { "".to_string() };
                    found_text = true;
                    break;
                }
            }
            
            // If no text child found and we need to show checkmark, add one
            if !found_text && toggle.enabled {
                commands.entity(entity).with_children(|btn| {
                    btn.spawn((
                        Text::new("X"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
            }
        }
    }
}

/// Handle slider interactions using the new SliderBuilder system
pub fn handle_slider_interactions(
    sliders: Query<(&crate::ui::Slider, &SettingsSlider), Changed<crate::ui::Slider>>,
    mut temp_settings: ResMut<TempGameSettings>,
) {
    for (slider, settings_slider) in &sliders {
        match settings_slider.setting_type {
            SettingType::RenderScale => temp_settings.0.graphics.render_scale = slider.value,
            SettingType::MasterVolume => temp_settings.0.audio.master_volume = slider.value,
            SettingType::SfxVolume | SettingType::SFXVolume => temp_settings.0.audio.sfx_volume = slider.value,
            SettingType::UiScale | SettingType::UIScale => temp_settings.0.interface.ui_scale = slider.value,
            SettingType::TooltipDelay => temp_settings.0.interface.tooltip_delay = slider.value,
            SettingType::EdgePanSpeed => temp_settings.0.controls.edge_pan_speed = slider.value,
            SettingType::ZoomSensitivity => temp_settings.0.controls.zoom_sensitivity = slider.value,
            SettingType::CameraSpeed => temp_settings.0.controls.camera_speed = slider.value,
            SettingType::ZoomSpeed => temp_settings.0.controls.zoom_speed = slider.value,
            _ => {}
        }
    }
}

/// Handle Apply and Exit buttons
pub fn handle_apply_cancel_buttons(
    interactions: Query<(&Interaction, AnyOf<(&ApplyButton, &CancelButton)>), Changed<Interaction>>,
    mut commands: Commands,
    settings_root: Query<Entity, With<SettingsMenuRoot>>,
    mut settings: ResMut<GameSettings>,
    mut temp_settings: ResMut<TempGameSettings>,
    mut events: EventWriter<SettingsChanged>,
    mut resolution_events: EventWriter<RequestResolutionConfirm>,
    mut pkv: ResMut<PkvStore>,
    dirty_state: Res<SettingsDirtyState>,
) {
    for (interaction, (apply_button, cancel_button)) in &interactions {
        if *interaction == Interaction::Pressed {
            if apply_button.is_some() {
                // Apply button pressed
                if !dirty_state.is_dirty {
                    println!("No changes to apply");
                    // Close menu even if no changes
                    if let Ok(entity) = settings_root.single() {
                        commands.entity(entity).despawn();
                    }
                    continue;
                }
                
                println!("Applying settings");
                
                let resolution_changed = settings.graphics.resolution.width != temp_settings.0.graphics.resolution.width
                    || settings.graphics.resolution.height != temp_settings.0.graphics.resolution.height
                    || settings.graphics.window_mode != temp_settings.0.graphics.window_mode;
                
                // Copy temp settings to actual settings
                *settings = temp_settings.0.clone();
                save_settings(&*settings, &mut *pkv);
                // Fire event to apply settings
                events.write(SettingsChanged);
                
                // Trigger resolution confirmation if needed
                if resolution_changed {
                    resolution_events.write(RequestResolutionConfirm);
                }
                
                // Close settings menu after applying
                if let Ok(entity) = settings_root.single() {
                    commands.entity(entity).despawn();
                }
            } else if cancel_button.is_some() {
                // Exit button pressed
                if dirty_state.is_dirty {
                    // Show unsaved changes dialog
                    println!("Unsaved changes detected - spawning confirmation dialog");
                    spawn_unsaved_changes_dialog(commands.reborrow());
                } else {
                    // No changes, just close
                    println!("Exiting settings (no changes)");
                    // Revert temp settings to match current settings
                    temp_settings.0 = settings.clone();
                    if let Ok(entity) = settings_root.single() {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}

/// Handle graphics preset buttons
pub fn handle_preset_buttons(
    mut preset_queries: ParamSet<(
        Query<(&Interaction, &PresetButton, Entity), (Changed<Interaction>, With<Button>)>,
        Query<(Entity, &PresetButton, &mut BackgroundColor, &mut BorderColor), With<Button>>,
    )>,
    mut temp_settings: ResMut<TempGameSettings>,
    mut dirty_state: ResMut<SettingsDirtyState>,
    mut slider_queries: Query<(&mut Slider, &Children)>,
    mut text_query: Query<&mut Text, With<SliderValueText>>,
) {
    // First, collect information about which buttons were interacted with
    let mut pressed_preset = None;
    let mut hover_interactions = Vec::new();
    let mut none_interactions = Vec::new();
    
    for (interaction, preset_button, entity) in preset_queries.p0().iter() {
        match *interaction {
            Interaction::Pressed => {
                pressed_preset = Some(preset_button.preset);
            }
            Interaction::Hovered => {
                hover_interactions.push((entity, preset_button.preset));
            }
            Interaction::None => {
                none_interactions.push((entity, preset_button.preset));
            }
        }
    }
    
    // If a preset was pressed, apply it
    if let Some(pressed) = pressed_preset {
        println!("Applying graphics preset: {:?}", pressed);
        temp_settings.0.graphics.apply_preset(pressed);
        dirty_state.is_dirty = true;
        
        for (mut slider, children) in &mut slider_queries {
            match slider.setting_type {
                SettingType::RenderScale => {
                    slider.value = temp_settings.0.graphics.render_scale;
                }
                _ => {}
            }
            
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    if slider.setting_type == SettingType::RenderScale {
                        text.0 = format!("{:.0}%", slider.value * 100.0);
                    }
                }
            }
        }
        
        for (entity, button_preset, mut bg_color, mut border_color) in preset_queries.p1().iter_mut() {
            let is_selected = button_preset.preset == pressed;
            if is_selected {
                // This is the newly selected preset - make it green
                *bg_color = BackgroundColor(Color::srgb(0.15, 0.3, 0.15));
                *border_color = BorderColor(Color::srgb(0.3, 0.5, 0.3));
            } else {
                // This is not selected - make it gray
                *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.18));
                *border_color = BorderColor(Color::srgb(0.3, 0.3, 0.35));
            }
        }
    }
    
    for (entity, preset) in hover_interactions {
        let is_selected = temp_settings.0.graphics.current_preset() == Some(preset);
        if !is_selected {
            // Only change hover color if not currently selected
            if let Ok((_, _, mut bg_color, mut border_color)) = preset_queries.p1().get_mut(entity) {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.22, 0.25));
                *border_color = BorderColor(Color::srgb(0.4, 0.45, 0.5));
            }
        }
    }
    
    for (entity, preset) in none_interactions {
        let is_selected = temp_settings.0.graphics.current_preset() == Some(preset);
        if let Ok((_, _, mut bg_color, mut border_color)) = preset_queries.p1().get_mut(entity) {
            if is_selected {
                *bg_color = BackgroundColor(Color::srgb(0.15, 0.3, 0.15));  // Green for selected
                *border_color = BorderColor(Color::srgb(0.3, 0.5, 0.3));
            } else {
                *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.18));
                *border_color = BorderColor(Color::srgb(0.3, 0.3, 0.35));
            }
        }
    }
}

/// Handle reset to defaults button
pub fn handle_reset_button(
    mut interactions: Query<(&Interaction, &ResetButton), (Changed<Interaction>, With<Button>)>,
    mut temp_settings: ResMut<TempGameSettings>,
    mut dirty_state: ResMut<SettingsDirtyState>,
    settings: Res<GameSettings>,
) {
    for (interaction, _reset_button) in &mut interactions {
        if *interaction == Interaction::Pressed {
            println!("Resetting settings to defaults");
            
            // Reset temp settings to defaults
            temp_settings.0 = GameSettings::default();
            
            // Mark as dirty if different from current settings
            dirty_state.is_dirty = temp_settings.0 != *settings;
            
            println!("Settings reset to defaults - dirty state: {}", dirty_state.is_dirty);
        }
    }
}

/// Update UI elements when settings change
pub fn update_ui_on_settings_change(
    temp_settings: Res<TempGameSettings>,
    mut preset_buttons: Query<(&PresetButton, &mut BackgroundColor, &mut BorderColor), Without<Interaction>>,
    mut cycle_buttons: Query<(&CycleButton, &Children)>,
    mut toggle_buttons: Query<(&mut ToggleButton, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    for (preset_button, mut bg_color, mut border_color) in &mut preset_buttons {
        let is_selected = temp_settings.0.graphics.current_preset() == Some(preset_button.preset);
        if is_selected {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.3, 0.15));  // Green for selected
            *border_color = BorderColor(Color::srgb(0.3, 0.5, 0.3));
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.18));
            *border_color = BorderColor(Color::srgb(0.3, 0.3, 0.35));
        }
    }
    
    for (cycle_button, children) in &mut cycle_buttons {
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                match cycle_button.setting_type {
                    SettingType::WindowMode => {
                        text.0 = temp_settings.0.graphics.window_mode.as_str().to_string();
                    }
                    SettingType::Resolution => {
                        text.0 = temp_settings.0.graphics.resolution.as_str();
                    }
                    SettingType::ShadowQuality => {
                        text.0 = temp_settings.0.graphics.shadow_quality.as_str().to_string();
                    }
                    _ => {}
                }
            }
        }
    }
    
    for (mut toggle_button, children) in &mut toggle_buttons {
        let is_enabled = match toggle_button.setting_type {
            SettingType::VSync => temp_settings.0.graphics.vsync,
            SettingType::ShowFps | SettingType::ShowFPS => temp_settings.0.interface.show_fps,
            SettingType::ShowProvinceInfo => temp_settings.0.interface.show_province_info,
            SettingType::ShowTooltips => temp_settings.0.interface.show_tooltips,
            SettingType::MuteWhenUnfocused => temp_settings.0.audio.mute_when_unfocused,
            SettingType::InvertZoom => temp_settings.0.controls.invert_zoom,
            _ => false,
        };
        toggle_button.enabled = is_enabled;
        
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                text.0 = if is_enabled { "X".to_string() } else { "".to_string() };
            }
        }
    }
}

/// Track whether settings have been modified
pub fn track_dirty_state(
    settings: Res<GameSettings>,
    temp_settings: Res<TempGameSettings>,
    mut dirty_state: ResMut<SettingsDirtyState>,
) {
    // Compare temp settings with saved settings
    let is_dirty = settings.graphics.window_mode != temp_settings.0.graphics.window_mode
        || settings.graphics.resolution.width != temp_settings.0.graphics.resolution.width
        || settings.graphics.resolution.height != temp_settings.0.graphics.resolution.height
        || settings.graphics.vsync != temp_settings.0.graphics.vsync
        || (settings.graphics.render_scale - temp_settings.0.graphics.render_scale).abs() > 0.01
        || settings.graphics.shadow_quality != temp_settings.0.graphics.shadow_quality
        || (settings.audio.master_volume - temp_settings.0.audio.master_volume).abs() > 0.01
        || (settings.audio.sfx_volume - temp_settings.0.audio.sfx_volume).abs() > 0.01
        || settings.interface.ui_scale != temp_settings.0.interface.ui_scale
        || settings.interface.show_fps != temp_settings.0.interface.show_fps
        || settings.interface.show_tooltips != temp_settings.0.interface.show_tooltips
        || settings.controls.camera_speed != temp_settings.0.controls.camera_speed
        || settings.controls.zoom_speed != temp_settings.0.controls.zoom_speed
        || settings.controls.invert_zoom != temp_settings.0.controls.invert_zoom;
    
    if is_dirty != dirty_state.is_dirty {
        dirty_state.is_dirty = is_dirty;
        
        // This could be moved to a separate system if needed
    }
}

/// Apply settings changes to the game
pub fn apply_settings_changes(
    settings: Res<GameSettings>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if !settings.is_changed() {
        return;
    }
    
    println!("Applying settings changes");
    
    // Apply graphics settings
    if let Ok(mut window) = windows.get_single_mut() {
        // Apply window mode
        window.mode = match settings.graphics.window_mode {
            WindowModeOption::Windowed => WindowMode::Windowed,
            WindowModeOption::Borderless => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            WindowModeOption::Fullscreen => WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current),
        };
        
        // Apply resolution (only in windowed mode)
        if matches!(window.mode, WindowMode::Windowed) {
            window.resolution.set(
                settings.graphics.resolution.width,
                settings.graphics.resolution.height,
            );
        }
        
        // Apply VSync
        window.present_mode = if settings.graphics.vsync {
            PresentMode::AutoVsync
        } else {
            PresentMode::AutoNoVsync
        };
    }
    
    // Apply audio settings
    
    // Log the audio settings for now
    println!("  Master Volume: {:.0}%", settings.audio.master_volume * 100.0);
    println!("  SFX Volume: {:.0}%", settings.audio.sfx_volume * 100.0);
}

/// Update Apply button visual state based on dirty state
pub fn update_apply_button_state(
    dirty_state: Res<SettingsDirtyState>,
    mut apply_buttons: Query<&mut BackgroundColor, With<ApplyButton>>,
) {
    if dirty_state.is_changed() {
        for mut bg_color in &mut apply_buttons {
            if dirty_state.is_dirty {
                // Enable button - green tint
                *bg_color = BackgroundColor(Color::srgb(0.15, 0.25, 0.15));
            } else {
                // Disable button - grayed out
                *bg_color = BackgroundColor(Color::srgb(0.1, 0.1, 0.1));
            }
        }
    }
}

/// Validate settings to ensure they're within hardware capabilities
pub fn validate_settings(
    mut temp_settings: ResMut<TempGameSettings>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.single() else { return; };
    
    let monitor_width = window.width();
    let monitor_height = window.height();
    
    // Clamp resolution to monitor size
    if temp_settings.0.graphics.resolution.width > monitor_width {
        temp_settings.0.graphics.resolution.width = monitor_width;
    }
    if temp_settings.0.graphics.resolution.height > monitor_height {
        temp_settings.0.graphics.resolution.height = monitor_height;
    }
    
    // Ensure minimum resolution
    if temp_settings.0.graphics.resolution.width < 800.0 {
        temp_settings.0.graphics.resolution.width = 800.0;
    }
    if temp_settings.0.graphics.resolution.height < 600.0 {
        temp_settings.0.graphics.resolution.height = 600.0;
    }
    
    // Clamp all values to sensible ranges
    temp_settings.0.graphics.render_scale = temp_settings.0.graphics.render_scale.clamp(0.5, 2.0);
    temp_settings.0.audio.master_volume = temp_settings.0.audio.master_volume.clamp(0.0, 1.0);
    temp_settings.0.audio.sfx_volume = temp_settings.0.audio.sfx_volume.clamp(0.0, 1.0);
    temp_settings.0.interface.ui_scale = temp_settings.0.interface.ui_scale.clamp(0.75, 2.0);
    temp_settings.0.controls.camera_speed = temp_settings.0.controls.camera_speed.clamp(0.1, 5.0);
    temp_settings.0.controls.zoom_speed = temp_settings.0.controls.zoom_speed.clamp(0.1, 5.0);
}

/// Update slider visuals on hover
pub fn update_slider_visuals(
    mut interactions: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Slider>)>,
) {
    for (interaction, mut bg_color) in &mut interactions {
        match *interaction {
            Interaction::Hovered => *bg_color = BackgroundColor(Color::srgb(0.12, 0.12, 0.15)),
            Interaction::Pressed => *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.18)),
            Interaction::None => *bg_color = BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
        }
    }
}

/// Update Apply/Exit button hover effects
pub fn update_apply_exit_button_hover(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, AnyOf<(&ApplyButton, &CancelButton)>), 
        Changed<Interaction>
    >,
    dirty_state: Res<SettingsDirtyState>,
) {
    for (interaction, mut bg_color, mut border_color, (apply_button, _cancel_button)) in &mut interactions {
        if apply_button.is_some() {
            // Apply button
            match *interaction {
                Interaction::Hovered => {
                    if dirty_state.is_dirty {
                        *bg_color = BackgroundColor(Color::srgb(0.2, 0.35, 0.2));
                        *border_color = BorderColor(Color::srgb(0.4, 0.6, 0.4));
                    } else {
                        *bg_color = BackgroundColor(Color::srgb(0.12, 0.12, 0.12));
                        *border_color = BorderColor(Color::srgb(0.25, 0.25, 0.25));
                    }
                }
                Interaction::Pressed => {
                    if dirty_state.is_dirty {
                        *bg_color = BackgroundColor(Color::srgb(0.25, 0.4, 0.25));
                        *border_color = BorderColor(Color::srgb(0.5, 0.7, 0.5));
                    }
                }
                Interaction::None => {
                    if dirty_state.is_dirty {
                        *bg_color = BackgroundColor(Color::srgb(0.15, 0.25, 0.15));
                        *border_color = BorderColor(Color::srgb(0.3, 0.5, 0.3));
                    } else {
                        *bg_color = BackgroundColor(Color::srgb(0.1, 0.1, 0.1));
                        *border_color = BorderColor(Color::srgb(0.2, 0.2, 0.2));
                    }
                }
            }
        } else {
            // Exit button
            match *interaction {
                Interaction::Hovered => {
                    *bg_color = BackgroundColor(Color::srgb(0.35, 0.2, 0.2));
                    *border_color = BorderColor(Color::srgb(0.6, 0.4, 0.4));
                }
                Interaction::Pressed => {
                    *bg_color = BackgroundColor(Color::srgb(0.4, 0.25, 0.25));
                    *border_color = BorderColor(Color::srgb(0.7, 0.5, 0.5));
                }
                Interaction::None => {
                    *bg_color = BackgroundColor(Color::srgb(0.25, 0.15, 0.15));
                    *border_color = BorderColor(Color::srgb(0.5, 0.3, 0.3));
                }
            }
        }
    }
}

/// Spawn unsaved changes confirmation dialog
fn spawn_unsaved_changes_dialog(commands: Commands) {
    // Use the new dialog builder system
    use crate::ui::dialog_presets;
    dialog_presets::unsaved_changes_dialog(commands);
}

/// Handle unsaved changes dialog buttons
pub fn handle_unsaved_changes_dialog(
    interactions: Query<(&Interaction, AnyOf<(&crate::ui::SaveButton, &crate::ui::DiscardButton, &crate::ui::CancelButton)>), Changed<Interaction>>,
    mut commands: Commands,
    dialog_query: Query<Entity, With<crate::ui::UnsavedChangesDialog>>,
    settings_root: Query<Entity, With<SettingsMenuRoot>>,
    mut settings: ResMut<GameSettings>,
    temp_settings: Res<TempGameSettings>,
    mut events: EventWriter<SettingsChanged>,
    mut pkv: ResMut<PkvStore>,
) {
    for (interaction, (save_button, discard_button, cancel_button)) in &interactions {
        if *interaction == Interaction::Pressed {
            // Close the dialog first
            if let Ok(dialog_entity) = dialog_query.get_single() {
                commands.entity(dialog_entity).despawn();
            }
            
            if save_button.is_some() {
                println!("Saving changes and exiting");
                *settings = temp_settings.0.clone();
                save_settings(&*settings, &mut *pkv);
                events.write(SettingsChanged);
                
                // Close settings menu
                if let Ok(entity) = settings_root.single() {
                    commands.entity(entity).despawn();
                }
            } else if discard_button.is_some() {
                // Discard changes and exit
                println!("Discarding changes and exiting");
                
                // Close settings menu without saving
                if let Ok(entity) = settings_root.single() {
                    commands.entity(entity).despawn();
                }
            } else if cancel_button.is_some() {
                // Cancel - stay in settings
                println!("Staying in settings menu");
                // Dialog is already closed, do nothing else
            }
        }
    }
}

// Hover effects for dialog buttons are now handled by the StyledButton system in ui/buttons.rs
// The old update_dialog_button_hover function has been removed as it's no longer needed