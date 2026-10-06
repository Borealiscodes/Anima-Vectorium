//! ======================= Thermodynamics Module — Copilot Guidance ===============
//! Implements free-energy functional, gradients, dissipation, and attractor basins.
//!
//! Copilot must:
//! - Use F(s) = A(s) + C(s) + S(s) exactly as defined in v5.0
//! - Preserve dissipation invariants
//! - Maintain gradient correctness for A, C, and S
//! - Avoid implementing spectral curvature or collapse-fractal recursion
//! ==============================================================================

pub mod dissipation;
pub mod free_energy;
pub mod gradients;

pub use dissipation::*;
pub use free_energy::*;
pub use gradients::*;

pub const DISSIPATION_EPSILON: f32 = 1e-5;
