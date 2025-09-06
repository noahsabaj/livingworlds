# Living Worlds - Bevy-Powered Civilization Simulator

## IMPORTANT: Date and Version Guidelines
- **ALWAYS use `date` command for current date** - We are in September 2025, NOT 2024
- **Project created**: August 25, 2025 (approximately)
- **Bevy version in use**: 0.16.1 (check Cargo.toml)
- **Claude's training**: Only up to Bevy 0.14 (July 2024)
- When referencing Bevy features, check the actual codebase rather than assuming

## Use the WebSearch tool as frequently as possible to obtain information about anything
- Keep dates out of your search params (DO NOT DO example: wgpu best pracices 2024) (good example: wgpu best practices)
- Search for version-specific information (check what version we have from Cargo.toml, research that) (good example: bevy v0.16.1 entity-entity connections relations graph)
- Search for implementation/usage examples to get an idea on how to use a tool, as the Internet tends to have very valuable information

## CRITICAL: Evidence-Based Documentation
- **ALWAYS collect concrete evidence instead of relying on memory** when updating documentation
- **NEVER trust memory alone** - it leads to documentation drift and "telephone game" effects
- **For project structure**: Use `ls`, `find`, or other filesystem commands to verify actual structure
- **For code references**: Use `Read`, `Grep`, or `Glob` to check actual implementation
- **For dependencies**: Check `Cargo.toml` and `Cargo.lock` for actual versions and features
- **Why this matters**: Hallucinated or memory-based updates compound over time, creating increasingly inaccurate documentation
- **Example**: The project structure section had 8 crates listed when only 4 actually exist (verified via `ls /home/nsabaj/Code/livingworlds/crates/`)

## When working on ANYTHING, always be mindful of what has been implemented/wrote already, what you are implementing/writing now, and what will be implemented/written in the future

## Executive Summary

**Title**: Living Worlds  
**Genre**: Perpetual Civilization Simulator / God Game  
**Engine**: Bevy 0.16 - Modern Rust Game Engine  
**Platform**: Windows, Linux (no Web whatsoever, this game is being sold on Steam)
**Distribution Platform**: Steam only
**Team**: Solo Developer with Claude Code assistance  
**Core Hook**: Fully procedural civilization simulator where empires rise and fall eternally  

## Vision Statement

Living Worlds is a perpetual civilization simulator built on the powerful Bevy game engine. Watch as empires develop organically through emergent gameplay - no two games are alike. Every texture, sound, and piece of text is procedurally generated at runtime. Civilizations advance through technologies at their own pace, build infrastructure that permanently marks the landscape, and manage complex economies based on Austrian economic principles. There is no victory condition - only the eternal cycle of rise and fall.

**Visual Style**: 2D orthographic province-based map (like Europa Universalis/Crusader Kings) with colored political boundaries, terrain overlays, and grid lines. Each province is a polygon mesh with dynamic coloring based on nation ownership.

## Why Bevy?

Bevy gives us everything we need for Living Worlds:

### 🚀 **Performance**
- **Parallel ECS**: Massively parallel and cache-friendly - perfect for simulating thousands of entities
- **Fast Compile Times**: 0.8-3.0 seconds for iterative development
- **Multi-threaded Systems**: Automatic parallelization of game systems

### 🎮 **Features We Use**
- **2D/3D Rendering**: Full rendering pipeline with lights, shadows, cameras
- **Bevy UI**: Native ECS-driven UI system for game interfaces
- **Hot Reloading**: Instant feedback on changes without restarts
- **Animation System**: Smooth transitions and visual effects
- **Audio System**: Procedural audio generation and playback
- **Scene System**: Save/load game states seamlessly
- **Transform Hierarchy**: Parent-child relationships for organized entities

### 🛠️ **Developer Experience**
- **Simple API**: Components are structs, systems are functions
- **Great Documentation**: Extensive examples and community support
- **Cross-Platform**: Windows, Linux, distributed on Steam
- **Asset Pipeline**: Even though we generate everything, the pipeline helps during development

## Technical Architecture

### Core Technology Stack

