//! Simplified tectonic plate system for terrain generation
//!
//! This module implements the core tectonic features that affect gameplay:
//! - Real Voronoi-based plate tessellation
//! - Plate boundaries for mountains and rifts
//! - Volcanic hotspots for island chains
//! - Mineral deposits at geological features
//! - Continental vs oceanic distinction

use bevy::prelude::*;
use rand::{Rng, rngs::StdRng};
use voronator::{delaunator::Point, VoronoiDiagram};
use crate::constants::*;
use crate::resources::MapBounds;

/// Builder for generating tectonic systems following the builder pattern
/// 
/// This builder encapsulates tectonic plate generation with configurable continent count.
pub struct TectonicsBuilder<'a> {
    rng: &'a mut StdRng,
    bounds: MapBounds,
    seed: u32,
    continent_count: Option<u32>,
    min_plates: u32,
    max_plates: u32,
}

impl<'a> TectonicsBuilder<'a> {
    /// Create a new tectonics builder
    pub fn new(rng: &'a mut StdRng, bounds: MapBounds, seed: u32) -> Self {
        Self {
            rng,
            bounds,
            seed,
            continent_count: None, // Auto-determine by default
            min_plates: 6,
            max_plates: 20,
        }
    }
    
    /// Set specific number of continents/plates
    pub fn with_continent_count(mut self, count: u32) -> Self {
        self.continent_count = Some(count.clamp(self.min_plates, self.max_plates));
        self
    }
    
    /// Set minimum number of plates
    pub fn with_min_plates(mut self, min: u32) -> Self {
        self.min_plates = min;
        self
    }
    
    /// Set maximum number of plates
    pub fn with_max_plates(mut self, max: u32) -> Self {
        self.max_plates = max;
        self
    }
    
    /// Build the tectonic system
    pub fn build(mut self) -> TectonicSystem {
        // Delegate to the existing internal implementation
        let continent_count = self.continent_count.unwrap_or_else(|| {
            self.rng.gen_range(7..=12)
        });
        generate_tectonics_internal(self.rng, self.bounds, self.seed, continent_count)
    }
}

// ============================================================================
// CORE DATA STRUCTURES (5 SYSTEMS ONLY)
// ============================================================================

/// Simplified tectonic system focused on visible gameplay features
#[derive(Debug, Clone)]
pub struct TectonicSystem {
    /// All tectonic plates in the system
    pub plates: Vec<TectonicPlate>,
    
    /// Boundaries between plates for mountains/rifts
    pub boundaries: Vec<PlateBoundary>,
    
    /// Volcanic hotspots for island chains
    pub hotspots: Vec<Hotspot>,
}

/// Simplified tectonic plate for terrain generation
#[derive(Debug, Clone)]
pub struct TectonicPlate {
    /// Unique identifier
    pub id: u32,
    
    /// Voronoi cell vertices defining real plate boundary
    pub polygon: Vec<Vec2>,
    
    /// Centroid of the plate
    pub center: Vec2,
    
    /// Area for determining plate importance
    pub area: f32,
    
    /// Simple continental vs oceanic distinction
    pub is_continental: bool,
    
    /// Direct elevation modifier for terrain generation
    pub elevation_boost: f32,
    
    /// Has ancient stable core (simplified craton)
    pub has_ancient_core: bool,
}

/// Boundary between two tectonic plates
#[derive(Debug, Clone)]
pub struct PlateBoundary {
    /// First plate ID
    pub plate_a: u32,
    
    /// Second plate ID
    pub plate_b: u32,
    
    /// Type of interaction
    pub boundary_type: BoundaryType,
    
    /// Line segments making up the boundary
    pub segments: Vec<BoundarySegment>,
    
    /// Total length in km
    pub length: f32,
    
    /// Average relative velocity in cm/year (simplified constant)
    pub relative_velocity: f32,
}

/// Simplified boundary type for terrain features
#[derive(Debug, Clone)]
pub enum BoundaryType {
    /// Creates rifts/valleys (negative elevation)
    Divergent {
        /// How deep the rift goes
        rift_depth: f32,
    },
    
    /// Creates mountains (positive elevation)
    Convergent {
        /// How high the mountains go
        mountain_height: f32,
        /// Optional named mountain range
        mountain_range: Option<MountainRange>,
    },
    
