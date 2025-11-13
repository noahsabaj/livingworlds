//! Law passage and reform systems gateway
//!
//! Provides controlled access to law passage mechanics including
//! evaluation, voting, reforms, and revolutionary changes.

// Private modules - gateway architecture
mod evaluation;
mod reforms;
mod revolution;
mod types;
mod voting;

// Re-export core passage functions
pub use evaluation::evaluate_law_passage;
pub use reforms::{emergency_law_powers, process_law_reform};
pub use revolution::revolutionary_law_changes;
pub use voting::trigger_law_vote;

// Re-export types
pub use types::{RevolutionLawAction, LawVoteResult};