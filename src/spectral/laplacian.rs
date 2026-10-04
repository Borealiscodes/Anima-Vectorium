#[derive(Debug, Clone)]
pub struct LaplacianWeights {
    pub w: [f32; 9],
}

impl LaplacianWeights {
    pub fn default() -> Self {
        Self { w: [1.0 / 9.0; 9] }
    }
}

pub fn laplacian_load(vector: &[f32; 9], weights: &LaplacianWeights) -> f32 {
    let mut load = 0.0;
    for i in 0..9 {
        load += weights.w[i] * vector[i].powi(2);
    }
    load
}
