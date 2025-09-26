//! Law-related events
//!
//! Events fired when laws are enacted, repealed, or modified.

use bevy::prelude::*;

use super::core::{LawId, LawCategory};
use crate::nations::NationId;

/// Event fired when a law is enacted
#[derive(Event, Debug, Clone)]
pub struct LawEnactmentEvent {
    pub nation_id: NationId,
    pub nation_name: String,
    pub law_id: LawId,
    pub law_name: String,
    pub category: LawCategory,
}

/// Event fired when a law is repealed
#[derive(Event, Debug, Clone)]
pub struct LawRepealEvent {
    pub nation_id: NationId,
    pub nation_name: String,
    pub law_id: LawId,
    pub law_name: String,
    pub category: LawCategory,
    pub years_active: i32,
}