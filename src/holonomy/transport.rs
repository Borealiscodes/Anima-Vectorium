pub fn holonomy(x: &[f32], curvature: &[f32]) -> Vec<f32> {
    let mut out = Vec::with_capacity(x.len());
    for i in 0..x.len() {
        let value = x[i] + curvature.get(i).copied().unwrap_or(0.0);
        out.push(value.clamp(-1.0, 1.0));
    }
    out
}