Engine: Bevy 0.16.1
Language: Rust 2021 Edition
Graphics: wgpu (via Bevy) - Modern GPU API
Audio: bevy_audio with procedural generation
Math: Fixed-point for deterministic simulation
UI: Bevy UI + egui for developer tools
Physics: Not needed - we handle our own spatial logic
Save System: Bevy Scenes + bincode serialization

Platform Support:
- Native: Windows, Linux, MacOS
- Future: iOS, Android (Bevy supports them!)
- Sold exclusively on Steam

### Bevy App Structure

```rust
fn main() {
    App::new()
        // Core Bevy plugins
        .add_plugins(DefaultPlugins)
        
        // Our game plugins
        .add_plugins((
            ProceduralGenerationPlugin,
            SimulationPlugin,
            DiplomacyPlugin,
            EconomyPlugin,
            MilitaryPlugin,
            UIPlugin,
        ))
        
        // Game states
        .add_state::<GameState>()
        
        // Resources
        .insert_resource(WorldSettings::default())
        .insert_resource(SimulationSpeed::Normal)
        
        // Startup systems
        .add_systems(Startup, (
            setup_camera,
            generate_world,
            spawn_initial_nations,
        ))
        
        // Game loop systems
        .add_systems(Update, (
            // Simulation systems run in parallel automatically!
            population_growth_system,
            economic_production_system,
            technology_advancement_system,
            military_movement_system,
            diplomatic_relations_system,
        ).run_if(in_state(GameState::Playing)))
        
        .run();
}
```

## Game Systems Design

### Entity Component System (ECS) Architecture

Living Worlds fully embraces Bevy's ECS pattern:

```rust
// Components are simple structs
#[derive(Component)]
struct Nation {
    name: String,
    government: GovernmentType,
    treasury: Fixed32,
    stability: Fixed32,
}

#[derive(Component)]
struct Province {
    terrain: TerrainType,
    population: u32,
    development: Fixed32,
}

// Systems are functions that query components
fn economic_system(
    time: Res<Time>,
    mut nations: Query<(&mut Nation, &Economy)>,
    provinces: Query<&Province, With<OwnedBy>>,
) {
    for (mut nation, economy) in &mut nations {
        // Bevy automatically parallelizes this!
        let income = calculate_income(&provinces, economy);
        nation.treasury += income * time.delta_seconds();
    }
}
```

### Procedural Generation with Bevy

Everything is generated at runtime using Bevy's powerful systems:

```rust
fn generate_terrain_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<WorldSettings>,
) {
    // Generate heightmap
    let heightmap = generate_heightmap(settings.seed, settings.size);
    
    // Create terrain mesh
    let mesh = create_terrain_mesh(&heightmap);
    
    // Spawn terrain entity
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.3, 0.5, 0.3))),
            ..default()
        },
        Terrain,
        Name::new("World Terrain"),
    ));
}
```

### Bevy Rendering Pipeline

We leverage Bevy's rendering for beautiful visuals:

```rust
// Custom shaders for procedural textures
let terrain_material = materials.add(TerrainMaterial {
    base_color: Color::rgb(0.3, 0.5, 0.3),
    height_texture: heightmap_handle,
    noise_scale: 10.0,
});

// Camera with smooth controls
commands.spawn((
    Camera2dBundle::default(),
    PanOrbitCamera::default(),
    CameraController,
));

// UI overlays with Bevy UI
commands.spawn((
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ..default()
    },
    NationInfoPanel,
));
```

## Development Workflow

### Bevy's Hot Reloading

One of Bevy's killer features for rapid development:

```bash
# Run with hot reloading
cargo run --features bevy/dynamic_linking

# Now edit assets, scenes, or even shaders - changes appear instantly!
```

### Testing with Bevy

```rust
#[cfg(test)]
mod tests {
    use bevy::app::App;
    
    #[test]
    fn test_economic_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_systems(Update, economic_system);
           
        // Add test entities
        app.world.spawn((Nation::default(), Economy::default()));
        
        // Run one frame
        app.update();
        
        // Assert results
        let nation = app.world.query::<&Nation>().single(&app.world);
        assert!(nation.treasury > Fixed32::ZERO);
    }
}
```

## Performance & Optimization

### Bevy's Automatic Optimizations

