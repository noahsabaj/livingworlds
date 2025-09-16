//! Random number generation utilities
//!
//! This module provides centralized random number generation utilities
//! for Living Worlds. It ensures consistent seeding and provides

use bevy::prelude::Vec2;
use rand::{rngs::StdRng, Rng, SeedableRng};

///
/// This is the standard way to create an RNG in Living Worlds.
/// Using the same seed will produce the same sequence of random numbers.
///
/// # Example
/// ```
/// let mut rng = create_rng(12345);
/// let value = random_range(&mut rng, 0.0, 1.0);
/// ```
#[inline]
pub fn create_rng(seed: u32) -> StdRng {
    StdRng::seed_from_u64(seed as u64)
}

///
/// Useful when you need to combine multiple factors into a seed
///
/// # Example
/// ```
/// let mut rng = create_rng_multi(&[world_seed, chunk_x, chunk_y]);
/// ```
pub fn create_rng_multi(seeds: &[u32]) -> StdRng {
    // Combine seeds using a simple hash-like operation
    let combined = seeds.iter().enumerate().fold(0u64, |acc, (i, &seed)| {
        acc.wrapping_add((seed as u64).wrapping_mul(2654435761u64.wrapping_add(i as u64)))
    });
    StdRng::seed_from_u64(combined)
}

///
/// # Example
/// ```
/// let health = random_range(&mut rng, 50.0, 100.0);
/// ```
#[inline]
pub fn random_range<T>(rng: &mut StdRng, min: T, max: T) -> T
where
    T: rand::distributions::uniform::SampleUniform + PartialOrd,
{
    rng.gen_range(min..max)
}

///
/// # Example
/// ```
/// if random_bool(&mut rng, 0.3) {
///     // 30% chance this executes
/// }
/// ```
#[inline]
pub fn random_bool(rng: &mut StdRng, probability: f32) -> bool {
    rng.r#gen::<f32>() < probability
}

#[inline]
pub fn random_01(rng: &mut StdRng) -> f32 {
    rng.r#gen()
}

#[inline]
pub fn random_11(rng: &mut StdRng) -> f32 {
    rng.r#gen::<f32>() * 2.0 - 1.0
}

///
/// # Example
/// ```
/// let pos = random_point_in_rect(&mut rng, 0.0, 0.0, 100.0, 100.0);
/// ```
#[inline]
pub fn random_point_in_rect(rng: &mut StdRng, x: f32, y: f32, width: f32, height: f32) -> Vec2 {
    Vec2::new(
        x + random_range(rng, 0.0, width),
        y + random_range(rng, 0.0, height),
    )
}

///
/// Uses sqrt for uniform distribution (not clustered at center)
///
/// # Example
/// ```
/// let pos = random_point_in_circle(&mut rng, 50.0, 50.0, 25.0);
/// ```
pub fn random_point_in_circle(rng: &mut StdRng, center_x: f32, center_y: f32, radius: f32) -> Vec2 {
    let angle = random_range(rng, 0.0, std::f32::consts::TAU);
    let r = radius * random_01(rng).sqrt(); // sqrt for uniform distribution

    Vec2::new(center_x + angle.cos() * r, center_y + angle.sin() * r)
}

///
/// # Example
/// ```
/// let pos = random_point_on_circle(&mut rng, 50.0, 50.0, 25.0);
/// ```
#[inline]
pub fn random_point_on_circle(rng: &mut StdRng, center_x: f32, center_y: f32, radius: f32) -> Vec2 {
    let angle = random_range(rng, 0.0, std::f32::consts::TAU);
    Vec2::new(
        center_x + angle.cos() * radius,
        center_y + angle.sin() * radius,
    )
}

