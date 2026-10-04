use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HapticEnvelopePacket {
    pub amplitude: f32,
    pub frequency: f32,
    pub temporal_curve: [f32; 4],
    pub device_class: u8,
    pub ndh_safe: bool,
}
