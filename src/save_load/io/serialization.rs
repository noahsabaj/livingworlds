//! Serialization operations for save data
//!
//! This module handles RON format serialization and deserialization.

use super::SaveGameData;

/// Serialize save data to RON format
pub fn serialize_save_data(data: &SaveGameData) -> Result<String, String> {
    ron::to_string(data).map_err(|e| format!("Failed to serialize save data: {:?}", e))
}

/// Deserialize save data from RON format
pub fn deserialize_save_data(data: &str) -> Result<SaveGameData, String> {
    ron::from_str(data).map_err(|e| format!("Failed to deserialize save data: {:?}", e))
}
