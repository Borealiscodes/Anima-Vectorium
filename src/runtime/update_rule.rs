use super::state::RuntimeState;
use crate::curvature::{activation_curvature::phi, curvature_bound, laplacian};
use crate::holonomy::{holonomy_bound, transport::holonomy};
use crate::operators::{activation::gelu, normalization::layer_norm, residual::residual};
use crate::spectral::laplacian::LaplacianWeights;
use crate::thermodynamics::telemetry::Telemetry;

pub fn update(state: &RuntimeState) -> RuntimeState {
    let mut next = state.clone();

    let base = &state.expressive.data;
    let curvature = laplacian(base);
    let activation = phi(base);
    let holonomy_state = holonomy(base, &curvature);
    let mixed = residual(base, &holonomy_state);
    let normalized = layer_norm(&mixed);
    let activated = gelu(&normalized);

    let mut updated = [0.0; 9];
    for (index, value) in activated.iter().enumerate() {
        updated[index] = value.clamp(-1.0, 1.0);
    }

    next.expressive.data = updated;
    next.expressive.normalize();

    if !curvature_bound(&curvature) {
        next.expressive.data = [0.0; 9];
    }
    if !holonomy_bound(&holonomy_state) {
        next.expressive.data = [0.0; 9];
    }

    next.telemetry = Telemetry::compute(&next.expressive.data, &LaplacianWeights::default());
    next.envelope_violation = next.telemetry.power_mw > 500.0
        || next.telemetry.spectral_variance > 0.75
        || next.telemetry.laplacian_load > 0.50;
    next.timestamp_ms = next.timestamp_ms.saturating_add(16);
    next
}

pub fn jacobian_stability(state: &RuntimeState) -> f32 {
    let magnitude = state.expressive.magnitude();
    let stability = 1.0 / (1.0 + magnitude.max(1e-6));
    stability.clamp(0.0, 1.0)
}
