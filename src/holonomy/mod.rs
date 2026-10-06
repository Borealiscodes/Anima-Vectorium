pub mod transport;

pub use transport::*;

pub const HOLONOMY_BOUND: f32 = 1.0;

pub fn holonomy_bound(values: &[f32]) -> bool {
    values
        .iter()
        .map(|value| value.abs())
        .fold(0.0, f32::max)
        <= HOLONOMY_BOUND
}
