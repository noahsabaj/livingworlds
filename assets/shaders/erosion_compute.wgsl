// Hydraulic Erosion Compute Shader for Living Worlds
//
// This shader simulates water droplets eroding terrain in parallel.
// Each thread simulates one water droplet flowing downhill, picking up
// and depositing sediment to create realistic valleys and drainage.

// Input/Output buffers
@group(0) @binding(0) var<storage, read_write> heightmap: array<atomic<i32>>;
@group(0) @binding(1) var<storage, read> droplet_starts: array<vec2<f32>>;
@group(0) @binding(2) var<uniform> params: ErosionParams;

struct ErosionParams {
    width: u32,
    height: u32,
    cell_size: f32,
    seed: u32,
    // Erosion constants
    initial_water: f32,
    evaporation_rate: f32,
    sediment_capacity: f32,
    min_slope: f32,
    erosion_rate: f32,
    deposition_rate: f32,
    gravity: f32,
    inertia: f32,
    max_lifetime: u32,
    thermal_angle_threshold: f32,
    thermal_rate: f32,
}

// Constants for fixed-point conversion
const FIXED_POINT_SCALE: f32 = 1000000.0;  // 6 decimal places precision

// Convert float to fixed-point integer for atomic operations
fn float_to_fixed(val: f32) -> i32 {
    return i32(val * FIXED_POINT_SCALE);
}

// Convert fixed-point integer back to float
fn fixed_to_float(val: i32) -> f32 {
    return f32(val) / FIXED_POINT_SCALE;
}

// Hash function for pseudo-random numbers
fn hash(x: u32, y: u32, seed: u32) -> u32 {
    var h = x ^ (y * 2654435761u) ^ seed;
    h = h * 1664525u + 1013904223u;
    h = h ^ (h >> 16u);
    return h;
}

// Convert hash to random float [0, 1]
fn hash_to_float(h: u32) -> f32 {
    return f32(h) / 4294967295.0;
}

// Get heightmap index from grid coordinates
fn get_index(x: u32, y: u32) -> u32 {
    return y * params.width + x;
}

// Bilinear interpolation for smooth height sampling
fn get_height_interpolated(pos: vec2<f32>) -> f32 {
    // Convert world position to grid coordinates
    let grid_pos = pos / params.cell_size;
    let x = clamp(grid_pos.x, 0.0, f32(params.width - 1u));
    let y = clamp(grid_pos.y, 0.0, f32(params.height - 1u));

    let x0 = u32(floor(x));
    let y0 = u32(floor(y));
    let x1 = min(x0 + 1u, params.width - 1u);
    let y1 = min(y0 + 1u, params.height - 1u);

    let fx = x - f32(x0);
    let fy = y - f32(y0);

    // Read heights from atomic buffer (convert from fixed-point)
    let h00 = fixed_to_float(atomicLoad(&heightmap[get_index(x0, y0)]));
    let h10 = fixed_to_float(atomicLoad(&heightmap[get_index(x1, y0)]));
    let h01 = fixed_to_float(atomicLoad(&heightmap[get_index(x0, y1)]));
    let h11 = fixed_to_float(atomicLoad(&heightmap[get_index(x1, y1)]));

    // Bilinear interpolation
    let h0 = mix(h00, h10, fx);
    let h1 = mix(h01, h11, fx);
    return mix(h0, h1, fy);
}

// Calculate terrain gradient at position
fn calculate_gradient(pos: vec2<f32>) -> vec2<f32> {
    let epsilon = params.cell_size * 0.5;

    let h_left = get_height_interpolated(pos - vec2<f32>(epsilon, 0.0));
    let h_right = get_height_interpolated(pos + vec2<f32>(epsilon, 0.0));
    let h_down = get_height_interpolated(pos - vec2<f32>(0.0, epsilon));
    let h_up = get_height_interpolated(pos + vec2<f32>(0.0, epsilon));

    return vec2<f32>(
        (h_left - h_right) / (2.0 * epsilon),
        (h_down - h_up) / (2.0 * epsilon)
    );
}

