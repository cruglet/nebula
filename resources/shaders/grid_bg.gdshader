shader_type canvas_item;

uniform float grid_thickness = 0.25; // Control the grid line thickness

varying vec2 world_pos;

void vertex() {
    world_pos = (MODEL_MATRIX * vec4(VERTEX, 1.0, 1.0)).xy;
}

void fragment() {
    float line_x = abs(mod(world_pos.x, 32.0) - 8.0); // Center grid lines at 8.0
    float line_y = abs(mod(world_pos.y, 32.0) - 8.0); // Center grid lines at 8.0

    if (line_x < grid_thickness || line_y < grid_thickness) {
        COLOR = vec4(0.3, 0.3, 0.35, 0.5);
    }
}
