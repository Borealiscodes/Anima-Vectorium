//! Gradients of free-energy components for optimization.

pub fn accuracy_gradient(x: &[f32], x_reconstructed: &[f32]) -> Vec<f32> {
    let mut grad = Vec::with_capacity(x.len());
    for i in 0..x.len() {
        let diff = x.get(i).copied().unwrap_or(0.0) - x_reconstructed.get(i).copied().unwrap_or(0.0);
        grad.push(-2.0 * diff / (x.len() as f32).max(1.0));
    }
    grad
}

pub fn complexity_gradient(s: &[f32], lambda: f32) -> Vec<f32> {
    s.iter().map(|x| 2.0 * lambda * x).collect()
}

pub fn surprise_gradient(p_log: f32) -> f32 {
    1.0 / p_log.abs().max(1e-6)
}
