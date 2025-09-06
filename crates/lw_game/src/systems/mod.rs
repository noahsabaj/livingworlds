//! Game systems organized by domain
//! 
//! Each domain has its own subdirectory with related systems.
//! This mirrors the component organization for consistency.

// Domain-specific system modules
pub mod individual;
pub mod culture;
pub mod military;
pub mod geography;
pub mod economics;
pub mod governance;
pub mod core;
pub mod simulation_phases;

// Legacy system modules (to be organized)
pub mod event;
pub mod collapse;
pub mod technology;
pub mod time;
pub mod economy;
pub mod diplomacy;

// Re-export domain systems
pub use individual::*;
pub use culture::*;
pub use military::*;
pub use geography::*;
pub use core::*;

// Re-export legacy systems
pub use event::*;
pub use collapse::*;
pub use technology::*;
pub use time::*;
pub use economy::*;
pub use diplomacy::*;