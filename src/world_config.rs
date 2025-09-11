//! World configuration UI and settings management
//! 
//! This module provides the interface for players to configure world generation parameters
//! before starting a new game. Since Living Worlds is an OBSERVER game, settings focus on
//! world parameters that affect emergent gameplay rather than difficulty or player advantages.

use bevy::prelude::*;
use rand::Rng;

use crate::states::{GameState, RequestStateTransition};
use crate::resources::{WorldSize, WorldSeed};
use crate::ui::buttons::{ButtonBuilder, ButtonStyle, ButtonSize};
use crate::ui::styles::{colors, dimensions};

// ============================================================================
// PLUGIN
// ============================================================================

pub struct WorldConfigPlugin;

impl Plugin for WorldConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<WorldGenerationSettings>()
            
            // State enter/exit systems
            .add_systems(OnEnter(GameState::WorldConfiguration), (
                init_default_settings,
                spawn_world_config_ui,
            ))
            .add_systems(OnExit(GameState::WorldConfiguration), despawn_world_config_ui)
            
            // Update systems
            .add_systems(Update, (
                handle_config_interactions,
                handle_preset_selection,
                handle_size_selection,
                handle_advanced_toggle,
                handle_slider_interactions,
                handle_climate_selection,
                handle_island_selection,
                handle_aggression_selection,
                handle_resource_selection,
                update_seed_display,
                update_slider_displays,
                handle_generate_button,
                handle_back_button,
                handle_random_buttons,
            ).run_if(in_state(GameState::WorldConfiguration)));
    }
}

// ============================================================================
// RESOURCES & TYPES
// ============================================================================

/// Complete world generation settings
#[derive(Resource, Clone, Debug)]
pub struct WorldGenerationSettings {
    // Basic settings
    pub world_name: String,
    pub world_size: WorldSize,
    pub custom_dimensions: Option<(u32, u32)>,
    pub seed: u32,
    pub preset: WorldPreset,
    
    // Advanced - Geography
    pub continent_count: u32,
    pub island_frequency: IslandFrequency,
    pub ocean_coverage: f32,
    pub climate_type: ClimateType,
    pub mountain_density: MountainDensity,
    pub river_density: f32,
    
    // Advanced - Civilizations
    pub starting_nations: u32,
    pub aggression_level: AggressionLevel,
    pub tech_progression_speed: f32,
    pub empire_stability: f32,
    pub trade_propensity: TradePropensity,
    
    // Advanced - Resources
    pub resource_abundance: ResourceAbundance,
    pub mineral_distribution: MineralDistribution,
    pub fertility_variance: f32,
}