    /// Transform boundaries (minimal terrain effect)
    Transform,
}

/// Simplified mountain range for visible terrain
#[derive(Debug, Clone, PartialEq)]
pub struct MountainRange {
    /// Ridge line of the mountains
    pub ridge_line: Vec<Vec2>,
    
    /// Width of affected area
    pub width: f32,
    
    /// Peak elevation for terrain generation
    pub peak_elevation: f32,
}

/// Segment of a plate boundary
#[derive(Debug, Clone)]
pub struct BoundarySegment {
    /// Start point
    pub start: Vec2,
    
    /// End point
    pub end: Vec2,
    
    /// Normal vector pointing away from plate A
    pub normal: Vec2,
    
    /// Relative velocity at this segment (simplified)
    pub local_velocity: Vec2,
    
    /// Stress accumulation (kept for future use)
    pub stress: f32,
}

/// Volcanic hotspot creating island chains
#[derive(Debug, Clone)]
pub struct Hotspot {
    /// Fixed position in mantle reference frame
    pub position: Vec2,
    
    /// Intensity of volcanic activity
    pub intensity: f32,
    
    /// Radius of influence
    pub radius: f32,
    
    /// Age of hotspot
    pub age: f32,
    
    /// Chain of volcanoes created as plates move over it
    pub volcanic_chain: Vec<Volcano>,
}

/// Simplified volcano for terrain features
#[derive(Debug, Clone)]
pub struct Volcano {
    /// Position on map
    pub position: Vec2,
    
    /// Elevation for terrain generation
    pub elevation: f32,
    
    /// Is this the active volcano in the chain
    pub is_active: bool,
}


// ============================================================================
// GENERATION FUNCTIONS
// ============================================================================


// Internal implementation moved to TectonicsBuilder
fn generate_tectonics_internal(rng: &mut StdRng, bounds: MapBounds, seed: u32, continent_count: u32) -> TectonicSystem {
    println!("Generating tectonic system with {} continents using real Voronoi...", continent_count);
    
    // Generate the specified number of plates (continents)
    let plates = generate_tectonic_plates_voronoi_with_count(rng, bounds, seed, continent_count as usize);
    
    // Step 2: Determine plate boundaries for mountains/rifts
    let boundaries = determine_plate_boundaries(&plates);
    
    // Step 3: Generate hotspots for volcanic islands
    let hotspots = generate_hotspots(rng, bounds);
    
    let continental_count = plates.iter().filter(|p| p.is_continental).count();
    
    println!("Generated {} plates ({} continental), {} boundaries, {} hotspots", 
             plates.len(), 
             continental_count,
             boundaries.len(), 
             hotspots.len());
    
    TectonicSystem {
        plates,
        boundaries,
        hotspots,
    }
}

// ============================================================================
// PLATE GENERATION WITH REAL VORONOI
// ============================================================================

/// Generate tectonic plates using REAL Voronoi tessellation
fn generate_tectonic_plates_voronoi(rng: &mut StdRng, bounds: MapBounds, seed: u32) -> Vec<TectonicPlate> {
    // Calculate number of plates based on world size
    let world_area = (bounds.x_max - bounds.x_min) * (bounds.y_max - bounds.y_min);
    let base_plates = TECTONIC_PLATES_BASE + (seed % TECTONIC_PLATES_VARIATION) as usize;
    let num_plates = (base_plates as f32 * (world_area / 1_000_000.0).sqrt()) as usize;
    let num_plates = num_plates.max(6).min(20); // Reasonable range
    
    generate_tectonic_plates_voronoi_with_count(rng, bounds, seed, num_plates)
}

