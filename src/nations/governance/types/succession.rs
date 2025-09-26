//! Succession and gender types for governance
//!
//! This module contains types related to succession mechanics and gender
//! representation in government systems.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// How succession works in different governments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum SuccessionType {
    Hereditary,    // Family inheritance
    Elective,      // Chosen by electors
    Appointment,   // Appointed by party/council
    Democratic,    // Elected by citizens
    Meritocratic,  // Based on competence
    Random,        // Sortition/lottery
    Rotation,      // Rotating positions
    Consensus,     // Unanimous agreement needed
    Combat,        // Trial by combat
    None,          // No succession (collective)
    Revolutionary, // Violent overthrow only
    Corporate,     // Board of directors
}

/// Gender representation in governance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum Gender {
    Male,
    Female,
    Neutral,
}