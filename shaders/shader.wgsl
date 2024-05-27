@vertex
fn vs(@builtin(vertex_index) index: u32) -> @builtin(position) vec4f {
	var positions = array(
		vec2f(-0.5, -0.5),
		vec2f(0, 0.5),
		vec2f(0.5, -0.5),
	);
	
	let pos = positions[index];
	return vec4f(pos.x, pos.y, 0.0, 1.0);
}

@fragment
fn fs() -> @location(0) vec4f {
	return vec4f(1.0, 0.0, 0.0, 1.0);
}