impl Default for WorldGenerationSettings {
    fn default() -> Self {
        Self {
            world_name: generate_random_world_name(),
            world_size: WorldSize::Medium,
            custom_dimensions: None,
            seed: rand::thread_rng().gen(),
            preset: WorldPreset::Balanced,
            
            continent_count: 7,
            island_frequency: IslandFrequency::Moderate,
            ocean_coverage: 0.6,
            climate_type: ClimateType::Mixed,
            mountain_density: MountainDensity::Normal,
            river_density: 1.0,
            
            starting_nations: 8,
            aggression_level: AggressionLevel::Balanced,
            tech_progression_speed: 1.0,
            empire_stability: 0.5,
            trade_propensity: TradePropensity::Normal,
            
            resource_abundance: ResourceAbundance::Normal,
            mineral_distribution: MineralDistribution::Clustered,
            fertility_variance: 0.5,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorldPreset {
    Balanced,
    Pangaea,
    Archipelago,
    IceAge,
    DesertWorld,
    Custom,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IslandFrequency {
    None,
    Sparse,
    Moderate,
    Abundant,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClimateType {
    Arctic,
    Temperate,
    Tropical,
    Desert,
    Mixed,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MountainDensity {
    Few,
    Normal,
    Many,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AggressionLevel {
    Peaceful,
    Balanced,
    Warlike,
    Chaotic,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TradePropensity {
    Isolationist,
    Normal,
    Mercantile,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ResourceAbundance {
    Scarce,
    Normal,
    Rich,
    Bountiful,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MineralDistribution {
    Even,
    Clustered,
    Strategic,
}

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component)]
struct WorldConfigRoot;

#[derive(Component)]
struct WorldNameInput;

#[derive(Component)]
struct SeedInput;

#[derive(Component)]
struct PresetButton(WorldPreset);

#[derive(Component)]
struct SizeButton(WorldSize);

#[derive(Component)]
struct AdvancedToggle;

#[derive(Component)]
struct AdvancedPanel;

#[derive(Component)]
struct GenerateButton;

#[derive(Component)]
struct BackButton;

#[derive(Component)]
struct RandomNameButton;

#[derive(Component)]
struct RandomSeedButton;

// Advanced settings components
#[derive(Component)]
struct ContinentSlider;

#[derive(Component)]
struct ContinentValueText;

#[derive(Component)]
struct OceanSlider;

#[derive(Component)]
struct OceanValueText;

#[derive(Component)]
struct RiverSlider;

#[derive(Component)]
struct RiverValueText;

#[derive(Component)]
struct StartingNationsSlider;

#[derive(Component)]
struct StartingNationsValueText;

#[derive(Component)]
struct TechSpeedSlider;

#[derive(Component)]
struct TechSpeedValueText;

#[derive(Component)]
struct ClimateButton(ClimateType);

#[derive(Component)]
struct IslandButton(IslandFrequency);

#[derive(Component)]
struct AggressionButton(AggressionLevel);

#[derive(Component)]
struct ResourceButton(ResourceAbundance);

// ============================================================================
// SYSTEMS - INITIALIZATION
// ============================================================================

fn init_default_settings(mut commands: Commands) {
    commands.insert_resource(WorldGenerationSettings::default());
    println!("Initialized default world generation settings");
}

fn spawn_world_config_ui(mut commands: Commands) {
    println!("Spawning world configuration UI");
    
    // Root container with dark overlay
    commands.spawn((
        Button, // Block clicks behind
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(colors::OVERLAY_DARK),
        WorldConfigRoot,
    )).with_children(|parent| {
        // Main configuration panel
        parent.spawn((
            Node {
                width: Val::Px(700.0),
                min_height: Val::Px(600.0),
                padding: UiRect::all(Val::Px(30.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            BorderRadius::all(Val::Px(10.0)),
        )).with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("Configure New World"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));
            
            // World Name Section
            spawn_world_name_section(panel);
            
            // World Size Section
            spawn_world_size_section(panel);
            
            // Seed Section
            spawn_seed_section(panel);
            
            // Preset Section
            spawn_preset_section(panel);
            
            // Advanced Settings Toggle
            panel.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(45.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                BorderRadius::all(Val::Px(5.0)),
                AdvancedToggle,
            )).with_children(|button| {
                button.spawn((
                    Text::new("▼ Advanced Settings"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_SECONDARY),
                ));
            });
            
            // Advanced Settings Panel (initially hidden)
            spawn_advanced_panel(panel);
            
            // Bottom buttons
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                },
            )).with_children(|buttons| {
                // Back button
                ButtonBuilder::new("Back")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Large)
                    .with_marker(BackButton)
                    .build(buttons);
                
                // Generate World button
                ButtonBuilder::new("Generate World")
                    .style(ButtonStyle::Primary)
                    .size(ButtonSize::Large)
                    .with_marker(GenerateButton)
                    .build(buttons);
            });
        });
    });
}

fn spawn_world_name_section(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        },
    )).with_children(|section| {
        // Label
        section.spawn((
            Text::new("World Name"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(colors::TEXT_SECONDARY),
        ));
        
        // Input row
        section.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                ..default()
            },
        )).with_children(|row| {
            // Text input field (styled as button for now)
            row.spawn((
                Button,
                Node {
                    flex_grow: 1.0,
                    height: Val::Px(40.0),
                    padding: UiRect::horizontal(Val::Px(15.0)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                BorderRadius::all(Val::Px(5.0)),
                WorldNameInput,
            )).with_children(|input| {
                input.spawn((
                    Text::new("Aetheria Prime"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                ));
            });
            
            // Random button
            ButtonBuilder::new("🎲")
                .style(ButtonStyle::Secondary)
                .size(ButtonSize::Small)
                .with_marker(RandomNameButton)
                .build(row);
        });
    });
}

fn spawn_world_size_section(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        },
    )).with_children(|section| {
        // Label
        section.spawn((
            Text::new("World Size"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(colors::TEXT_SECONDARY),
        ));
        
        // Size buttons
        section.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                ..default()
            },
        )).with_children(|row| {
            for (size, label, desc) in [
                (WorldSize::Small, "Small", "300k provinces"),
                (WorldSize::Medium, "Medium", "600k provinces"),
                (WorldSize::Large, "Large", "900k provinces"),
            ] {
                row.spawn((
                    Button,
                    Node {
                        flex_grow: 1.0,
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(if size == WorldSize::Medium {
                        colors::PRIMARY
                    } else {
                        Color::srgb(0.2, 0.2, 0.2)
                    }),
                    BorderRadius::all(Val::Px(5.0)),
                    SizeButton(size),
                )).with_children(|button| {
                    button.spawn((
                        Text::new(label),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    button.spawn((
                        Text::new(desc),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_SECONDARY),
                    ));
                });
            }
        });
    });
}

