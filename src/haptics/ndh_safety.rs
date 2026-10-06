pub fn check_ndh_safety(amplitude: f32, frequency: f32) -> bool {
    amplitude < 1.0 && frequency < 500.0
}

pub fn enforce_ndh_limits(amplitude: &mut f32, frequency: &mut f32) {
    *amplitude = amplitude.clamp(0.0, 0.95);
    *frequency = frequency.clamp(0.0, 450.0);
}
