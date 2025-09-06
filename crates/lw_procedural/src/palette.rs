//! Color palette generation for nations and terrain

use lw_core::DeterministicRNG;

pub struct PaletteGenerator {
    rng: DeterministicRNG,
}

impl PaletteGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: DeterministicRNG::new(seed),
        }
    }
    
    /// Generate distinct nation colors using golden ratio
    pub fn generate_nation_colors(&mut self, count: usize) -> Vec<u32> {
        let mut colors = Vec::with_capacity(count);
        let golden_ratio = 0.618033988749895;
        let mut hue = self.rng.next_f32();
        
        for _ in 0..count {
            hue = (hue + golden_ratio) % 1.0;
            let saturation = 0.5 + self.rng.next_f32() * 0.3; // 0.5-0.8
            let value = 0.6 + self.rng.next_f32() * 0.3; // 0.6-0.9
            
            let rgb = hsv_to_rgb(hue, saturation, value);
            colors.push(rgb);
        }
        
        colors
    }
    
    /// Generate terrain color for height value
    pub fn terrain_color(height: f32, moisture: f32) -> u32 {
        let (r, g, b) = if height < 0.1 {
            // Deep ocean
            (0.1, 0.2, 0.4)
        } else if height < 0.2 {
            // Ocean
            (0.15, 0.3, 0.5)
        } else if height < 0.3 {
            // Shore
            (0.8, 0.7, 0.5)
        } else if height < 0.5 {
            // Plains/grass
            if moisture > 0.5 {
                (0.2, 0.6, 0.2) // Green grass
            } else {
                (0.7, 0.6, 0.3) // Dry grass
            }
        } else if height < 0.7 {
            // Hills
            (0.5, 0.4, 0.3)
        } else if height < 0.9 {
            // Mountains
            (0.6, 0.5, 0.4)
        } else {
            // Peaks (snow)
            (0.9, 0.9, 0.95)
        };
        
        rgb_to_u32(r, g, b)
    }
}

/// Convert HSV to RGB color
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> u32 {
    let c = v * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = v - c;
    
    let (r, g, b) = if h < 1.0/6.0 {
        (c, x, 0.0)
    } else if h < 2.0/6.0 {
        (x, c, 0.0)
    } else if h < 3.0/6.0 {
        (0.0, c, x)
    } else if h < 4.0/6.0 {
        (0.0, x, c)
    } else if h < 5.0/6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    rgb_to_u32(r + m, g + m, b + m)
}

/// Convert RGB float values to packed u32
fn rgb_to_u32(r: f32, g: f32, b: f32) -> u32 {
    let r = (r.clamp(0.0, 1.0) * 255.0) as u32;
    let g = (g.clamp(0.0, 1.0) * 255.0) as u32;
    let b = (b.clamp(0.0, 1.0) * 255.0) as u32;
    
    (r << 16) | (g << 8) | b
}