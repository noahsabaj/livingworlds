//! User Interface module for Living Worlds
//! 
//! Handles all UI elements including simulation controls,
//! and game state information display.

// Sub-modules for UI components
pub mod styles;
pub mod buttons;
pub mod dialogs;
pub mod components;
pub mod text_inputs;
pub mod sliders;
pub mod form;
pub mod toolbar;
pub mod builders;  // Centralized re-exports for all builders

use bevy::prelude::*;
use crate::resources::{ResourceOverlay, SelectedProvinceInfo, GameTime};
use crate::components::{TileInfoPanel, TileInfoText, MineralType};
use crate::constants::COLOR_TILE_INFO_BACKGROUND;
use crate::mesh::ProvinceStorage;

/// Marker component for the resource overlay display text
#[derive(Component)]
pub struct ResourceOverlayText;

/// Marker component for the game time display
#[derive(Component)]
pub struct GameTimeDisplay;

/// Marker component for the game speed display
#[derive(Component)]
pub struct GameSpeedDisplay;

/// Marker component for the control hints text
#[derive(Component)]
pub struct ControlHintsText;

/// UI Plugin that handles all user interface elements
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        use crate::states::GameState;
        
        // Add sub-plugins for UI systems
        app.add_plugins(buttons::ButtonPlugin);
        app.add_plugins(dialogs::DialogPlugin);
        app.add_plugins(text_inputs::TextInputPlugin);
        app.add_plugins(sliders::SliderPlugin);
        
        app
            .add_systems(OnEnter(GameState::InGame), setup_ui)
            .add_systems(OnExit(GameState::InGame), cleanup_game_ui)
            .add_systems(Update, (
                update_overlay_display,
                update_mineral_legend_visibility.run_if(resource_changed::<ResourceOverlay>),
                update_tile_info_ui.run_if(resource_changed::<SelectedProvinceInfo>),
                update_time_display,
                update_speed_display,
                update_control_hints,
            ).run_if(in_state(GameState::InGame)));
    }
}

/// Marker component for all in-game UI elements for easy cleanup
#[derive(Component)]
pub struct GameUIRoot;

/// Marker component for the mineral legend container
#[derive(Component)]
pub struct MineralLegendContainer;

