//! Fast metadata extraction from save files
//!
//! This module provides efficient extraction of save metadata without
//! fully decompressing and parsing entire save files.

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use zstd::stream::Decoder;

/// Extract minimal metadata from a save file efficiently
/// Only reads the first 8KB of the compressed file to avoid performance issues
pub fn extract_save_metadata(path: &Path) -> Option<(String, f32, u32, String, u32)> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let mut limited_reader = (&mut reader).take(8192); // 8KB should be enough for metadata
    let mut compressed_chunk = Vec::new();
    limited_reader.read_to_end(&mut compressed_chunk).ok()?;

    // Try to decompress what we have
    let decompressed = match super::decompress_data(&compressed_chunk) {
        Ok(data) => data,
        Err(_) => {
            // Fallback: read the full file if partial decompression fails
            let mut file = File::open(path).ok()?;
            let mut compressed_data = Vec::new();
            file.read_to_end(&mut compressed_data).ok()?;

            // But only decompress the first 64KB of content to keep it fast
            let mut decoder = Decoder::new(&compressed_data[..]).ok()?;
            let mut partial_decompressed = Vec::new();
            let _ = (&mut decoder).take(65536).read_to_end(&mut partial_decompressed);
            partial_decompressed
        }
    };

    // Convert only what we need to string (first 16KB should have all metadata)
    let check_len = decompressed.len().min(16384);
    let data_str = String::from_utf8_lossy(&decompressed[..check_len]);

    // Extract world_size
    let world_size = if data_str.contains("world_size: Small") {
        "Small".to_string()
    } else if data_str.contains("world_size: Medium") {
        "Medium".to_string()
    } else if data_str.contains("world_size: Large") {
        "Large".to_string()
    } else {
        "Unknown".to_string()
    };

    // Extract game_time
    let game_time = extract_field_f32(&data_str, "current_date:");

    // Extract version
    let version = extract_field_u32(&data_str, "version:");

    // Extract world_name
    let world_name = extract_quoted_string(&data_str, "world_name:");

    // Extract world_seed
    let world_seed = extract_field_u32(&data_str, "world_seed:");

    Some((world_size, game_time, version, world_name, world_seed))
}

fn extract_field_f32(data: &str, field_name: &str) -> f32 {
    if let Some(idx) = data.find(field_name) {
        let substr = &data[idx + field_name.len()..];
        if let Some(end_idx) = substr.find(|c: char| c == ',' || c == '\n') {
            substr[..end_idx].trim().parse::<f32>().unwrap_or(0.0)
        } else {
            0.0
        }
    } else {
        0.0
    }
}

fn extract_field_u32(data: &str, field_name: &str) -> u32 {
    if let Some(idx) = data.find(field_name) {
        let substr = &data[idx + field_name.len()..];
        if let Some(end_idx) = substr.find(|c: char| c == ',' || c == '\n') {
            substr[..end_idx].trim().parse::<u32>().unwrap_or(0)
        } else {
            0
        }
    } else {
        0
    }
}

fn extract_quoted_string(data: &str, field_name: &str) -> String {
    if let Some(idx) = data.find(field_name) {
        let substr = &data[idx + field_name.len()..];
        if let Some(start_quote) = substr.find('"') {
            let string_start = &substr[start_quote + 1..];
            if let Some(end_quote) = string_start.find('"') {
                return string_start[..end_quote].to_string();
            }
        }
    }
    "Unknown".to_string()
}