fn generate_tectonic_plates_voronoi_with_count(rng: &mut StdRng, bounds: MapBounds, seed: u32, num_plates: usize) -> Vec<TectonicPlate> {
    // Generate plate centers using Poisson disk sampling for even distribution
    let plate_centers = generate_plate_centers_poisson(rng, bounds, num_plates);
    
    // Convert to voronator Points
    let points: Vec<Point> = plate_centers.iter()
        .map(|c| Point { x: c.x as f64, y: c.y as f64 })
        .collect();
    
    // Create REAL Voronoi diagram using voronator
    let min = Point { x: bounds.x_min as f64, y: bounds.y_min as f64 };
    let max = Point { x: bounds.x_max as f64, y: bounds.y_max as f64 };
    let diagram = VoronoiDiagram::new(&min, &max, &points)
        .expect("Failed to build Voronoi diagram");
    
    // Create plates from Voronoi cells
    let mut plates = Vec::new();
    
    for (id, cell) in diagram.cells().iter().enumerate() {
        // Get the polygon vertices for this cell
        let polygon: Vec<Vec2> = cell.points()
            .iter()
            .map(|p| Vec2::new(p.x as f32, p.y as f32))
            .collect();
        
        // Skip invalid cells
        if polygon.len() < 3 {
            continue;
        }
        
        let center = plate_centers[id];
        let area = calculate_polygon_area(&polygon);
        
        // Determine plate type based on position and random factors
        let distance_from_center = ((center.x - (bounds.x_min + bounds.x_max) / 2.0).powi(2) + 
                                   (center.y - (bounds.y_min + bounds.y_max) / 2.0).powi(2)).sqrt();
        let max_distance = ((bounds.x_max - bounds.x_min).powi(2) + 
                           (bounds.y_max - bounds.y_min).powi(2)).sqrt() / 2.0;
        let centrality = 1.0 - (distance_from_center / max_distance);
        
        // Continental plates more likely in center, oceanic at edges
        let continental_chance = 0.4 + centrality * 0.4 + rng.gen_range(-0.2..0.2);
        let is_continental = rng.gen::<f32>() < continental_chance;
        
        // Simplified elevation boost
        let elevation_boost = if is_continental {
            rng.gen_range(200.0..800.0) // Continental elevation
        } else {
            rng.gen_range(-4000.0..-2000.0) // Ocean depth
        };
        
        // Has ancient core for some large continental plates
        let has_ancient_core = is_continental && area > 50000.0 && rng.gen::<f32>() < 0.5;
        
        plates.push(TectonicPlate {
            id: id as u32,
            polygon,
            center,
            area,
            is_continental,
            elevation_boost,
            has_ancient_core,
        });
    }
    
    plates
}

/// Generate evenly distributed plate centers using Poisson disk sampling
fn generate_plate_centers_poisson(rng: &mut StdRng, bounds: MapBounds, num_plates: usize) -> Vec<Vec2> {
    let mut centers = Vec::new();
    let width = bounds.x_max - bounds.x_min;
    let height = bounds.y_max - bounds.y_min;
    
    // Minimum distance between plates
    let min_distance = ((width * height) / (num_plates as f32 * 2.0)).sqrt();
    
    // Start with random first point
    centers.push(Vec2::new(
        rng.gen_range(bounds.x_min..bounds.x_max),
        rng.gen_range(bounds.y_min..bounds.y_max),
    ));
    
    // Generate remaining points
    let max_attempts = 100;
    while centers.len() < num_plates {
        let mut best_candidate = Vec2::ZERO;
        let mut best_distance = 0.0;
        
        // Try multiple candidates and pick the one furthest from existing points
        for _ in 0..max_attempts {
            let candidate = Vec2::new(
                rng.gen_range(bounds.x_min..bounds.x_max),
                rng.gen_range(bounds.y_min..bounds.y_max),
            );
            
            // Find minimum distance to existing centers
            let min_dist = centers.iter()
                .map(|c| c.distance(candidate))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(f32::MAX);
            
            if min_dist > best_distance {
                best_distance = min_dist;
                best_candidate = candidate;
            }
        }
        
        // Add the best candidate if it's far enough
        if best_distance >= min_distance * 0.7 {
            centers.push(best_candidate);
        } else {
            // If we can't find a good spot, just add random
            centers.push(Vec2::new(
                rng.gen_range(bounds.x_min..bounds.x_max),
                rng.gen_range(bounds.y_min..bounds.y_max),
            ));
        }
    }
    
    centers
}

/// Calculate area of a polygon using shoelace formula
fn calculate_polygon_area(vertices: &[Vec2]) -> f32 {
    if vertices.len() < 3 {
        return 0.0;
    }
    
    let mut area = 0.0;
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        area += vertices[i].x * vertices[j].y;
        area -= vertices[j].x * vertices[i].y;
    }
    
    area.abs() / 2.0
}

// ============================================================================
// BOUNDARY DETECTION
// ============================================================================

