pub fn gradients(s: &[f32], lambda: f32) -> Vec<f32> {
    s.iter().map(|v| 2.0 * lambda * v).collect()
}