Bevy handles many optimizations for us:
- **Parallel Systems**: Systems automatically run in parallel when possible
- **Change Detection**: Only process entities that changed
- **Frustum Culling**: Don't render off-screen entities
- **Batching**: Automatic draw call batching for sprites
- **Asset Loading**: Async asset loading with progress tracking

### Our Optimizations

- **Spatial Hashing**: For efficient neighbor queries
- **LOD System**: Less detail for distant provinces
- **Pooling**: Reuse common entities like armies
- **Fixed-point Math**: Deterministic simulation across platforms

## Bevy 0.16.1 Feature Reference

### Core Engine Features We're Using

#### 🚀 GPU-Driven Rendering (NEW in 0.16!)
- **3x Performance**: On complex scenes like Call of Duty's Caldera hotel, Bevy 0.16 performs ~3x better than 0.15
- **Automatic Instancing**: Bevy automatically batches and instances similar meshes
- **GPU Culling**: Frustum and occlusion culling happens on the GPU
- **Supports Skinned Meshes**: GPU-driven rendering works with animated characters
- **Future-Proof**: Ready for Vulkan device generated commands and DirectX 12 work graphs
- **Automatic Enablement**: Enabled by default on supported platforms, no configuration needed
- **Note**: Morph targets still use CPU rendering, but don't block GPU rendering for other objects

#### 🏗️ Occlusion Culling (NEW in 0.16!)
Two-phase occlusion culling for massive performance gains:
```rust
// Enable occlusion culling on camera
commands.spawn((
    Camera3dBundle::default(),
    DepthPrepass,      // Required for occlusion culling
    OcclusionCulling,  // Enables the feature
));
```
- Works with GPU-driven rendering architecture
- Experimental but highly effective for complex scenes
- Some precision issues may mark visible meshes as occluded (rare)

#### 🌍 Entity Relationships (NEW in 0.16!)
Perfect for Living Worlds' complex entity connections:
```rust
// Define custom relationships with automatic cleanup
#[derive(Component)]
#[relationship(relationship_target = ProvinceBuildings)]
struct BuildingInProvince(Entity);

#[derive(Component)]
#[relationship_target(relationship = BuildingInProvince, linked_spawn)]
struct ProvinceBuildings(Vec<Entity>);

// Nations own provinces, provinces contain cities, cities have populations
app.register_relationship::<Owns>();
app.register_relationship::<Contains>();
app.register_relationship::<BelongsTo>();
```
- Bidirectional relationships with automatic consistency
- `linked_spawn` automatically despawns related entities
- Graph primitives built into the ECS core
- Replaces old Parent/Child with ChildOf/Children

#### ☁️ Procedural Atmospheric Scattering (NEW in 0.16!)
- Physically-based Earth-like sky simulation
- Perfect for our day/night cycle
- Based on Sebastien Hillaire's 2020 paper
- Low performance cost for stunning visuals
- Scales dynamically with hardware capabilities

#### 🎨 Visual Effects Arsenal
- **Decals** (0.16): Layer textures onto terrain dynamically, adapting to geometry
- **Chromatic Aberration** (0.15): Lens distortion effects for impact/atmosphere
- **Virtual Geometry/Meshlets** (0.14): Nanite-like LOD system
- **Screen Space Reflections** (0.14): Real-time water reflections
- **Volumetric Fog** (0.14-0.15): God rays and atmospheric effects
  - 0.15 adds point/spot light support with volumetric fog
  - Fog volumes for localized fog effects
- **Depth of Field** (0.14): Hexagonal bokeh and Gaussian blur options
- **Motion Blur** (0.14): Camera-relative blur for smooth movement
- **Auto Exposure** (0.14): Dynamic HDR-like brightness adaptation

#### 🎭 Animation System (Enhanced in 0.15)
Complete overhaul with generalized entity animation:
```rust
// Animate any component property
let curve = AnimatableCurve::new(
    animated_field!(Province::population),
    EasingCurve::new(1000.0, 50000.0, EaseFunction::ExponentialOut)
);

// Animation events trigger game logic
animation_clip.add_event(1.5, BattleCompleteEvent);
```
- **Generalized Entity Animation**: Animate any component property
- **Animation Masks**: Partial animation blending
- **Animation Events**: Trigger game logic from animations
- **Additive Blending**: Layer multiple animations
- **Keyframe Traits**: Extensible animation system
- **Curve-based Interpolation**: Smooth transitions

