use super::membrane_state::MembraneState;

pub fn apply_clamp(expressive: &mut [f32; 9], state: MembraneState) {
    match state {
        MembraneState::Clamp => {
            // Reduce expressive magnitude safely
            for i in 0..9 {
                expressive[i] *= 0.5;
            }
        }
        MembraneState::Reset => {
            // Full reset to baseline
            for i in 0..9 {
                expressive[i] = 0.0;
            }
        }
        _ => {}
    }
}
