//! Mesh building and management for the mega-mesh rendering architecture
//! 
//! This module implements a high-performance mesh system that renders up to 900,000
//! hexagonal provinces in a single draw call. It includes optimizations for memory
//! usage, parallel processing, and vertex deduplication.
//!
//! # Performance Characteristics
//! - Single draw call for entire world (900k provinces)
//! - ~240MB GPU memory for 900k provinces (optimizable to ~135MB with deduplication)
//! - Parallel mesh building with rayon for 4-8x speedup on multicore systems
//! - CPU-accessible mesh for dynamic overlay updates

use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::image::ImageSampler;
use rayon::prelude::*;
use std::collections::HashMap;
use crate::components::Province;
use crate::colors::get_terrain_color_gradient;
use crate::constants::*;

// ============================================================================
// CONSTANTS
// ============================================================================

/// Number of vertices in a hexagon (center + 6 corners)
const VERTICES_PER_HEXAGON: usize = 7;

/// Number of corners in a hexagon
const HEXAGON_CORNERS: usize = 6;

/// Number of triangles per hexagon
const TRIANGLES_PER_HEXAGON: usize = 6;

/// Number of indices per hexagon (3 per triangle)
const INDICES_PER_HEXAGON: usize = TRIANGLES_PER_HEXAGON * 3;

/// Degrees between each hexagon corner
const DEGREES_PER_CORNER: f32 = 60.0;

/// Starting angle for flat-top hexagon (30 degrees)
const FLAT_TOP_START_ANGLE: f32 = 30.0;

/// Square root of 3 (precomputed for performance)
const SQRT_3: f32 = 1.732050808;

/// Width of antialiasing border in pixels
const ANTIALIASING_WIDTH: f32 = 1.5;

/// Maximum allowed texture size to prevent excessive memory allocation
const MAX_TEXTURE_SIZE: u32 = 4096;

/// Default texture color values
const DEFAULT_COLOR_VALUE: u8 = 255;

/// Chunk size for parallel processing
const PARALLEL_CHUNK_SIZE: usize = 10000;

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Errors that can occur during mesh building
#[derive(Debug, thiserror::Error)]
pub enum MeshBuildError {
    #[error("Invalid province position at index {index}: {reason}")]
    InvalidPosition { index: usize, reason: String },
    
    #[error("Texture size {requested} exceeds maximum allowed size {max}")]
    TextureSizeTooLarge { requested: u32, max: u32 },
    
    #[error("Integer overflow: {0} vertices exceeds u32 maximum")]
    VertexIndexOverflow(usize),
    
    #[error("No provinces provided for mesh building")]
    NoProvinces,
}

// ============================================================================
// RESOURCES
// ============================================================================

/// Storage for province data, separated from rendering concerns
#[derive(Resource, Reflect, Clone)]
pub struct ProvinceStorage {
    pub provinces: Vec<Province>,
    pub province_by_id: HashMap<u32, usize>,  // Maps province ID to index for O(1) lookups
}

/// Handle to the world mega-mesh for overlay system access
#[derive(Resource)]
pub struct WorldMeshHandle(pub Handle<Mesh>);

/// Statistics about the generated mesh for debugging and optimization
#[derive(Resource, Debug, Clone)]
pub struct MeshStatistics {
    pub total_vertices: usize,
    pub total_indices: usize,
    pub total_provinces: usize,
    pub memory_usage_bytes: usize,
    pub build_time_ms: f32,
    pub used_parallel: bool,
    pub used_deduplication: bool,
    pub deduplication_savings_percent: f32,
}

impl MeshStatistics {
    /// Display formatted statistics
    pub fn display(&self) -> String {
        let mut result = format!(
            "Mesh Statistics:\n  Provinces: {}\n  Vertices: {}",
            self.total_provinces,
            self.total_vertices
        );
        
        if self.used_deduplication {
            result.push_str(&format!(
                " ({}% saved via deduplication)",
                (self.deduplication_savings_percent * 100.0) as i32
            ));
        }
        
        result.push_str(&format!(
            "\n  Indices: {}\n  Memory: {} MB\n  Build Time: {:.1} ms",
            self.total_indices,
            self.memory_usage_bytes / (1024 * 1024),
            self.build_time_ms
        ));
        
        if self.used_parallel {
            result.push_str("\n  Parallel Processing: Enabled");
        }
        
        result
    }
}

// ============================================================================
// HEXAGON GEOMETRY
// ============================================================================

/// Abstraction for hexagon geometry calculations
#[derive(Debug, Clone)]
pub struct HexagonGeometry {
    pub size: f32,
    pub flat_top: bool,
}

