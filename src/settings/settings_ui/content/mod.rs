//! Settings Tab Content Subsystem - Gateway
//!
//! Gateway for all tab content creation functionality. Each tab has its own
//! focused module for spawning appropriate content.
//!
//! All manual implementations have been replaced with declarative versions using
//! the define_setting_tab! macro. This eliminates 400+ lines of repetitive code
//! while providing complete field coverage and type safety.

// PRIVATE MODULES - Implementation hidden (DECLARATIVE VERSIONS)
mod audio_declarative;
mod controls_declarative;
mod graphics_declarative;
mod interface_declarative;
mod performance;

// CONTROLLED EXPORTS - Generated spawning functions from macros
pub use audio_declarative::spawn_audiotabdeclarative_content;
pub use controls_declarative::spawn_controlstabdeclarative_content;
pub use graphics_declarative::spawn_graphicstabdeclarative_content;
pub use interface_declarative::spawn_interfacetabdeclarative_content;
pub use performance::spawn_performance_content;

// CONTROLLED EXPORTS - Generated event handlers from macros
pub use audio_declarative::handle_audiotabdeclarative_interactions;
pub use controls_declarative::handle_controlstabdeclarative_interactions;
pub use graphics_declarative::handle_graphicstabdeclarative_interactions;
pub use interface_declarative::handle_interfacetabdeclarative_interactions;
