# Living Worlds - Bevy-Powered Civilization OBSERVER Simulator

## ⚠️ CRITICAL: This is an OBSERVER game like Fantasy Map Simulator
**You WATCH civilizations, you DO NOT control them. Zero player interaction with the simulation.**

## NOTE: ALWAYS read files before editing them, or else it will error out

## 🚫 CRITICAL ARCHITECTURAL RULES - MODULAR ARCHITECTURE ACHIEVED

### Current Development Philosophy
- **MODULAR PLUGIN ARCHITECTURE** - Successfully refactored from prototype to 14 clean modules
- **BEVY PLUGIN SYSTEM** - Each major system is a self-contained Bevy Plugin
- **SINGLE RESPONSIBILITY** - Each module has a clear, focused purpose (simulation.rs, overlay.rs, etc.)
- **ITERATIVE REFINEMENT** - Continue improving module boundaries as patterns emerge
- **EVIDENCE-BASED UPDATES** - Always verify with actual code before updating documentation

### Where Things Currently Live (15 Modules, 192KB Total, 4571 lines)
- **World Generation**: `src/setup.rs` (28KB, 605 lines) - World generation, rivers, deltas, agriculture zones
- **Cloud System**: `src/clouds.rs` (28KB, 617 lines) - Procedural cloud generation and animation
- **Minerals**: `src/minerals.rs` (24KB, 611 lines) - Mineral resources, extraction, technology
- **Resources**: `src/resources.rs` (16KB, 369 lines) - WorldSeed, GameTime, WorldTension, SpatialIndex
- **Terrain**: `src/terrain.rs` (12KB, 321 lines) - Terrain types including new Delta, climate zones
- **Music**: `src/music.rs` (12KB, 332 lines) - Procedural music with FunDSP, tension-based
- **Entry Point**: `src/main.rs` (12KB, 321 lines) - Binary launcher, input handling, UI updates
- **Constants**: `src/constants.rs` (12KB, 225 lines) - All game parameters, hexagon calculations
- **Simulation**: `src/simulation.rs` (8KB, 219 lines) - Population growth with river/agriculture bonuses
- **Colors**: `src/colors.rs` (8KB, 198 lines) - All terrain and mineral color functions (NEW)
- **UI**: `src/ui.rs` (8KB, 230 lines) - User interface, HUD, agriculture display
- **Components**: `src/components.rs` (8KB, 155 lines) - Province with agriculture/water fields
- **Camera**: `src/camera.rs` (8KB, 167 lines) - Pan, zoom, edge scrolling systems
- **Overlay**: `src/overlay.rs` (4KB, 94 lines) - Map overlay rendering for all visualization modes
- **Library Core**: `src/lib.rs` (4KB, 107 lines) - Exports all modules, build_app() function

### Architecture Achievement
- Successfully refactored from 1600+ line main.rs to 15 clean modules
- Each major system is a Bevy Plugin for clean separation
- Total codebase: 192KB across 15 well-organized files (up from 159KB)
- No single module exceeds 617 lines (excellent maintainability)
- Added river gameplay with deltas, agriculture, and fresh water systems
- Extracted all color functions to dedicated colors.rs module
- Removed ghost province code completely (simplified architecture)

### Recent Improvements (September 8, 2025)
- **Fixed Large World Generation Bug**: Terrain now properly scales with world size
  - Small: 15,000 provinces (150x100)
  - Medium: 60,000 provinces (300x200) 
  - Large: 135,000 provinces (450x300)
- **Rivers Now Have Gameplay Impact**:
  - Added Delta terrain type for fertile river mouths
  - Agriculture system (0.0-3.0 scale) based on terrain and water proximity
  - Fresh water distance tracking affects population growth
  - Population growth varies by agriculture, terrain type, and water access
  - River deltas become cradles of civilization with 3x population multiplier
- **Code Organization Improvements**:
  - Removed duplicate cloud texture functions
  - Extracted all color functions to new colors.rs module
  - Moved hexagon calculations to constants.rs
  - Removed unnecessary ghost province code
- **Dynamic World Sizes**: Random world size selection when not specified

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