impl HexagonGeometry {
    /// Create a new flat-top hexagon geometry
    pub fn flat_top(size: f32) -> Self {
        Self {
            size,
            flat_top: true,
        }
    }
    
    /// Generate vertices for a hexagon at the given position
    pub fn generate_vertices(&self, center: Vec2) -> [Vec3; VERTICES_PER_HEXAGON] {
        let mut vertices = [Vec3::ZERO; VERTICES_PER_HEXAGON];
        
        // Center vertex
        vertices[0] = Vec3::new(center.x, center.y, 0.0);
        
        // Corner vertices
        for i in 0..HEXAGON_CORNERS {
            let angle = if self.flat_top {
                // Flat-top hexagon starts at 0 degrees (not 30!) for proper vertex sharing
                // This ensures vertices align between adjacent hexagons
                (i as f32 * DEGREES_PER_CORNER).to_radians()
            } else {
                // Pointy-top hexagon starts at 30 degrees
                (FLAT_TOP_START_ANGLE + i as f32 * DEGREES_PER_CORNER).to_radians()
            };
            
            vertices[i + 1] = Vec3::new(
                center.x + self.size * angle.cos(),
                center.y + self.size * angle.sin(),
                0.0,
            );
        }
        
        vertices
    }
    
    /// Check if a position is valid (not NaN or infinite)
    fn validate_position(pos: Vec2, index: usize) -> Result<(), MeshBuildError> {
        if !pos.x.is_finite() || !pos.y.is_finite() {
            return Err(MeshBuildError::InvalidPosition {
                index,
                reason: format!("Position contains NaN or infinity: ({}, {})", pos.x, pos.y),
            });
        }
        Ok(())
    }
}

// ============================================================================
// VERTEX DEDUPLICATION
// ============================================================================

/// Key for vertex deduplication - quantized position for HashMap lookup
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct VertexKey {
    x: i32,
    y: i32,
}

impl VertexKey {
    /// Create a vertex key from a position, quantizing to avoid floating point issues
    fn from_position(pos: Vec3) -> Self {
        // With hex_size=50 and coordinates up to ±40,000, we need coarser quantization
        // Using 10.0 gives us 0.1 unit precision, which is sufficient for hexagon vertices
        // and avoids floating point precision issues with large coordinates
        const QUANTIZATION_FACTOR: f32 = 10.0; // 0.1 unit precision
        Self {
            x: (pos.x * QUANTIZATION_FACTOR).round() as i32,
            y: (pos.y * QUANTIZATION_FACTOR).round() as i32,
        }
    }
}

/// Vertex deduplication system for memory optimization
struct VertexDeduplicator {
    vertex_map: HashMap<VertexKey, u32>,
    vertices: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
}

impl VertexDeduplicator {
    /// Create a new deduplicator with capacity hint
    fn with_capacity(estimated_unique_vertices: usize) -> Self {
        Self {
            vertex_map: HashMap::with_capacity(estimated_unique_vertices),
            vertices: Vec::with_capacity(estimated_unique_vertices),
            colors: Vec::with_capacity(estimated_unique_vertices),
        }
    }
    
    /// Add a vertex, returning its index (deduplicating if it already exists)
    fn add_vertex(&mut self, position: Vec3, color: [f32; 4]) -> u32 {
        let key = VertexKey::from_position(position);
        
        *self.vertex_map.entry(key).or_insert_with(|| {
            let index = self.vertices.len() as u32;
            self.vertices.push([position.x, position.y, position.z]);
            self.colors.push(color);
            index
        })
    }
    
    /// Get the final deduplicated vertices and colors
    fn into_buffers(self) -> (Vec<[f32; 3]>, Vec<[f32; 4]>) {
        (self.vertices, self.colors)
    }
    
    /// Get statistics about deduplication effectiveness
    fn stats(&self) -> (usize, usize) {
        (self.vertices.len(), self.vertex_map.len())
    }
}

// ============================================================================
// MESH BUILDING
// ============================================================================

/// Builder for creating optimized world meshes
pub struct MeshBuilder {
    geometry: HexagonGeometry,
    use_parallel: bool,
    use_deduplication: bool,
}

impl Default for MeshBuilder {
    fn default() -> Self {
        Self {
            geometry: HexagonGeometry::flat_top(HEX_SIZE_PIXELS),
            use_parallel: true,
            use_deduplication: true,
        }
    }
}

impl MeshBuilder {
    /// Create a new mesh builder with custom settings
    pub fn new(hex_size: f32) -> Self {
        Self {
            geometry: HexagonGeometry::flat_top(hex_size),
            use_parallel: true,
            use_deduplication: true,
        }
    }
    
