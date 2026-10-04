pub fn apply_power_budget(expressive: &mut [f32; 9], power_mw: f32, max_power_mw: f32) {
    if power_mw > max_power_mw {
        let scale = max_power_mw / power_mw;
        for i in 0..9 {
            expressive[i] *= scale;
        }
    }
}
