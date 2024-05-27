@vertex
fn vs(@builtin(vertex_index) index: u32) -> @builtin(position) vec4f {
	let positions = [
		vec2f(-1, -1),
		vec2f(0, 1),
		vec2f(1, -1),
	];
	
	let pos = positions[index]
	return vec4f(pos.x, pos.y, 0.0, 1.0);
}

@fragment
fn fs() -> @location(0) vec4f {
	return vec4f(1.0, 0.0, 0.0, 1.0);
}