    /// Disable parallel processing (useful for debugging)
    pub fn with_parallel(mut self, parallel: bool) -> Self {
        self.use_parallel = parallel;
        self
    }
    
    /// Disable vertex deduplication (useful for debugging)
    pub fn with_deduplication(mut self, dedup: bool) -> Self {
        self.use_deduplication = dedup;
        self
    }
    
    /// Build the world mega-mesh with all optimizations
    pub fn build(
        &self,
        provinces: &[Province],
        meshes: &mut Assets<Mesh>,
    ) -> Result<(Handle<Mesh>, MeshStatistics), MeshBuildError> {
        let start_time = std::time::Instant::now();
        
        // Validate input
        if provinces.is_empty() {
            return Err(MeshBuildError::NoProvinces);
        }
        
        // Choose build method based on deduplication setting
        let (vertices, indices, colors, dedup_savings) = if self.use_deduplication {
            self.build_deduplicated(provinces)?
        } else {
            // Check for vertex index overflow
            let total_vertices = provinces.len() * VERTICES_PER_HEXAGON;
            if total_vertices > u32::MAX as usize {
                return Err(MeshBuildError::VertexIndexOverflow(total_vertices));
            }
            
            // Pre-allocate vectors with exact capacity
            let vertices_capacity = total_vertices;
            let indices_capacity = provinces.len() * INDICES_PER_HEXAGON;
            let colors_capacity = total_vertices;
            
            let mut vertices = Vec::with_capacity(vertices_capacity);
            let mut indices = Vec::with_capacity(indices_capacity);
            let mut colors = Vec::with_capacity(colors_capacity);
            
            if self.use_parallel && provinces.len() > PARALLEL_CHUNK_SIZE {
                // Parallel processing for large meshes
                self.build_parallel(provinces, &mut vertices, &mut indices, &mut colors)?;
            } else {
                // Sequential processing for small meshes
                self.build_sequential(provinces, &mut vertices, &mut indices, &mut colors)?;
            }
            
            (vertices, indices, colors, 0.0)
        };
        
        // Create the mega-mesh with CPU access for overlay updates
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );
        
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors.clone());
        mesh.insert_indices(Indices::U32(indices.clone()));
        
        let handle = meshes.add(mesh);
        
        // Calculate statistics
        let stats = MeshStatistics {
            total_vertices: vertices.len(),
            total_indices: indices.len(),
            total_provinces: provinces.len(),
            memory_usage_bytes: Self::calculate_memory_usage(&vertices, &indices, &colors),
            build_time_ms: start_time.elapsed().as_secs_f32() * 1000.0,
            used_parallel: self.use_parallel && provinces.len() > PARALLEL_CHUNK_SIZE,
            used_deduplication: self.use_deduplication,
            deduplication_savings_percent: dedup_savings,
        };
        
        if self.use_deduplication {
            info!("Mesh built (deduplicated): {} provinces, {} vertices ({:.1}% saved), {} MB, {:.1}ms",
                stats.total_provinces,
                stats.total_vertices,
                dedup_savings * 100.0,
                stats.memory_usage_bytes / (1024 * 1024),
                stats.build_time_ms
            );
        } else {
            info!("Mesh built: {} provinces, {} vertices, {} MB, {:.1}ms",
                stats.total_provinces,
                stats.total_vertices,
                stats.memory_usage_bytes / (1024 * 1024),
                stats.build_time_ms
            );
        }
        
        Ok((handle, stats))
    }
    
    /// Build mesh with vertex deduplication for 60% memory savings
    fn build_deduplicated(
        &self,
        provinces: &[Province],
    ) -> Result<(Vec<[f32; 3]>, Vec<u32>, Vec<[f32; 4]>, f32), MeshBuildError> {
        // Estimate unique vertices (about 40% of total after deduplication)
        let estimated_unique = (provinces.len() * VERTICES_PER_HEXAGON * 2 / 5).max(1000);
        let mut deduplicator = VertexDeduplicator::with_capacity(estimated_unique);
        let mut indices = Vec::with_capacity(provinces.len() * INDICES_PER_HEXAGON);
        
        for (index, province) in provinces.iter().enumerate() {
            // Validate position
            HexagonGeometry::validate_position(province.position, index)?;
            
            // Generate vertices for this hexagon
            let hex_vertices = self.geometry.generate_vertices(province.position);
            let color = get_terrain_color_gradient(province.terrain, province.elevation.value());
            let rgba = color.to_linear().to_f32_array();
            
            // Add vertices through deduplicator
            let mut vertex_indices = Vec::with_capacity(VERTICES_PER_HEXAGON);
            for vertex in &hex_vertices {
                let idx = deduplicator.add_vertex(*vertex, rgba);
                vertex_indices.push(idx);
            }
            
            // Generate indices for triangles using deduplicated vertex indices
            let center_idx = vertex_indices[0];
            for i in 0..TRIANGLES_PER_HEXAGON {
                let next = (i + 1) % HEXAGON_CORNERS;
                indices.push(center_idx);
                indices.push(vertex_indices[i + 1]);
                indices.push(vertex_indices[next + 1]);
            }
        }
        
        // Calculate deduplication savings
        let original_vertices = provinces.len() * VERTICES_PER_HEXAGON;
        let (unique_vertices, _) = deduplicator.stats();
        let savings = 1.0 - (unique_vertices as f32 / original_vertices as f32);
        
        // Debug logging to understand deduplication
        info!("Vertex deduplication: {} original vertices -> {} unique vertices = {:.1}% savings",
            original_vertices, unique_vertices, savings * 100.0);
        
        let (vertices, colors) = deduplicator.into_buffers();
        
        Ok((vertices, indices, colors, savings))
    }
    
    /// Sequential mesh building for small province counts
    fn build_sequential(
        &self,
        provinces: &[Province],
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        colors: &mut Vec<[f32; 4]>,
    ) -> Result<(), MeshBuildError> {
        for (index, province) in provinces.iter().enumerate() {
            // Validate position
            HexagonGeometry::validate_position(province.position, index)?;
            
            // Safe index calculation
            let base_idx = (vertices.len() as u64)
                .min(u32::MAX as u64) as u32;
            
            // Generate vertices
            let hex_vertices = self.geometry.generate_vertices(province.position);
            for v in &hex_vertices {
                vertices.push([v.x, v.y, v.z]);
            }
            
            // Generate indices for triangles
            for i in 0..TRIANGLES_PER_HEXAGON {
                let next = (i + 1) % HEXAGON_CORNERS;
                indices.push(base_idx);                          // Center
                indices.push(base_idx + i as u32 + 1);          // Current corner
                indices.push(base_idx + next as u32 + 1);       // Next corner
            }
            
            // Generate colors based on terrain
            let color = get_terrain_color_gradient(province.terrain, province.elevation.value());
            let rgba = color.to_linear().to_f32_array();
            for _ in 0..VERTICES_PER_HEXAGON {
                colors.push(rgba);
            }
        }
        
        Ok(())
    }
    
    /// Parallel mesh building for large province counts
    fn build_parallel(
        &self,
        provinces: &[Province],
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        colors: &mut Vec<[f32; 4]>,
    ) -> Result<(), MeshBuildError> {
        // Process provinces in parallel chunks
        let chunks: Result<Vec<_>, _> = provinces
            .par_chunks(PARALLEL_CHUNK_SIZE)
            .enumerate()
            .map(|(chunk_idx, chunk)| {
                self.process_chunk(chunk, chunk_idx * PARALLEL_CHUNK_SIZE)
            })
            .collect();
        
        let chunks = chunks?;
        
        // Merge chunks into final vectors
        for (chunk_vertices, chunk_indices, chunk_colors) in chunks {
            let vertex_offset = vertices.len() as u32;
            
            // Add vertices
            vertices.extend(chunk_vertices);
            
            // Add indices with offset
            indices.extend(chunk_indices.iter().map(|&idx| idx + vertex_offset));
            
            // Add colors
            colors.extend(chunk_colors);
        }
        
        Ok(())
    }
    
    /// Process a single chunk of provinces
    fn process_chunk(
        &self,
        provinces: &[Province],
        start_index: usize,
    ) -> Result<(Vec<[f32; 3]>, Vec<u32>, Vec<[f32; 4]>), MeshBuildError> {
        let mut vertices = Vec::with_capacity(provinces.len() * VERTICES_PER_HEXAGON);
        let mut indices = Vec::with_capacity(provinces.len() * INDICES_PER_HEXAGON);
        let mut colors = Vec::with_capacity(provinces.len() * VERTICES_PER_HEXAGON);
        
        for (local_idx, province) in provinces.iter().enumerate() {
            let global_idx = start_index + local_idx;
            HexagonGeometry::validate_position(province.position, global_idx)?;
            
            let base_idx = (local_idx * VERTICES_PER_HEXAGON) as u32;
            
            // Generate vertices
            let hex_vertices = self.geometry.generate_vertices(province.position);
            for v in &hex_vertices {
                vertices.push([v.x, v.y, v.z]);
            }
            
            // Generate indices
            for i in 0..TRIANGLES_PER_HEXAGON {
                let next = (i + 1) % HEXAGON_CORNERS;
                indices.push(base_idx);
                indices.push(base_idx + i as u32 + 1);
                indices.push(base_idx + next as u32 + 1);
            }
            
            // Generate colors
            let color = get_terrain_color_gradient(province.terrain, province.elevation.value());
            let rgba = color.to_linear().to_f32_array();
            for _ in 0..VERTICES_PER_HEXAGON {
                colors.push(rgba);
            }
        }
        
        Ok((vertices, indices, colors))
    }
    
    /// Calculate memory usage of the mesh
    fn calculate_memory_usage(
        vertices: &[[f32; 3]],
        indices: &[u32],
        colors: &[[f32; 4]],
    ) -> usize {
        vertices.len() * std::mem::size_of::<[f32; 3]>() +
        indices.len() * std::mem::size_of::<u32>() +
        colors.len() * std::mem::size_of::<[f32; 4]>()
    }
}

