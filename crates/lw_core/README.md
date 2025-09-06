# lw_core

Core mathematics and utilities for Living Worlds simulation.

## Purpose

Provides the mathematical foundation for deterministic simulation:
- Fixed-point arithmetic (Fixed32) for cross-platform determinism
- 2D vector operations (Vec2fx)
- Deterministic random number generation
- Bounded types (Percentage, UnitInterval)
- Shared type definitions used across all crates

## Key Components

- `fixed.rs` - Fixed-point math implementation
- `vector.rs` - 2D vector operations
- `random.rs` - Deterministic RNG
- `bounded_types.rs` - Types constrained to ranges
- `shared_types.rs` - Common types used everywhere