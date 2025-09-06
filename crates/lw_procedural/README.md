# lw_procedural

Procedural generation algorithms for Living Worlds.

## Purpose

Generates all game content procedurally at runtime - no pre-made assets.
Everything from terrain to music is created algorithmically.

## Components

- `terrain.rs` - Heightmap and biome generation
- `provinces.rs` - Voronoi-based territory creation
- `names.rs` - Culture-specific name generation
- `audio.rs` - Procedural music and sound effects
- `font.rs` - Runtime font generation
- `palette.rs` - Dynamic color scheme generation

## Key Features

- Deterministic generation from seeds
- No shipped assets required
- Infinite variety of worlds
- Culture-aware content generation