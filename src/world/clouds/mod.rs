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
mod generation;
mod rendering;
mod types;
mod weather;

// PUBLIC EXPORTS - The only way to access cloud functionality

// Core types
pub use types::{CloudData, CloudEntity, CloudLayer, CloudSystem};

// Weather system
pub use weather::{WeatherState, WeatherSystem};

// Generation
pub use generation::CloudBuilder;

// Rendering components and systems
pub use rendering::{
    animate_clouds, create_cloud_texture, dynamic_cloud_spawn_system, generate_cloud_formation,
    update_weather_system, CloudFormationType, CloudPlugin, CloudSprite, CloudTextureParams,
};