fn spawn_seed_section(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        },
    )).with_children(|section| {
        // Label
        section.spawn((
            Text::new("World Seed"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(colors::TEXT_SECONDARY),
        ));
        
        // Input row
        section.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                ..default()
            },
        )).with_children(|row| {
            // Seed input field
            row.spawn((
                Button,
                Node {
                    flex_grow: 1.0,
                    height: Val::Px(40.0),
                    padding: UiRect::horizontal(Val::Px(15.0)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                BorderRadius::all(Val::Px(5.0)),
                SeedInput,
            )).with_children(|input| {
                input.spawn((
                    Text::new("1234567890"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                ));
            });
            
            // Random button
            ButtonBuilder::new("🎲")
                .style(ButtonStyle::Secondary)
                .size(ButtonSize::Small)
                .with_marker(RandomSeedButton)
                .build(row);
        });
    });
}

fn spawn_preset_section(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        },
    )).with_children(|section| {
        // Label
        section.spawn((
            Text::new("Quick Presets"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(colors::TEXT_SECONDARY),
        ));
        
        // Preset buttons (2 rows)
        for row_presets in [
            vec![
                (WorldPreset::Balanced, "Balanced", "Default settings"),
                (WorldPreset::Pangaea, "Pangaea", "One supercontinent"),
                (WorldPreset::Archipelago, "Archipelago", "Many islands"),
            ],
            vec![
                (WorldPreset::IceAge, "Ice Age", "Frozen world"),
                (WorldPreset::DesertWorld, "Desert", "Arid with oases"),
                (WorldPreset::Custom, "Custom", "Your settings"),
            ],
        ] {
            section.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|row| {
                for (preset, label, _desc) in row_presets {
                    row.spawn((
                        Button,
                        Node {
                            flex_grow: 1.0,
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(if preset == WorldPreset::Balanced {
                            colors::PRIMARY
                        } else {
                            Color::srgb(0.2, 0.2, 0.2)
                        }),
                        BorderRadius::all(Val::Px(5.0)),
                        PresetButton(preset),
                    )).with_children(|button| {
                        button.spawn((
                            Text::new(label),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
                }
            });
        }
    });
}

fn spawn_advanced_panel(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            display: Display::None, // Initially hidden
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
        BorderRadius::all(Val::Px(5.0)),
        AdvancedPanel,
    )).with_children(|panel| {
        // Create a horizontal container for the columns
        panel.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(30.0),
                ..default()
            },
        )).with_children(|columns| {
            // ===== LEFT COLUMN: GEOGRAPHY & CLIMATE =====
            columns.spawn((
                Node {
                    flex_basis: Val::Percent(33.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    ..default()
                },
            )).with_children(|left_col| {
                // Section header
                left_col.spawn((
                    Text::new("Geography & Climate"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    Node {
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                ));
                
                // Continent Count Slider
                spawn_slider_control(left_col, "Continents", "7", 1.0, 12.0, 7.0, ContinentSlider, ContinentValueText);
                
                // Ocean Coverage Slider
                spawn_slider_control(left_col, "Ocean Coverage", "60%", 30.0, 80.0, 60.0, OceanSlider, OceanValueText);
                
                // River Density Slider
                spawn_slider_control(left_col, "River Density", "1.0x", 0.5, 2.0, 1.0, RiverSlider, RiverValueText);
            });
            
            // ===== MIDDLE COLUMN: CLIMATE & TERRAIN =====
            columns.spawn((
                Node {
                    flex_basis: Val::Percent(33.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    ..default()
                },
            )).with_children(|middle_col| {
                // Section header (invisible for alignment)
                middle_col.spawn((
                    Text::new(""),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::NONE),
                    Node {
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                ));
                
                // Climate Type Selection
                spawn_selection_row(
                    middle_col,
                    "Climate Type",
                    vec![
                        ("Arctic", ClimateType::Arctic),
                        ("Temperate", ClimateType::Temperate),
                        ("Tropical", ClimateType::Tropical),
                        ("Desert", ClimateType::Desert),
                        ("Mixed", ClimateType::Mixed),
                    ],
                    ClimateType::Mixed,
                    |climate| ClimateButton(climate),
                );
                
                // Island Frequency Selection
                spawn_selection_row(
                    middle_col,
                    "Islands",
                    vec![
                        ("None", IslandFrequency::None),
                        ("Sparse", IslandFrequency::Sparse),
                        ("Moderate", IslandFrequency::Moderate),
                        ("Abundant", IslandFrequency::Abundant),
                    ],
                    IslandFrequency::Moderate,
                    |freq| IslandButton(freq),
                );
                
                // Resource Abundance Selection
                spawn_selection_row(
                    middle_col,
                    "Resources",
                    vec![
                        ("Scarce", ResourceAbundance::Scarce),
                        ("Normal", ResourceAbundance::Normal),
                        ("Rich", ResourceAbundance::Rich),
                        ("Bountiful", ResourceAbundance::Bountiful),
                    ],
                    ResourceAbundance::Normal,
                    |res| ResourceButton(res),
                );
            });
            
            // ===== RIGHT COLUMN: CIVILIZATIONS =====
            columns.spawn((
                Node {
                    flex_basis: Val::Percent(33.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    ..default()
                },
            )).with_children(|right_col| {
                // Section header
                right_col.spawn((
                    Text::new("Civilizations"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    Node {
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                ));
                
                // Starting Nations Slider
                spawn_slider_control(right_col, "Starting Nations", "8", 2.0, 20.0, 8.0, StartingNationsSlider, StartingNationsValueText);
                
                // Tech Progression Speed Slider
                spawn_slider_control(right_col, "Tech Speed", "1.0x", 0.5, 2.0, 1.0, TechSpeedSlider, TechSpeedValueText);
                
                // Aggression Level Selection
                spawn_selection_row(
                    right_col,
                    "Aggression",
                    vec![
                        ("Peaceful", AggressionLevel::Peaceful),
                        ("Balanced", AggressionLevel::Balanced),
                        ("Warlike", AggressionLevel::Warlike),
                        ("Chaotic", AggressionLevel::Chaotic),
                    ],
                    AggressionLevel::Balanced,
                    |aggr| AggressionButton(aggr),
                );
            });
        });
    });
}

// Helper function to create a slider control
fn spawn_slider_control<S: Component, T: Component>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    initial_value: &str,
    min: f32,
    max: f32,
    current: f32,
    slider_marker: S,
    text_marker: T,
) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            margin: UiRect::bottom(Val::Px(15.0)),
            ..default()
        },
    )).with_children(|control| {
        // Label row
        control.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
        )).with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
            ));
            
            row.spawn((
                Text::new(initial_value),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                text_marker,
            ));
        });
        
        // Slider track
        control.spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.05)),
            BorderRadius::all(Val::Px(15.0)),
            slider_marker,
        )).with_children(|track| {
            // Slider handle
            let percentage = ((current - min) / (max - min) * 100.0) as f32;
            track.spawn((
                Node {
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(percentage),
                    ..default()
                },
                BackgroundColor(colors::PRIMARY),
                BorderRadius::all(Val::Px(10.0)),
            ));
        });
    });
}

