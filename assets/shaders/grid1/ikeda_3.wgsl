#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> time: f32;
@group(2) @binding(1) var<uniform> window_size: vec2<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var st: vec2<f32> = mesh.position.xy / window_size.xy; // Normalize the screen space
    // st.x *= window_size.x / window_size.y;
    var grid: vec2<f32> = vec2(100., 100.); // Multiply to create grid of unit-sizes
    st *= grid;

    let ipos: vec2<f32> = floor(st);
    let fpos: vec2<f32> = fract(st);

    var vel: vec2<f32> = vec2(time * 0.1 * max(grid.x, grid.y));
    vel *= vec2(-1., 0.0) * (random_f(1.0 + ipos.y) - 0.5) * (random_f(ipos.y) - 0.5);
    var offset: vec2<f32> = vec2(0.1, 0.);

    var color: vec4<f32> = vec4(0., 0., 0., 1.);

    color.r = pattern(st + offset, vel, 0.5);
    color.g = pattern(st, vel, 0.5);
    color.b = pattern(st - offset, vel, 0.5);

    // Margins
    color *= step(0.2, fpos.y);

    return color;
}

fn round_at_f32(in: f32, turn: f32) -> f32 {
    var out: f32 = 0.0;
    if in > turn {
        out = 1.0;
    }
    return out;
}

fn fit01(in: f32, min: f32, max: f32) -> f32 {
    return mix(min, max, in);
}


fn random_f(seed: f32) -> f32 {
    return fract(sin(seed) * 1e4);
}

fn random_vec2(pos: vec2<f32>) -> f32 {
    return fract(sin(dot(pos.xy, vec2(12.9898, 78.233))) * 43758.5453123);
}

fn pattern(pos: vec2<f32>, v: vec2<f32>, t: f32) -> f32 {
    let p: vec2<f32> = floor(pos + v);
    return step(t, random_vec2(100. + p * .000001) + random_f(p.x) * 0.5);
}