#### 📊 Curves & Interpolation (NEW in 0.15)
```rust
// Perfect for economy/population growth curves
let growth_curve = EasingCurve::new(
    0.0,
    1000.0, 
    EaseFunction::ExponentialOut
);

// Cyclic splines for repeating patterns
let seasonal_curve = CyclicCubicGenerator::new(
    vec![0.0, 0.5, 1.0, 0.5],  // Winter, Spring, Summer, Fall
).to_curve();

// Color gradients for terrain
let terrain_gradient = ColorGradient::new(vec![
    (0.0, Color::BLUE),      // Ocean
    (0.3, Color::YELLOW),    // Beach
    (0.5, Color::GREEN),     // Plains
    (1.0, Color::WHITE),     // Mountains
]);
```

#### 🎯 Entity Picking System (NEW in 0.15)
Battle-tested bevy_mod_picking integrated into core:
```rust
// Modular picking across 2D/3D/UI
commands.spawn((
    Province::new(id),
    PickableBundle::default(),  // Makes entity pickable
    On::<Pointer<Click>>::run(handle_province_click),
));
```
- Modular selection across 2D/3D/UI
- Perfect for selecting provinces and units
- Ray casting and area selection
- Used by Foresight Spatial Labs since 2020
- Supports hover, click, drag events

#### 🖲️ UI Improvements (0.15)
Scrolling support for UI containers:
```rust
commands.spawn((
    NodeBundle {
        style: Style {
            overflow: Overflow::scroll(),
            ..default()
        },
        ..default()
    },
    ScrollPosition { 
        offset_x: 0.0,
        offset_y: 0.0,
    },
));
```
- Native scrolling for UI containers
- Mouse wheel support
- Touch scrolling for mobile

#### ⚙️ Required Components (NEW in 0.15)
Cleaner entity spawning with automatic dependencies:
```rust
// Define required components
#[derive(Component)]
#[require(Position, Velocity)]  // Automatically adds these
struct Army {
    size: u32,
}

// Spawn without boilerplate
commands.spawn(Army { size: 1000 }); // Position & Velocity added automatically!
```
- Reduces boilerplate significantly
- Ensures entities always have required components
- Can't remove required components from external types

#### 🔧 no_std Support (NEW in 0.16)
- Run on embedded systems
- Smaller binary size for embedded systems
- Reduced binary size potential
- More control over allocations

### Performance Features

#### Rendering Optimizations
- **Mesh Slabbing** (0.15): 2x CPU speedup by coalescing vertex/index buffers
  - Benchmark results: 8.74ms → 5.53ms frame time (1.58x overall speedup)
  - Render system: 6.57ms → 3.54ms (1.86x speedup)
  - Opaque pass: 4.64ms → 2.33ms (1.99x speedup)
  - Configurable via `MeshAllocatorSettings` resource
  - Note: Full vertex/index buffer combining on all native platforms
- **GPU-Driven Rendering** (0.16): 3x performance on complex scenes
- **Improved Batching** (0.13): Fewer draw calls in large scenes
- **Transform Propagation** (0.16): Optimized for static objects
- **Indirect Drawing**: Now default, disable with `NoIndirectDrawing` component

#### Parallel Processing
- **Automatic System Parallelization**: Bevy schedules systems optimally
- **Multi-threaded Rendering**: Render and game logic on separate threads
- **Parallel Queries**: Iterate entities across CPU cores
- **Change Detection**: Only process entities that changed

### Developer Experience Improvements

#### 🛠️ Better Error Handling (0.16)
```rust
// Unified error handling across Bevy
asset_server.load::<Image>("texture.png")
    .inspect_err(|e| error!("Failed to load: {}", e));

// BRP query error handling
// Now skips invalid components by default instead of erroring
let query_params = BrpQueryParams {
    strict: false,  // Set true for old behavior
    ..default()
};
```

