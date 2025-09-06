//! Procedural font generation from Bezier curves
//! Generate fonts mathematically, convert to SDF at startup

use lw_core::{Vec2fx, Fixed32};
use std::collections::HashMap;

/// Bezier curve control points
#[derive(Debug, Clone)]
pub struct BezierCurve {
    pub p0: Vec2fx,
    pub p1: Vec2fx,
    pub p2: Vec2fx,
    pub p3: Vec2fx,
}

impl BezierCurve {
    /// Evaluate curve at parameter t [0, 1]
    pub fn evaluate(&self, t: f32) -> Vec2fx {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        
        self.p0 * Fixed32::from_float(mt3) +
        self.p1 * Fixed32::from_float(3.0 * mt2 * t) +
        self.p2 * Fixed32::from_float(3.0 * mt * t2) +
        self.p3 * Fixed32::from_float(t3)
    }
}

/// A glyph contour made of multiple curves
#[derive(Debug, Clone)]
pub struct GlyphContour {
    pub curves: Vec<BezierCurve>,
    pub closed: bool,
}

impl GlyphContour {
    pub fn new() -> Self {
        Self {
            curves: Vec::new(),
            closed: true,
        }
    }
    
    /// Add a line segment (converted to Bezier)
    pub fn add_line(&mut self, from: Vec2fx, to: Vec2fx) {
        let curve = BezierCurve {
            p0: from,
            p1: from + (to - from) * Fixed32::from_float(1.0 / 3.0),
            p2: from + (to - from) * Fixed32::from_float(2.0 / 3.0),
            p3: to,
        };
        self.curves.push(curve);
    }
    
    /// Add cubic Bezier
    pub fn add_cubic(&mut self, p0: Vec2fx, p1: Vec2fx, p2: Vec2fx, p3: Vec2fx) {
        self.curves.push(BezierCurve { p0, p1, p2, p3 });
    }
}

/// Complete glyph definition
#[derive(Debug, Clone)]
pub struct Glyph {
    pub character: char,
    pub advance: f32,
    pub min_bounds: Vec2fx,
    pub max_bounds: Vec2fx,
    pub contours: Vec<GlyphContour>,
}

impl Glyph {
    /// Test if point is inside glyph (for SDF generation) - simplified
    pub fn contains_point(&self, _point: Vec2fx) -> bool {
        // Simplified - would use ray casting in full implementation
        false
    }
    
    /// Get distance to nearest edge (unsigned) - simplified
    pub fn distance_to_edge(&self, point: Vec2fx) -> f32 {
        // Simplified - just return distance to bounds
        let center = Vec2fx::new(
            (self.min_bounds.x + self.max_bounds.x) * Fixed32::from_float(0.5),
            (self.min_bounds.y + self.max_bounds.y) * Fixed32::from_float(0.5)
        );
        (center - point).length().to_f32()
    }
}

/// Signed Distance Field representation
#[derive(Debug, Clone)]
pub struct SDFGlyph {
    pub character: char,
    pub advance: f32,
    pub width: usize,
    pub height: usize,
    pub distances: Vec<f32>,
}

/// Procedural font generator
pub struct FontGenerator {
    glyphs: HashMap<char, Glyph>,
}

impl FontGenerator {
    pub fn new() -> Self {
        let mut generator = Self {
            glyphs: HashMap::new(),
        };
        generator.generate_basic_font();
        generator
    }
    
    /// Generate basic ASCII characters
    fn generate_basic_font(&mut self) {
        // Generate letter 'O' as example
        self.generate_letter_o();
        self.generate_letter_a();
        self.generate_letter_i();
        
        // Generate digits
        for digit in '0'..='9' {
            self.generate_digit(digit);
        }
    }
    
    /// Generate letter 'O' - simplified circle
    fn generate_letter_o(&mut self) {
        let mut glyph = Glyph {
            character: 'O',
            advance: 1.0,
            min_bounds: Vec2fx::new(Fixed32::from_float(0.1), Fixed32::from_float(0.2)),
            max_bounds: Vec2fx::new(Fixed32::from_float(0.9), Fixed32::from_float(0.8)),
            contours: Vec::new(),
        };
        
        // Simple box outline for 'O'
        let mut outer = GlyphContour::new();
        outer.add_line(
            Vec2fx::new(Fixed32::from_float(0.2), Fixed32::from_float(0.3)),
            Vec2fx::new(Fixed32::from_float(0.8), Fixed32::from_float(0.3))
        );
        outer.add_line(
            Vec2fx::new(Fixed32::from_float(0.8), Fixed32::from_float(0.3)),
            Vec2fx::new(Fixed32::from_float(0.8), Fixed32::from_float(0.7))
        );
        outer.add_line(
            Vec2fx::new(Fixed32::from_float(0.8), Fixed32::from_float(0.7)),
            Vec2fx::new(Fixed32::from_float(0.2), Fixed32::from_float(0.7))
        );
        outer.add_line(
            Vec2fx::new(Fixed32::from_float(0.2), Fixed32::from_float(0.7)),
            Vec2fx::new(Fixed32::from_float(0.2), Fixed32::from_float(0.3))
        );
        
        glyph.contours.push(outer);
        self.glyphs.insert('O', glyph);
    }
    
