//! ======================= Runtime Module — Copilot Guidance =======================
//! Implements state evolution, update rule, and runtime invariants.
//!
//! Copilot must:
//! - Implement the update rule exactly:
//!   s_{t+1} = ResidualMix(s_t, H(s_t - η * SelectedDrive(s_t)))
//! - Preserve dissipation: F(s_{t+1}) ≤ F(s_t)
//! - Maintain Jacobian stability: ||J|| ≤ 1
//! - Enforce operator ordering strictly
//! - Avoid fossilized spectral constructs
//! ================================================================================

pub mod init;
pub mod tick;
pub mod state;
pub mod operator_dispatch;
pub mod handle;