#### 📝 Function Reflection (0.15)
Advanced reflection capabilities:
```rust
// Function overloading support
#[reflect(overload)]
fn process_value(val: i32) -> i32 { val * 2 }
fn process_value(val: f32) -> f32 { val * 2.0 }

// Generic and variadic functions
#[reflect(generic)]
fn transform<T: Component>(entity: Entity, component: T) { }
```
- Inspect and call functions dynamically
- Function overloading (generic & variadic)
- Perfect for modding support
- Remote debugging capabilities

#### 🌐 Bevy Remote Protocol (0.15)
External tool integration:
```rust
// Enable BRP in your app
app.add_plugins(RemotePlugin {
    port: 15702,
});

// Custom methods for Living Worlds
remote_methods.register_method(
    "livingworlds/inspect_province",
    inspect_province_handler,
);
```
- External editor connectivity
- Live game state inspection
- Custom method registration
- Transport-agnostic protocol
- Request/response style like HTTP
- Built-in ECS query methods
- Planned for upcoming Bevy Editor
- `bevy/query` now skips missing components by default

### Features for Living Worlds

#### For World Generation
- **Procedural Meshes**: Generate terrain at runtime with GPU efficiency
- **Curves**: Population/economic growth curves with easing functions
- **Noise Functions**: Built-in Perlin/Simplex via community crates
- **Atmospheric Scattering**: Procedural sky without texture assets

#### For Simulation
- **Entity Relationships**: Nation→Province→City hierarchies with automatic cleanup
- **Change Detection**: Only update changed entities
- **Event System**: Diplomatic events, battles, trade with animation triggers
- **Required Components**: Ensure all entities have needed data
- **Parallel Systems**: Thousands of entities processed concurrently

#### For Rendering
- **GPU-Driven Rendering**: Handle thousands of provinces (3x performance)
- **Mesh Slabbing**: 2x CPU performance for entity rendering
- **Atmospheric Scattering**: Beautiful procedural skies
- **Instanced Rendering**: Efficient army/city rendering
- **LOD via Meshlets**: Detail levels for zoom
- **Occlusion Culling**: Don't render hidden provinces
- **Volumetric Fog**: Atmospheric god rays for battles

#### For UI
- **Picking System**: Battle-tested province/unit selection
- **UI Scrolling**: Native scrolling for long lists
- **Bevy UI**: Native ECS-driven UI
- **Immediate Mode Option**: egui integration available
- **Event Bubbling**: UI events propagate properly

#### For Development
- **BRP Integration**: External tools can inspect game state
- **Function Reflection**: Runtime inspection and modding
- **Hot Reloading**: Instant feedback during development
- **Better Errors**: Clearer error messages throughout

### Critical Bevy 0.16 Patterns (Lessons Learned)

#### State Management
**Correct imports for Bevy 0.16:**
```rust
use bevy::state::state::{States, NextState, State};
```
**NOT** from `bevy::ecs::schedule` (that was pre-0.16)!

**Proper state enum definition:**
```rust
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]  // Must mark one variant as default
    MainMenu,
    Playing,
    Paused,
}
```

#### Component vs Resource Distinction
**Critical pattern that causes many errors:**
- **Component**: Attaches to entities → needs `#[derive(Component)]`
- **Resource**: Global/singleton → needs `#[derive(Resource)]`

```rust
// WRONG - causes "is not a Component" error
#[derive(Clone, Debug)]
pub struct SupplyChain { ... }

// CORRECT
#[derive(Component, Clone, Debug)]
pub struct SupplyChain { ... }

// For global state
#[derive(Resource)]
pub struct SimulationState { ... }
```

**Common compilation errors from missing derives:**
- `the trait bound X is not a Component` → Add `#[derive(Component)]`
- `the trait bound X is not a Resource` → Add `#[derive(Resource)]`
- `cannot find trait States` → Import from `bevy::state::state`

#### Keyboard Input Changes (0.15→0.16)
**BREAKING CHANGE: Input renamed to ButtonInput**

In Bevy 0.16, the `Input` type was renamed to `ButtonInput` for clarity:

```rust
// WRONG - Bevy 0.15 and earlier
pub fn input_system(keyboard: Res<Input<KeyCode>>) { ... }

// CORRECT - Bevy 0.16+
pub fn input_system(keyboard: Res<ButtonInput<KeyCode>>) { ... }
```