///
/// Approximates hexagon as a circle for simplicity
///
/// # Example
/// ```
/// let offset = random_hex_offset(&mut rng, HEX_SIZE);
/// let final_pos = hex_center + offset;
/// ```
#[inline]
pub fn random_hex_offset(rng: &mut StdRng, hex_size: f32) -> Vec2 {
    // Approximate hexagon as circle with radius slightly less than hex_size
    // to ensure we stay within bounds
    let effective_radius = hex_size * 0.866; // ~sqrt(3)/2
    let angle = random_range(rng, 0.0, std::f32::consts::TAU);
    let r = effective_radius * random_01(rng).sqrt();

    Vec2::new(angle.cos() * r, angle.sin() * r)
}

///
/// Useful for random directions
#[inline]
pub fn random_unit_vector(rng: &mut StdRng) -> Vec2 {
    let angle = random_range(rng, 0.0, std::f32::consts::TAU);
    Vec2::new(angle.cos(), angle.sin())
}

#[inline]
pub fn random_vector(rng: &mut StdRng, magnitude: f32) -> Vec2 {
    random_unit_vector(rng) * magnitude
}

///
/// # Example
/// ```
/// let height = random_normal(&mut rng, 170.0, 10.0); // Mean 170cm, stddev 10cm
/// ```
pub fn random_normal(rng: &mut StdRng, mean: f32, std_dev: f32) -> f32 {
    // Box-Muller transform for normal distribution
    let u1 = random_01(rng);
    let u2 = random_01(rng);

    let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).cos();
    mean + z0 * std_dev
}

///
/// Useful for timing events, decay processes
///
/// # Example
/// ```
/// let time_until_event = random_exponential(&mut rng, 1.0 / 60.0); // Average 60 seconds
/// ```
#[inline]
pub fn random_exponential(rng: &mut StdRng, lambda: f32) -> f32 {
    -random_01(rng).ln() / lambda
}

/// Choose a weighted random index
///
/// # Example
/// ```
/// let weights = vec![1.0, 2.0, 1.0]; // Middle option twice as likely
/// let chosen = random_weighted_index(&mut rng, &weights);
/// ```
pub fn random_weighted_index(rng: &mut StdRng, weights: &[f32]) -> Option<usize> {
    if weights.is_empty() {
        return None;
    }

    let total: f32 = weights.iter().sum();
    if total <= 0.0 {
        return None;
    }

    let mut roll = random_range(rng, 0.0, total);

    for (i, &weight) in weights.iter().enumerate() {
        roll -= weight;
        if roll <= 0.0 {
            return Some(i);
        }
    }

    // Fallback (shouldn't happen with valid weights)
    Some(weights.len() - 1)
}

// GAME-SPECIFIC UTILITIES

///
/// Returns a value between (1.0 - variation) and (1.0 + variation)
///
/// # Example
/// ```
/// let price = base_price * random_variation(&mut rng, 0.1); // ±10% variation
/// ```
#[inline]
pub fn random_variation(rng: &mut StdRng, variation: f32) -> f32 {
    1.0 + random_11(rng) * variation
}

/// Generate multiple random positions with minimum spacing
///
/// Useful for placing objects that shouldn't overlap
///
/// # Example
/// ```
/// let positions = random_spaced_positions(&mut rng, 10, 0.0, 0.0, 100.0, 100.0, 5.0);
/// ```
pub fn random_spaced_positions(
    rng: &mut StdRng,
    count: usize,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    min_spacing: f32,
) -> Vec<Vec2> {
    let mut positions = Vec::with_capacity(count);
    let max_attempts = count * 100; // Prevent infinite loop
    let mut attempts = 0;

    while positions.len() < count && attempts < max_attempts {
        attempts += 1;
        let candidate = random_point_in_rect(rng, x, y, width, height);

        let valid = positions
            .iter()
            .all(|pos: &Vec2| pos.distance(candidate) >= min_spacing);

        if valid {
            positions.push(candidate);
        }
    }

    positions
}