// Helper function to create a selection row
fn spawn_selection_row<T, F, C>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    options: Vec<(&str, T)>,
    default_value: T,
    make_component: F,
) where
    T: Clone + PartialEq + 'static + Send + Sync,
    F: Fn(T) -> C,
    C: Component,
{
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            margin: UiRect::bottom(Val::Px(15.0)),
            ..default()
        },
    )).with_children(|control| {
        // Label
        control.spawn((
            Text::new(label),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(colors::TEXT_SECONDARY),
        ));
        
        // Options row
        control.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                ..default()
            },
        )).with_children(|row| {
            for (option_label, value) in options {
                let is_selected = value == default_value;
                row.spawn((
                    Button,
                    Node {
                        flex_grow: 1.0,
                        height: Val::Px(35.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(if is_selected {
                        colors::PRIMARY
                    } else {
                        Color::srgb(0.15, 0.15, 0.15)
                    }),
                    BorderRadius::all(Val::Px(5.0)),
                    make_component(value.clone()),
                )).with_children(|button| {
                    button.spawn((
                        Text::new(option_label),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            }
        });
    });
}

fn despawn_world_config_ui(
    mut commands: Commands,
    query: Query<Entity, With<WorldConfigRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    println!("Despawned world configuration UI");
}

// ============================================================================
// SYSTEMS - INTERACTIONS
// ============================================================================

fn handle_config_interactions(
    // TODO: Implement interaction handling
) {
    // Placeholder for handling various UI interactions
}

fn handle_preset_selection(
    mut interactions: Query<(&Interaction, &PresetButton, &mut BackgroundColor), Changed<Interaction>>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    for (interaction, preset_button, mut bg_color) in &mut interactions {
        if *interaction == Interaction::Pressed {
            settings.preset = preset_button.0.clone();
            apply_preset(&mut settings);
            println!("Selected preset: {:?}", preset_button.0);
        }
    }
}

fn handle_size_selection(
    mut interactions: Query<(&Interaction, &SizeButton, &mut BackgroundColor), Changed<Interaction>>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    for (interaction, size_button, mut bg_color) in &mut interactions {
        if *interaction == Interaction::Pressed {
            settings.world_size = size_button.0.clone();
            println!("Selected world size: {:?}", size_button.0);
        }
    }
}

fn handle_advanced_toggle(
    mut interactions: Query<&Interaction, (Changed<Interaction>, With<AdvancedToggle>)>,
    mut advanced_panel: Query<&mut Node, With<AdvancedPanel>>,
    mut toggle_text: Query<&mut Text, With<AdvancedToggle>>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            if let Ok(mut panel_style) = advanced_panel.get_single_mut() {
                panel_style.display = match panel_style.display {
                    Display::None => Display::Flex,
                    _ => Display::None,
                };
                println!("Toggled advanced settings");
            }
        }
    }
}

fn update_seed_display(
    settings: Res<WorldGenerationSettings>,
    mut seed_text: Query<&mut Text, With<SeedInput>>,
) {
    if settings.is_changed() {
        for mut text in &mut seed_text {
            text.0 = settings.seed.to_string();
        }
    }
}

fn handle_generate_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<GenerateButton>)>,
    settings: Res<WorldGenerationSettings>,
    mut state_events: EventWriter<RequestStateTransition>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            println!("Generate World button pressed");
            println!("Settings: {:?}", *settings);
            
            // Transition to world generation loading screen
            state_events.write(RequestStateTransition {
                from: GameState::WorldConfiguration,
                to: GameState::WorldGenerationLoading,
            });
        }
    }
}

