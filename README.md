# Living Worlds

<p align="center">
  <img src="images/world-generation-hero.png" alt="Living Worlds - Procedurally generated world with realistic ocean depths, continents, and dynamic weather" width="100%">
</p>

<p align="center">
  <i>A hands-off civilization observer simulation built with Bevy where you WATCH (not control) empires rise and fall eternally through emergent gameplay.</i>
</p>

## Overview

Living Worlds is a fully procedural civilization OBSERVER - like Fantasy Map Simulator, you have zero control over the civilizations. You can only watch as they emerge, grow, fight, and collapse. Every texture, sound, and piece of text is generated at runtime. Observe as civilizations develop organically, advance through technologies at their own pace, build infrastructure that permanently marks the landscape, and manage complex economies based on Austrian economic principles. There is no victory condition and no player interaction - only the eternal cycle of rise and fall that you witness as a passive observer.

## 🎮 Features

### Currently Implemented
- **🗺️ Hexagonal World Map**: Configurable sizes with flat-top honeycomb layout
  - Small: 300,000 provinces (600x500)
  - Medium: 600,000 provinces (800x750)
  - Large: 900,000 provinces (1000x900)
- **⚡ Mega-Mesh Rendering**: Revolutionary performance breakthrough
  - **60+ FPS** on large worlds (900,000 provinces!)
  - Single mesh with 2.7M+ vertices handled efficiently
  - One GPU draw call for entire world
  - Dynamic vertex color updates for overlays
- **🌊 Realistic Ocean Depths**: Three-tier water depth system with beautiful gradients
  - Shallow coastal waters
  - Medium depth continental shelves  
  - Deep ocean trenches
- **🏔️ Procedural Terrain**: 12 terrain types including rivers and deltas
  - Dynamic biome distribution based on latitude
  - Rivers flowing from mountains to ocean with gameplay impact
  - Agriculture zones near water sources
  - Forests, jungles, deserts, tundra, and ice caps
- **☁️ Dynamic Weather**: Multi-layer procedural cloud system with wind
- **⛏️ Mineral Resources**: 9 mineral types with realistic vein distribution
  - Iron, Copper, Tin, Gold, Coal, Gems, Stone, Bronze, Steel
  - Heat map overlays for resource visualization
  - Combined richness view for all minerals
- **🏛️ Nations**: Territory-based civilizations with expansion mechanics
- **⏱️ Time Simulation**: Pause/play with 1x, 3x, 6x, 9x speed controls
- **📊 Map Overlays**: Political, individual minerals, all minerals, infrastructure views

### Controls
- **Camera**: WASD/Arrow keys for panning, mouse wheel for zoom, edge scrolling
- **Time**: Space to pause, 1-4 keys for speed control
- **Overlays**: M to cycle through map modes

## ⚡ Performance Achievements

- **World Generation**: 900,000 provinces generate in ~7 seconds
- **Rendering**: 60+ FPS with 2.7 million vertices (single draw call)
- **Memory Usage**: ~200MB for entire world state
- **O(1) Province Lookups**: HashMap-based architecture throughout
- **Zero O(n²) Patterns**: All quadratic algorithms eliminated
- **Parallel Processing**: 75% CPU utilization with rayon

### Optimization History
- Fixed O(n²) spatial index bug: 1160s → 7s (162x speedup)
- Fixed O(n²) ocean depth calculation: 30s → 0.1s (300x speedup)
- Mega-mesh architecture: 900k entities → 1 entity
- HashMap lookups: 900k comparisons → 1 lookup

## 🛠️ Technology Stack

- **Engine**: Bevy 0.16.1 (Modern Rust game engine)
- **Language**: Rust 2021 Edition
- **Graphics**: wgpu (Modern GPU API)
- **Audio**: Procedural generation with Bevy audio
- **Platform**: Windows, Linux, MacOS (Steam distribution planned)

## 📁 Architecture

The project uses a **modular plugin architecture** with Bevy's ECS (Entity Component System):

