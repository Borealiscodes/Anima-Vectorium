pub fn residual(x: &[f32], y: &[f32]) -> Vec<f32> {
    let len = x.len().min(y.len());
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let merged = x[i] + y[i] * 0.5;
        out.push(merged.clamp(-1.0, 1.0));
    }
    out
}
