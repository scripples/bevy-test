struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) vertex_index: u32
}

struct LineMaterial {
    color: vec4<f32>,
};

@group(2) @binding(0) var<uniform> material: LineMaterial;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var color = mesh.color;
    if mesh.vertex_index % 5 == 1 {
        color = vec4<f32>(1., 1., 0., 1.);
    }
    return color;
}
