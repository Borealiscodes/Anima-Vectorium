use std::f32;

pub fn phi(x: &[f32]) -> Vec<f32> {
    x.iter()
        .map(|value| {
            let magnitude = value.abs();
            let clipped = magnitude.min(1.0);
            (value.signum() * clipped).clamp(-1.0, 1.0)
        })
        .collect()
}
