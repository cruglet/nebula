shader_type canvas_item;

uniform float zoom : hint_range(0.1, 10.0) = 1.0;
uniform float snapping_distance = 32.0;
uniform int grid_pattern = 0; // 0 = lines, 1 = dots
uniform vec4 grid_minor : source_color = vec4(0.5, 0.5, 0.5, 0.5);
uniform vec4 grid_major : source_color = vec4(0.2, 0.2, 0.2, 1.0);

const int GRID_MINOR_STEPS_PER_MAJOR = 8;

void fragment() {
	vec2 uv = (FRAGCOORD.xy / zoom) / snapping_distance;
	ivec2 grid_pos = ivec2(floor(uv));
	vec2 cell = fract(uv);
	vec4 col = vec4(0.0);

	if (grid_pattern == 0) {
		float line_thickness = 1.0 / zoom;
		bool major_x = (abs(grid_pos.x) % GRID_MINOR_STEPS_PER_MAJOR == 0);
		bool major_y = (abs(grid_pos.y) % GRID_MINOR_STEPS_PER_MAJOR == 0);

		if (cell.x < line_thickness) {
			col = major_x ? grid_major : grid_minor;
		}
		if (cell.y < line_thickness) {
			col = major_y ? grid_major : grid_minor;
		}
	} else {
		float minor_alpha = clamp((zoom - 0.4), 0.0, 1.0);
		vec4 minor_col = vec4(grid_minor.rgb, grid_minor.a * minor_alpha);
		bool is_major = (abs(grid_pos.x) % GRID_MINOR_STEPS_PER_MAJOR == 0) &&
		                (abs(grid_pos.y) % GRID_MINOR_STEPS_PER_MAJOR == 0);

		vec2 dot_center = vec2(0.5);
		float dist = distance(cell, dot_center);
		float radius = 0.1 / zoom;

		if (dist < radius) {
			col = is_major ? grid_major : minor_col;
		}
	}

	COLOR = col;
}