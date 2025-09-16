//! Mesh building and management for the mega-mesh rendering architecture
//!
//! This module implements a high-performance mesh system that renders up to 9,000,000
//! hexagonal provinces in a single draw call. It includes optimizations for memory
//! usage and parallel processing.
//!
//! # Performance Characteristics
//! - Single draw call for entire world (9M provinces)
//! - ~2.4GB GPU memory for 9M provinces
//! - Parallel mesh building with rayon for 4-8x speedup on multicore systems
//! - CPU-accessible mesh for dynamic overlay updates
//!
//! # Design Decision: No Vertex Deduplication
//! We deliberately do NOT use vertex deduplication because:
//! 1. It causes unwanted color blending at tile boundaries (GPU interpolation)
//! 2. The memory savings (75MB) are negligible on modern systems
//! 3. Independent vertices provide crisp, distinct tile boundaries
//! 4. Simplifies color overlay updates (1:1 vertex-to-color mapping)

use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use rayon::prelude::*;
use std::collections::HashMap;

use crate::math::{
    Hexagon, CORNERS as HEXAGON_CORNERS, HEX_SIZE as HEX_SIZE_PIXELS,
    INDICES_PER_HEX as INDICES_PER_HEXAGON, TRIANGLES_PER_HEX as TRIANGLES_PER_HEXAGON,
    VERTICES_PER_HEX as VERTICES_PER_HEXAGON,
};
use crate::world::WorldColors;
use crate::world::{Province, ProvinceId};

/// Minimum number of provinces to trigger parallel processing
const PARALLEL_CHUNK_SIZE: usize = 10_000;

/// Chunk size for parallel processing batches
const PARALLEL_BATCH_SIZE: usize = 5_000;

/// Errors that can occur during mesh building
#[derive(Debug, thiserror::Error)]
pub enum MeshBuildError {
    #[error("No provinces provided for mesh building")]
    NoProvinces,

    #[error("Too many vertices ({0}) - exceeds u32 index limit")]
    VertexIndexOverflow(usize),

    #[error("Invalid province position at index {index}: ({x}, {y})")]
    InvalidPosition { index: usize, x: f32, y: f32 },
}

/// Statistics about the mesh build process
#[derive(Debug, Clone)]
pub struct MeshBuildStats {
    pub total_provinces: usize,
    pub total_vertices: usize,
    pub total_indices: usize,
    pub memory_usage_mb: f32,
    pub build_time_ms: f32,
}

impl std::fmt::Display for MeshBuildStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Built {} provinces with {} vertices and {} indices ({:.1}MB in {:.1}ms)",
            self.total_provinces,
            self.total_vertices,
            self.total_indices,
            self.memory_usage_mb,
            self.build_time_ms
        )
    }
}

/// Helper for hexagon vertex generation
struct HexagonGeometry {
    hex_size: f32,
}

impl HexagonGeometry {
    fn new(hex_size: f32) -> Self {
        Self { hex_size }
    }

    /// Generate the 7 vertices for a hexagon (center + 6 corners)
    fn generate_vertices(&self, position: Vec2) -> Vec<Vec3> {
        let hexagon = Hexagon::with_size(position, self.hex_size);
        let corners = hexagon.corners();

        let mut vertices = Vec::with_capacity(VERTICES_PER_HEXAGON);

        // Center vertex
        vertices.push(Vec3::new(position.x, position.y, 0.0));

        // Corner vertices
        for corner in &corners {
            vertices.push(Vec3::new(corner.x, corner.y, 0.0));
        }

        vertices
    }

    /// Validate that a position is reasonable
    fn validate_position(pos: Vec2, index: usize) -> Result<(), MeshBuildError> {
        if !pos.x.is_finite() || !pos.y.is_finite() {
            return Err(MeshBuildError::InvalidPosition {
                index,
                x: pos.x,
                y: pos.y,
            });
        }
        Ok(())
    }
}

/// Handle to the world mega-mesh
#[derive(Resource)]
pub struct WorldMeshHandle(pub Handle<Mesh>);

/// Storage for province data (not entities in mega-mesh architecture)
#[derive(Resource, Default, Reflect)]
pub struct ProvinceStorage {
    pub provinces: Vec<Province>,
    pub province_by_id: HashMap<ProvinceId, usize>,
}

impl ProvinceStorage {
    /// Create storage from a list of provinces
    pub fn from_provinces(provinces: Vec<Province>) -> Self {
        let province_by_id = provinces
            .iter()
            .enumerate()
            .map(|(idx, p)| (p.id, idx))
            .collect();

        Self {
            provinces,
            province_by_id,
        }
    }

    /// Get a province by its ID
    pub fn get_by_id(&self, id: ProvinceId) -> Option<&Province> {
        self.province_by_id
            .get(&id)
            .map(|&idx| &self.provinces[idx])
    }
}

/// Mesh builder with configuration options
pub struct MeshBuilder {
    geometry: HexagonGeometry,
    world_seed: u32,
}

impl Default for MeshBuilder {
    fn default() -> Self {
        Self {
            geometry: HexagonGeometry::new(HEX_SIZE_PIXELS),
            world_seed: 0, // Default seed - should be overridden by with_seed()
        }
    }
}

impl MeshBuilder {
    pub fn new(hex_size: f32) -> Self {
        Self {
            geometry: HexagonGeometry::new(hex_size),
            world_seed: 0, // Default seed - should be overridden by with_seed()
        }
    }

    pub fn with_seed(mut self, seed: u32) -> Self {
        self.world_seed = seed;
        self
    }

