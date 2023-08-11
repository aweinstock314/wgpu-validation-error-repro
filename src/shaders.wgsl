const QUAD_VERTICES: array<vec4<f32>, 6> = array<vec4<f32>, 6>(
    vec4<f32>(-1.0, -1.0, 0.0, 1.0),
    vec4<f32>(1.0, -1.0, 0.0, 1.0),
    vec4<f32>(-1.0, 1.0, 0.0, 1.0),
    vec4<f32>(-1.0, 1.0, 0.0, 1.0),
    vec4<f32>(1.0, -1.0, 0.0, 1.0),
    vec4<f32>(1.0, 1.0, 0.0, 1.0),
);

const OFFSETS: array<vec4<f32>, 4> = array<vec4<f32>, 4>(
	vec4(-1.0, -1.0, 0.0, 0.0),
	vec4(-1.0, 1.0, 0.0, 0.0),
	vec4(1.0, -1.0, 1.0, 0.0),
	vec4(1.0, 1.0, 0.0, 0.0),
);

const COLORS: array<vec4<f32>, 4> = array<vec4<f32>, 4>(
	vec4(1.0, 0.0, 0.0, 1.0),
	vec4(0.0, 1.0, 0.0, 1.0),
	vec4(0.0, 0.0, 1.0, 1.0),
	vec4(1.0, 1.0, 0.0, 1.0),
);

struct FragData {
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec4<f32>,
}

@vertex
fn vert_main(@builtin(vertex_index) vertex_index: u32, @builtin(instance_index) instance_index: u32) -> FragData {
    var quad_vertices = QUAD_VERTICES;
    let position = quad_vertices[vertex_index % 6u];
	var offsets = OFFSETS;
	let offset = offsets[instance_index % 4u];
	var colors = COLORS;
	let color = colors[instance_index % 4u];
	var frag_data: FragData;
	frag_data.position = vec4<f32>(position.xy / 2.0, 0.0, 1.0) + offset / 2.0;
	frag_data.color = color;
	return frag_data;
}

@fragment
fn frag_main(frag_data: FragData) -> @location(0) vec4<f32> {
	return frag_data.color;
}