## ALWAYS Use Context7 MCP for Library Documentation
- **What it is**: Context7 is a free MCP (Model Context Protocol) tool that provides instant access to up-to-date documentation for any library
- **When to use**: ALWAYS use it when working with any library, framework, or package - it's free and instant (in our case, when doing ANYTHING with Bevy engine, use Context7)
- **How to use**: 
  1. First call `mcp__context7__resolve-library-id` with the library name to get the Context7 ID
  2. Then call `mcp__context7__get-library-docs` with that ID to get documentation
- **Why use it**: Provides accurate, up-to-date documentation without hallucination risks
- **Examples**: Use for Bevy, steamworks, any Rust crate, JavaScript libraries, Python packages, etc. (in our case, anything for our game)
- **Cost**: Completely FREE and takes no time - there's no reason not to use it!

## ALWAYS Use Filesystem MCP for Efficient File Operations
- **What it is**: Filesystem MCP provides powerful bulk file operations that are 10-100x more efficient than standard tools
- **When to use**: ALWAYS for bulk operations, searching, or structured data - it's instant and way more powerful
- **Available tools**:
  - `mcp__filesystem__read_multiple_files` - Read many files in ONE call instead of many Read calls
  - `mcp__filesystem__directory_tree` - Get JSON structure of entire directories (perfect for understanding project layout)
  - `mcp__filesystem__search_files` - Recursively search with patterns and exclusions (better than grep for finding files)
  - `mcp__filesystem__list_directory_with_sizes` - List files with sizes, sortable (find large files instantly)
  - `mcp__filesystem__get_file_info` - Detailed metadata including timestamps and permissions
  - `mcp__filesystem__write_file` - Create/overwrite files
  - `mcp__filesystem__edit_file` - Line-based edits with diff preview
  - `mcp__filesystem__create_directory` - Create nested directories
  - `mcp__filesystem__move_file` - Move/rename files and directories
  - `mcp__filesystem__list_allowed_directories` - Check accessible directories

## Practical Filesystem MCP Examples for Living Worlds
```bash
# Read the main game file
mcp__filesystem__read_text_file
path: /home/nsabaj/Code/livingworlds/src/main.rs

# Find all references to a specific component or system
mcp__filesystem__search_files
pattern: "Province|Nation|TerrainType"
path: /home/nsabaj/Code/livingworlds

# Get current project structure as JSON
mcp__filesystem__directory_tree
path: /home/nsabaj/Code/livingworlds

# When we eventually refactor - bulk edits
mcp__filesystem__edit_file
edits: [multiple find/replace operations]
dryRun: true  # Preview changes first!
```

## When to Use Filesystem MCP vs Standard Tools
**USE FILESYSTEM MCP FOR:**
- Reading 2+ related files (use read_multiple_files)
- Getting directory structure (use directory_tree for JSON)
- Searching across codebase (use search_files with patterns)
- Finding files by size/date (use list_directory_with_sizes)
- Bulk refactoring (use edit_file with multiple edits)
- Understanding project layout (use directory_tree)

**USE STANDARD TOOLS ONLY FOR:**
- Single file quick read (Read is fine for one file)
- Simple one-line edits (Edit is fine for small changes)
- When you know exact file path and need one thing

**Rule of thumb**: If you're about to use Read/Grep/Bash more than once in a row, STOP and use Filesystem MCP instead!

## CRITICAL: Evidence-Based Documentation
- **ALWAYS collect concrete evidence instead of relying on memory** when updating documentation
- **NEVER trust memory alone** - it leads to documentation drift and "telephone game" effects
- **For project structure**: Use `ls`, `find`, or other filesystem commands to verify actual structure
- **For code references**: Use `Read`, `Grep`, or `Glob` to check actual implementation
- **For dependencies**: Check `Cargo.toml` and `Cargo.lock` for actual versions and features
- **Why this matters**: Hallucinated or memory-based updates compound over time, creating increasingly inaccurate documentation
- **Example**: The project structure section had 8 crates listed when only 4 actually exist (verified via `ls /home/nsabaj/Code/livingworlds/crates/`)

