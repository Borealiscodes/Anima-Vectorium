pub fn gelu(x: &[f32]) -> Vec<f32> {
    x.iter()
        .map(|value| {
            let y = *value;
            0.5 * y * (1.0 + y.tanh())
        })
        .collect()
}
