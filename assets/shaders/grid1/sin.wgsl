
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> time: f32;
@group(2) @binding(1) var<uniform> window_size: vec2<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var st: vec2<f32> = mesh.position.xy / window_size.xy; // Normalize the screen space
    let aspect_ratio_st = (2.0 * mesh.position.xy - window_size.xy) / window_size.y;

    // Apply aspect ratio correction to st
    // let stripes = floor(st.x * 13.) % 4;
    let stripes = symmetry_stripes(st.x + 0.5, 80., 1.5);
    // let stripes_vert_filter = symmetry_stripes(st.y, 20., 2.);
    // let stripes_vert_filter_fine = symmetry_stripes(st.y, 4., window_size.y * 0.1);
    // let stripes_vert = step(0.1, sin(st.y * 50) + 1 / 2);
    // let stripes = stripes_horiz_filter * stripes_vert_filter * stripes_vert_filter_fine;

    var color = vec4(stripes, 0., 0., 1.);

    let box = sdf_box(aspect_ratio_st, vec2(0.3, 0.4));

    let bars = sin(-st.y * st.y * st.y * 400 - time * 10);
    if bars > 0.2 && stripes < 0.5 {
        color.b = 1.;
    }

    if box < 0. {
        let grad1 = mix(0.0, 1.0, st.x - 0.2);
        let grad2 = mix(0.0, 0.1, st.y - 0.3);
        color = vec4(grad1, grad2, 0., 0.);
    }
    return color;
}

fn symmetry_stripes(pos: f32, num_stripes: f32, stripe_bias: f32) -> f32 {
    return floor(pos * num_stripes % stripe_bias);
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

fn sdfCircle(pos: vec2<f32>, rad: f32) -> f32 {
    return length(pos) - rad;
}

fn sdf_box(pos: vec2<f32>, corner: vec2<f32>) -> f32 {
    let d: vec2<f32> = abs(pos) - corner;
    return length(max(d, vec2(0.0, 0.0))) + min(max(d.x, d.y), 0.0);
}