## When working on ANYTHING, always be mindful of what has been implemented/wrote already, what you are implementing/writing now, and what will be implemented/written in the future

## Important Paths and Resources
- **Steam SDK Location**: `/home/nsabaj/Code/sdk` - Contains the Steamworks SDK needed for Steam integration
- **bevy_steamworks status**: As of 2025, bevy_steamworks 0.13 only supports Bevy 0.15. PR #64 adds Bevy 0.16.1 support but isn't merged yet

## Testing Strategy
- **Crate-level tests**: Each crate should have its own tests in `src/` (unit tests) and `tests/` directory (integration tests)
- **Why**: Keeps tests close to code, enables `cargo test -p crate_name`, follows Rust idioms
- **Workspace tests**: Cross-crate integration tests can live at the workspace root
- **Running tests**: 
  - Single crate: `cargo test -p lw_economics`
  - All crates: `cargo test --workspace`
  - With output: `cargo test -- --nocapture`

## Executive Summary

**Title**: Living Worlds  
**Genre**: Perpetual Civilization OBSERVER / Hands-Off God Simulator  
**Similar To**: Fantasy Map Simulator, NOT Europa Universalis or Civilization  
**Engine**: Bevy 0.16 - Modern Rust Game Engine  
**Platform**: Windows, Linux (no Web whatsoever, this game is being sold on Steam)
**Distribution Platform**: Steam only
**Team**: Solo Developer with Claude Code assistance  
**Core Hook**: WATCH (not control) fully procedural civilizations rise and fall eternally  

## Vision Statement

Living Worlds is a hands-off observation simulator where you watch - BUT NEVER CONTROL - civilizations as they emerge, grow, fight, trade, and eventually collapse. Like Fantasy Map Simulator, you are purely an observer watching history unfold. You cannot play as a nation, give orders, or influence events directly. You can only watch, speed up time, pause, and observe the emergent stories that unfold.

Every texture, sound, and piece of text is procedurally generated at runtime. Civilizations advance through technologies at their own pace, build infrastructure that permanently marks the landscape, and manage complex economies based on Austrian economic principles. There is no victory condition - only the eternal cycle of rise and fall that you witness as a passive observer.

**Visual Style**: 2D orthographic province-based map (using the MAP STYLE of Europa Universalis/Crusader Kings - NOT their gameplay) with colored political boundaries, terrain overlays, and grid lines. Each province is a polygon mesh with dynamic coloring based on which nation currently controls it.

## 🎯 What This Game IS and ISN'T

### ✅ What Living Worlds IS:
- **A PURE OBSERVER SIMULATION** - You watch history unfold, you don't participate
- **Like Fantasy Map Simulator** - Civilizations act on their own with zero player input
- **An Emergent Story Generator** - Every simulation creates unique histories and narratives
- **A Meditation on History** - Watch the eternal cycles of rise and fall
- **A Sandbox for Observation** - Generate worlds and watch what happens
- **An Ambient Experience** - Can run in the background while you work
- **Procedural Everything** - Maps, cultures, languages, religions emerge dynamically
- **Infinitely Replayable** - Every world tells different stories

### ❌ What Living Worlds IS NOT:
- **NOT a Strategy Game** - You cannot control any nation or faction
- **NOT like EU4/CK3/HOI4** - Those let you play AS a country, we don't
- **NOT Civilization or Total War** - No playing as leaders or commanders
- **NOT a City Builder** - You don't build anything
- **NOT a 4X Game** - No exploring, expanding, exploiting, or exterminating
- **NOT SimCity or Dwarf Fortress** - No management or control mechanics
- **NOT a God Game with Powers** - No miracles, interventions, or divine actions
- **NOT Goal-Oriented** - No objectives, no winning, no losing
- **NOT Interactive** - You can't click on units or give any orders

### 🎮 Player Actions Limited To:
- **Generate New Worlds** - Set parameters and create a new simulation
- **Control Time** - Pause, play, speed up, slow down
- **Observe** - Pan camera, zoom in/out, watch events unfold
- **Toggle Overlays** - View different map modes (political, cultural, economic, etc.)
- **Take Screenshots** - Capture interesting moments
- **Export History** - Save the generated history as data/text
- **Configure Notifications** - Get alerted to interesting events

