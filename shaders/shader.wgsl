struct CameraUniform {
	view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) color: vec3<f32>,
}

struct VertexOutput {
	@builtin(position) clip_position: vec4<f32>,
	@location(0) color: vec3<f32>,
}

@vertex
fn vs(input: VertexInput) -> VertexOutput {
	
	var out: VertexOutput;
	out.clip_position = camera.view_proj * vec4<f32>(input.position, 1.0);
	out.color = input.color;
	return out;
}

@fragment
fn fs(in: VertexOutput) -> @location(0) vec4f {
	return vec4f(in.color, 1.0);
}
