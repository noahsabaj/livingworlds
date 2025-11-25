//! Nation color generation utilities
//!
//! Provides functions for generating distinct, visually appealing colors for nations.

use bevy::prelude::*;
use rand::Rng;
use rand_chacha::ChaCha8Rng as StdRng;

/// Generate a unique color for a nation
///
/// Uses the golden angle for hue distribution to ensure visually distinct colors
/// even for neighboring nation IDs.
pub fn generate_nation_color(nation_id: u32, rng: &mut StdRng) -> Color {
    // Use nation ID to ensure consistent colors
    let hue = (nation_id as f32 * 137.5) % 360.0; // Golden angle for good distribution
    let saturation = 0.6 + rng.gen_range(0.0..0.3);
    let lightness = 0.4 + rng.gen_range(0.0..0.3);

    // Convert HSL to RGB
    hsl_to_rgb(hue, saturation, lightness)
}

/// Convert HSL color to RGB
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let h = h / 360.0;
    let r;
    let g;
    let b;

    if s == 0.0 {
        r = l;
        g = l;
        b = l;
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    }

    Color::srgb(r, g, b)
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}
