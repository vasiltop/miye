struct CameraUniform {
	view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var smpler: sampler;

struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) clip_position: vec4<f32>,
	@location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs(input: VertexInput) -> VertexOutput {
	
	var out: VertexOutput;
	out.clip_position = camera.view_proj * vec4<f32>(input.position, 1.0);
	out.tex_coords = input.tex_coords;
	return out;
}

@fragment
fn fs(in: VertexOutput) -> @location(0) vec4f {
	return textureSample(texture, smpler, in.tex_coords);
}
