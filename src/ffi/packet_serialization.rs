use serde::{Serialize, Deserialize};
use crate::runtime::state::VectoriumState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpressiveStatePacket {
    pub timestamp_ms: u64,
    pub expressive_vector: [f32; 9],
    pub drift_vector: [f32; 3],
    pub stability_score: f32,
    pub membrane_state: u8,
    pub fog_density: f32,
    pub glyph_hint: u16,
}

impl ExpressiveStatePacket {
    pub fn from_state(state: &VectoriumState) -> Self {
        Self {
            timestamp_ms: state.timestamp_ms,
            expressive_vector: state.expressive_vector,
            drift_vector: state.drift_vector,
            stability_score: state.stability_score,
            membrane_state: state.membrane_state,
            fog_density: state.fog_density,
            glyph_hint: state.glyph_hint,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperatorInjectionPacket {
    pub op_code: u16,
    pub magnitude: f32,
    pub direction: [f32; 3],
    pub envelope: u8,
    pub reversible: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThermodynamicTelemetry {
    pub power_mw: f32,
    pub envelope_violation: bool,
    pub laplacian_load: f32,
    pub spectral_variance: f32,
}

impl ThermodynamicTelemetry {
    pub fn from_state(state: &VectoriumState) -> Self {
        Self {
            power_mw: state.power_mw,
            envelope_violation: state.envelope_violation,
            laplacian_load: state.laplacian_load,
            spectral_variance: state.spectral_variance,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HapticEnvelopePacket {
    pub amplitude: f32,
    pub frequency: f32,
    pub temporal_curve: [f32; 4],
    pub device_class: u8,
    pub ndh_safe: bool,
}

impl HapticEnvelopePacket {
    pub fn from_state(state: &VectoriumState) -> Self {
        Self {
            amplitude: state.haptic_envelope.amplitude,
            frequency: state.haptic_envelope.frequency,
            temporal_curve: state.haptic_envelope.temporal_curve,
            device_class: state.device_class as u8,
            ndh_safe: state.ndh_safe,
        }
    }
}
