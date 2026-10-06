pub mod activation_curvature;
pub mod laplacian;

pub use activation_curvature::*;
pub use laplacian::*;

pub const CURVATURE_BOUND: f32 = 1.0;

pub fn curvature_bound(values: &[f32]) -> bool {
    values
        .iter()
        .map(|value| value.abs())
        .fold(0.0, f32::max)
        <= CURVATURE_BOUND
}
