//! Procedural name generation using Markov chains

use lw_core::DeterministicRNG;

pub struct NameGenerator {
    rng: DeterministicRNG,
    syllables: Vec<String>,
}

impl NameGenerator {
    pub fn new(seed: u64) -> Self {
        let syllables = vec![
            // Common syllables for fantasy names
            "ar", "el", "or", "an", "th", "gar", "mor", "dor", 
            "val", "kar", "tor", "mar", "nor", "sar", "bar", "far",
            "ia", "ea", "os", "us", "is", "as", "um", "em",
            "oth", "eth", "ath", "ith", "ain", "ein", "oin",
            "land", "gard", "heim", "stad", "burg", "ford", "shire",
        ].iter().map(|s| s.to_string()).collect();
        
        Self {
            rng: DeterministicRNG::new(seed),
            syllables,
        }
    }
    
    pub fn generate_nation_name(&mut self) -> String {
        let syllable_count = self.rng.range_i32(2, 4);
        let mut name = String::new();
        
        for _ in 0..syllable_count {
            if let Some(syllable) = self.rng.choose(&self.syllables) {
                name.push_str(syllable);
            }
        }
        
        // Capitalize first letter
        if let Some(first) = name.chars().next() {
            name = first.to_uppercase().collect::<String>() + &name[1..];
        }
        
        // Add suffix sometimes
        if self.rng.next_bool(0.3) {
            let suffixes = ["ia", "land", "burg", "reich", "stan"];
            if let Some(suffix) = self.rng.choose(&suffixes) {
                name.push_str(suffix);
            }
        }
        
        name
    }
    
    pub fn generate_city_name(&mut self) -> String {
        let prefixes = ["New ", "Old ", "North ", "South ", "East ", "West ", "Upper ", "Lower "];
        let mut name = String::new();
        
        // Sometimes add prefix
        if self.rng.next_bool(0.2) {
            if let Some(prefix) = self.rng.choose(&prefixes) {
                name.push_str(prefix);
            }
        }
        
        // Base name
        let syllable_count = self.rng.range_i32(1, 3);
        for _ in 0..syllable_count {
            if let Some(syllable) = self.rng.choose(&self.syllables) {
                name.push_str(syllable);
            }
        }
        
        // City suffixes
        if self.rng.next_bool(0.4) {
            let suffixes = ["ton", "ville", "ford", "bridge", "haven", "port"];
            if let Some(suffix) = self.rng.choose(&suffixes) {
                name.push_str(suffix);
            }
        }
        
        // Capitalize
        if let Some(first) = name.chars().next() {
            if !first.is_uppercase() {
                name = first.to_uppercase().collect::<String>() + &name[1..];
            }
        }
        
        name
    }
    
    pub fn generate_character_name(&mut self) -> String {
        let first = self.generate_first_name();
        let last = self.generate_last_name();
        format!("{} {}", first, last)
    }
    
    fn generate_first_name(&mut self) -> String {
        let syllable_count = self.rng.range_i32(1, 3);
        let mut name = String::new();
        
        for _ in 0..syllable_count {
            if let Some(syllable) = self.rng.choose(&self.syllables) {
                name.push_str(syllable);
            }
        }
        
        // Capitalize
        if let Some(first) = name.chars().next() {
            name = first.to_uppercase().collect::<String>() + &name[1..];
        }
        
        name
    }
    
    fn generate_last_name(&mut self) -> String {
        let mut name = self.generate_first_name();
        
        // Add family name suffix
        if self.rng.next_bool(0.5) {
            let suffixes = ["son", "sen", "ovich", "ez", "ski"];
            if let Some(suffix) = self.rng.choose(&suffixes) {
                name.push_str(suffix);
            }
        }
        
        name
    }
}