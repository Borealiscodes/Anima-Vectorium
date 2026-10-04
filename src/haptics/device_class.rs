#[derive(Debug, Clone)]
pub struct HapticEnvelope {
    pub amplitude: f32,
    pub frequency: f32,
    pub temporal_curve: [f32; 4],
}

impl HapticEnvelope {
    pub fn new(amplitude: f32, frequency: f32, temporal_curve: [f32; 4]) -> Self {
        Self {
            amplitude,
            frequency,
            temporal_curve,
        }
    }

    pub fn scale(&mut self, factor: f32) {
        self.amplitude *= factor;
        self.frequency *= factor;
    }
}