**Why this change?**
- `ButtonInput` better describes what it does (tracks button states)
- Distinguishes from other input types (mouse position, gamepad axes, etc.)
- Part of Bevy's API clarity improvements

**Common patterns:**
```rust
// Check if key is currently held
if keyboard.pressed(KeyCode::ShiftLeft) { ... }

// Check if key was just pressed this frame
if keyboard.just_pressed(KeyCode::Space) { ... }

// Check if key was just released this frame
if keyboard.just_released(KeyCode::Escape) { ... }
```

**Error indicator:**
- `cannot find type Input in this scope` → Change to `ButtonInput`

### Migration Benefits from 0.12→0.16.1

1. **5-6x Combined Performance**: GPU-driven (3x) + Mesh slabbing (2x) = massive speedup
2. **Entity Relationships**: Cleaner code for nation/province/army connections
3. **Better Spawning**: Required components reduce boilerplate by 30-40%
4. **Visual Polish**: Atmospheric scattering, volumetric fog, SSR, chromatic aberration
5. **Developer Tools**: BRP for external editors, better errors, function reflection
6. **Picking System**: Production-ready selection system from bevy_mod_picking
7. **Animation Power**: Animate any component with curves and events
8. **Future Ready**: no_std, GPU work graphs, device commands

## Project Structure

