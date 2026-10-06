use super::CURVATURE_BOUND;

pub fn laplacian(x: &[f32]) -> Vec<f32> {
    let mut out = vec![0.0; x.len()];
    for i in 0..x.len() {
        let left = if i == 0 { x[i] } else { x[i] - x[i - 1] };
        let right = if i + 1 == x.len() { x[i] } else { x[i + 1] - x[i] };
        out[i] = left + right;
        out[i] = out[i].clamp(-CURVATURE_BOUND, CURVATURE_BOUND);
    }
    out
}
