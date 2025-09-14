//! Cloud feature module gateway
//!
//! This module contains everything related to clouds in Living Worlds:
//! - Data structures (CloudSystem, CloudData, CloudLayer)
//! - Generation (CloudBuilder)
//! - Rendering (cloud animation and display)
//!
//! Following gateway architecture - all submodules are private and only
//! carefully selected APIs are exposed.

// PRIVATE MODULES - No direct access allowed
mod types;
mod generation;
mod rendering;
mod weather;

// PUBLIC EXPORTS - The only way to access cloud functionality

// Core types
pub use types::{CloudSystem, CloudData, CloudLayer, CloudEntity};

// Weather system
pub use weather::{WeatherState, WeatherSystem};

// Generation
pub use generation::CloudBuilder;

// Rendering components and systems
pub use rendering::{
    CloudPlugin,
    CloudSprite,
    CloudFormationType,
    generate_cloud_formation,
    create_cloud_texture,
    CloudTextureParams,
    animate_clouds,
    update_weather_system,
    dynamic_cloud_spawn_system,
};