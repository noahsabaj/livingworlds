//! Deterministic random number generator for consistent simulation
//!
//! Uses ChaCha8 for fast, deterministic random number generation that
//! produces the same sequence across all platforms.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use crate::Fixed32;

/// Deterministic random number generator
/// 
/// Provides reproducible random numbers for game simulation.
/// Always produces the same sequence for the same seed.
#[derive(Clone, Debug)]
pub struct DeterministicRNG {
    rng: ChaCha8Rng,
    seed: u64,
}

// Custom serialization - only store the seed
impl Serialize for DeterministicRNG {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.seed.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DeterministicRNG {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let seed = u64::deserialize(deserializer)?;
        Ok(DeterministicRNG::new(seed))
    }
}

impl DeterministicRNG {
    /// Create a new RNG with the given seed
    pub fn new(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
            seed,
        }
    }
    
    /// Check if the RNG has been initialized (seed != 0)
    pub fn is_initialized(&self) -> bool {
        self.seed != 0
    }
    
    /// Reset to original seed
    pub fn reset(&mut self) {
        self.rng = ChaCha8Rng::seed_from_u64(self.seed);
    }
    
    /// Set a new seed
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
        self.rng = ChaCha8Rng::seed_from_u64(seed);
    }
    
    /// Get current seed
    pub fn seed(&self) -> u64 {
        self.seed
    }
    
    /// Generate a random u32
    pub fn next_u32(&mut self) -> u32 {
        self.rng.gen()
    }
    
    /// Generate a random u64
    pub fn next_u64(&mut self) -> u64 {
        self.rng.gen()
    }
    
    /// Generate a random i32
    pub fn next_i32(&mut self) -> i32 {
        self.rng.gen()
    }
    
    /// Generate a random f32 in [0, 1)
    pub fn next_f32(&mut self) -> f32 {
        self.rng.gen()
    }
    
    /// Generate a random f64 in [0, 1)
    pub fn next_f64(&mut self) -> f64 {
        self.rng.gen()
    }
    
    /// Generate a random Fixed32 in [0, 1)
    pub fn next_fixed(&mut self) -> Fixed32 {
        Fixed32::from_float(self.next_f32())
    }
    
    /// Generate a random bool with probability p of being true
    pub fn next_bool(&mut self, p: f32) -> bool {
        self.next_f32() < p
    }
    
    /// Generate a random integer in range [min, max)
    pub fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            return min;
        }
        self.rng.gen_range(min..max)
    }
    
    /// Generate a random usize in range [min, max)
    pub fn range_usize(&mut self, min: usize, max: usize) -> usize {
        if min >= max {
            return min;
        }
        self.rng.gen_range(min..max)
    }
    
    /// Generate a random f32 in range [min, max)
    pub fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        if min >= max {
            return min;
        }
        self.rng.gen_range(min..max)
    }
    
    /// Generate a random Fixed32 in range [min, max)
    pub fn range_fixed(&mut self, min: Fixed32, max: Fixed32) -> Fixed32 {
        if min >= max {
            return min;
        }
        let t = self.next_fixed();
        min + (max - min) * t
    }
    
    /// Shuffle a slice randomly
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let len = slice.len();
        if len <= 1 {
            return;
        }
        
        for i in (1..len).rev() {
            let j = self.range_usize(0, i + 1);
            slice.swap(i, j);
        }
    }
    
    /// Choose a random element from a slice
    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            let idx = self.range_usize(0, slice.len());
            Some(&slice[idx])
        }
    }
    
    /// Choose a random mutable element from a slice
    pub fn choose_mut<'a, T>(&mut self, slice: &'a mut [T]) -> Option<&'a mut T> {
        if slice.is_empty() {
            None
        } else {
            let idx = self.range_usize(0, slice.len());
            Some(&mut slice[idx])
        }
    }
    
    /// Choose multiple unique random elements from a slice
    pub fn choose_multiple<'a, T>(&mut self, slice: &'a [T], amount: usize) -> Vec<&'a T> {
        let len = slice.len();
        let amount = amount.min(len);
        
        if amount == 0 {
            return Vec::new();
        }
        
        if amount == len {
            return slice.iter().collect();
        }
        
        // For small amounts, use simple random selection
        if amount < len / 4 {
            let mut result = Vec::with_capacity(amount);
            let mut used = vec![false; len];
            
            while result.len() < amount {
                let idx = self.range_usize(0, len);
                if !used[idx] {
                    used[idx] = true;
                    result.push(&slice[idx]);
                }
            }
            
            result
        } else {
            // For larger amounts, shuffle indices
            let mut indices: Vec<usize> = (0..len).collect();
            self.shuffle(&mut indices);
            indices.truncate(amount);
            indices.into_iter().map(|i| &slice[i]).collect()
        }
    }
    
    /// Generate a random point in a unit circle
    pub fn in_unit_circle(&mut self) -> (Fixed32, Fixed32) {
        loop {
            let x = self.range_fixed(Fixed32::from_num(-1), Fixed32::ONE);
            let y = self.range_fixed(Fixed32::from_num(-1), Fixed32::ONE);
            
            if x * x + y * y <= Fixed32::ONE {
                return (x, y);
            }
        }
    }
    
    /// Generate a random point on a unit circle
    pub fn on_unit_circle(&mut self) -> (Fixed32, Fixed32) {
        let angle = self.range_fixed(Fixed32::ZERO, Fixed32::TAU);
        // TODO: Implement fixed-point sin/cos
        let cos_a = Fixed32::from_float(angle.to_f32().cos());
        let sin_a = Fixed32::from_float(angle.to_f32().sin());
        (cos_a, sin_a)
    }
    
    /// Normal distribution (Box-Muller transform)
    pub fn normal(&mut self, mean: f32, std_dev: f32) -> f32 {
        let u1 = self.next_f32();
        let u2 = self.next_f32();
        
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).cos();
        mean + z0 * std_dev
    }
    
    /// Weighted choice - returns index based on weights
    pub fn weighted_choice(&mut self, weights: &[f32]) -> Option<usize> {
        if weights.is_empty() {
            return None;
        }
        
        let total: f32 = weights.iter().sum();
        if total <= 0.0 {
            return None;
        }
        
        let mut threshold = self.range_f32(0.0, total);
        
        for (i, &weight) in weights.iter().enumerate() {
            threshold -= weight;
            if threshold <= 0.0 {
                return Some(i);
            }
        }
        
        Some(weights.len() - 1)
    }
}