fn handle_back_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut state_events: EventWriter<RequestStateTransition>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            println!("Back button pressed");
            state_events.write(RequestStateTransition {
                from: GameState::WorldConfiguration,
                to: GameState::MainMenu,
            });
        }
    }
}

fn handle_random_buttons(
    mut name_interactions: Query<&Interaction, (Changed<Interaction>, With<RandomNameButton>)>,
    mut seed_interactions: Query<&Interaction, (Changed<Interaction>, With<RandomSeedButton>, Without<RandomNameButton>)>,
    mut settings: ResMut<WorldGenerationSettings>,
    mut name_text: Query<&mut Text, (With<WorldNameInput>, Without<SeedInput>)>,
) {
    // Random name button
    for interaction in &name_interactions {
        if *interaction == Interaction::Pressed {
            settings.world_name = generate_random_world_name();
            for mut text in &mut name_text {
                text.0 = settings.world_name.clone();
            }
            println!("Generated random name: {}", settings.world_name);
        }
    }
    
    // Random seed button
    for interaction in &seed_interactions {
        if *interaction == Interaction::Pressed {
            settings.seed = rand::thread_rng().gen();
            println!("Generated random seed: {}", settings.seed);
        }
    }
}

fn handle_slider_interactions(
    mut interactions: Query<(&Interaction, &Node, &Children), With<Button>>,
    mut continent_sliders: Query<&mut Node, (With<ContinentSlider>, Without<Button>)>,
    mut ocean_sliders: Query<&mut Node, (With<OceanSlider>, Without<Button>, Without<ContinentSlider>)>,
    mut river_sliders: Query<&mut Node, (With<RiverSlider>, Without<Button>, Without<ContinentSlider>, Without<OceanSlider>)>,
    mut settings: ResMut<WorldGenerationSettings>,
    windows: Query<&Window>,
) {
    // TODO: Implement actual slider dragging logic
    // For now, just handle clicks on the slider tracks
}

