use crate::spectral::expressive_vector::ExpressiveVector;
use crate::spectral::drift_clamps::DriftVector;
use crate::membrane::membrane_state::MembraneState;
use crate::thermodynamics::telemetry::Telemetry;
use crate::haptics::device_class::HapticEnvelope;
use crate::haptics::envelope::DeviceClass;

#[derive(Debug, Clone)]
pub struct RuntimeState {
    // Existing fields
    pub expressive: ExpressiveVector,
    pub drift: DriftVector,
    pub membrane: MembraneState,
    pub telemetry: Telemetry,
    pub timestamp_ms: u64,

    // Added fields for ExpressiveStatePacket
    pub stability_score: f32,
    pub fog_density: f32,
    pub glyph_hint: u16,

    // Added fields for ThermodynamicTelemetry
    pub envelope_violation: bool,

    // Added fields for HapticEnvelopePacket
    pub haptic_envelope: HapticEnvelope,
    pub device_class: DeviceClass,
    pub ndh_safe: bool,
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

            // New fields initialized
            stability_score: 0.0,
            fog_density: 0.0,
            glyph_hint: 0,

            envelope_violation: false,

            haptic_envelope: HapticEnvelope::new(0.0, 0.0, [0.0; 4]),
            device_class: DeviceClass::Mobile,
            ndh_safe: true,
        }
    }
}