    pub fn build(
        &self,
        provinces: &[Province],
        meshes: &mut Assets<Mesh>,
    ) -> Result<(Handle<Mesh>, MeshBuildStats), MeshBuildError> {
        let start_time = std::time::Instant::now();

        // Validate input
        if provinces.is_empty() {
            return Err(MeshBuildError::NoProvinces);
        }

        let total_vertices = provinces.len() * VERTICES_PER_HEXAGON;
        if total_vertices > u32::MAX as usize {
            return Err(MeshBuildError::VertexIndexOverflow(total_vertices));
        }

        // Pre-allocate vectors with exact capacity
        let mut vertices = Vec::with_capacity(total_vertices);
        let mut indices = Vec::with_capacity(provinces.len() * INDICES_PER_HEXAGON);
        let mut colors = Vec::with_capacity(total_vertices);

        // Choose build method based on province count
        if provinces.len() > PARALLEL_CHUNK_SIZE {
            self.build_parallel(provinces, &mut vertices, &mut indices, &mut colors)?;
        } else {
            self.build_sequential(provinces, &mut vertices, &mut indices, &mut colors)?;
        }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors.clone());
        mesh.insert_indices(Indices::U32(indices.clone()));

        let handle = meshes.add(mesh);

        let memory_usage = Self::calculate_memory_usage(&vertices, &indices, &colors);
        let stats = MeshBuildStats {
            total_vertices: vertices.len(),
            total_indices: indices.len(),
            total_provinces: provinces.len(),
            memory_usage_mb: memory_usage as f32 / (1024.0 * 1024.0),
            build_time_ms: start_time.elapsed().as_secs_f32() * 1000.0,
        };

        info!("Mesh built: {}", stats);

        Ok((handle, stats))
    }

    /// Parallel mesh building for large province counts
    fn build_parallel(
        &self,
        provinces: &[Province],
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        colors: &mut Vec<[f32; 4]>,
    ) -> Result<(), MeshBuildError> {
        let chunks: Vec<_> = provinces
            .par_chunks(PARALLEL_BATCH_SIZE)
            .enumerate()
            .map(|(chunk_idx, chunk)| {
                let mut chunk_vertices = Vec::with_capacity(chunk.len() * VERTICES_PER_HEXAGON);
                let mut chunk_indices = Vec::with_capacity(chunk.len() * INDICES_PER_HEXAGON);
                let mut chunk_colors = Vec::with_capacity(chunk.len() * VERTICES_PER_HEXAGON);

                let base_province_idx = chunk_idx * PARALLEL_BATCH_SIZE;

                for (local_idx, province) in chunk.iter().enumerate() {
                    let province_idx = base_province_idx + local_idx;
                    let base_vertex_idx = (province_idx * VERTICES_PER_HEXAGON) as u32;

                    // Validate position
                    if !province.position.x.is_finite() || !province.position.y.is_finite() {
                        return Err(MeshBuildError::InvalidPosition {
                            index: province_idx,
                            x: province.position.x,
                            y: province.position.y,
                        });
                    }

                    // Generate vertices
                    let hex_vertices = self.geometry.generate_vertices(province.position);
                    for v in &hex_vertices {
                        chunk_vertices.push([v.x, v.y, v.z]);
                    }

                    // Generate indices
                    for i in 0..TRIANGLES_PER_HEXAGON {
                        let next = (i + 1) % HEXAGON_CORNERS;
                        chunk_indices.push(base_vertex_idx);
                        chunk_indices.push(base_vertex_idx + i as u32 + 1);
                        chunk_indices.push(base_vertex_idx + next as u32 + 1);
                    }

                    // Generate colors
                    let world_colors = WorldColors::new(self.world_seed);
                    let color = world_colors.terrain(
                        province.terrain,
                        province.elevation.value(),
                        province.position,
                    );

                    // Use proper linear sRGB conversion as per Bevy 0.16 docs
                    let rgba = color.to_linear().to_f32_array();
                    for _ in 0..VERTICES_PER_HEXAGON {
                        chunk_colors.push(rgba);
                    }
                }

                Ok((chunk_vertices, chunk_indices, chunk_colors))
            })
            .collect::<Result<Vec<_>, MeshBuildError>>()?;

        // Combine chunks
        for (chunk_vertices, chunk_indices, chunk_colors) in chunks {
            vertices.extend(chunk_vertices);
            indices.extend(chunk_indices);
            colors.extend(chunk_colors);
        }

        Ok(())
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

            let base_idx = (index * VERTICES_PER_HEXAGON) as u32;

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
            let world_colors = WorldColors::new(self.world_seed);
            let color =
                world_colors.terrain(province.terrain, province.elevation.value(), province.position);
            // Use proper linear sRGB conversion as per Bevy 0.16 docs
            let rgba = color.to_linear().to_f32_array();
            for _ in 0..VERTICES_PER_HEXAGON {
                colors.push(rgba);
            }
        }

        Ok(())
    }

    /// Calculate memory usage of the mesh
    fn calculate_memory_usage(
        vertices: &[[f32; 3]],
        indices: &[u32],
        colors: &[[f32; 4]],
    ) -> usize {
        vertices.len() * std::mem::size_of::<[f32; 3]>()
            + indices.len() * std::mem::size_of::<u32>()
            + colors.len() * std::mem::size_of::<[f32; 4]>()
    }
}

pub fn build_world_mesh(provinces: &[Province], meshes: &mut Assets<Mesh>, world_seed: u32) -> Handle<Mesh> {
    match MeshBuilder::default().with_seed(world_seed).build(provinces, meshes) {
        Ok((handle, _stats)) => handle,
        Err(e) => {
            error!("Failed to build world mesh: {}", e);
            meshes.add(Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
            ))
        }
    }
}
