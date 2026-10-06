pub fn stability_metric(state: &[f32]) -> f32 {
    let norm = state.iter().map(|v| v * v).sum::<f32>().sqrt();
    (1.0 / (1.0 + norm.max(1e-6))).clamp(0.0, 1.0)
}
