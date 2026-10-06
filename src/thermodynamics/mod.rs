//! ======================= Thermodynamics Module — Copilot Guidance ===============
//! Implements free-energy functional, gradients, dissipation, and attractor basins.
//!
//! Copilot must:
//! - Use F(s) = A(s) + C(s) + S(s) exactly as defined in v5.0
//! - Preserve dissipation invariants
//! - Maintain gradient correctness for A, C, and S
//! - Avoid implementing spectral curvature or collapse-fractal recursion
//! ================================================================================

pub mod envelope;
pub mod telemetry;
pub mod power_budget;