fn handle_climate_selection(
    mut param_set: ParamSet<(
        Query<(&Interaction, &ClimateButton, Entity), Changed<Interaction>>,
        Query<(&ClimateButton, &mut BackgroundColor)>,
    )>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    // First, check for pressed buttons
    let mut pressed_climate = None;
    for (interaction, climate_button, entity) in param_set.p0().iter() {
        if *interaction == Interaction::Pressed {
            pressed_climate = Some((climate_button.0.clone(), entity));
            break;
        }
    }
    
    // Then update colors if a button was pressed
    if let Some((climate_type, pressed_entity)) = pressed_climate {
        settings.climate_type = climate_type.clone();
        
        // Update all climate button colors
        for (_, mut bg_color) in param_set.p1().iter_mut() {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
        }
        
        // Set the pressed button to primary color
        if let Ok((_, mut bg_color)) = param_set.p1().get_mut(pressed_entity) {
            *bg_color = BackgroundColor(colors::PRIMARY);
        }
        
        println!("Selected climate type: {:?}", climate_type);
    }
}

fn handle_island_selection(
    mut param_set: ParamSet<(
        Query<(&Interaction, &IslandButton, Entity), Changed<Interaction>>,
        Query<(&IslandButton, &mut BackgroundColor)>,
    )>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    // First, check for pressed buttons
    let mut pressed_island = None;
    for (interaction, island_button, entity) in param_set.p0().iter() {
        if *interaction == Interaction::Pressed {
            pressed_island = Some((island_button.0.clone(), entity));
            break;
        }
    }
    
    // Then update colors if a button was pressed
    if let Some((island_freq, pressed_entity)) = pressed_island {
        settings.island_frequency = island_freq.clone();
        
        // Update all island button colors
        for (_, mut bg_color) in param_set.p1().iter_mut() {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
        }
        
        // Set the pressed button to primary color
        if let Ok((_, mut bg_color)) = param_set.p1().get_mut(pressed_entity) {
            *bg_color = BackgroundColor(colors::PRIMARY);
        }
        
        println!("Selected island frequency: {:?}", island_freq);
    }
}

fn handle_aggression_selection(
    mut param_set: ParamSet<(
        Query<(&Interaction, &AggressionButton, Entity), Changed<Interaction>>,
        Query<(&AggressionButton, &mut BackgroundColor)>,
    )>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    // First, check for pressed buttons
    let mut pressed_aggression = None;
    for (interaction, aggression_button, entity) in param_set.p0().iter() {
        if *interaction == Interaction::Pressed {
            pressed_aggression = Some((aggression_button.0.clone(), entity));
            break;
        }
    }
    
    // Then update colors if a button was pressed
    if let Some((aggression_level, pressed_entity)) = pressed_aggression {
        settings.aggression_level = aggression_level.clone();
        
        // Update all aggression button colors
        for (_, mut bg_color) in param_set.p1().iter_mut() {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
        }
        
        // Set the pressed button to primary color
        if let Ok((_, mut bg_color)) = param_set.p1().get_mut(pressed_entity) {
            *bg_color = BackgroundColor(colors::PRIMARY);
        }
        
        println!("Selected aggression level: {:?}", aggression_level);
    }
}

fn handle_resource_selection(
    mut param_set: ParamSet<(
        Query<(&Interaction, &ResourceButton, Entity), Changed<Interaction>>,
        Query<(&ResourceButton, &mut BackgroundColor)>,
    )>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    // First, check for pressed buttons
    let mut pressed_resource = None;
    for (interaction, resource_button, entity) in param_set.p0().iter() {
        if *interaction == Interaction::Pressed {
            pressed_resource = Some((resource_button.0.clone(), entity));
            break;
        }
    }
    
    // Then update colors if a button was pressed
    if let Some((resource_abundance, pressed_entity)) = pressed_resource {
        settings.resource_abundance = resource_abundance.clone();
        
        // Update all resource button colors
        for (_, mut bg_color) in param_set.p1().iter_mut() {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
        }
        
        // Set the pressed button to primary color
        if let Ok((_, mut bg_color)) = param_set.p1().get_mut(pressed_entity) {
            *bg_color = BackgroundColor(colors::PRIMARY);
        }
        
        println!("Selected resource abundance: {:?}", resource_abundance);
    }
}

