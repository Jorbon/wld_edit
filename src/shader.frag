#version 150

in vec2 screen_position;
out vec4 color;

uniform float aspect_ratio;
uniform vec2 camera_position;
uniform float zoom;
uniform vec2 wld_size;

uniform usampler2D tile_data_buffer;
uniform sampler2D block_texture;

void main() {
	vec2 position = vec2(screen_position.x - 0.5, -(screen_position.y - 0.5) * aspect_ratio) * zoom + camera_position;
	ivec2 tile_position = ivec2(position);
	vec2 sub_tile_position = mod(position, 1.0);
	
	uvec3 tile_data = texelFetch(tile_data_buffer, tile_position, 0).xyz;
	
	if (position.x < 0.0 || position.x > wld_size.x || position.y < 0.0 || position.y > wld_size.y ) {
		color = vec4(0.0, 0.0, 0.0, 1.0);
		return;
	}
	
	if ((tile_data.x & uint(1)) == uint(1) && (tile_data.y & uint(65535)) == uint(21)) {
		color = texelFetch(block_texture, ivec2(sub_tile_position * 8.0) + ivec2(tile_data.z & uint(65535), tile_data.z >> 16), 0);
	} else {
		color = vec4(0.5, 0.5, 0.5, 1.0);
	}
	
}