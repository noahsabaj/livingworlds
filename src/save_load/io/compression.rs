//! Compression operations for save files
//!
//! This module handles zstd compression and decompression of save data.

use zstd::stream::{encode_all, decode_all};

/// Compress data using zstd
pub fn compress_data(data: &[u8]) -> Result<Vec<u8>, String> {
    encode_all(data, 3).map_err(|e| format!("Failed to compress data: {}", e))
}

/// Decompress data using zstd
pub fn decompress_data(compressed: &[u8]) -> Result<Vec<u8>, String> {
    decode_all(compressed).map_err(|e| format!("Failed to decompress data: {}", e))
}