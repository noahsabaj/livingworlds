//! World generation loading screen with real-time visualization
//! 
//! This module provides a visual loading screen that shows the world being generated
//! in real-time, allowing players to watch as continents form, oceans fill, and
//! civilizations are placed.

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;

use crate::states::{GameState, RequestStateTransition};
use crate::world_config::WorldGenerationSettings;
use crate::resources::WorldSeed;
use crate::ui::styles::{colors, dimensions};
use crate::ui::buttons::{ButtonBuilder, ButtonStyle, ButtonSize};
use crate::generation::types::MapDimensions;
use crate::constants::*;

// ============================================================================
// PLUGIN
// ============================================================================

pub struct WorldGenerationLoadingPlugin;

impl Plugin for WorldGenerationLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources for tracking generation progress
            .init_resource::<GenerationProgress>()
            .init_resource::<GenerationVisualization>()
            
            // State transitions
            .add_systems(OnEnter(GameState::WorldGenerationLoading), (
                setup_loading_screen,
                start_world_generation,
            ))
            .add_systems(OnExit(GameState::WorldGenerationLoading), cleanup_loading_screen)
            
            // Update systems
            .add_systems(Update, (
                update_generation_progress,
                update_visualization,
                update_ui,
                handle_cancel_button,
                check_generation_complete,
            ).run_if(in_state(GameState::WorldGenerationLoading)));
    }
}

// ============================================================================
// RESOURCES
// ============================================================================

/// Tracks the current progress of world generation
#[derive(Resource, Default)]
pub struct GenerationProgress {
    pub stage: GenerationStage,
    pub progress: f32, // 0.0 to 1.0
    pub message: String,
    pub is_complete: bool,
    pub is_cancelled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum GenerationStage {
    #[default]
    Initializing,
    GeneratingTectonics,
    FormingContinents,
    FillingOceans,
    CreatingRivers,
    PlacingResources,
    SpawningCivilizations,
    Finalizing,
    Complete,
}

/// Holds the visualization mesh and texture data
#[derive(Resource, Default)]
pub struct GenerationVisualization {
    pub mesh_handle: Option<Handle<Mesh>>,
    pub vertices: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
    pub province_count: usize,
}

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component)]
struct LoadingScreenRoot;

#[derive(Component)]
struct VisualizationCamera;

#[derive(Component)]
struct WorldPreviewMesh;

#[derive(Component)]
struct ProgressBar;

#[derive(Component)]
struct ProgressBarFill;

#[derive(Component)]
struct ProgressText;

#[derive(Component)]
struct StageText;

#[derive(Component)]
struct CancelButton;

// ============================================================================
// SETUP
// ============================================================================

