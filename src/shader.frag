#version 150

in vec2 screen_position;
out vec4 color;

uniform float aspect_ratio;
uniform vec2 camera_position;
uniform float zoom;
uniform vec2 wld_size;
uniform sampler2D wld_texture;

void main() {
	vec2 position = (vec2(screen_position.x - 0.5, (screen_position.y - 0.5) * aspect_ratio) * zoom + camera_position) / wld_size;
	if (position.x < 0.0 || position.x > 1.0 || position.y < 0.0 || position.y > 1.0 ) color = vec4(0.0, 0.0, 0.0, 1.0);
	else color = texture(wld_texture, position);
}