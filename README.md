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
  - Small: 1,000,000 provinces (1250x800)
  - Medium: 2,000,000 provinces (1600x1250)
  - Large: 3,000,000 provinces (2000x1500)
- **⚡ Mega-Mesh Rendering**: Revolutionary performance breakthrough
  - **60+ FPS** on all world sizes up to 3,000,000 provinces
  - Single mesh with millions of vertices handled efficiently
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

- **Rendering**: 60+ FPS with millions of vertices (single draw call)
- **Memory Usage**: Efficient for entire world state
- **O(1) Province Lookups**: HashMap-based architecture throughout
- **Zero O(n²) Patterns**: All quadratic algorithms eliminated
- **Parallel Processing**: 75% CPU utilization with rayon

### Optimization History
- Fixed O(n²) spatial index bug: 1160s → 7s (162x speedup)
- Fixed O(n²) ocean depth calculation: 30s → 0.1s (300x speedup)
- Mega-mesh architecture: 9M entities → 1 entity
- HashMap lookups: 9M comparisons → 1 lookup

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
├── src/                    # Source code (modular architecture)
│   ├── world/             # World systems with 11 subdirectories
│   │   ├── borders/       # Province border rendering
│   │   ├── clouds/        # Cloud rendering and animation
│   │   ├── colors/        # Terrain and mineral colors
│   │   ├── mesh/          # Mega-mesh generation
│   │   ├── minerals/      # Resource generation
│   │   ├── overlay/       # Map overlay modes
│   │   ├── provinces/     # Province data structures
│   │   ├── rivers/        # River generation
│   │   ├── simulation/    # World simulation systems
│   │   ├── terrain/       # Terrain types and climate
│   │   ├── ui/            # World configuration UI
│   │   └── (gateway mod.rs controls all access)
│   ├── ui/                # User interface system
│   │   ├── hud/           # HUD components
│   │   ├── overlay_display/ # Overlay display
│   │   ├── components/    # UI components
│   │   ├── form.rs        # Form handling
│   │   ├── loading.rs     # Loading UI
│   │   ├── toolbar.rs     # Toolbar system
│   │   └── (gateway architecture)
│   ├── simulation/        # Simulation systems (gateway architecture)
│   │   ├── time/          # Time management subsystem
│   │   ├── input/         # Input handling (refactored from 137 to 91 lines)
│   │   ├── tension/       # World tension subsystem
│   │   └── (each subsystem has gateway mod.rs)
│   ├── math/              # Mathematics module (single source of truth)
│   │   ├── perlin.rs      # ALL noise generation
│   │   ├── hexagon.rs     # ALL hex calculations
│   │   ├── interpolation.rs # Smoothing functions
│   │   └── (gateway enforces single source)
│   ├── name_generator/    # Name generation system
│   │   ├── data/          # Culture-specific data
│   │   ├── generator.rs   # Core generation logic
│   │   └── (gateway architecture)
│   ├── settings/          # Settings management
│   ├── modding/           # Modding system
│   ├── menus/             # Menu systems
│   │   ├── main_menu.rs   # Main menu
│   │   └── pause_menu.rs  # Pause menu
│   ├── save_load.rs       # Save/load functionality
│   ├── loading_screen.rs  # Loading UI
│   ├── states.rs          # Game state management
│   ├── resources.rs       # Global game resources
│   ├── lib.rs             # Library root, plugin orchestration
│   └── main.rs            # Binary entry point
├── images/                 # Screenshots and documentation
├── Cargo.toml             # Rust dependencies
├── CLAUDE.md              # Detailed technical documentation
└── README.md              # This file

NOTE: No assets/ directory - everything is procedurally generated!
```

### Key Systems
- **Mega-Mesh Renderer**: Single mesh with millions of vertices for 60+ FPS
- **Builder Pattern Architecture**: All generation uses fluent builder APIs (WorldBuilder, ProvinceBuilder, etc.)
- **ECS Architecture**: Leverages Bevy's parallel processing
- **Plugin System**: Each module is a self-contained Bevy plugin
- **Gateway Architecture**: Module interfaces control all imports/exports
- **Math Module**: All hex calculations in math/, single source of truth
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