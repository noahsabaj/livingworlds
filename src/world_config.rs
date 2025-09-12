//! World configuration UI and settings management
//! 
//! This module provides the interface for players to configure world generation parameters
//! before starting a new game. Since Living Worlds is an OBSERVER game, settings focus on
//! world parameters that affect emergent gameplay rather than difficulty or player advantages.

use bevy::prelude::*;
use rand::Rng;
use bevy_simple_text_input::{
    TextInputPlugin, TextInput, TextInputSettings, TextInputSubmitEvent,
    TextInputValue, TextInputTextFont, TextInputTextColor
};

use crate::states::{GameState, RequestStateTransition};
use crate::resources::WorldSize;
use crate::ui::buttons::{ButtonBuilder, ButtonStyle, ButtonSize, StyledButton};
use crate::ui::styles::{colors, dimensions, helpers};
use crate::name_generator::{NameGenerator, NameType};

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
                handle_preset_hover,
                handle_size_selection,
                handle_advanced_toggle,
                handle_slider_interactions,
                handle_climate_selection,
                handle_island_selection,
                handle_aggression_selection,
                handle_resource_selection,
                update_seed_display,
                update_slider_displays,
                handle_text_input_changes,
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
        let mut gen = NameGenerator::new();
        Self {
            world_name: gen.generate(NameType::World),
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WorldPreset {
    Balanced,
    Pangaea,
    Archipelago,
    IceAge,
    DesertWorld,
    Custom,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IslandFrequency {
    None,
    Sparse,
    Moderate,
    Abundant,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClimateType {
    Arctic,
    Temperate,
    Tropical,
    Desert,
    Mixed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MountainDensity {
    Few,
    Normal,
    Many,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AggressionLevel {
    Peaceful,
    Balanced,
    Warlike,
    Chaotic,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TradePropensity {
    Isolationist,
    Normal,
    Mercantile,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResourceAbundance {
    Scarce,
    Normal,
    Rich,
    Bountiful,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
struct WorldNameText;  // The actual text display

#[derive(Component)]
struct SeedInput;

#[derive(Component)]
struct SeedText;  // The actual text display

#[derive(Component)]
struct WorldPreviewText;

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

#[derive(Component)]
struct PresetDescription(String);

#[derive(Component)]
struct PresetDescriptionText;

#[derive(Component)]
struct GenerationTimeEstimate;

#[derive(Component)]
struct AdvancedToggleText;

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
        // Main configuration panel - expanded width with helpful information
        parent.spawn((
            Node {
                width: Val::Px(1000.0),  // Expanded from 700px for better spacing
                min_height: Val::Px(700.0),  // Increased to accommodate help text
                padding: UiRect::all(Val::Px(40.0)),  // More generous padding
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::MARGIN_LARGE),  // Use standard spacing
                border: helpers::standard_border(),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_DARK),
            BorderColor(colors::BORDER_DEFAULT),
            BorderRadius::all(Val::Px(dimensions::CORNER_RADIUS)),
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
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));
            
            // World Preview Info Section
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(15.0)),
                    margin: UiRect::bottom(Val::Px(15.0)),
                    border: helpers::standard_border(),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_LIGHT),
                BorderColor(colors::BORDER_DEFAULT),
                BorderRadius::all(Val::Px(5.0)),
            )).with_children(|info| {
                info.spawn((
                    Text::new("World Preview"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_SECONDARY),
                ));
                info.spawn((
                    Text::new("• Estimated land coverage: ~40%\n• Starting civilizations: 8 nations\n• World complexity: Moderate"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    WorldPreviewText,
                ));
            });
            
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
                    border: helpers::standard_border(),
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_LIGHT),
                BorderColor(colors::BORDER_DEFAULT),
                BorderRadius::all(Val::Px(dimensions::CORNER_RADIUS)),
                AdvancedToggle,
            )).with_children(|button| {
                button.spawn((
                    Text::new("⚙️ Show Advanced Settings"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_SECONDARY),
                ));
            });
            
            // Advanced Settings Panel (initially hidden)
            spawn_advanced_panel(panel);
            
            // Generation time estimate
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    margin: UiRect::top(Val::Px(20.0)),
                    border: helpers::standard_border(),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_LIGHT),
                BorderColor(colors::BORDER_DEFAULT),
                BorderRadius::all(Val::Px(5.0)),
            )).with_children(|estimate| {
                estimate.spawn((
                    Text::new("⏱️ Estimated generation time: ~3-7 seconds"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_MUTED),
                    GenerationTimeEstimate,
                ));
            });
            
            // Bottom buttons
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    margin: UiRect::top(Val::Px(15.0)),
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
            row_gap: Val::Px(5.0),
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
            // Real text input field using bevy_simple_text_input
            row.spawn((
                Node {
                    flex_grow: 1.0,
                    height: Val::Px(40.0),
                    padding: UiRect::horizontal(Val::Px(15.0)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    border: helpers::standard_border(),
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_LIGHT),
                BorderColor(colors::BORDER_DEFAULT),
                BorderRadius::all(Val::Px(5.0)),
            )).with_children(|input_container| {
                // Add the text input components
                input_container.spawn((
                    Text::new("Aetheria Prime"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    TextInput,
                    TextInputSettings {
                        retain_on_submit: true,
                        ..default()
                    },
                    TextInputValue("Aetheria Prime".to_string()),
                    TextInputTextFont(TextFont {
                        font_size: 18.0,
                        ..default()
                    }),
                    TextInputTextColor(TextColor(colors::TEXT_PRIMARY)),
                    WorldNameInput,
                    WorldNameText,
                ));
            });
            
            // Random button with ButtonBuilder
            ButtonBuilder::new("🎲 Random")
                .style(ButtonStyle::Secondary)
                .size(ButtonSize::Small)
                .with_marker(RandomNameButton)
                .build(row);
        });
        
        // Help text
        section.spawn((
            Text::new("Give your world a unique identity. The name will appear in game history."),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::TEXT_MUTED),
            Node {
                margin: UiRect::left(Val::Px(5.0)),
                ..default()
            },
        ));
    });
}

