// Batch Perlin Noise Generation Compute Shader for Living Worlds
//
// This shader generates terrain elevation for millions of provinces in parallel.
// It implements Perlin noise with Fractal Brownian Motion (FBM) for realistic terrain.

// Input/Output buffers
@group(0) @binding(0) var<storage, read> positions: array<vec2<f32>>;
// Note: Using vec4 for elevations to satisfy WebGPU's 16-byte alignment requirement
// Only the .x component contains the actual elevation value
@group(0) @binding(1) var<storage, read_write> elevations: array<vec4<f32>>;

// Noise parameters uniform
@group(0) @binding(2) var<uniform> params: NoiseParams;

struct NoiseParams {
    seed: u32,
    octaves: u32,
    frequency: f32,
    persistence: f32,
    lacunarity: f32,
    amplitude: f32,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
}

// Constants for hash functions
const HASH_MULTIPLIER: u32 = 1664525u;
const HASH_INCREMENT: u32 = 1013904223u;

// Hash function for pseudo-random number generation
fn hash(x: u32) -> u32 {
    var h = x;
    h = h * HASH_MULTIPLIER + HASH_INCREMENT;
    h = h ^ (h >> 16u);
    h = h * HASH_MULTIPLIER + HASH_INCREMENT;
    h = h ^ (h >> 16u);
    return h;
}

// Convert hash to float in range [0, 1]
fn hash_to_float(h: u32) -> f32 {
    return f32(h) / 4294967295.0;
}

// 2D hash function
fn hash2(x: i32, y: i32, seed: u32) -> u32 {
    let h = hash(u32(x) ^ hash(u32(y) ^ seed));
    return h;
}

// Smooth interpolation curve
fn smoothstep(t: f32) -> f32 {
    return t * t * (3.0 - 2.0 * t);
}

// Linear interpolation
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}

// 2D gradient vectors for Perlin noise
fn gradient2(hash: u32) -> vec2<f32> {
    let h = hash & 7u;
    let u = select(0.0, 1.0, (h & 1u) == 0u);
    let v = select(0.0, 1.0, (h & 2u) == 0u);
    let x = select(-u, u, (h & 4u) == 0u);
    let y = select(-v, v, h >= 4u);
    return normalize(vec2<f32>(x, y));
}

// 2D Perlin noise function
fn perlin2d(p: vec2<f32>, seed: u32) -> f32 {
    let pi = vec2<i32>(floor(p));
    let pf = fract(p);

    // Smooth interpolation weights
    let w = vec2<f32>(smoothstep(pf.x), smoothstep(pf.y));

    // Get gradients at four corners
    let g00 = gradient2(hash2(pi.x, pi.y, seed));
    let g10 = gradient2(hash2(pi.x + 1, pi.y, seed));
    let g01 = gradient2(hash2(pi.x, pi.y + 1, seed));
    let g11 = gradient2(hash2(pi.x + 1, pi.y + 1, seed));

    // Calculate dot products
    let d00 = dot(g00, pf);
    let d10 = dot(g10, pf - vec2<f32>(1.0, 0.0));
    let d01 = dot(g01, pf - vec2<f32>(0.0, 1.0));
    let d11 = dot(g11, pf - vec2<f32>(1.0, 1.0));

    // Interpolate
    let x0 = lerp(d00, d10, w.x);
    let x1 = lerp(d01, d11, w.x);
    return lerp(x0, x1, w.y);
}

// Fractal Brownian Motion (FBM) - combines multiple octaves of noise
fn fbm(p: vec2<f32>, seed: u32, octaves: u32, persistence: f32, lacunarity: f32) -> f32 {
    var value = 0.0;
    var amplitude = 1.0;
    var frequency = 1.0;
    var max_amplitude = 0.0;

    for (var i = 0u; i < octaves; i = i + 1u) {
        value += perlin2d(p * frequency, seed + i * 7u) * amplitude;
        max_amplitude += amplitude;
        amplitude *= persistence;
        frequency *= lacunarity;
    }

    // Normalize to [-1, 1] then to [0, 1]
    return (value / max_amplitude) * 0.5 + 0.5;
}

// Ridge noise for mountain ridges
fn ridge_noise(p: vec2<f32>, seed: u32, frequency: f32) -> f32 {
    let noise = perlin2d(p * frequency, seed);
    let ridge = 1.0 - abs(noise);
    return ridge * ridge; // Square for sharper peaks
}

// Main terrain generation function combining multiple noise layers
fn generate_terrain(pos: vec2<f32>, params: NoiseParams) -> f32 {
    let p = (pos + vec2<f32>(params.offset_x, params.offset_y)) * params.scale;

    // Continental shelf layer (40% influence) - massive landmasses
    let continental = fbm(
        p * 0.001,
        params.seed,
        4u,
        0.4,
        2.0
    ) * 0.4;

    // Major landmass layer (30% influence) - large terrain features
    let landmass = fbm(
        p * 0.005,
        params.seed + 100u,
        6u,
        0.5,
        2.1
    ) * 0.3;

    // Island chains layer (15% influence) - medium scale features
    let islands = fbm(
        p * 0.02,
        params.seed + 200u,
        6u,
        0.45,
        2.2
    ) * 0.15;

    // Coastal detail layer (10% influence) - fine features
    let coastal = fbm(
        p * 0.08,
        params.seed + 300u,
        6u,
        0.4,
        2.3
    ) * 0.1;

    // Mountain ridge layer (5% influence) - sharp peaks
    let ridge = ridge_noise(p, params.seed + 400u, 0.025) * 0.05;

    // Combine all layers
    let combined = continental + landmass + islands + coastal + ridge;

    // Apply amplitude and ensure [0, 1] range
    return clamp(combined * params.amplitude, 0.0, 1.0);
}

// Compute shader entry point
@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    // Bounds check
    if (index >= arrayLength(&positions)) {
        return;
    }

    // Get position for this province
    let pos = positions[index];

    // Generate terrain elevation
    let elevation = generate_terrain(pos, params);

    // Write result
    elevations[index] = vec4<f32>(elevation, 0.0, 0.0, 0.0);
}

// Alternative entry point for simple noise (for testing)
@compute @workgroup_size(256)
fn simple_noise(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    if (index >= arrayLength(&positions)) {
        return;
    }

    let pos = positions[index];
    let scaled_pos = pos * params.frequency;

    // Simple FBM without all the terrain layers
    let noise = fbm(
        scaled_pos,
        params.seed,
        params.octaves,
        params.persistence,
        params.lacunarity
    );

    elevations[index] = vec4<f32>(noise * params.amplitude, 0.0, 0.0, 0.0);
}