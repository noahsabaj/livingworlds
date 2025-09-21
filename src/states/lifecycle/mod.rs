//! State Lifecycle Management Gateway
//!
//! This module provides comprehensive lifecycle management for all game states,
//! organized by functional areas for maintainability and clear separation of concerns.
//!
//! ## Gateway Architecture
//!
//! This module follows Living Worlds' gateway architecture pattern. All submodules
//! are private, and external access is controlled through this mod.rs file.
//!
//! ## Functional Areas
//!
//! - **Loading**: Asset loading workflows and loading screen management
//! - **Menus**: Main menu and pause menu entity/camera management
//! - **Configuration**: World configuration and generation state flows
//! - **Gameplay**: In-game state entry/exit and world mesh spawning
//! - **Errors**: Error state handling and dialog management
//! - **Utils**: Cross-cutting lifecycle utilities and logging
//!
//! ## Usage
//!
//! ```rust
//! use crate::states::lifecycle::{enter_loading, exit_loading};
//!
//! // Functions are imported directly from the lifecycle gateway
//! app.add_systems(OnEnter(GameState::Loading), enter_loading);
//! ```

// Private submodules - all access controlled through this gateway
mod configuration;
mod errors;
mod gameplay;
mod loading;
mod menus;
mod utils;

// Public exports - controlled API surface

// Loading state lifecycle
pub use loading::{enter_loading, enter_loading_world, exit_loading, exit_loading_world};

// Menu state lifecycle
pub use menus::{enter_main_menu, enter_paused, exit_main_menu, exit_paused};

// Configuration state lifecycle
pub use configuration::{
    enter_world_configuration, enter_world_generation, exit_world_configuration,
    exit_world_generation,
};

// Gameplay state lifecycle
pub use gameplay::{enter_in_game, exit_in_game};

// Error state lifecycle
pub use errors::{
    enter_world_generation_failed, exit_world_generation_failed, handle_error_dialog_buttons,
};

// Lifecycle utilities
pub use utils::{check_and_trigger_world_generation, log_state_changes};