fn setup_loading_screen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<WorldGenerationSettings>,
) {
    println!("Setting up world generation loading screen");
    
    // Create camera for visualization
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        VisualizationCamera,
    ));
    
    // Create UI root
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.05, 0.08)),
        LoadingScreenRoot,
    )).with_children(|root| {
        // Title
        root.spawn((
            Text::new(format!("Generating {}", settings.world_name)),
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
        
        // Visualization area (will show the world being generated)
        root.spawn((
            Node {
                width: Val::Px(800.0),
                height: Val::Px(500.0),
                border: UiRect::all(Val::Px(2.0)),
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
            BorderColor(colors::PRIMARY.with_alpha(0.5)),
        ));
        
        // Stage text
        root.spawn((
            Text::new("Initializing world generation..."),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(colors::TEXT_SECONDARY),
            StageText,
            Node {
                margin: UiRect::vertical(Val::Px(10.0)),
                ..default()
            },
        ));
        
        // Progress bar container
        root.spawn((
            Node {
                width: Val::Px(600.0),
                height: Val::Px(30.0),
                border: UiRect::all(Val::Px(2.0)),
                margin: UiRect::vertical(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            BorderColor(colors::PRIMARY.with_alpha(0.3)),
            BorderRadius::all(Val::Px(5.0)),
            ProgressBar,
        )).with_children(|bar| {
            // Progress fill
            bar.spawn((
                Node {
                    width: Val::Percent(0.0), // Will be updated
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(colors::PRIMARY),
                BorderRadius::all(Val::Px(3.0)),
                ProgressBarFill,
            ));
        });
        
        // Progress percentage
        root.spawn((
            Text::new("0%"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(colors::TEXT_PRIMARY),
            ProgressText,
            Node {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
        ));
        
        // Cancel button
        ButtonBuilder::new("Cancel")
            .style(ButtonStyle::Secondary)
            .size(ButtonSize::Medium)
            .with_marker(CancelButton)
            .build(root);
    });
    
    // Initialize the preview mesh
    let mesh = create_initial_preview_mesh(&settings);
    let mesh_handle = meshes.add(mesh);
    
    // Create the world preview entity
    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
        WorldPreviewMesh,
        Visibility::Hidden, // Start hidden, show when generation begins
    ));
    
    // Store mesh handle in visualization resource
    commands.insert_resource(GenerationVisualization {
        mesh_handle: Some(mesh_handle),
        ..default()
    });
}

fn create_initial_preview_mesh(settings: &WorldGenerationSettings) -> Mesh {
    // Create a LOW-RESOLUTION preview mesh for performance
    // Use only a fraction of the actual provinces to maintain smooth FPS
    let (width, height) = settings.world_size.dimensions();
    let hex_size = 10.0; // Larger hexagons for preview
    
    // Reduce resolution by factor of 10 for smooth animation
    let provinces_per_row = ((width / 10) as u32).max(60);
    let provinces_per_col = ((height / 10) as u32).max(50);
    let total_provinces = provinces_per_row * provinces_per_col;
    
    // Pre-allocate with exact capacity
    let vertices_per_hex = 7;
    let mut positions = Vec::with_capacity(total_provinces as usize * vertices_per_hex);
    let mut colors = Vec::with_capacity(total_provinces as usize * vertices_per_hex);
    let mut indices = Vec::with_capacity(total_provinces as usize * 18);
    
    // Generate placeholder hexagons
    for idx in 0..total_provinces {
        let col = idx % provinces_per_row;
        let row = idx / provinces_per_row;
        
        let (center_x, center_y) = calculate_hex_position(
            col, row, hex_size, provinces_per_row, provinces_per_col
        );
        
        // Center vertex
        positions.push([center_x, center_y, 0.0]);
        colors.push([0.1, 0.1, 0.1, 1.0]); // Dark gray initially
        
        // Hexagon vertices
        for i in 0..6 {
            let angle = (i as f32 * 60.0).to_radians();
            let x = center_x + hex_size * angle.cos();
            let y = center_y + hex_size * angle.sin();
            positions.push([x, y, 0.0]);
            colors.push([0.1, 0.1, 0.1, 1.0]);
        }
        
        // Create triangles
        let base_idx = (idx as usize * vertices_per_hex) as u32;
        for i in 0..6 {
            let next = (i + 1) % 6;
            indices.push(base_idx);
            indices.push(base_idx + 1 + i);
            indices.push(base_idx + 1 + next);
        }
    }
    
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    
    mesh
}

// ============================================================================
// GENERATION SIMULATION
// ============================================================================

fn start_world_generation(
    mut progress: ResMut<GenerationProgress>,
) {
    println!("Starting world generation process");
    progress.stage = GenerationStage::GeneratingTectonics;
    progress.progress = 0.0;
    progress.message = "Simulating continental drift...".to_string();
    progress.is_complete = false;
    progress.is_cancelled = false;
}

fn update_generation_progress(
    mut progress: ResMut<GenerationProgress>,
    time: Res<Time>,
) {
    if progress.is_complete || progress.is_cancelled {
        return;
    }
    
    // Simulate generation progress (in real implementation, this would track actual generation)
    let speed = 0.15; // Progress per second
    progress.progress += speed * time.delta_secs();
    
    // Update stage based on progress
    let stage_progress = progress.progress;
    
    let (new_stage, message) = match stage_progress {
        p if p < 0.15 => (GenerationStage::GeneratingTectonics, "Simulating continental drift..."),
        p if p < 0.30 => (GenerationStage::FormingContinents, "Raising mountain ranges..."),
        p if p < 0.45 => (GenerationStage::FillingOceans, "Filling ocean basins..."),
        p if p < 0.60 => (GenerationStage::CreatingRivers, "Carving river valleys..."),
        p if p < 0.75 => (GenerationStage::PlacingResources, "Distributing mineral deposits..."),
        p if p < 0.90 => (GenerationStage::SpawningCivilizations, "Placing civilizations..."),
        p if p < 0.98 => (GenerationStage::Finalizing, "Finalizing world..."),
        _ => {
            progress.is_complete = true;
            (GenerationStage::Complete, "World generation complete!")
        }
    };
    
    if progress.stage != new_stage {
        println!("Generation stage: {:?}", new_stage);
        progress.stage = new_stage;
        progress.message = message.to_string();
    }
    
    // Clamp progress
    progress.progress = progress.progress.min(1.0);
}

fn update_visualization(
    mut meshes: ResMut<Assets<Mesh>>,
    mut visualization: ResMut<GenerationVisualization>,
    progress: Res<GenerationProgress>,
    mut preview_query: Query<&mut Visibility, With<WorldPreviewMesh>>,
) {
    // Show the preview mesh once generation starts
    if let Ok(mut visibility) = preview_query.get_single_mut() {
        if progress.stage != GenerationStage::Initializing {
            *visibility = Visibility::Visible;
        }
    }
    
    // Update mesh colors based on generation stage
    if let Some(mesh_handle) = &visualization.mesh_handle {
        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            if let Some(VertexAttributeValues::Float32x4(colors)) = 
                mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR) 
            {
                // Update colors based on generation stage
                let color = match progress.stage {
                    GenerationStage::GeneratingTectonics => [0.3, 0.2, 0.1, 1.0], // Brown
                    GenerationStage::FormingContinents => [0.4, 0.3, 0.2, 1.0], // Light brown
                    GenerationStage::FillingOceans => [0.1, 0.2, 0.4, 1.0], // Ocean blue
                    GenerationStage::CreatingRivers => [0.2, 0.3, 0.5, 1.0], // River blue
                    GenerationStage::PlacingResources => [0.5, 0.4, 0.3, 1.0], // Resource tan
                    GenerationStage::SpawningCivilizations => [0.6, 0.5, 0.4, 1.0], // Civ colors
                    GenerationStage::Finalizing | GenerationStage::Complete => [0.3, 0.5, 0.3, 1.0], // Green
                    _ => [0.1, 0.1, 0.1, 1.0], // Dark gray
                };
                
                // Gradually update colors based on progress
                let total_colors = colors.len();
                let progress_factor = (progress.progress * total_colors as f32) as usize;
                
                for (i, c) in colors.iter_mut().enumerate() {
                    if i < progress_factor {
                        *c = color;
                    }
                }
            }
        }
    }
}

// ============================================================================
// UI UPDATES
// ============================================================================

fn update_ui(
    progress: Res<GenerationProgress>,
    mut fill_query: Query<&mut Node, With<ProgressBarFill>>,
    mut progress_text: Query<&mut Text, (With<ProgressText>, Without<StageText>)>,
    mut stage_text: Query<&mut Text, (With<StageText>, Without<ProgressText>)>,
) {
    // Update progress bar fill
    if let Ok(mut fill) = fill_query.get_single_mut() {
        fill.width = Val::Percent(progress.progress * 100.0);
    }
    
    // Update progress text
    if let Ok(mut text) = progress_text.get_single_mut() {
        **text = format!("{:.0}%", progress.progress * 100.0);
    }
    
    // Update stage text
    if let Ok(mut text) = stage_text.get_single_mut() {
        **text = progress.message.clone();
    }
}

// ============================================================================
// INTERACTIONS
// ============================================================================

fn handle_cancel_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<CancelButton>)>,
    mut progress: ResMut<GenerationProgress>,
    mut state_events: EventWriter<RequestStateTransition>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            println!("Cancelling world generation");
            progress.is_cancelled = true;
            
            // Return to configuration screen with settings preserved
            state_events.write(RequestStateTransition {
                from: GameState::WorldGenerationLoading,
                to: GameState::WorldConfiguration,
            });
        }
    }
}

fn check_generation_complete(
    progress: Res<GenerationProgress>,
    mut state_events: EventWriter<RequestStateTransition>,
) {
    if progress.is_complete && !progress.is_cancelled {
        println!("World generation visualization complete, transitioning to actual generation");
        
        // Transition to actual world generation
        state_events.write(RequestStateTransition {
            from: GameState::WorldGenerationLoading,
            to: GameState::WorldGeneration,
        });
    }
}

// ============================================================================
// CLEANUP
// ============================================================================

fn cleanup_loading_screen(
    mut commands: Commands,
    query: Query<Entity, Or<(With<LoadingScreenRoot>, With<VisualizationCamera>, With<WorldPreviewMesh>)>>,
    mut progress: ResMut<GenerationProgress>,
    mut visualization: ResMut<GenerationVisualization>,
) {
    println!("Cleaning up world generation loading screen");
    
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    
    // Reset resources
    *progress = GenerationProgress::default();
    *visualization = GenerationVisualization::default();
}