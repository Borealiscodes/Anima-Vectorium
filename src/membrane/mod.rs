//! ======================= Membrane Module — Copilot Guidance ======================
//! Implements geometric and thermodynamic boundary conditions for state flow.
//!
//! Copilot must:
//! - Preserve curvature and holonomy invariants at boundaries
//! - Maintain dissipation and stability conditions
//! - Avoid generating deprecated spectral constructs
//! ================================================================================

pub mod membrane_state;
pub mod membrane_rules;
pub mod membrane_clamp;
