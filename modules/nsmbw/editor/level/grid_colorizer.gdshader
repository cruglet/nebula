shader_type canvas_item;

void fragment() {
    vec4 tex_color = texture(TEXTURE, UV);
    COLOR = tex_color * vec4(1, 1, 1, 1);
}
