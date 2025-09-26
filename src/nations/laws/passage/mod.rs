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
pub use evaluation::{calculate_debate_duration, check_prerequisites, evaluate_law_passage};
pub use reforms::{emergency_law_powers, process_law_reform};
pub use revolution::{is_revolutionary_transition, revolutionary_law_changes};
pub use voting::{calculate_passage_threshold, trigger_law_vote};

// Re-export types
pub use types::{EmergencyPower, RevolutionLawAction, LawProposal, LawVoteResult};