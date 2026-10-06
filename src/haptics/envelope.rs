#[derive(Debug, Clone)]
pub struct HapticEnvelope {
    pub amplitude: f32,
    pub frequency: f32,
    pub temporal_curve: [f32; 4],
}

impl HapticEnvelope {
    pub fn new(amplitude: f32, frequency: f32, temporal_curve: [f32; 4]) -> Self {
        Self {
            amplitude: amplitude.clamp(0.0, 1.0),
            frequency: frequency.clamp(0.0, 1000.0),
            temporal_curve,
        }
    }
}