impl Default for DeterministicRNG {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_determinism() {
        let mut rng1 = DeterministicRNG::new(12345);
        let mut rng2 = DeterministicRNG::new(12345);
        
        for _ in 0..100 {
            assert_eq!(rng1.next_u32(), rng2.next_u32());
        }
    }
    
    #[test]
    fn test_reset() {
        let mut rng = DeterministicRNG::new(42);
        
        let val1 = rng.next_u32();
        let val2 = rng.next_u32();
        
        rng.reset();
        
        assert_eq!(val1, rng.next_u32());
        assert_eq!(val2, rng.next_u32());
    }
    
    #[test]
    fn test_range() {
        let mut rng = DeterministicRNG::new(999);
        
        for _ in 0..100 {
            let val = rng.range_i32(10, 20);
            assert!(val >= 10 && val < 20);
        }
    }
    
    #[test]
    fn test_shuffle() {
        let mut rng = DeterministicRNG::new(111);
        let mut array = [1, 2, 3, 4, 5];
        
        rng.shuffle(&mut array);
        
        // Check that all elements are still present
        let mut sorted = array;
        sorted.sort();
        assert_eq!(sorted, [1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_weighted_choice() {
        let mut rng = DeterministicRNG::new(777);
        let weights = [1.0, 2.0, 3.0, 4.0];
        
        let mut counts = [0; 4];
        for _ in 0..1000 {
            if let Some(idx) = rng.weighted_choice(&weights) {
                counts[idx] += 1;
            }
        }
        
        // Check that higher weights were chosen more often
        assert!(counts[3] > counts[0]);
        assert!(counts[2] > counts[0]);
    }
}