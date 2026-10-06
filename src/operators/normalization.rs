pub fn layer_norm(x: &[f32]) -> Vec<f32> {
    if x.is_empty() {
        return Vec::new();
    }

    let mean = x.iter().sum::<f32>() / x.len() as f32;
    let variance = x.iter().map(|value| (value - mean).powi(2)).sum::<f32>() / x.len() as f32;
    let scale = 1.0 / variance.sqrt().max(1e-6);

    x.iter().map(|value| (value - mean) * scale).collect()
}
