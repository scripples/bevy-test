#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> time: f32;
@group(2) @binding(1) var<uniform> window_size: vec2<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {

    let st: vec2<f32> = mesh.position.xy / window_size.xy;

    let num_cols = 48.;
    let num_rows = 48.;
    let row_offset = 12.;

    var c = 0.0;
    if floor(st.x * num_cols) % 5. == 0 {
        c = 1.0;
    };

    // Calculate the color
    let r = 1.0 - st.y;       // Increases as we go down
    let g = st.x;             // Increases as we go right
    let b = st.y;             // Increases as we go up
    let a = 1.0 - st.x;       // Increases as we go left

    return vec4<f32>(c, c, c, 1.0);
}

