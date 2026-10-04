use super::membrane_state::MembraneState;

#[derive(Debug, Clone)]
pub struct MembraneRules {
    pub clamp_threshold: f32,
    pub reset_threshold: f32,
}

impl MembraneRules {
    pub fn default() -> Self {
        Self {
            clamp_threshold: 1.0,
            reset_threshold: 1.2,
        }
    }

    pub fn evaluate(
        &self,
        expressive_magnitude: f32,
        envelope_violation: bool,
    ) -> MembraneState {
        if expressive_magnitude > self.reset_threshold {
            return MembraneState::Reset;
        }

        if expressive_magnitude > self.clamp_threshold || envelope_violation {
            return MembraneState::Clamp;
        }

        if expressive_magnitude > 0.8 {
            return MembraneState::Active;
        }

        MembraneState::Neutral
    }
}
