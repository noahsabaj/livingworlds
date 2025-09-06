# Living Worlds

A perpetual civilization simulator built with Bevy where empires rise and fall eternally through emergent gameplay.

## Overview

Living Worlds is a fully procedural civilization simulator where every texture, sound, and piece of text is generated at runtime. Watch as civilizations develop organically, advance through technologies at their own pace, build infrastructure that permanently marks the landscape, and manage complex economies based on Austrian economic principles. There is no victory condition - only the eternal cycle of rise and fall.

## Features

- **Fully Procedural**: Everything generated at runtime - no pre-made assets
- **Individual Simulation**: Every person is simulated with needs, skills, and decisions  
- **Austrian Economics**: Prices emerge from individual actions, not abstract models
- **Emergent Gameplay**: No scripted events - everything emerges from systems
- **Domain-Driven Architecture**: Clean separation of concerns across 10 specialized crates
- **Deterministic Simulation**: Fixed-point math ensures cross-platform consistency

## Technology Stack

- **Engine**: Bevy 0.16.1 (Rust game engine)
- **Language**: Rust 2021 Edition
- **Graphics**: wgpu (modern GPU API)
- **Math**: Fixed-point arithmetic for determinism
- **Platform**: Windows, Linux (Steam distribution)

## Architecture

The project uses a domain-driven design with 10 specialized crates:

| Crate | Purpose |
|-------|---------|
| `lw_core` | Mathematics, utilities, and shared types |
| `lw_economics` | Economic systems and market emergence |
| `lw_military` | Military units, combat, and strategy |
| `lw_culture` | Cultural transmission and evolution |
| `lw_governance` | Government and supranational entities |
| `lw_world` | Geography and environmental systems |
| `lw_simulation` | Orchestrates all domain systems |
| `lw_procedural` | Procedural content generation |
| `lw_game` | Main game plugin and integration |
| `lw_platform` | Platform-specific code (Steam) |

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Cargo (comes with Rust)

### Building

```bash
# Clone the repository
git clone https://github.com/yourusername/livingworlds.git
cd livingworlds

# Build the project
cargo build --release

# Run the game
cargo run --release
```

### Development

```bash
# Fast iterative development with dynamic linking
cargo run --features bevy/dynamic_linking

# Run with hot reloading
cargo run --features bevy/file_watcher

# Run tests
cargo test

# Check specific crate
cargo check -p lw_economics
```

## Project Structure

```
livingworlds/
├── src/                 # Main application entry point
├── crates/             # Domain-specific crates
│   ├── lw_core/        # Core utilities
│   ├── lw_economics/   # Economic systems
│   ├── lw_military/    # Military systems
│   ├── lw_culture/     # Cultural systems
│   ├── lw_governance/  # Government systems
│   ├── lw_world/       # Geographic systems
│   ├── lw_simulation/  # System orchestration
│   ├── lw_procedural/  # Content generation
│   ├── lw_game/        # Game integration
│   └── lw_platform/    # Platform integration
├── data/               # Game data files
└── CLAUDE.md           # Detailed technical documentation
```

## Documentation

- See `CLAUDE.md` for detailed technical documentation
- Each crate has its own README with specific details
- Code is extensively commented with design decisions

## Contributing

Living Worlds is currently in early development. Contributions are welcome!

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests (`cargo test`)
5. Submit a pull request

## Design Philosophy

- **Bottom-up Emergence**: Complex behaviors emerge from simple rules
- **No Abstraction**: Model individuals, not statistics
- **Player as Observer**: Watch civilizations evolve naturally
- **Infinite Replayability**: Every world is unique
- **Historical Authenticity**: Grounded in real historical processes

## License

[License information to be added]

## Status

🚧 **Early Development** - Core systems being implemented

### Completed
- Domain-driven architecture
- Fixed-point math system
- ECS component definitions
- Procedural generation framework

### In Progress
- System implementations
- Bevy rendering pipeline
- UI development

### Upcoming
- Save/load system
- Steam integration
- Multiplayer support

## Contact

[Contact information to be added]

---

*Built with [Bevy](https://bevyengine.org/) - A refreshingly simple data-driven game engine built in Rust*