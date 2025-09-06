# lw_game

Main game orchestration plugin for Living Worlds.

## Purpose

Top-level game plugin that integrates all domain plugins into a cohesive whole.
This crate acts as the primary entry point for the Bevy application.

## Components

- `LivingWorldsPlugin` - Main plugin that registers all domain plugins
- Game state management
- Top-level systems coordination
- Integration of all domain-specific plugins

## Domain Integration

Brings together:
- Economics systems
- Military systems
- Cultural systems
- Governance systems
- World/geography systems
- Simulation orchestration