//! Development level types for governance selection
//!
//! This module defines the development levels used to determine appropriate
//! government types for different eras and cultures.

/// Development level for determining appropriate government types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevelopmentLevel {
    Primitive,
    Medieval,
    Renaissance,
    Modern,
}

impl From<crate::nations::types::StartingDevelopment> for DevelopmentLevel {
    fn from(starting: crate::nations::types::StartingDevelopment) -> Self {
        match starting {
            crate::nations::types::StartingDevelopment::Primitive => DevelopmentLevel::Primitive,
            crate::nations::types::StartingDevelopment::Medieval => DevelopmentLevel::Medieval,
            crate::nations::types::StartingDevelopment::Renaissance => DevelopmentLevel::Renaissance,
            crate::nations::types::StartingDevelopment::Mixed => DevelopmentLevel::Medieval, // Default to medieval for mixed
        }
    }
}