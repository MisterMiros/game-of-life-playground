@group(0) @binding(0) var<storage, read> current_grid: array<u32>;
@group(0) @binding(1) var<storage, read_write> next_grid: array<u32>;

struct Params {
    width_in_u32s: u32,
    height: u32,
    cols: u32,
    _padding: u32,
};

@group(0) @binding(2) var<uniform> params: Params;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let u32_idx = global_id.x;
    let y = global_id.y;

    if (u32_idx >= params.width_in_u32s || y >= params.height) {
        return;
    }

    let current_idx = y * params.width_in_u32s + u32_idx;
    let row_curr = current_grid[current_idx];
    
    var row_above = 0u;
    if (y > 0u) {
        row_above = current_grid[(y - 1u) * params.width_in_u32s + u32_idx];
    }
    
    var row_below = 0u;
    if (y + 1u < params.height) {
        row_below = current_grid[(y + 1u) * params.width_in_u32s + u32_idx];
    }

    var row_curr_left = 0u;
    if (u32_idx > 0u) {
        row_curr_left = current_grid[current_idx - 1u];
    }
    
    var row_curr_right = 0u;
    if (u32_idx + 1u < params.width_in_u32s) {
        row_curr_right = current_grid[current_idx + 1u];
    }

    var row_above_left = 0u;
    var row_above_right = 0u;
    if (y > 0u) {
        if (u32_idx > 0u) {
            row_above_left = current_grid[(y - 1u) * params.width_in_u32s + u32_idx - 1u];
        }
        if (u32_idx + 1u < params.width_in_u32s) {
            row_above_right = current_grid[(y - 1u) * params.width_in_u32s + u32_idx + 1u];
        }
    }

    var row_below_left = 0u;
    var row_below_right = 0u;
    if (y + 1u < params.height) {
        if (u32_idx > 0u) {
            row_below_left = current_grid[(y + 1u) * params.width_in_u32s + u32_idx - 1u];
        }
        if (u32_idx + 1u < params.width_in_u32s) {
            row_below_right = current_grid[(y + 1u) * params.width_in_u32s + u32_idx + 1u];
        }
    }

    var result = 0u;
    
    for (var i = 0u; i < 32u; i++) {
        let x = u32_idx * 32u + i;
        if (x >= params.cols) {
            break;
        }

        let is_alive = (row_curr >> i) & 1u;
        var count = 0u;
        
        // Horizontals
        if (i > 0u) {
            count += (row_curr >> (i - 1u)) & 1u;
        } else {
            count += (row_curr_left >> 31u) & 1u;
        }
        if (i < 31u) {
            count += (row_curr >> (i + 1u)) & 1u;
        } else {
            count += (row_curr_right >> 0u) & 1u;
        }
        
        // Above
        count += (row_above >> i) & 1u;
        if (i > 0u) {
            count += (row_above >> (i - 1u)) & 1u;
        } else {
            count += (row_above_left >> 31u) & 1u;
        }
        if (i < 31u) {
            count += (row_above >> (i + 1u)) & 1u;
        } else {
            count += (row_above_right >> 0u) & 1u;
        }
        
        // Below
        count += (row_below >> i) & 1u;
        if (i > 0u) {
            count += (row_below >> (i - 1u)) & 1u;
        } else {
            count += (row_below_left >> 31u) & 1u;
        }
        if (i < 31u) {
            count += (row_below >> (i + 1u)) & 1u;
        } else {
            count += (row_below_right >> 0u) & 1u;
        }

        var next_state = 0u;
        if (is_alive == 1u) {
            if (count == 2u || count == 3u) {
                next_state = 1u;
            }
        } else {
            if (count == 3u) {
                next_state = 1u;
            }
        }
        
        result |= (next_state << i);
    }
    
    next_grid[current_idx] = result;
}
