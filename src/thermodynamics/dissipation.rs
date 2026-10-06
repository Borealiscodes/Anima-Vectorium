pub fn dissipates(prev: f32, next: f32) -> bool {
    next <= prev + 1e-5
}