// Deposit or erode at position using atomic operations
fn modify_heightmap(pos: vec2<f32>, amount: f32) {
    let grid_pos = pos / params.cell_size;
    let x = u32(grid_pos.x);
    let y = u32(grid_pos.y);

    // Distribute change to neighboring cells based on proximity
    for (var dy = 0u; dy <= 1u; dy = dy + 1u) {
        for (var dx = 0u; dx <= 1u; dx = dx + 1u) {
            let nx = min(x + dx, params.width - 1u);
            let ny = min(y + dy, params.height - 1u);

            let cell_pos = vec2<f32>(f32(nx), f32(ny)) * params.cell_size;
            let distance = length(pos - cell_pos);

            // Linear falloff weight
            var weight = 1.0 - (distance / params.cell_size);
            weight = max(weight, 0.0);

            if (weight > 0.0) {
                let idx = get_index(nx, ny);
                let delta = float_to_fixed(amount * weight);
                atomicAdd(&heightmap[idx], delta);
            }
        }
    }
}

// Water droplet structure (stored in registers)
struct WaterDroplet {
    position: vec2<f32>,
    velocity: vec2<f32>,
    water: f32,
    sediment: f32,
}

// Simulate a single water droplet
fn simulate_droplet(start_pos: vec2<f32>, thread_seed: u32) {
    var droplet: WaterDroplet;
    droplet.position = start_pos;
    droplet.velocity = vec2<f32>(0.0, 0.0);
    droplet.water = params.initial_water;
    droplet.sediment = 0.0;

    // Add small random offset to prevent all droplets taking same path
    let rand_offset = hash_to_float(hash(u32(start_pos.x), u32(start_pos.y), thread_seed));
    droplet.position += vec2<f32>(rand_offset - 0.5) * params.cell_size;

    // Simulate droplet flowing downhill
    for (var lifetime = 0u; lifetime < params.max_lifetime; lifetime = lifetime + 1u) {
        if (droplet.water < 0.001) {
            break;
        }

        let gradient = calculate_gradient(droplet.position);
        let flow_dir = -normalize(gradient);

        // Update velocity with inertia
        droplet.velocity = droplet.velocity * params.inertia + flow_dir * (1.0 - params.inertia);
        if (length(droplet.velocity) > 0.0) {
            droplet.velocity = normalize(droplet.velocity) * min(length(droplet.velocity), 1.0);
        }

        let old_pos = droplet.position;
        droplet.position += droplet.velocity * params.cell_size;

        // Keep within bounds
        droplet.position.x = clamp(droplet.position.x, 0.0, f32(params.width - 1u) * params.cell_size);
        droplet.position.y = clamp(droplet.position.y, 0.0, f32(params.height - 1u) * params.cell_size);

        let old_height = get_height_interpolated(old_pos);
        let new_height = get_height_interpolated(droplet.position);
        let height_diff = new_height - old_height;

        let slope = max(length(gradient), params.min_slope);
        let capacity = slope * droplet.water * length(droplet.velocity) * params.sediment_capacity;

        if (height_diff > 0.0 || droplet.sediment > capacity) {
            // Deposit sediment
            var amount_to_deposit: f32;
            if (height_diff > 0.0) {
                amount_to_deposit = min(height_diff, droplet.sediment);
            } else {
                amount_to_deposit = (droplet.sediment - capacity) * params.deposition_rate;
            }

            droplet.sediment -= amount_to_deposit;
            modify_heightmap(old_pos, amount_to_deposit);
        } else {
            // Erode terrain
            let amount_to_erode = min((capacity - droplet.sediment) * params.erosion_rate, -height_diff);

            droplet.sediment += amount_to_erode;
            modify_heightmap(old_pos, -amount_to_erode);
        }

        // Evaporate water
        droplet.water *= 1.0 - params.evaporation_rate;
    }
}

// Main compute shader entry point
@compute @workgroup_size(64)
fn hydraulic_erosion(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let droplet_index = global_id.x;

    // Bounds check
    if (droplet_index >= arrayLength(&droplet_starts)) {
        return;
    }

    // Get starting position for this droplet
    let start_pos = droplet_starts[droplet_index];

    // Generate unique seed for this droplet
    let thread_seed = params.seed + droplet_index;

    // Simulate the water droplet
    simulate_droplet(start_pos, thread_seed);
}

