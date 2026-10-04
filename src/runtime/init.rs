use crate::spectral::expressive_vector::ExpressiveVector;
use crate::spectral::drift_clamps::DriftVector;
use crate::membrane::membrane_state::MembraneState;
use crate::thermodynamics::telemetry::Telemetry;

#[derive(Debug, Clone)]
pub struct RuntimeState {
    pub expressive: ExpressiveVector,
    pub drift: DriftVector,
    pub membrane: MembraneState,
    pub telemetry: Telemetry,
    pub timestamp_ms: u64,
}

impl RuntimeState {
    pub fn new() -> Self {
        Self {
            expressive: ExpressiveVector::new([0.0; 9]),
            drift: DriftVector { data: [0.0; 3] },
            membrane: MembraneState::Neutral,
            telemetry: Telemetry {
                power_mw: 0.0,
                spectral_variance: 0.0,
                laplacian_load: 0.0,
            },
            timestamp_ms: 0,
        }
    }
}
