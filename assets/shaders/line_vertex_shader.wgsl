#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}

struct LineMaterial {
    color: vec4<f32>,
};
@group(2) @binding(0) var<uniform> material: LineMaterial;
@group(2) @binding(1) var<uniform> time: f32;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) vertex_index: u32
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var vtx = vertex;
    vtx.position.y += sin(time) / 4;
    out.clip_position = mesh_position_local_to_clip(
        get_model_matrix(vtx.instance_index),
        vec4<f32>(vtx.position, 1.0),
    );
    out.color = out.clip_position;
    out.vertex_index = vertex.vertex_index + u32(time);
    return out;
}
