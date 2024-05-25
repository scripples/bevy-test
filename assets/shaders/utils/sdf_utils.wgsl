#define_import_path sdf_utils

fn sdfBox(p: vec2<f32>, b: vec2<f32>) {
    let dist = abs(p) - b;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

fn sdfCircle(pos: vec2<f32>, rad: f32) -> f32 {
    return length(pos) - rad;
}
