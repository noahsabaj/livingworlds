//! Warfare module - Battle resolution, war management, and combat systems
//!
//! This module implements:
//! - Auto-resolve battle system with dice rolls
//! - War state tracking (goals, participants, war score)
//! - War declaration and resolution systems

mod battle;
mod war;
mod systems;

pub use battle::{Battle, BattleConfig, BattleResult, record_battle_outcome};
pub use war::{War, WarGoal, WarOutcome, CasusBelli};
pub use systems::{
    DeclareWarEvent, BattleEvent, WarEndEvent, process_war_declarations, process_battle_events,
    check_war_resolution,
};