fn update_slider_displays(
    settings: Res<WorldGenerationSettings>,
    mut continent_text: Query<&mut Text, With<ContinentValueText>>,
    mut ocean_text: Query<&mut Text, (With<OceanValueText>, Without<ContinentValueText>)>,
    mut river_text: Query<&mut Text, (With<RiverValueText>, Without<OceanValueText>, Without<ContinentValueText>)>,
    mut nations_text: Query<&mut Text, (With<StartingNationsValueText>, Without<RiverValueText>, Without<OceanValueText>, Without<ContinentValueText>)>,
    mut tech_text: Query<&mut Text, (With<TechSpeedValueText>, Without<StartingNationsValueText>, Without<RiverValueText>, Without<OceanValueText>, Without<ContinentValueText>)>,
) {
    // Update continent count display
    for mut text in &mut continent_text {
        text.0 = settings.continent_count.to_string();
    }
    
    // Update ocean coverage display
    for mut text in &mut ocean_text {
        text.0 = format!("{}%", (settings.ocean_coverage * 100.0) as u32);
    }
    
    // Update river density display
    for mut text in &mut river_text {
        text.0 = format!("{:.1}x", settings.river_density);
    }
    
    // Update starting nations display
    for mut text in &mut nations_text {
        text.0 = settings.starting_nations.to_string();
    }
    
    // Update tech speed display
    for mut text in &mut tech_text {
        text.0 = format!("{:.1}x", settings.tech_progression_speed);
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn apply_preset(settings: &mut WorldGenerationSettings) {
    match settings.preset {
        WorldPreset::Balanced => {
            settings.continent_count = 7;
            settings.island_frequency = IslandFrequency::Moderate;
            settings.ocean_coverage = 0.6;
            settings.climate_type = ClimateType::Mixed;
        },
        WorldPreset::Pangaea => {
            settings.continent_count = 1;
            settings.island_frequency = IslandFrequency::Sparse;
            settings.ocean_coverage = 0.7;
            settings.climate_type = ClimateType::Mixed;
        },
        WorldPreset::Archipelago => {
            settings.continent_count = 3;
            settings.island_frequency = IslandFrequency::Abundant;
            settings.ocean_coverage = 0.75;
            settings.climate_type = ClimateType::Tropical;
        },
        WorldPreset::IceAge => {
            settings.continent_count = 5;
            settings.island_frequency = IslandFrequency::Sparse;
            settings.ocean_coverage = 0.5;
            settings.climate_type = ClimateType::Arctic;
        },
        WorldPreset::DesertWorld => {
            settings.continent_count = 4;
            settings.island_frequency = IslandFrequency::None;
            settings.ocean_coverage = 0.3;
            settings.climate_type = ClimateType::Desert;
        },
        WorldPreset::Custom => {
            // Don't change settings for custom
        },
    }
}

fn generate_random_world_name() -> String {
    let prefixes = vec![
        "New", "Ancient", "Lost", "Eternal", "Prime", "Nova", "Neo", "Crystal",
        "Golden", "Silver", "Mystic", "Shadow", "Dawn", "Twilight", "Astral",
    ];
    
    let roots = vec![
        "Terra", "Gaia", "Eden", "Avalon", "Elysium", "Pangaea", "Atlantis",
        "Aetheria", "Celestia", "Arcadia", "Zephyr", "Olympus", "Valhalla",
        "Midgard", "Asgard", "Nibiru", "Xanadu", "Shangri-La", "Lemuria",
    ];
    
    let suffixes = vec![
        "", " Prime", " Nova", " Alpha", " Beta", " Omega", " Major", " Minor",
        " III", " VII", " IX", " XI", " Reborn", " Ascendant", " Eternal",
    ];
    
    let mut rng = rand::thread_rng();
    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let root = roots[rng.gen_range(0..roots.len())];
    let suffix = suffixes[rng.gen_range(0..suffixes.len())];
    
    format!("{} {}{}", prefix, root, suffix).trim().to_string()
}