use super::laplacian::{laplacian_load, LaplacianWeights};

pub fn bisimilar(
    expressive_a: &[f32; 9],
    expressive_b: &[f32; 9],
    weights: &LaplacianWeights,
    membrane_state_a: u8,
    membrane_state_b: u8,
) -> bool {
    let lap_a = laplacian_load(expressive_a, weights);
    let lap_b = laplacian_load(expressive_b, weights);

    (lap_a - lap_b).abs() < 1e-6 &&
    membrane_state_a == membrane_state_b
}