// Thermal erosion pass - material sliding down slopes
@compute @workgroup_size(8, 8)
fn thermal_erosion(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;

    // Bounds check with margin
    if (x == 0u || x >= params.width - 1u || y == 0u || y >= params.height - 1u) {
        return;
    }

    let center_idx = get_index(x, y);
    let center_height = fixed_to_float(atomicLoad(&heightmap[center_idx]));

    var max_diff = 0.0;
    var lowest_neighbor_x = x;
    var lowest_neighbor_y = y;

    // Check all 8 neighbors
    for (var dy = -1i; dy <= 1i; dy = dy + 1i) {
        for (var dx = -1i; dx <= 1i; dx = dx + 1i) {
            if (dx == 0i && dy == 0i) {
                continue;
            }

            let nx = u32(i32(x) + dx);
            let ny = u32(i32(y) + dy);

            if (nx > 0u && nx < params.width - 1u && ny > 0u && ny < params.height - 1u) {
                let neighbor_idx = get_index(nx, ny);
                let neighbor_height = fixed_to_float(atomicLoad(&heightmap[neighbor_idx]));
                let height_diff = center_height - neighbor_height;

                // Account for diagonal distance
                let distance = select(1.0, 1.414213562, abs(dx) + abs(dy) == 2i);
                let slope = height_diff / (distance * params.cell_size);

                if (slope > params.thermal_angle_threshold && height_diff > max_diff) {
                    max_diff = height_diff;
                    lowest_neighbor_x = nx;
                    lowest_neighbor_y = ny;
                }
            }
        }
    }

    // Transfer material if slope exceeds threshold
    if (max_diff > 0.0) {
        let amount = max_diff * params.thermal_rate;
        let delta = float_to_fixed(amount);

        // Erode from current cell
        atomicSub(&heightmap[center_idx], delta);

        // Deposit to lowest neighbor
        let neighbor_idx = get_index(lowest_neighbor_x, lowest_neighbor_y);
        atomicAdd(&heightmap[neighbor_idx], delta);
    }
}

// Smoothing pass to remove artifacts
@compute @workgroup_size(8, 8)
fn smooth_terrain(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;

    if (x >= params.width || y >= params.height) {
        return;
    }

    var sum = 0.0;
    var count = 0u;
    let radius = 1u;  // Smoothing radius

    // Average heights in neighborhood
    for (var dy = -i32(radius); dy <= i32(radius); dy = dy + 1i) {
        for (var dx = -i32(radius); dx <= i32(radius); dx = dx + 1i) {
            let nx = i32(x) + dx;
            let ny = i32(y) + dy;

            if (nx >= 0i && nx < i32(params.width) && ny >= 0i && ny < i32(params.height)) {
                let idx = get_index(u32(nx), u32(ny));
                sum += fixed_to_float(atomicLoad(&heightmap[idx]));
                count = count + 1u;
            }
        }
    }

    // Write smoothed value
    if (count > 0u) {
        let idx = get_index(x, y);
        let smoothed = float_to_fixed(sum / f32(count));
        atomicStore(&heightmap[idx], smoothed);
    }
}

// Elevation buffers for init and readback operations
// Using vec4 for WebGPU 16-byte alignment - only .x component used
@group(0) @binding(3) var<storage, read> input_elevations: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> output_elevations: array<vec4<f32>>;

// Initialize heightmap from input elevations
@compute @workgroup_size(256)
fn init_heightmap(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    if (index >= arrayLength(&input_elevations)) {
        return;
    }

    // Convert float elevation to fixed-point atomic integer (using .x component)
    let fixed_elevation = float_to_fixed(input_elevations[index].x);
    atomicStore(&heightmap[index], fixed_elevation);
}

// Read back heightmap to float buffer
@compute @workgroup_size(256)
fn readback_heightmap(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    if (index >= arrayLength(&output_elevations)) {
        return;
    }

    // Convert fixed-point atomic integer back to float
    let elevation = fixed_to_float(atomicLoad(&heightmap[index]));
    output_elevations[index] = vec4<f32>(elevation, 0.0, 0.0, 0.0);
}