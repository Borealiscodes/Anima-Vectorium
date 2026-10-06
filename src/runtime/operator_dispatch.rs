use crate::spectral::operator_algebra::Operator;
use crate::spectral::drift_clamps::DriftVector;
use crate::spectral::expressive_vector::ExpressiveVector;

pub fn apply_operator(
    expressive: &mut ExpressiveVector,
    drift: &mut DriftVector,
    operator: &Operator,
) {
    operator.apply(&mut expressive.data);
    
    for i in 0..3 {
        drift.data[i] += operator.direction[i] * operator.magnitude * 0.1;
    }
    
    expressive.normalize();
}
