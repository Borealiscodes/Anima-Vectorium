use crate::spectral::laplacian::{laplacian_load, LaplacianWeights};

#[derive(Debug, Clone)]
pub struct Telemetry {
    pub power_mw: f32,
    pub spectral_variance: f32,
    pub laplacian_load: f32,
}

impl Telemetry {
    pub fn compute(
        expressive: &[f32; 9],
        weights: &LaplacianWeights,
    ) -> Self {
        let variance = expressive.iter().map(|x| x * x).sum::<f32>() / 9.0;
        let lap_load = laplacian_load(expressive, weights);

        // Power is proportional to expressive activity
        let power = variance * 1000.0;

        Self {
            power_mw: power,
            spectral_variance: variance,
            laplacian_load: lap_load,
        }
    }
}
