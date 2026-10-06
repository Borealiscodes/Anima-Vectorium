pub fn free_energy(s: &[f32], lambda: f32) -> f32 {
    let complexity = lambda * s.iter().map(|v| v * v).sum::<f32>();
    complexity
}