fn determine_plate_boundaries(plates: &[TectonicPlate]) -> Vec<PlateBoundary> {
    let mut boundaries = Vec::new();
    
    // Check each pair of plates for adjacency using real Voronoi adjacency
    for i in 0..plates.len() {
        for j in (i + 1)..plates.len() {
            let plate_a = &plates[i];
            let plate_b = &plates[j];
            
            // Check if plates are neighbors by checking if they share vertices
            let mut shared_vertices = Vec::new();
            
            for vertex_a in &plate_a.polygon {
                for vertex_b in &plate_b.polygon {
                    // If vertices are very close, they're shared
                    if vertex_a.distance(*vertex_b) < 1.0 {
                        shared_vertices.push(*vertex_a);
                    }
                }
            }
            
            // If plates share at least 2 vertices, they have a boundary
            if shared_vertices.len() >= 2 {
                // Create simple boundary segments
                let mut segments = Vec::new();
                for i in 0..shared_vertices.len().saturating_sub(1) {
                    let start = shared_vertices[i];
                    let end = shared_vertices[i + 1];
                    let normal = Vec2::new(
                        -(end.y - start.y),
                        end.x - start.x
                    ).normalize();
                    
                    segments.push(BoundarySegment {
                        start,
                        end,
                        normal,
                        local_velocity: Vec2::ZERO,
                        stress: 0.0,
                    });
                }
                
                if !segments.is_empty() {
                    // Determine boundary type based on plate types
                    let boundary_type = classify_boundary_type(plate_a, plate_b);
                    
                    // Calculate total boundary length
                    let length = segments.iter()
                        .map(|s| s.start.distance(s.end))
                        .sum();
                    
                    boundaries.push(PlateBoundary {
                        plate_a: plate_a.id,
                        plate_b: plate_b.id,
                        boundary_type,
                        segments,
                        length,
                        relative_velocity: 5.0, // Simplified constant
                    });
                }
            }
        }
    }
    
    boundaries
}

/// Simplified boundary classification based on plate types
fn classify_boundary_type(plate_a: &TectonicPlate, plate_b: &TectonicPlate) -> BoundaryType {
    // Simple classification: continental collision = mountains, else = rifts
    if plate_a.is_continental && plate_b.is_continental {
        // Continental collision creates mountains
        BoundaryType::Convergent {
            mountain_height: 8000.0,
            mountain_range: Some(MountainRange {
                ridge_line: Vec::new(),
                width: 200.0,
                peak_elevation: 8000.0,
            }),
        }
    } else if !plate_a.is_continental && !plate_b.is_continental {
        // Oceanic boundaries create rifts
        BoundaryType::Divergent {
            rift_depth: -2500.0,
        }
    } else {
        // Mixed boundaries create moderate mountains
        BoundaryType::Convergent {
            mountain_height: 4000.0,
            mountain_range: None,
        }
    }
}

// ============================================================================
// HOTSPOTS AND MINERALS
// ============================================================================

fn generate_hotspots(rng: &mut StdRng, bounds: MapBounds) -> Vec<Hotspot> {
    let mut hotspots = Vec::new();
    
    // Generate 2-8 hotspots based on world size
    let world_area = (bounds.x_max - bounds.x_min) * (bounds.y_max - bounds.y_min);
    let num_hotspots = (3.0 + (world_area / 500000.0).sqrt()).min(8.0) as usize;
    
    for _i in 0..num_hotspots {
        let position = Vec2::new(
            rng.gen_range(bounds.x_min..bounds.x_max),
            rng.gen_range(bounds.y_min..bounds.y_max),
        );
        
        // Simplified volcanic chain - just a few volcanoes in a line
        let mut volcanic_chain = Vec::new();
        let num_volcanoes = rng.gen_range(3..8);
        let direction = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize();
        
        for j in 0..num_volcanoes {
            let chain_position = position + direction * (j as f32 * 50.0);
            
            volcanic_chain.push(Volcano {
                position: chain_position,
                elevation: rng.gen_range(1000.0..4000.0) * (1.0 - j as f32 / num_volcanoes as f32),
                is_active: j == 0, // Only the first is active
            });
        }
        
        hotspots.push(Hotspot {
            position,
            intensity: rng.gen_range(0.5..1.0),
            radius: rng.gen_range(50.0..200.0),
            age: rng.gen_range(0.0..100.0),
            volcanic_chain,
        });
    }
    
    hotspots
}