    /// Generate letter 'A' - simple triangle with crossbar
    fn generate_letter_a(&mut self) {
        let mut glyph = Glyph {
            character: 'A',
            advance: 1.0,
            min_bounds: Vec2fx::new(Fixed32::from_float(0.1), Fixed32::from_float(0.2)),
            max_bounds: Vec2fx::new(Fixed32::from_float(0.9), Fixed32::from_float(0.8)),
            contours: Vec::new(),
        };
        
        let mut contour = GlyphContour::new();
        
        // Left stroke
        contour.add_line(
            Vec2fx::new(Fixed32::from_float(0.2), Fixed32::from_float(0.2)),
            Vec2fx::new(Fixed32::from_float(0.5), Fixed32::from_float(0.8))
        );
        
        // Right stroke  
        contour.add_line(
            Vec2fx::new(Fixed32::from_float(0.5), Fixed32::from_float(0.8)),
            Vec2fx::new(Fixed32::from_float(0.8), Fixed32::from_float(0.2))
        );
        
        // Cross bar
        contour.add_line(
            Vec2fx::new(Fixed32::from_float(0.35), Fixed32::from_float(0.45)),
            Vec2fx::new(Fixed32::from_float(0.65), Fixed32::from_float(0.45))
        );
        
        glyph.contours.push(contour);
        self.glyphs.insert('A', glyph);
    }
    
    /// Generate letter 'I' - vertical line with serifs
    fn generate_letter_i(&mut self) {
        let mut glyph = Glyph {
            character: 'I',
            advance: 0.5,
            min_bounds: Vec2fx::new(Fixed32::from_float(0.35), Fixed32::from_float(0.2)),
            max_bounds: Vec2fx::new(Fixed32::from_float(0.65), Fixed32::from_float(0.8)),
            contours: Vec::new(),
        };
        
        let mut contour = GlyphContour::new();
        
        // Vertical stroke
        contour.add_line(
            Vec2fx::new(Fixed32::from_float(0.5), Fixed32::from_float(0.2)),
            Vec2fx::new(Fixed32::from_float(0.5), Fixed32::from_float(0.8))
        );
        
        // Top serif
        contour.add_line(
            Vec2fx::new(Fixed32::from_float(0.35), Fixed32::from_float(0.8)),
            Vec2fx::new(Fixed32::from_float(0.65), Fixed32::from_float(0.8))
        );
        
        // Bottom serif
        contour.add_line(
            Vec2fx::new(Fixed32::from_float(0.35), Fixed32::from_float(0.2)),
            Vec2fx::new(Fixed32::from_float(0.65), Fixed32::from_float(0.2))
        );
        
        glyph.contours.push(contour);
        self.glyphs.insert('I', glyph);
    }
    
    /// Generate a digit character
    fn generate_digit(&mut self, digit: char) {
        let digit_val = digit as u8 - '0' as u8;
        
        let mut glyph = Glyph {
            character: digit,
            advance: 0.8,
            min_bounds: Vec2fx::new(Fixed32::from_float(0.2), Fixed32::from_float(0.2)),
            max_bounds: Vec2fx::new(Fixed32::from_float(0.8), Fixed32::from_float(0.8)),
            contours: Vec::new(),
        };
        
        // Simple representations for digits
        let mut contour = GlyphContour::new();
        
        match digit_val {
            0 => {
                // Rectangle for '0'
                contour.add_line(
                    Vec2fx::new(Fixed32::from_float(0.3), Fixed32::from_float(0.3)),
                    Vec2fx::new(Fixed32::from_float(0.7), Fixed32::from_float(0.3))
                );
                contour.add_line(
                    Vec2fx::new(Fixed32::from_float(0.7), Fixed32::from_float(0.3)),
                    Vec2fx::new(Fixed32::from_float(0.7), Fixed32::from_float(0.7))
                );
                contour.add_line(
                    Vec2fx::new(Fixed32::from_float(0.7), Fixed32::from_float(0.7)),
                    Vec2fx::new(Fixed32::from_float(0.3), Fixed32::from_float(0.7))
                );
                contour.add_line(
                    Vec2fx::new(Fixed32::from_float(0.3), Fixed32::from_float(0.7)),
                    Vec2fx::new(Fixed32::from_float(0.3), Fixed32::from_float(0.3))
                );
            }
            1 => {
                // Vertical line for '1'
                contour.add_line(
                    Vec2fx::new(Fixed32::from_float(0.5), Fixed32::from_float(0.2)),
                    Vec2fx::new(Fixed32::from_float(0.5), Fixed32::from_float(0.8))
                );
            }
            _ => {
                // Default diagonal line for other digits
                contour.add_line(
                    Vec2fx::new(Fixed32::from_float(0.3), Fixed32::from_float(0.3)),
                    Vec2fx::new(Fixed32::from_float(0.7), Fixed32::from_float(0.7))
                );
            }
        }
        
        glyph.contours.push(contour);
        self.glyphs.insert(digit, glyph);
    }
    
    /// Generate SDF from glyph - simplified
    pub fn generate_sdf(&self, glyph: &Glyph, size: usize) -> SDFGlyph {
        let mut distances = vec![0.0; size * size];
        
        // Simplified SDF generation
        for y in 0..size {
            for x in 0..size {
                let u = x as f32 / size as f32;
                let v = y as f32 / size as f32;
                
                let point = Vec2fx::new(Fixed32::from_float(u), Fixed32::from_float(v));
                let dist = glyph.distance_to_edge(point);
                
                distances[y * size + x] = if glyph.contains_point(point) {
                    -dist
                } else {
                    dist
                };
            }
        }
        
        SDFGlyph {
            character: glyph.character,
            advance: glyph.advance,
            width: size,
            height: size,
            distances,
        }
    }
    
    /// Get glyph for character
    pub fn get_glyph(&self, ch: char) -> Option<&Glyph> {
        self.glyphs.get(&ch)
    }
}

impl Default for FontGenerator {
    fn default() -> Self {
        Self::new()
    }
}