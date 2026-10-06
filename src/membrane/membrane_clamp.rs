use super::membrane_state::MembraneState;

pub fn apply_clamp(expressive: &mut [f32; 9], membrane: MembraneState) {
    match membrane {
        MembraneState::Neutral => {
            for i in 0..9 {
                expressive[i] = expressive[i].clamp(-1.0, 1.0);
            }
        }
        MembraneState::Active => {
            for i in 0..9 {
                expressive[i] = expressive[i].clamp(-0.95, 0.95);
            }
        }
        MembraneState::Clamp => {
            for i in 0..9 {
                expressive[i] = expressive[i].clamp(-0.8, 0.8);
            }
        }
        MembraneState::Reset => {
            for i in 0..9 {
                expressive[i] = 0.0;
            }
        }
    }
}
