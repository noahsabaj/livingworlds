//! UI interaction module gateway
//!
//! This module contains types and logic for user interaction with the UI,
//! particularly province selection and other interactive elements.
//!
//! # Gateway Pattern
//!
//! This is a PURE gateway - no implementations, only module declarations
//! and controlled exports.

// PRIVATE MODULES - Implementation details
mod selection;

// CONTROLLED EXPORTS - Public API for UI interaction

// Selection-related types
pub use selection::SelectedProvinceInfo;