fn spawn_world_size_section(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
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
        
        // Size buttons row
        section.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                ..default()
            },
        )).with_children(|row| {
            for (size, label, desc, provinces) in [
                (WorldSize::Small, "Small", "Quick games", "300,000 provinces"),
                (WorldSize::Medium, "Medium", "Balanced", "600,000 provinces"),
                (WorldSize::Large, "Large", "Epic scale", "900,000 provinces"),
            ] {
                // Create a container for each size option
                row.spawn((
                    Button,
                    Node {
                        flex_grow: 1.0,
                        height: Val::Px(65.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.0)),
                        border: helpers::standard_border(),
                        ..default()
                    },
                    BackgroundColor(if size == WorldSize::Medium {
                        colors::PRIMARY
                    } else {
                        colors::BACKGROUND_LIGHT
                    }),
                    BorderColor(if size == WorldSize::Medium {
                        colors::PRIMARY
                    } else {
                        colors::BORDER_DEFAULT
                    }),
                    BorderRadius::all(Val::Px(5.0)),
                    SizeButton(size),
                    // Hover effects handled by interaction system
                )).with_children(|button| {
                    button.spawn((
                        Text::new(label),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(if size == WorldSize::Medium {
                            Color::WHITE
                        } else {
                            colors::TEXT_PRIMARY
                        }),
                    ));
                    button.spawn((
                        Text::new(desc),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(if size == WorldSize::Medium {
                            Color::srgba(1.0, 1.0, 1.0, 0.8)
                        } else {
                            colors::TEXT_MUTED
                        }),
                    ));
                    button.spawn((
                        Text::new(provinces),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(if size == WorldSize::Medium {
                            Color::srgba(1.0, 1.0, 1.0, 0.9)
                        } else {
                            colors::TEXT_SECONDARY
                        }),
                    ));
                });
            }
        });
        
        // Help text
        section.spawn((
            Text::new("Larger worlds offer more strategic depth but take longer to generate and simulate."),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::TEXT_MUTED),
            Node {
                margin: UiRect::left(Val::Px(5.0)),
                ..default()
            },
        ));
    });
}