```
livingworlds/
├── Cargo.toml           # Workspace configuration
├── Cargo.lock           # Dependency lock file
├── CLAUDE.md            # Project documentation and AI instructions
├── .gitignore           # Git ignore configuration
├── .claude/             # Claude Code configuration
├── .xmake/              # XMake build system (alternative to cargo)
├── src/
│   └── main.rs          # Bevy app entry point
├── crates/              # Workspace members (4 crates total)
│   ├── lw_core/         # Fixed-point math, core types, deterministic simulation
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs       # Module exports
│   │       ├── constants.rs # Game constants
│   │       ├── error.rs     # Error types
│   │       ├── fixed.rs     # Fixed32 implementation
│   │       ├── random.rs    # Deterministic RNG
│   │       └── vector.rs    # Vec2fx implementation
│   ├── lw_game/         # All game logic (ECS components & systems)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs       # Plugin definition (LivingWorldsPlugin)
│   │       ├── types.rs     # Shared types (TimeState, etc.)
│   │       ├── components/  # Domain-driven component modules
│   │       │   ├── mod.rs
│   │       │   ├── ai.rs           # AI components
│   │       │   ├── diplomacy.rs    # Diplomatic relations
│   │       │   ├── individual.rs   # Individual entities
│   │       │   ├── military.rs     # Military units & armies
│   │       │   ├── simulation.rs   # SimulationState resource
│   │       │   ├── common/         # Shared component utilities
│   │       │   │   ├── mod.rs
│   │       │   │   └── bounded_types.rs  # Percentage, UnitInterval
│   │       │   ├── culture/        # Cultural systems
│   │       │   │   ├── mod.rs
│   │       │   │   ├── core.rs     # Cultural beliefs
│   │       │   │   └── contact.rs  # Culture transmission
│   │       │   ├── economics/      # Economic components
│   │       │   │   ├── mod.rs
│   │       │   │   ├── banking.rs      # Banking systems
│   │       │   │   ├── credit.rs       # Credit & loans
│   │       │   │   ├── goods.rs        # GoodType definition
│   │       │   │   ├── markets.rs      # Market structures
│   │       │   │   ├── money.rs        # Monetary systems
│   │       │   │   ├── production.rs   # Production & industry
│   │       │   │   ├── trade.rs        # Trade routes
│   │       │   │   └── transactions.rs # Market transactions
│   │       │   ├── geography/      # Geographic components
│   │       │   │   ├── mod.rs
│   │       │   │   ├── climate.rs      # Climate & weather
│   │       │   │   ├── province.rs     # Province (id, position, owner)
│   │       │   │   ├── resources.rs    # Natural resources
│   │       │   │   ├── templates.rs    # Province creation helpers
│   │       │   │   ├── terrain.rs      # Terrain components
│   │       │   │   └── water.rs        # Water bodies & harbors
│   │       │   ├── governance/     # Government components
│   │       │   │   ├── mod.rs
│   │       │   │   ├── policy_incentives.rs  # Government policies
│   │       │   │   └── supranational.rs      # EU/UN-like entities
│   │       │   └── infrastructure/ # Built infrastructure
│   │       │       ├── mod.rs
│   │       │       ├── defense.rs      # Fortifications
│   │       │       ├── transport.rs    # Roads, rails, ports
│   │       │       ├── urban.rs        # Cities & settlements
│   │       │       └── utilities.rs    # Power, water, sewage
│   │       └── systems/       # Game systems (logic)
│   │           ├── mod.rs
│   │           ├── collapse.rs         # Nation collapse
│   │           ├── diplomacy.rs        # Diplomatic actions
│   │           ├── economy.rs          # Economic system
│   │           ├── event.rs            # Event handling
│   │           ├── geography_systems.rs # Province logic
│   │           ├── market_emergence.rs # Price discovery
│   │           ├── technology.rs       # Tech advancement
│   │           ├── time.rs             # Time management
│   │           ├── core/               # Core coordination
│   │           │   └── mod.rs
│   │           ├── culture/            # Cultural systems
│   │           │   └── mod.rs
│   │           ├── economics/          # Economic systems
│   │           │   └── mod.rs
│   │           ├── geography/          # Geographic systems
│   │           │   └── mod.rs
│   │           ├── governance/         # Government systems
│   │           │   └── mod.rs
│   │           ├── individual/         # Individual systems
│   │           │   └── mod.rs
│   │           ├── military/           # Military systems
│   │           │   └── mod.rs
│   │           └── simulation_phases/  # Phase-based simulation
│   │               ├── mod.rs
│   │               ├── cultural_transmission.rs
│   │               ├── demographic_transition.rs
│   │               ├── diplomatic_evolution.rs
│   │               ├── economic_emergence.rs
│   │               ├── government_response.rs
│   │               ├── individual_decisions.rs
│   │               ├── military_actions.rs
│   │               ├── synchronization.rs
│   │               └── world_changes.rs
│   ├── lw_procedural/   # Procedural generation algorithms
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs       # Module exports
│   │       ├── audio.rs     # Procedural audio generation
│   │       ├── font.rs      # Procedural font generation
│   │       ├── names.rs     # Name generation algorithms
│   │       ├── palette.rs   # Color palette generation
│   │       ├── provinces.rs # Voronoi province generation
│   │       └── terrain.rs   # Terrain generation
│   └── lw_platform/     # Platform-specific code (Steam integration)
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs       # Steam API integration
├── data/                # Game data files
│   └── content/         # Content directory
├── tests/               # Integration tests (empty - to be populated)
├── benches/             # Performance benchmarks (empty - to be populated)
└── target/              # Build output (git-ignored)

NOTE: No assets/ directory - everything is procedurally generated
NOTE: No wasm/ or web/ directories - Steam distribution only
```

### Crate Descriptions

#### `lw_core` - Deterministic Simulation Foundation
**Purpose**: Provides the mathematical and utility foundation for deterministic simulation.
- **Fixed32**: Fixed-point arithmetic for cross-platform determinism (no floating-point errors)
- **Vec2fx**: 2D vector operations using Fixed32
- **DeterministicRNG**: Seedable random number generator for reproducible worlds
- **Constants**: Game-wide constants and configuration values
- **Error Types**: Common error handling across the project

#### `lw_game` - Game Logic & ECS Architecture
**Purpose**: Contains all game logic following Bevy's ECS (Entity Component System) pattern.
- **Components**: Pure data structures organized by domain (economics, military, culture, etc.)
  - Each component is data-only - NO business logic methods
  - Uses bounded types (Percentage) for constrained values
  - Organized into domain modules matching real-world concepts
- **Systems**: Pure functions that operate on components
  - All game logic lives here, not in components
  - Organized into domain directories matching component structure
  - Simulation phases orchestrate system execution order
- **Types**: Shared type definitions used across components and systems

