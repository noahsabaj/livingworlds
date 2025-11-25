//! Law history tracking
//!
//! Tracks historical trends and patterns in law adoption and repeal.

use bevy::prelude::*;
use std::collections::VecDeque;
// NationId deleted - now using Entity directly
use crate::nations::laws::types::LawId;
use crate::nations::NationId;
use super::types::LawChangeType;

/// Resource tracking historical law trends
#[derive(Resource, Default)]
pub struct LawHistory {
    /// Timeline of law changes globally
    pub global_timeline: VecDeque<(i32, NationId, LawId, LawChangeType)>,

    /// Most commonly adopted laws
    pub popular_laws: Vec<(LawId, u32)>,

    /// Most frequently repealed laws
    pub unstable_laws: Vec<(LawId, u32)>,

    /// Average lifespan of laws before repeal
    pub average_law_lifespan: f32,
}