**Think of it as**: A digital ant farm or aquarium for civilizations. You set it up and watch it go.

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
// From lib.rs - Our actual plugin architecture
pub fn build_app() -> App {
    let mut app = App::new();
    
    // Configure Bevy's default plugins with our settings
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Living Worlds".into(),
                    resolution: (1920.0, 1080.0).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
    )
    .add_plugins(FrameTimeDiagnosticsPlugin::default());
    
    // Add all Living Worlds game plugins
    app.add_plugins(CloudPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(MineralPlugin)
        .add_plugins(OverlayPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ProceduralMusicPlugin);
    
    app
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

#### Hexagon Implementation (Complete Guide)

##### Overview
**Living Worlds uses FLAT-TOP hexagons with odd-q offset coordinate system**
- **Flat-top**: Hexagons have flat edges on top/bottom, points on sides
- **Odd-q offset**: Odd columns (q) are shifted UP by half a row
- This creates a honeycomb pattern for perfect tessellation

##### Visual Characteristics
```
    Flat-Top Hexagon (USED)         Pointy-Top (NOT USED)
         ____                              /\
        /    \                            /  \
       /      \                          |    |
      |        |                         |    |
       \      /                           \  /
        \____/                             \/
```

##### Mathematical Constants
```rust
const HEX_SIZE_PIXELS: f32 = 50.0;  // Radius of hexagon
const SQRT_3: f32 = 1.732050808;    // √3 for calculations

// Spacing between hexagon centers
const HORIZONTAL_SPACING: f32 = HEX_SIZE_PIXELS * SQRT_3;  // Column to column
const VERTICAL_SPACING: f32 = HEX_SIZE_PIXELS * 1.5;      // Row to row
const ODD_COLUMN_OFFSET: f32 = HEX_SIZE_PIXELS * 0.75;    // Y offset for odd columns
```

##### 1. Positioning (Odd-q Offset System)
```rust
// Calculate world position from grid coordinates (col, row)
fn hex_position(col: u32, row: u32, hex_size: f32) -> (f32, f32) {
    let sqrt3 = 1.732050808;
    
    // Odd columns shift down by half the vertical spacing
    let y_offset = if col % 2 == 1 { hex_size * 0.75 } else { 0.0 };
    
    // Horizontal: columns are sqrt(3) * radius apart
    let x = col as f32 * hex_size * sqrt3;
    
    // Vertical: rows are 3/2 * radius apart, plus offset
    let y = row as f32 * hex_size * 1.5 + y_offset;
    
    (x, y)
}
```

##### 2. Texture Generation with Antialiasing
```rust
// Create smooth hexagon texture using distance field
fn create_hexagon_texture(size: f32) -> Image {
    // For each pixel, calculate distance from hexagon edge
    let dist_vertical = abs_x - radius * sqrt3 / 2.0;
    let dist_diagonal = (sqrt3 * abs_y + abs_x) / sqrt3 - radius;
    let distance_from_edge = dist_vertical.max(dist_diagonal);
    
    // Apply smooth antialiasing transition
    let aa_width = 1.5; // Pixels of smooth transition
    let alpha = smooth_step(distance_from_edge, -aa_width, aa_width);
    
    // Configure linear filtering for additional smoothness
    image.sampler = ImageSampler::linear();
}
```

##### 3. Border Drawing (Gizmos)
```rust
// Draw hexagon borders for FLAT-TOP hexagons
fn draw_hexagon_border(center: Vec2, radius: f32) {
    let mut vertices = Vec::new();
    for i in 0..=6 {
        // FLAT-TOP starts at 0° (flat edge at top)
        let angle = (i as f32 * 60.0).to_radians();
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        vertices.push(Vec2::new(x, y));
    }
    // Draw lines between vertices
}
```

##### 4. Point-in-Hexagon Test
```rust
// Check if a point (px, py) is inside a flat-top hexagon
fn point_in_hexagon(px: f32, py: f32, center_x: f32, center_y: f32, radius: f32) -> bool {
    let dx = (px - center_x).abs();
    let dy = (py - center_y).abs();
    let sqrt3 = 1.732050808;
    
    // Two constraints define the hexagon boundary
    dy <= radius * sqrt3 / 2.0 &&                    // Within horizontal sides
    (dy / sqrt3 + dx / 2.0 <= radius)                // Within diagonal edges
}
```

##### 5. Common Pitfalls and Solutions

**Pitfall 1: Wrong Orientation**
- ❌ Using pointy-top math with flat-top visuals
- ✅ Ensure ALL components use flat-top: texture, positioning, borders

**Pitfall 2: Incorrect Border Vertices**
- ❌ Starting at 30° (creates pointy-top borders)
- ✅ Start at 0° for flat-top: `(i * 60.0).to_radians()`

**Pitfall 3: Wrong Spacing Values**
- ❌ Swapping horizontal/vertical spacing values
- ✅ Horizontal = √3 * radius, Vertical = 3/2 * radius

**Pitfall 4: Jagged Edges**
- ❌ Hard binary edges in texture (no antialiasing)
- ✅ Use distance field with smooth transition + linear filtering

**Pitfall 5: Gaps Between Tiles**
- ❌ Scaling hexagon texture down (e.g., radius * 0.9)
- ✅ Use full radius for perfect tessellation

##### 6. References
- **Red Blob Games Hexagonal Grids**: The definitive guide for hex math
- **Odd-q offset**: One of four offset coordinate systems for hexagons
- **Distance field rendering**: Technique for smooth antialiased shapes

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

#### UI Structure Changes (0.15→0.16) 
**CRITICAL: Text entities must be children of Node entities**

In Bevy 0.16, UI text elements cannot have Node components directly. They must follow proper parent-child hierarchy:

```rust
// WRONG - Will compile but text won't display!
commands.spawn((
    Text::new("Hello"),
    Node { position_type: PositionType::Absolute, ... },
    TextFont { ... },
    MyMarkerComponent,
));

// CORRECT - Proper parent-child structure
commands.spawn((
    Node { position_type: PositionType::Absolute, ... },
    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
)).with_children(|parent| {
    parent.spawn((
        Text::new("Hello"),
        TextFont { ... },
        TextColor(Color::WHITE),
        MyMarkerComponent,
    ));
});
```

**Why this matters:**
- Text won't render if Node is on the same entity
- Console might show toggles working but nothing appears on screen
- Common issue when migrating from older Bevy versions

#### Hierarchy Component Changes (0.15→0.16)
**BREAKING CHANGE: Parent renamed to ChildOf**

```rust
// WRONG - Bevy 0.15 and earlier
use bevy::hierarchy::Parent;
fn system(query: Query<&Parent>) { 
    let parent_entity = parent.get();
}

// CORRECT - Bevy 0.16+
// ChildOf is in prelude, no special import needed
fn system(query: Query<&ChildOf>) {
    let parent_entity = child_of.parent();  // Note: .parent() method, not .get()
}
```

**Key changes:**
- `Parent` → `ChildOf` (clearer semantics - entities with ChildOf are children)
- `parent.get()` → `child_of.parent()` (method name change)
- Part of new entity relationships system for better performance
- `Children` still exists but is now a `RelationshipTarget`

**Error indicators:**
- `cannot find type Parent` → Change to `ChildOf`
- `cannot find value parent.get()` → Use `child_of.parent()`
- `unresolved import bevy::hierarchy` → ChildOf is in prelude

#### Sprite API Changes (0.15→0.16)
**BREAKING CHANGE: SpriteBundle removed, Sprite now includes image**

In Bevy 0.16, the sprite API was simplified with Required Components:

```rust
// WRONG - Bevy 0.15 and earlier
commands.spawn(SpriteBundle {
    sprite: Sprite {
        color: Color::WHITE,
        custom_size: Some(Vec2::new(100.0, 100.0)),
        ..default()
    },
    texture: asset_handle,
    transform: Transform::from_xyz(0.0, 0.0, 0.0),
    ..default()
});

// CORRECT - Bevy 0.16+
commands.spawn((
    Sprite {
        image: asset_handle,  // Image is now part of Sprite!
        color: Color::WHITE,
        custom_size: Some(Vec2::new(100.0, 100.0)),
        ..default()
    },
    Transform::from_xyz(0.0, 0.0, 0.0),
));

// Or even simpler:
commands.spawn(Sprite::from_image(asset_handle));
```

**Key changes:**
- `SpriteBundle` no longer exists
- `Sprite` now has an `image` field (was `texture` in bundle)
- Transform and Visibility are automatically added (Required Components)
- Simpler API with `Sprite::from_image()` constructor

**Error indicators:**
- `cannot find struct SpriteBundle` → Use Sprite directly with image field
- `no field texture on Sprite` → Use `image` field instead

## Project Structure (Modular Architecture Achieved)

```
livingworlds/
├── Cargo.toml           # Project configuration with [lib] and [[bin]]
├── Cargo.lock           # Dependency lock file
├── CLAUDE.md            # Project documentation and AI instructions
├── README.md            # Public-facing documentation
├── .gitignore           # Git ignore configuration
├── .git/                # Git repository data
├── .github/             # GitHub workflows and configuration
├── .claude/             # Claude Code configuration
├── images/              # Screenshots and documentation images
├── src/                 # Source code (14 files, 161.69 KB total)
│   ├── setup.rs         # 31.47 KB - World generation, terrain, rivers, nations, minerals
│   ├── clouds.rs        # 24.59 KB - Procedural cloud generation and animation
│   ├── minerals.rs      # 20.08 KB - Mineral resources, extraction, tech progression
│   ├── resources.rs     # 12.39 KB - Global resources, world tension, spatial index
│   ├── terrain.rs       # 12.39 KB - Terrain types, climate zones, biome generation
│   ├── music.rs         # 11.08 KB - Procedural music generation with world tension
│   ├── main.rs          # 9.93 KB - Binary entry point, input handling (291 lines)
│   ├── ui.rs            # 7.67 KB - User interface, FPS display, UI panels
│   ├── camera.rs        # 7.03 KB - Camera controls, pan/zoom, edge scrolling
│   ├── constants.rs     # 6.90 KB - All game constants and tuning parameters
│   ├── simulation.rs    # 6.48 KB - Time simulation, world tension, population growth
│   ├── components.rs    # 4.46 KB - ECS components (Province, Nation, Resources)
│   ├── overlay.rs       # 3.89 KB - Map overlay rendering for all visualization modes
│   └── lib.rs           # 3.32 KB - Library root, exports modules, build_app()
└── target/              # Build output (git-ignored)

NOTE: No crates/ directory - focusing on modular single-crate architecture
NOTE: No assets/ directory - everything is procedurally generated
NOTE: No wasm/ or web/ directories - Steam distribution only
```

### Module Architecture (Library + Plugin-Based)

The game uses a hybrid library/binary architecture with Bevy's Plugin system:

#### World Tension System
The World Tension system drives the dynamic music and atmosphere of the game:

**Location and Components**:
- **WorldTension Resource** (`src/resources.rs` lines 71-132): Global tension metric (0.0 to 1.0)
  - Tracks current tension, target tension, and physics-based smoothing
  - Uses exponential curve: 50% nations at war ≈ 70% tension via `calculate_from_war_percentage()`
  - Non-Newtonian physics: rises quickly (heating_rate: 2.0), falls slowly (cooling_rate: 0.3)
  - Contributing factors: war_factor, power_imbalance, economic_stress, instability_factor
  - Starts at 0.0 (perfect peace) when the game begins

- **Music System** (`src/music.rs`): Procedural music generation based on tension
  - `ProceduralMusicPlugin`: Manages all music systems and resources
  - `MusicState` enum: 8 moods from Silence (0-5%) to Apocalypse (90-100%)
  - `continuous_music_system`: Plays background music continuously, tempo adjusts with tension
  - `calculate_tension_physics`: Smoothly interpolates tension changes with inertia
  - `map_tension_to_mood`: Converts tension values to music moods
  - `update_tension_manual`: Test controls (T/G keys) for tension adjustment

- **Tension Calculation** (`src/simulation.rs`): 
  - `calculate_world_tension` system: Reads province states and calculates global tension
  - Currently uses manual controls until nation systems are fully implemented
  - Will eventually track actual wars, economic disruption, and nation collapses
  - Updates the WorldTension.target which physics system smoothly transitions to

#### `src/lib.rs` (Core Library)
Library root that exports all game functionality:
- **Module Exports**: Public access to all 14 game modules
- **Prelude Module**: Convenient re-exports of commonly used items
- **build_app()**: Constructs the Bevy app with all plugins
- **Re-exports**: Resources and constants for backward compatibility

#### `src/main.rs` (10.71KB - Binary Entry Point)
Game orchestrator and system implementations:
- **CLI Parsing**: Command-line arguments for seed and world size
- **App Setup**: Calls build_app() and adds game-specific systems
- **Input Handling**: Keyboard controls, province selection
- **Game Systems**: Province updates, tile selection, UI updates
- **Simulation**: Time system, speed controls, pause/play
- **Province Interaction**: Hover effects, selection feedback

#### `src/setup.rs` (22.11KB - World Generation)
Complete world generation and initialization:
- **setup_world()**: Main world generation orchestrator
- **Terrain Generation**: Uses elevation maps with continent centers
- **River System**: Gradient descent from mountains to ocean (50+ rivers)
- **Nation Placement**: Distance-based territory assignment
- **Province Creation**: 60,000 hexagonal tiles with terrain
- **Ocean Depth**: Spatial grid for efficient depth calculation
- **create_hexagon_texture()**: Antialiased hexagon sprites
- **generate_elevation_with_edges()**: Map edge ocean forcing

#### `src/terrain.rs` (14.79KB - Terrain Systems)
Terrain generation and classification:
- **TerrainPlugin**: Bevy plugin for terrain systems
- **TerrainType Enum**: 11 types including new River type
- **ClimateZone Enum**: 5 climate zones from Arctic to Tropical
- **Climate System**: Latitude-based biome determination
- **Color Gradients**: Smooth terrain color transitions
- **Elevation Generation**: Multi-octave Perlin noise
- **Continent Generation**: Tectonic plate simulation
- **Population Multipliers**: Terrain-based population factors

#### `src/constants.rs` (7.6KB - Game Parameters)
Centralized configuration values:
- **World Constants**: Map dimensions, province counts
- **Camera Constants**: Zoom/pan speeds, limits
- **UI Constants**: Font sizes, margins
- **Simulation Constants**: Starting year, time speeds
- **World Generation**: Continent sizes, falloff curves
- **Colors**: All UI and terrain colors
- **River Parameters**: Count, minimum elevation
- **Spatial Indexing**: Grid cell multipliers

#### `src/clouds.rs` (7.5KB - Cloud System)
Procedural cloud generation and animation:
- **CloudPlugin**: Bevy plugin for cloud systems
- **Multi-Layer System**: High/Medium/Low for parallax
- **Procedural Generation**: Multi-octave Perlin noise
- **Radial Falloff**: Eliminates square edges
- **Wind Animation**: Clouds drift and wrap
- **spawn_clouds()**: Creates cloud sprites across layers
- **animate_clouds()**: Handles movement and wrapping

#### `src/camera.rs` (6.6KB - Camera Controls)
Complete camera system:
- **CameraPlugin**: Bevy plugin for camera systems
- **Movement**: WASD/Arrow keys with Shift speed boost
- **Zoom**: Mouse wheel with configurable limits
- **Edge Panning**: RTS-style screen edge movement
- **Reset**: Home key returns to origin
- **Constraints**: Y-axis clamped to map bounds
- **Dynamic Speed**: Scales with zoom level

#### `src/resources.rs` (4.7KB - Global State)
Game-wide singleton resources:
- **WorldSeed**: Procedural generation seed
- **WorldSize**: Small/Medium/Large world configurations
- **GameTime**: Simulation time and speed control
- **ProvincesSpatialIndex**: O(1) province lookups
- **SelectedProvinceInfo**: Currently selected tile

#### `src/ui.rs` (3.1KB - User Interface)
UI systems and displays:
- **UIPlugin**: Bevy plugin for UI systems
- **Responsive Layout**: Percentage-based positioning
- **setup_ui()**: Creates UI hierarchy

#### `src/music.rs` (10.0KB - Procedural Music Generation)
Infinite ambient music generation with FunDSP:
- **ProceduralMusicPlugin**: Bevy plugin for music systems
- **MusicState Enum**: Peace, Exploration, Building, Battle, Victory
- **MusicSettings Resource**: Root frequency, scale, intensity, tempo, volume
- **ScaleType Enum**: Pentatonic, Dorian, Mixolydian, MinorPentatonic
- **generate_ambient_audio()**: Creates 30-second WAV loops in memory
- **create_ambient_graph()**: FunDSP synthesis with drone, melody, reverb
- **MarkovMelody**: Probabilistic note progression for musical phrases
- **create_drum_pattern()**: Intensity-based rhythmic patterns
- **Zero external assets**: All music generated at runtime with synthesis

#### `src/components.rs` (1.5KB - ECS Components)
Core game entity data:
- **Province**: Tile data (terrain, population, elevation)
- **Nation**: Political entities with colors
- **SelectedProvince**: Marker for selection
- **GhostProvince**: World wrapping duplicates
- **TileInfoPanel/Text**: UI component markers

### Bevy Plugin Architecture

The modularization uses Bevy's Plugin pattern for clean separation of concerns:

```rust
// In module file (e.g., clouds.rs)
pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CloudSettings>()
            .add_systems(Update, animate_clouds);
    }
}

// In main.rs
app.add_plugins(CloudPlugin);
```

**Benefits of Plugin Architecture**:
- **Modularity**: Each system is self-contained with its own plugin
- **Reusability**: Plugins can be easily shared or moved between projects
- **Organization**: Clear separation of concerns and dependencies
- **Testing**: Individual plugins can be tested in isolation
- **Hot Swapping**: Plugins can be enabled/disabled at runtime

## Building & Running

### Development
```bash
# Fast iterative development with dynamic linking
cargo run --features bevy/dynamic_linking

# Run with hot reloading
cargo run --features bevy/file_watcher

# Run tests
cargo test
```

### Release Build
```bash
# Optimized native build
cargo build --release

# Bundle for distribution
cargo bundle --release
```


## Why This Architecture Works

1. **Bevy's ECS is Perfect for Simulation**: Thousands of entities with complex interactions
2. **Built-in Parallelization**: Free performance gains from Bevy's scheduler
3. **Hot Reloading**: Rapid iteration on game balance and features
4. **Cross-Platform**: Same code runs everywhere Bevy supports
5. **Great Ecosystem**: Tons of Bevy plugins we can leverage
6. **Modern Rust**: Safe, fast, and expressive

## Code Principles

### Evidence-Based Debugging and Investigation
- **ALWAYS investigate issues through code examination BEFORE implementing fixes**
- **NEVER guess or assume** - Look at the actual code to understand root causes
- **Investigation steps**:
  1. Read relevant code sections to understand current behavior
  2. Trace data flow to identify where issues originate
  3. Check for matching values (e.g., colors matching backgrounds)
  4. Verify assumptions with concrete evidence from the codebase
  5. Document findings before implementing solutions
- **Example**: Ocean tiles invisible? Check ocean color generation AND background color - they might match!
- **Benefits**:
  - Fixes address root causes, not symptoms
  - Avoid introducing new bugs from incorrect assumptions
  - Build accurate mental model of the system
  - Save time by fixing the right problem first time

### Dynamic Calculations Over Hardcoded Values
- **NEVER hardcode calculated values** - Always compute them dynamically
- **Example BAD**: `let map_bound_x = 2598.0; // 60 * 50 * 1.732 / 2`
- **Example GOOD**: `let map_bound_x = (provinces_per_row as f32 * hex_size * 1.732) / 2.0;`
- **Benefits**:
  - Self-documenting code that shows the calculation
  - Automatically adapts when parameters change
  - No risk of manual calculation errors
  - More maintainable and refactorable
- **Apply this to**: Map bounds, camera limits, grid calculations, any derived values

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