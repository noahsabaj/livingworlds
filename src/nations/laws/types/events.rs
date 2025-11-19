//! Law-related events
//!
//! Events fired when laws are enacted, repealed, or modified.

use bevy::prelude::*;

use super::core::{LawId, LawCategory};

/// Event fired when a law is enacted
#[derive(Message, Debug, Clone)]
pub struct LawEnactmentEvent {
    pub nation_entity: Entity,
    pub nation_name: String,
    pub law_id: LawId,
    pub law_name: String,
    pub category: LawCategory,
}

/// Event fired when a law is repealed
#[derive(Message, Debug, Clone)]
pub struct LawRepealEvent {
    pub nation_entity: Entity,
    pub nation_name: String,
    pub law_id: LawId,
    pub law_name: String,
    pub category: LawCategory,
    pub years_active: i32,
}