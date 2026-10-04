use crate::spectral::laplacian::LaplacianWeights;
use crate::membrane::membrane_rules::MembraneRules;
use crate::membrane::membrane_clamp::apply_clamp;
use crate::thermodynamics::telemetry::Telemetry;
use crate::thermodynamics::envelope::ThermodynamicEnvelope;
use crate::thermodynamics::power_budget::apply_power_budget;

use super::state::RuntimeState;
use super::operator_dispatch::apply_operator;

pub fn tick(
    state: &mut RuntimeState,
    operator: Option<crate::spectral::operator_algebra::Operator>,
    dt_ms: u64,
) {
    state.timestamp_ms += dt_ms;

    // 1. Apply operator if present
    if let Some(op) = operator {
        apply_operator(&mut state.expressive, &mut state.drift, &op);
    }

    // 2. Compute telemetry
    let weights = LaplacianWeights::default();
    state.telemetry = Telemetry::compute(&state.expressive.data, &weights);

    // 3. Thermodynamic envelope
    let envelope = ThermodynamicEnvelope::default();
    let violated = envelope.envelope_violated(
        state.telemetry.power_mw,
        state.telemetry.spectral_variance,
        state.telemetry.laplacian_load,
    );

    // 4. Membrane evaluation
    let rules = MembraneRules::default();
    state.membrane = rules.evaluate(
        state.expressive.magnitude(),
        violated,
    );

    // 5. Membrane clamp/reset enforcement
    apply_clamp(&mut state.expressive.data, state.membrane);

    // 6. Power budget enforcement
    apply_power_budget(
        &mut state.expressive.data,
        state.telemetry.power_mw,
        envelope.max_power_mw,
    );
}