///
/// Useful for adding variety to terrain colors
///
/// Returns (hue_shift, saturation_mult, value_mult)
#[inline]
pub fn random_color_variation(rng: &mut StdRng, amount: f32) -> (f32, f32, f32) {
    (
        random_11(rng) * amount * 0.1,       // Hue shift (small)
        random_variation(rng, amount * 0.2), // Saturation multiplier
        random_variation(rng, amount * 0.1), // Value multiplier
    )
}

/// Randomly shuffle a slice in-place
///
/// # Example
/// ```
/// let mut items = vec![1, 2, 3, 4, 5];
/// shuffle(&mut rng, &mut items);
/// ```
pub fn shuffle<T>(rng: &mut StdRng, slice: &mut [T]) {
    use rand::seq::SliceRandom;
    slice.shuffle(rng);
}

/// Choose a random element from a slice
///
/// # Example
/// ```
/// let names = vec!["Alice", "Bob", "Charlie"];
/// if let Some(name) = choose(&mut rng, &names) {
///     println!("Chosen: {}", name);
/// }
/// ```
pub fn choose<'a, T>(rng: &mut StdRng, slice: &'a [T]) -> Option<&'a T> {
    use rand::seq::SliceRandom;
    slice.choose(rng)
}

/// Choose multiple unique random elements from a slice
///
/// # Example
/// ```
/// let items = vec![1, 2, 3, 4, 5];
/// let chosen = choose_multiple(&mut rng, &items, 3);
/// ```
pub fn choose_multiple<'a, T>(rng: &mut StdRng, slice: &'a [T], amount: usize) -> Vec<&'a T> {
    use rand::seq::SliceRandom;
    slice.choose_multiple(rng, amount).collect()
}

// DETERMINISTIC PSEUDO-RANDOM

///
/// Always returns the same value for the same inputs.
/// No RNG needed.
///
/// # Example
/// ```
/// let noise = hash_random(x, y, seed);
/// ```
#[inline]
pub fn hash_random(x: f32, y: f32, seed: u32) -> f32 {
    let hash = ((x * 12.9898 + y * 78.233) * (seed as f32 * 0.001)).sin() * 43758.5453;
    hash.fract()
}

#[inline]
pub fn hash_random_int(x: i32, y: i32, seed: u32) -> f32 {
    hash_random(x as f32, y as f32, seed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng_creation() {
        let mut rng1 = create_rng(12345);
        let mut rng2 = create_rng(12345);

        // Same seed should produce same values
        assert_eq!(random_01(&mut rng1), random_01(&mut rng2));
    }

    #[test]
    fn test_random_ranges() {
        let mut rng = create_rng(42);

        for _ in 0..100 {
            let val = random_range(&mut rng, 10.0, 20.0);
            assert!(val >= 10.0 && val < 20.0);

            let val = random_11(&mut rng);
            assert!(val >= -1.0 && val <= 1.0);
        }
    }

    #[test]
    fn test_random_in_circle() {
        let mut rng = create_rng(42);
        let center = Vec2::new(50.0, 50.0);
        let radius = 25.0;

        for _ in 0..100 {
            let point = random_point_in_circle(&mut rng, center.x, center.y, radius);
            let distance = point.distance(center);
            assert!(distance <= radius);
        }
    }

    #[test]
    fn test_weighted_index() {
        let mut rng = create_rng(42);
        let weights = vec![1.0, 0.0, 1.0]; // Middle option impossible

        for _ in 0..20 {
            let idx = random_weighted_index(&mut rng, &weights).unwrap();
            assert!(idx == 0 || idx == 2);
            assert_ne!(idx, 1); // Should never be 1
        }
    }

    #[test]
    fn test_deterministic_hash() {
        // Same inputs should always produce same output
        let val1 = hash_random(10.0, 20.0, 12345);
        let val2 = hash_random(10.0, 20.0, 12345);
        assert_eq!(val1, val2);

        // Different inputs should produce different outputs
        let val3 = hash_random(10.0, 20.0, 12346);
        assert_ne!(val1, val3);
    }
}