/// Build the world mega-mesh (compatibility wrapper for existing code)
pub fn build_world_mesh(
    provinces: &[Province],
    meshes: &mut Assets<Mesh>,
) -> Handle<Mesh> {
    match MeshBuilder::default().build(provinces, meshes) {
        Ok((handle, _stats)) => handle,
        Err(e) => {
            error!("Failed to build world mesh: {}", e);
            // Return an empty mesh as fallback
            meshes.add(Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
            ))
        }
    }
}

// ============================================================================
// TEXTURE GENERATION
// ============================================================================

/// Create an antialiased hexagon texture with proper validation
pub fn create_hexagon_texture(size: f32) -> Result<Image, MeshBuildError> {
    // Validate size
    let texture_size = (size * 2.0) as u32;
    if texture_size > MAX_TEXTURE_SIZE {
        return Err(MeshBuildError::TextureSizeTooLarge {
            requested: texture_size,
            max: MAX_TEXTURE_SIZE,
        });
    }
    
    // Pre-allocate texture data
    let total_pixels = (texture_size * texture_size * 4) as usize;
    let mut data = vec![0u8; total_pixels];
    
    let center = texture_size as f32 / 2.0;
    
    // Generate texture pixels
    for y in 0..texture_size {
        for x in 0..texture_size {
            let fx = x as f32 - center;
            let fy = y as f32 - center;
            
            // For flat-top hexagon
            let abs_x = fx.abs();
            let abs_y = fy.abs();
            
            // FLAT-TOP hexagon bounds checking
            // Maximum extent in Y direction (height/2)
            let max_y = size * SQRT_3 / 2.0;
            
            // Distance from hexagon edge (flat-top orientation)
            let dist_vertical = abs_y - max_y;  // Distance from horizontal edges
            let dist_diagonal = (SQRT_3 * abs_x + abs_y) / 2.0 - size;  // Distance from diagonal edges
            let distance_from_edge = dist_vertical.max(dist_diagonal);
            
            // Antialiasing with smooth transition
            let alpha = if distance_from_edge < -ANTIALIASING_WIDTH {
                1.0  // Fully inside
            } else if distance_from_edge > ANTIALIASING_WIDTH {
                0.0  // Fully outside
            } else {
                // Smooth transition using smoothstep
                let t = (distance_from_edge + ANTIALIASING_WIDTH) / (2.0 * ANTIALIASING_WIDTH);
                1.0 - t * t * (3.0 - 2.0 * t)  // Smoothstep for better antialiasing
            };
            
            let idx = ((y * texture_size + x) * 4) as usize;
            let alpha_byte = (alpha * 255.0).clamp(0.0, 255.0) as u8;
            data[idx] = DEFAULT_COLOR_VALUE;        // R
            data[idx + 1] = DEFAULT_COLOR_VALUE;    // G
            data[idx + 2] = DEFAULT_COLOR_VALUE;    // B
            data[idx + 3] = alpha_byte;             // A
        }
    }
    
    let mut image = Image::new(
        Extent3d {
            width: texture_size,
            height: texture_size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    
    // Use linear filtering for smoother edges
    image.sampler = ImageSampler::linear();
    
    Ok(image)
}

/// Create hexagon texture with default size (compatibility wrapper)
pub fn create_default_hexagon_texture() -> Image {
    create_hexagon_texture(HEX_SIZE_PIXELS)
        .unwrap_or_else(|e| {
            error!("Failed to create hexagon texture: {}", e);
            // Return a 1x1 white pixel as fallback
            Image::new(
                Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                vec![255, 255, 255, 255],
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::default(),
            )
        })
}