```
livingworlds/
├── src/                    # Source code (~1 MB, ~25,000 lines)
│   ├── generation/        # World generation builders (7 files)
│   │   ├── mod.rs         # WorldBuilder orchestrator
│   │   ├── provinces.rs   # ProvinceBuilder with parallel processing
│   │   ├── rivers.rs      # RiverBuilder with flow accumulation
│   │   ├── clouds.rs      # CloudBuilder for atmospheric effects
│   │   ├── agriculture.rs # Agriculture and fertility calculations
│   │   └── utils.rs       # Utility functions
│   ├── world/             # World data and rendering (9 files)
│   │   ├── mod.rs         # World module exports
│   │   ├── data.rs        # Core data structures (World, RiverSystem, etc.)
│   │   ├── terrain.rs     # Terrain types and climate zones
│   │   ├── mesh.rs        # Mega-mesh building and vertex generation
│   │   ├── borders.rs     # Selection border rendering
│   │   ├── overlay.rs     # Map overlay with dynamic vertex colors
│   │   ├── clouds.rs      # Cloud rendering and animation
│   │   ├── config.rs      # World configuration UI
│   │   └── components.rs  # World-specific components
│   ├── ui/                # User interface system (10 files)
│   │   ├── mod.rs         # UI plugin and coordination
│   │   ├── styles.rs      # Centralized colors and dimensions
│   │   ├── buttons.rs     # StyledButton builder system
│   │   ├── dialogs.rs     # DialogBuilder for consistent dialogs
│   │   ├── text_inputs.rs # TextInputBuilder with validation
│   │   ├── sliders.rs     # SliderBuilder for value controls
│   │   ├── components.rs  # Common UI components
│   │   ├── form.rs        # Form handling
│   │   ├── toolbar.rs     # Toolbar system
│   │   └── builders.rs    # UI builder utilities
│   ├── geometry/          # Hexagon calculations (2 files)
│   │   ├── mod.rs         # Module exports
│   │   └── hexagon.rs     # Single source of truth for hex math
│   ├── settings/          # Settings management (8 files)
│   │   ├── mod.rs         # Settings plugin
│   │   ├── settings_ui.rs # Settings menu UI
│   │   ├── handlers.rs    # Event handlers
│   │   ├── persistence.rs # Save/load settings
│   │   ├── resolution.rs  # Resolution detection
│   │   ├── types.rs       # Settings data structures
│   │   ├── navigation.rs  # Tab navigation
│   │   └── components.rs  # Settings components
│   ├── modding/           # Modding system (5 files)
│   │   ├── mod.rs         # Modding plugin
│   │   ├── types.rs       # Mod data structures
│   │   ├── loader.rs      # Mod loading system
│   │   ├── manager.rs     # Mod management
│   │   └── ui.rs          # Mod browser UI
│   ├── lib.rs             # Library root, plugin orchestration
│   ├── main.rs            # Binary entry point
│   ├── setup.rs           # World initialization using builders
│   ├── simulation.rs      # Time simulation and population
│   ├── minerals.rs        # Mineral resources and extraction
│   ├── camera.rs          # Camera controls and viewport
│   ├── colors.rs          # Terrain and mineral color functions
│   ├── components.rs      # Core ECS components
│   ├── resources.rs       # Global game resources
│   ├── constants.rs       # Game configuration constants
│   ├── states.rs          # Game state management
│   ├── menus.rs           # Main and pause menus
│   ├── loading_screen.rs  # Loading screen system
│   ├── save_load.rs       # Save/load game functionality
│   ├── province_events.rs # Province event handling
│   ├── name_generator.rs  # Procedural name generation
│   └── steam.rs           # Steam integration
├── images/                 # Screenshots and documentation
├── Cargo.toml             # Rust dependencies
├── CLAUDE.md              # Detailed technical documentation
└── README.md              # This file

NOTE: No assets/ directory - everything is procedurally generated!
```

### Key Systems
- **Mega-Mesh Renderer**: Single mesh with 2.7M+ vertices for 60+ FPS on 900k provinces
- **Builder Pattern Architecture**: All generation uses fluent builder APIs (WorldBuilder, ProvinceBuilder, etc.)
- **ECS Architecture**: Leverages Bevy's parallel processing
- **Plugin System**: Each module is a self-contained Bevy plugin
- **Single Source of Truth**: Centralized data structures in world/data.rs
- **Hexagon Geometry**: All hex calculations in geometry/hexagon.rs
- **Deterministic Simulation**: Fixed-point math for consistency
- **Spatial Indexing**: O(1) province lookups for performance
- **Dynamic Vertex Colors**: Real-time overlay updates without recreating mesh

## 🚀 Getting Started

### Prerequisites

- Rust 1.75 or later
- Cargo (comes with Rust)

### Building & Running

```bash
# Clone the repository
git clone https://github.com/yourusername/livingworlds.git
cd livingworlds

# Run the game (optimized)
cargo run --release

# For faster compilation during development
cargo run --features bevy/dynamic_linking

# Run with specific seed for reproducible worlds
cargo run --release -- --seed 42

# Run with different world sizes
cargo run --release -- --world-size large
```

### Development Commands

```bash
# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## 🎯 Design Philosophy

- **Pure Observer**: You cannot control anything - only watch
- **Bottom-up Emergence**: Complex behaviors from simple rules
- **No Abstraction**: Model individuals, not statistics
- **Infinite Replayability**: Every world tells unique stories
- **Performance First**: Optimized for simulating thousands of entities

## 🗺️ Roadmap

### Near Term
- [ ] Individual agent simulation (every person as an entity)
- [ ] Austrian economics implementation
- [ ] Cultural emergence and language evolution
- [ ] Technology tree progression
- [ ] Infrastructure that modifies terrain

### Long Term
- [ ] Save/load system with Bevy Scenes
- [ ] Steam integration (achievements, cloud saves)
- [ ] Mod support through dynamic plugins
- [ ] Multiplayer observer mode
- [ ] Historical record export

## 🤝 Contributing

Living Worlds welcomes contributions! Please see `CLAUDE.md` for technical details.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Submit a pull request

## 📚 Documentation

- **CLAUDE.md**: Comprehensive technical documentation
- **Code Comments**: Extensive inline documentation
- **Bevy Book**: https://bevyengine.org/learn/

## 📝 License

[License information to be added]

## 🌟 Acknowledgments

- Built with [Bevy](https://bevyengine.org/) - A refreshingly simple data-driven game engine
- Inspired by Fantasy Map Simulator and similar observer games
- Hexagon math from [Red Blob Games](https://www.redblobgames.com/grids/hexagons/)

---

*Living Worlds - Watch civilizations rise and fall in an endless dance of history*