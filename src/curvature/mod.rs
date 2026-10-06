pub fn curvature_bound(values: &[f32]) -> bool {
    values
        .iter()
        .map(|value| value.abs())
        .fold(0.0, f32::max)
        <= CURVATURE_BOUND
}