fn spawn_seed_section(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
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
            // Real seed input field using bevy_simple_text_input
            row.spawn((
                Node {
                    flex_grow: 1.0,
                    height: Val::Px(40.0),
                    padding: UiRect::horizontal(Val::Px(15.0)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    border: helpers::standard_border(),
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_LIGHT),
                BorderColor(colors::BORDER_DEFAULT),
                BorderRadius::all(Val::Px(5.0)),
            )).with_children(|input_container| {
                // Add the text input components
                input_container.spawn((
                    Text::new("1234567890"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    TextInput,
                    TextInputSettings {
                        retain_on_submit: true,
                        ..default()
                    },
                    TextInputValue("1234567890".to_string()),
                    TextInputTextFont(TextFont {
                        font_size: 18.0,
                        ..default()
                    }),
                    TextInputTextColor(TextColor(colors::TEXT_PRIMARY)),
                    SeedInput,
                    SeedText,
                ));
            });
            
            // Random button with ButtonBuilder
            ButtonBuilder::new("🎲 Random")
                .style(ButtonStyle::Secondary)
                .size(ButtonSize::Small)
                .with_marker(RandomSeedButton)
                .build(row);
        });
        
        // Help text
        section.spawn((
            Text::new("Same seed = same world generation. Share seeds with friends for identical worlds."),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::TEXT_MUTED),
            Node {
                margin: UiRect::left(Val::Px(5.0)),
                ..default()
            },
        ));
    });
}

fn spawn_preset_section(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
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
        
        // Preset description that updates on hover
        section.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::bottom(Val::Px(5.0)),
                border: helpers::standard_border(),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_LIGHT),
            BorderColor(colors::BORDER_DEFAULT),
            BorderRadius::all(Val::Px(5.0)),
        )).with_children(|desc_box| {
            desc_box.spawn((
                Text::new("Hover over a preset to see its description"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT_MUTED),
                PresetDescriptionText,
            ));
        });
        
        // Preset buttons (2 rows)
        for row_presets in [
            vec![
                (WorldPreset::Balanced, "Balanced", "Default settings for a well-rounded experience"),
                (WorldPreset::Pangaea, "Pangaea", "One massive supercontinent surrounded by ocean"),
                (WorldPreset::Archipelago, "Archipelago", "Scattered islands connected by trade routes"),
            ],
            vec![
                (WorldPreset::IceAge, "Ice Age", "Frozen world with harsh survival conditions"),
                (WorldPreset::DesertWorld, "Desert", "Arid landscape with rare fertile oases"),
                (WorldPreset::Custom, "Custom", "Your personalized world settings"),
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
                for (preset, label, desc) in row_presets {
                    row.spawn((
                        Button,
                        Node {
                            flex_grow: 1.0,
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: helpers::standard_border(),
                            ..default()
                        },
                        BackgroundColor(if preset == WorldPreset::Balanced {
                            colors::PRIMARY
                        } else {
                            colors::BACKGROUND_LIGHT
                        }),
                        BorderColor(if preset == WorldPreset::Balanced {
                            colors::PRIMARY
                        } else {
                            colors::BORDER_DEFAULT
                        }),
                        BorderRadius::all(Val::Px(5.0)),
                        PresetButton(preset),
                        PresetDescription(desc.to_string()),
                        // Hover effects handled by interaction system
                    )).with_children(|button| {
                        button.spawn((
                            Text::new(label),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(if preset == WorldPreset::Balanced {
                                Color::WHITE
                            } else {
                                colors::TEXT_PRIMARY
                            }),
                        ));
                    });
                }
            });
        }
        
        // Help text
        section.spawn((
            Text::new("Presets automatically configure all settings for specific gameplay experiences."),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::TEXT_MUTED),
            Node {
                margin: UiRect::left(Val::Px(5.0)),
                ..default()
            },
        ));
    });
}

