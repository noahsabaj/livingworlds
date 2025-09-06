//! Simulation Phases
//! 
//! Breaking up the SimulationState god object into focused phase systems.
//! Each phase is a separate system that handles one aspect of the simulation.

pub mod individual_decisions;
pub mod economic_emergence;
pub mod government_response;
pub mod cultural_transmission;
pub mod military_actions;
pub mod diplomatic_evolution;
pub mod world_changes;
pub mod demographic_transition;
pub mod synchronization;

// Re-export all phase systems
pub use individual_decisions::*;
pub use economic_emergence::*;
pub use government_response::*;
pub use cultural_transmission::*;
pub use military_actions::*;
pub use diplomatic_evolution::*;
pub use world_changes::*;
pub use demographic_transition::*;
pub use synchronization::*;