//! Dissipation check: F(s_{t+1}) ≤ F(s_t)

pub fn dissipates(f_prev: f32, f_next: f32) -> bool {
    f_next <= f_prev + 1e-5
}

pub fn enforce_dissipation(value: f32, max_value: f32) -> f32 {
    if value > max_value {
        max_value * 0.99
    } else {
        value
    }
}
