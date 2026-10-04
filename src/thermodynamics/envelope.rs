#[derive(Debug, Clone)]
pub struct ThermodynamicEnvelope {
    pub max_power_mw: f32,
    pub max_variance: f32,
    pub max_laplacian_load: f32,
}

impl ThermodynamicEnvelope {
    pub fn default() -> Self {
        Self {
            max_power_mw: 500.0,        // mobile-safe power envelope
            max_variance: 0.75,        // expressive variance bound
            max_laplacian_load: 0.50,  // spectral load bound
        }
    }

    pub fn envelope_violated(
        &self,
        power_mw: f32,
        spectral_variance: f32,
        laplacian_load: f32,
    ) -> bool {
        power_mw > self.max_power_mw ||
        spectral_variance > self.max_variance ||
        laplacian_load > self.max_laplacian_load
    }
}