#### `lw_procedural` - Runtime Generation Algorithms
**Purpose**: Generates all game content procedurally at runtime - no pre-made assets.
- **Province Generation**: Voronoi-based territory creation with Lloyd relaxation
- **Terrain Generation**: Heightmap and biome generation using noise functions
- **Name Generation**: Culture-specific naming algorithms for nations/cities/people
- **Audio Generation**: Procedural music and sound effects
- **Font Generation**: Runtime font creation (no font files shipped)
- **Color Palettes**: Dynamic color scheme generation for nations

#### `lw_platform` - Platform Integration Layer
**Purpose**: Handles platform-specific code, initially focused on Steam integration.
- **Steam API**: Achievements, cloud saves, multiplayer lobby
- **Platform Detection**: OS-specific optimizations
- **Future**: Could expand to other platforms (GOG, Epic, console)

## Current Status & Roadmap

### ✅ Completed
- Core math library with fixed-point
- ECS components for all game entities
- Procedural generation systems
- Basic game systems (military, economy, diplomacy)
- Full Bevy framework migration
- MAJOR REFACTORING: Removed 500+ component methods, fixed 578-line god object
- Component/System separation following pure ECS patterns
- Type consolidation (removed duplicate ResourceType, consolidated PolicyType, etc.)
- Bounded types implementation (Percentage for 0-1 values)

### 🚧 In Progress
- Organizing systems into domain directories
- Fixing compilation errors from refactoring
- Implementing Bevy rendering pipeline
- Setting up Bevy UI

### 📋 Upcoming
- Save/load with Bevy Scenes
- Multiplayer foundation (Bevy has networking plugins!)
- Steam integration via bevy_steamworks

## Building & Running

### Development
```bash
# Fast iterative development with dynamic linking
cargo run --features bevy/dynamic_linking

# Run with hot reloading
cargo run --features bevy/file_watcher

# Run tests
cargo test

# Check specific crate
cargo check -p lw_game
```

### Release Build
```bash
# Optimized native build
cargo build --release

# Bundle for distribution
cargo bundle --release
```

## Austrian Economics in Bevy

Our economic model leverages Bevy's ECS perfectly:

```rust
#[derive(Component)]
struct Individual {
    needs: Vec<Need>,
    skills: Vec<Skill>,
    preferences: Preferences,
}

fn austrian_economy_system(
    individuals: Query<(&Individual, &mut Labor)>,
    mut market_orders: EventWriter<MarketOrder>,
    time: Res<Time>,
) {
    // Every individual makes decisions
    for (individual, labor) in &individuals {
        let order = create_market_order(&individual.skills, &labor);
        market_orders.send(order);
    }
}

// Separate system for price discovery
fn price_discovery_system(
    orders: EventReader<MarketOrder>,
    mut transactions: EventWriter<CompletedTransaction>,
) {
    // Prices emerge from order matching, not component methods
}
```

## Why This Architecture Works

1. **Bevy's ECS is Perfect for Simulation**: Thousands of entities with complex interactions
2. **Built-in Parallelization**: Free performance gains from Bevy's scheduler
3. **Hot Reloading**: Rapid iteration on game balance and features
4. **Cross-Platform**: Same code runs everywhere Bevy supports
5. **Great Ecosystem**: Tons of Bevy plugins we can leverage
6. **Modern Rust**: Safe, fast, and expressive

## Development Philosophy

- **Embrace Bevy Patterns**: Use the framework as intended
- **Iterate Quickly**: Leverage hot reloading and fast compiles
- **Test Everything**: Bevy makes testing easy
- **Profile Early**: Use Bevy's built-in diagnostics
- **Keep It Simple**: Bevy handles the complex stuff

## Community & Resources

- [Bevy Discord](https://discord.gg/bevy) - Active community for help
- [Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples) - Learn from official examples
- [Bevy Assets](https://bevyengine.org/assets/) - Plugins and extensions
- [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/) - Comprehensive guide

## Final Notes

Living Worlds is now a modern Bevy-powered game that leverages the full capabilities of this amazing engine. We're not fighting the framework - we're embracing it. The result is cleaner code, better performance, and faster development.

Remember: Bevy is doing the heavy lifting. We just need to focus on making a great game!