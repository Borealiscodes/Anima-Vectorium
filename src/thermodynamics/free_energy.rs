//! Free-energy functional F(s) = A(s) + C(s) + S(s)
//! A: accuracy (reconstruction error)
//! C: complexity (norm penalty)
//! S: surprise (entropy)

pub fn accuracy(x: &[f32], x_reconstructed: &[f32]) -> f32 {
    let mut acc = 0.0;
    for i in 0..x.len() {
        let diff = x.get(i).copied().unwrap_or(0.0) - x_reconstructed.get(i).copied().unwrap_or(0.0);
        acc += diff * diff;
    }
    acc / (x.len() as f32).max(1.0)
}

pub fn complexity(s: &[f32], lambda: f32) -> f32 {
    let norm_sq = s.iter().map(|x| x * x).sum::<f32>();
    lambda * norm_sq
}

pub fn surprise(p_log: f32) -> f32 {
    -p_log.clamp(-10.0, 0.0)
}

pub fn free_energy(x: &[f32], s: &[f32], x_reconstructed: &[f32], lambda: f32, p_log: f32) -> f32 {
    accuracy(x, x_reconstructed) + complexity(s, lambda) + surprise(p_log)
}
