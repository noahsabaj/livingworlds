# Living Worlds

<p align="center">
  <img src="images/world-generation-hero.png" alt="Living Worlds - Procedurally generated world with continents, rivers, and archipelagos" width="100%">
</p>

<p align="center">
  <i>A hands-off observer simulation built with Bevy where you WATCH (not control) empires rise and fall eternally through emergent gameplay.</i>
</p>

## Overview

Living Worlds is a fully procedural civilization OBSERVER - like Fantasy Map Simulator, you have zero control over the civilizations. You can only watch as they emerge, grow, fight, and collapse. Every texture, sound, and piece of text is generated at runtime. Observe as civilizations develop organically, advance through technologies at their own pace, build infrastructure that permanently marks the landscape, and manage complex economies based on Austrian economic principles. There is no victory condition and no player interaction - only the eternal cycle of rise and fall that you witness as a passive observer.

## Features

- **Pure Observer Mode**: Like watching a digital ant farm - you cannot control anything
- **Fully Procedural**: Everything generated at runtime - no pre-made assets
- **Individual Simulation**: Every person is simulated with needs, skills, and decisions  
- **Austrian Economics**: Prices emerge from individual actions, not abstract models
- **Emergent Stories**: No scripted events - watch unique histories unfold
- **Time Controls Only**: Your only power is pause/play/speed up time
- **Prototype-First Development**: Entire game in main.rs for rapid iteration
- **Deterministic Simulation**: Fixed-point math ensures cross-platform consistency

## Technology Stack

- **Engine**: Bevy 0.16.1 (Rust game engine)
- **Language**: Rust 2021 Edition
- **Graphics**: wgpu (modern GPU API)
- **Math**: Fixed-point arithmetic for determinism
- **Platform**: Windows, Linux (Steam distribution)

## Architecture

Currently in **prototype phase** with the entire game in a single `src/main.rs` file. This allows for:
- Rapid experimentation and iteration
- Easy refactoring without cross-crate concerns  
- Fast compile times
- Clear view of all game logic in one place

### What's Implemented
- Hexagonal province grid (300x200 = 60,000 provinces)
- Procedural terrain generation with Perlin noise
- Nation spawning and territory assignment
- Camera controls and province border visualization
- Time simulation with pause/play/speed controls
- Responsive UI with simulation statistics

### Future Architecture
Once core mechanics are proven, we'll refactor into a proper crate structure with specialized layers for core math, simulation, platform integration, and game orchestration.

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

# Check the project
cargo check
```

## Project Structure

```
livingworlds/
├── src/
│   └── main.rs         # ENTIRE GAME - All components, systems, and logic
├── Cargo.toml          # Project configuration
├── Cargo.lock          # Dependency lock file  
├── CLAUDE.md           # Detailed technical documentation
├── README.md           # This file
├── .gitignore          # Git ignore configuration
└── target/             # Build output (git-ignored)

NOTE: No crates/ directory - deleted to focus on prototyping
NOTE: No assets/ directory - everything is procedurally generated
```

## Documentation

- See `CLAUDE.md` for detailed technical documentation and development guidelines
- The entire game logic is in `src/main.rs` with inline comments
- Follow the "don't buy the wagon before the horse" principle - prototype first!

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
- Layer-based architecture (refactored from 10 to 4 crates)
- Fixed-point math system
- ECS component consolidation
- Procedural generation framework (integrated with Bevy)

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