/// Setup the UI elements
pub fn setup_ui(mut commands: Commands) {
    let ui_start = std::time::Instant::now();
    
    // Game time and speed display in top-right
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End,
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)),
        ZIndex(100),
        GameUIRoot,  // Mark for cleanup
    )).with_children(|parent| {
        // Year display
        parent.spawn((
                    Text::new("Year 1000"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    GameTimeDisplay,
                ));
                
                // Speed indicator with margin
                parent.spawn((
                    Text::new("Speed: 1x"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
                    GameSpeedDisplay,
                    Node {
                        margin: UiRect::top(Val::Px(4.0)),
                        ..default()
                    },
                ));
                
                // Control hints
                parent.spawn((
                    Text::new("[1-5] Speed • [Space] Pause"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.5, 0.5, 0.5, 1.0)),
                    ControlHintsText,
                    Node {
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    },
                ));
    });
    
    // Resource overlay legend in top-left with colored squares
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(8.0)),
            min_width: Val::Px(180.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)),
        ZIndex(100),
        GameUIRoot,  // Mark for cleanup
    )).with_children(|parent| {
        // Current overlay display
        parent.spawn((
            Node {
                margin: UiRect::bottom(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
        )).with_children(|p| {
            p.spawn((
                Text::new("Political Map"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                ResourceOverlayText,
            ));
        });
        
        // Control hint for overlay
        parent.spawn((
            Text::new("[M] Cycle Overlay"),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgba(0.5, 0.5, 0.5, 1.0)),
            Node {
                margin: UiRect::top(Val::Px(2.0)),
                ..default()
            },
        ));
        
        // Divider line - manually for now to avoid borrowing issues
        parent.spawn((
            Node {
                height: Val::Px(1.0),
                width: Val::Percent(100.0),
                margin: UiRect::vertical(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
        ));
        
        // Mineral legend container (will be hidden/shown based on overlay)
        // Create mineral legend container that can be hidden
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                display: Display::None,  // Start hidden
                ..default()
            },
            BackgroundColor(Color::NONE),
            MineralLegendContainer,
        )).with_children(|legend_parent| {
            // Title for legend
            legend_parent.spawn((
                Node {
                    margin: UiRect::bottom(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            )).with_children(|p| {
                p.spawn((
                    Text::new("Mineral Legend:"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
                ));
            });
            
            // Define minerals with their colors and chemical symbols
            let minerals = [
            (MineralType::Iron, "Fe", Color::srgb(0.7, 0.3, 0.2)),      // Rusty brown
            (MineralType::Copper, "Cu", Color::srgb(0.7, 0.4, 0.2)),    // Copper orange
            (MineralType::Tin, "Sn", Color::srgb(0.6, 0.6, 0.7)),       // Silver-grey
            (MineralType::Gold, "Au", Color::srgb(1.0, 0.84, 0.0)),     // Gold
            (MineralType::Coal, "C", Color::srgb(0.2, 0.2, 0.2)),       // Black
            (MineralType::Stone, "Si", Color::srgb(0.5, 0.5, 0.5)),     // Grey
            (MineralType::Gems, "Gm", Color::srgb(0.5, 0.2, 0.9)),      // Purple
            ];
            
            // Create a row for each mineral
            for (_mineral_type, symbol, color) in minerals.iter() {
                legend_parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            )).with_children(|row| {
                // Colored square
                row.spawn((
                    Node {
                        width: Val::Px(16.0),
                        height: Val::Px(16.0),
                        margin: UiRect::right(Val::Px(6.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor(*color),
                    BorderColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
                )).with_children(|square| {
                    // Chemical symbol in the square
                    square.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    )).with_children(|s| {
                        s.spawn((
                            Text::new(*symbol),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
                });
                
                // Mineral name
                let name = match symbol {
                    &"Fe" => "Iron",
                    &"Cu" => "Copper",
                    &"Sn" => "Tin",
                    &"Au" => "Gold",
                    &"C" => "Coal",
                    &"Si" => "Stone",
                    _ => "Gems",
                };
                
                row.spawn((
                    Text::new(name),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.7, 0.7, 0.7, 1.0)),
                ));
            });
            }
        });
    });
    
    // Tile info panel - moved to bottom-right to avoid overlap - Using PanelBuilder
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            right: Val::Px(10.0),  // Changed from left to right
            min_width: Val::Px(250.0),
            ..default()
        },
        TileInfoPanel,
        ZIndex(100),
        GameUIRoot,  // Mark for cleanup
    )).with_children(|parent| {
        // Create panel with custom background color
        let mut panel_commands = parent.spawn((
            Node {
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(COLOR_TILE_INFO_BACKGROUND),
        ));
        panel_commands.with_children(|panel| {
            panel.spawn((
                Text::new("Click a tile to see info"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TileInfoText,
            ));
        });
    });
    
    // Bottom-left controls removed - consolidated with top UI elements
    
    println!("UI setup completed in {:.2}s", ui_start.elapsed().as_secs_f32());
}

/// Update the resource overlay display text
pub fn update_overlay_display(
    overlay: Res<ResourceOverlay>,
    mut query: Query<&mut Text, With<ResourceOverlayText>>,
) {
    if !overlay.is_changed() {
        return;
    }
    
    for mut text in query.iter_mut() {
        *text = Text::new(overlay.display_name());
    }
}

/// Update UI panel showing selected tile info
pub fn update_tile_info_ui(
    selected_info: Res<SelectedProvinceInfo>,
    province_storage: Res<ProvinceStorage>,
    mut text_query: Query<&mut Text, With<TileInfoText>>,
) {
    // Update text if we have a UI panel
    if let Ok(mut text) = text_query.get_single_mut() {
        if let Some(province_id) = selected_info.province_id {
            // Use HashMap for O(1) lookup instead of O(n) linear search through 900k provinces
            if let Some(&idx) = province_storage.province_by_id.get(&province_id) {
                let province = &province_storage.provinces[idx];
                *text = Text::new(format!(
                    "Province #{}\nTerrain: {:?}\nElevation: {:.2}\nPopulation: {:.0}\nAgriculture: {:.1}\nWater Distance: {:.1} hex\nPosition: ({:.0}, {:.0})",
                    province.id,
                    province.terrain,
                    province.elevation,
                    province.population,
                    province.agriculture,
                    province.fresh_water_distance,
                    province.position.x,
                    province.position.y,
                ));
            }
        } else {
            *text = Text::new("Click a tile to see info");
        }
    }
}

/// Update mineral legend visibility based on current overlay
pub fn update_mineral_legend_visibility(
    overlay: Res<ResourceOverlay>,
    mut legend_query: Query<&mut Node, With<MineralLegendContainer>>,
) {
    if let Ok(mut node) = legend_query.get_single_mut() {
        // Only show legend when viewing mineral overlays
        node.display = match *overlay {
            ResourceOverlay::Mineral(_) | ResourceOverlay::AllMinerals => Display::Flex,
            _ => Display::None,
        };
    }
}

/// Cleanup all game UI elements when leaving InGame state
pub fn cleanup_game_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<GameUIRoot>>,
) {
    println!("Cleaning up game UI elements");
    for entity in &ui_query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Update the game time display
fn update_time_display(
    game_time: Res<GameTime>,
    mut query: Query<&mut Text, With<GameTimeDisplay>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        let year = 1000 + (game_time.current_date / 365.0) as u32;
        let day = (game_time.current_date % 365.0) as u32;
        **text = format!("Year {} - Day {}", year, day);
    }
}

/// Update the speed display when it changes
fn update_speed_display(
    game_time: Res<GameTime>,
    mut query: Query<&mut Text, With<GameSpeedDisplay>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        if game_time.paused {
            **text = "PAUSED".to_string();
        } else {
            let speed_text = match game_time.speed {
                s if s <= 0.0 => "Paused",
                s if s <= 1.0 => "Normal",
                s if s <= 3.0 => "Fast (3x)",
                s if s <= 6.0 => "Faster (6x)",
                _ => "Fastest (9x)",
            };
            **text = format!("Speed: {}", speed_text);
        }
    }
}

/// Update the control hints based on pause state
fn update_control_hints(
    game_time: Res<GameTime>,
    mut query: Query<&mut Text, With<ControlHintsText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        let pause_text = if game_time.paused {
            "Unpause"
        } else {
            "Pause"
        };
        **text = format!("[1-5] Speed • [Space] {}", pause_text);
    }
}