fn spawn_advanced_panel(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            display: Display::None, // Initially hidden
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            border: helpers::standard_border(),
            ..default()
        },
        BackgroundColor(colors::BACKGROUND_LIGHT),
        BorderColor(colors::BORDER_DEFAULT),
        BorderRadius::all(Val::Px(dimensions::CORNER_RADIUS)),
        AdvancedPanel,
    )).with_children(|panel| {
        // Title
        panel.spawn((
            Text::new("⚙️ Advanced Settings"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(colors::TEXT_PRIMARY),
            Node {
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
        ));
        
        // Help text for advanced settings
        panel.spawn((
            Text::new("Fine-tune world generation parameters for a customized experience."),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::TEXT_MUTED),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));
        
        // Create a two-column layout with better spacing
        panel.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(40.0),
                ..default()
            },
        )).with_children(|columns| {
            // ===== LEFT COLUMN: WORLD GEOGRAPHY =====
            columns.spawn((
                Node {
                    flex_basis: Val::Percent(50.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(dimensions::MARGIN_MEDIUM),
                    ..default()
                },
            )).with_children(|left_col| {
                // Section header with help
                left_col.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        margin: UiRect::bottom(Val::Px(10.0)),
                        border: helpers::standard_border(),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    BackgroundColor(colors::BACKGROUND_DARK),
                    BorderColor(colors::PRIMARY.with_alpha(0.3)),
                    BorderRadius::all(Val::Px(5.0)),
                )).with_children(|header| {
                    header.spawn((
                        Text::new("🌍 World Geography"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                    header.spawn((
                        Text::new("Shape the physical world: continents, oceans, and terrain."),
                        TextFont {
                            font_size: 13.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_MUTED),
                    ));
                });
                
                // Continent Count Slider
                spawn_slider_control(left_col, "Continents", "7", 1.0, 12.0, 7.0, ContinentSlider, ContinentValueText);
                
                // Ocean Coverage Slider
                spawn_slider_control(left_col, "Ocean Coverage", "60%", 30.0, 80.0, 60.0, OceanSlider, OceanValueText);
                
                // River Density Slider
                spawn_slider_control(left_col, "River Density", "1.0x", 0.5, 2.0, 1.0, RiverSlider, RiverValueText);
                
                // Climate Type Selection with help text
                left_col.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(3.0),
                        ..default()
                    },
                )).with_children(|climate_section| {
                    spawn_selection_row(
                        climate_section,
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
                    climate_section.spawn((
                        Text::new("Affects temperature, rainfall, and biome distribution."),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_MUTED),
                        Node {
                            margin: UiRect::horizontal(Val::Px(5.0)),
                            ..default()
                        },
                    ));
                });
                
                // Island Frequency Selection
                spawn_selection_row(
                    left_col,
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
            });
            
            // ===== RIGHT COLUMN: CIVILIZATIONS & RESOURCES =====
            columns.spawn((
                Node {
                    flex_basis: Val::Percent(50.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(dimensions::MARGIN_MEDIUM),
                    ..default()
                },
            )).with_children(|right_col| {
                // Section header with help
                right_col.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        margin: UiRect::bottom(Val::Px(10.0)),
                        border: helpers::standard_border(),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    BackgroundColor(colors::BACKGROUND_DARK),
                    BorderColor(colors::PRIMARY.with_alpha(0.3)),
                    BorderRadius::all(Val::Px(5.0)),
                )).with_children(|header| {
                    header.spawn((
                        Text::new("👑 Civilizations & Resources"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                    header.spawn((
                        Text::new("Configure nations, their behavior, and available resources."),
                        TextFont {
                            font_size: 13.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_MUTED),
                    ));
                });
                
                // Starting Nations Slider
                spawn_slider_control(right_col, "Starting Nations", "8", 2.0, 20.0, 8.0, StartingNationsSlider, StartingNationsValueText);
                
                // Tech Progression Speed Slider
                spawn_slider_control(right_col, "Tech Speed", "1.0x", 0.5, 2.0, 1.0, TechSpeedSlider, TechSpeedValueText);
                
                // Aggression Level Selection with help text
                right_col.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(3.0),
                        ..default()
                    },
                )).with_children(|aggression_section| {
                    spawn_selection_row(
                        aggression_section,
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
                    aggression_section.spawn((
                        Text::new("How likely nations are to declare war and expand."),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_MUTED),
                        Node {
                            margin: UiRect::horizontal(Val::Px(5.0)),
                            ..default()
                        },
                    ));
                });
                
                // Resource Abundance Selection
                spawn_selection_row(
                    right_col,
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

fn handle_preset_hover(
    interactions: Query<(&Interaction, &PresetDescription), (Changed<Interaction>, With<PresetButton>)>,
    mut description_text: Query<&mut Text, With<PresetDescriptionText>>,
) {
    for (interaction, preset_desc) in &interactions {
        if *interaction == Interaction::Hovered {
            if let Ok(mut text) = description_text.get_single_mut() {
                text.0 = preset_desc.0.clone();
            }
        }
    }
}

fn handle_text_input_changes(
    mut name_events: EventReader<TextInputSubmitEvent>,
    mut settings: ResMut<WorldGenerationSettings>,
    name_inputs: Query<&TextInputValue, With<WorldNameInput>>,
    seed_inputs: Query<&TextInputValue, (With<SeedInput>, Without<WorldNameInput>)>,
) {
    // Handle world name changes
    for event in name_events.read() {
        if let Ok(value) = name_inputs.get(event.entity) {
            settings.world_name = value.0.clone();
            println!("World name changed to: {}", settings.world_name);
        }
        if let Ok(value) = seed_inputs.get(event.entity) {
            if let Ok(seed) = value.0.parse::<u32>() {
                settings.seed = seed;
                println!("Seed changed to: {}", settings.seed);
            }
        }
    }
}

fn handle_preset_selection(
    mut interactions: Query<(&Interaction, &PresetButton, &mut BackgroundColor), Changed<Interaction>>,
    mut settings: ResMut<WorldGenerationSettings>,
) {
    for (interaction, preset_button, bg_color) in &mut interactions {
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
    for (interaction, size_button, bg_color) in &mut interactions {
        if *interaction == Interaction::Pressed {
            settings.world_size = size_button.0.clone();
            println!("Selected world size: {:?}", size_button.0);
        }
    }
}

fn handle_advanced_toggle(
    interactions: Query<&Interaction, (Changed<Interaction>, With<AdvancedToggle>)>,
    mut advanced_panel: Query<&mut Node, With<AdvancedPanel>>,
    mut toggle_button: Query<&Children, With<AdvancedToggle>>,
    mut text_query: Query<&mut Text>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            if let Ok(mut panel_style) = advanced_panel.get_single_mut() {
                let is_showing = panel_style.display == Display::Flex;
                panel_style.display = if is_showing {
                    Display::None
                } else {
                    Display::Flex
                };
                
                // Update button text
                if let Ok(children) = toggle_button.get_single() {
                    for child in children.iter() {
                        if let Ok(mut text) = text_query.get_mut(child) {
                            text.0 = if is_showing {
                                "⚙️ Show Advanced Settings".to_string()
                            } else {
                                "⚙️ Hide Advanced Settings".to_string()
                            };
                        }
                    }
                }
                
                println!("Toggled advanced settings: {}", if !is_showing { "showing" } else { "hidden" });
            }
        }
    }
}

fn update_seed_display(
    settings: Res<WorldGenerationSettings>,
    mut seed_text: Query<(&mut Text, &mut TextInputValue), With<SeedInput>>,
    mut time_estimate: Query<&mut Text, (With<GenerationTimeEstimate>, Without<SeedInput>)>,
) {
    if settings.is_changed() {
        // Update seed display
        for (mut text, mut input_value) in &mut seed_text {
            text.0 = settings.seed.to_string();
            input_value.0 = settings.seed.to_string();
        }
        
        // Update time estimate based on world size
        if let Ok(mut estimate_text) = time_estimate.get_single_mut() {
            let time_range = match settings.world_size {
                WorldSize::Small => "~1-3 seconds",
                WorldSize::Medium => "~3-5 seconds",
                WorldSize::Large => "~5-7 seconds",
            };
            estimate_text.0 = format!("⏱️ Estimated generation time: {}", time_range);
        }
    }
}

fn handle_generate_button(
    mut commands: Commands,
    interactions: Query<&Interaction, (Changed<Interaction>, With<GenerateButton>)>,
    settings: Res<WorldGenerationSettings>,
    mut state_events: EventWriter<RequestStateTransition>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            println!("Generate World button pressed");
            println!("Settings: {:?}", *settings);
            
            // Signal that we need to generate a world (with small delay for loading screen to render)
            commands.insert_resource(crate::states::PendingWorldGeneration {
                pending: true,
                delay_timer: 0.1,  // 100ms delay to render loading screen
            });
            
            // Initialize loading screen for world generation
            let mut loading_state = crate::loading_screen::LoadingState::default();
            crate::loading_screen::start_world_generation_loading(
                &mut loading_state,
                settings.seed,
                format!("{:?}", settings.world_size),
            );
            commands.insert_resource(loading_state);
            
            // Transition to loading screen first
            state_events.write(RequestStateTransition {
                from: GameState::WorldConfiguration,
                to: GameState::LoadingWorld,
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
    name_interactions: Query<&Interaction, (Changed<Interaction>, With<RandomNameButton>)>,
    seed_interactions: Query<&Interaction, (Changed<Interaction>, With<RandomSeedButton>, Without<RandomNameButton>)>,
    mut settings: ResMut<WorldGenerationSettings>,
    mut name_inputs: Query<(&mut Text, &mut TextInputValue), With<WorldNameInput>>,
    mut seed_inputs: Query<(&mut Text, &mut TextInputValue), (With<SeedInput>, Without<WorldNameInput>)>,
) {
    // Random name button
    for interaction in &name_interactions {
        if *interaction == Interaction::Pressed {
            let mut gen = NameGenerator::new();
            settings.world_name = gen.generate(NameType::World);
            for (mut text, mut input_value) in &mut name_inputs {
                text.0 = settings.world_name.clone();
                input_value.0 = settings.world_name.clone();
            }
            println!("Generated random name: {}", settings.world_name);
        }
    }
    
    // Random seed button
    for interaction in &seed_interactions {
        if *interaction == Interaction::Pressed {
            settings.seed = rand::thread_rng().gen();
            for (mut text, mut input_value) in &mut seed_inputs {
                text.0 = settings.seed.to_string();
                input_value.0 = settings.seed.to_string();
            }
            println!("Generated random seed: {}", settings.seed);
        }
    }
}

fn handle_slider_interactions(
    interactions: Query<(&Interaction, &Node, &Children), With<Button>>,
    continent_sliders: Query<&mut Node, (With<ContinentSlider>, Without<Button>)>,
    ocean_sliders: Query<&mut Node, (With<OceanSlider>, Without<Button>, Without<ContinentSlider>)>,
    river_sliders: Query<&mut Node, (With<RiverSlider>, Without<Button>, Without<ContinentSlider>, Without<OceanSlider>)>,
    settings: ResMut<WorldGenerationSettings>,
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

// World name generation has been moved to the universal name_generator module
// Use NameGenerator::new().generate(NameType::World) instead