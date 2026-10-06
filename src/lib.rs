//! ======================= Vectorium v5.0 — Copilot Guidance =======================
//! This crate implements the mathematical backbone defined in:
//! preprints/anima_vectorium_v5.0_mathématique/anima_vectorium_v5.0_mathématique.tex
//!
//! Copilot must follow these rules:
//! - Preserve operator ordering:
//!   Activation → Normalization → Drive Arbitration → Residual Mixing → Curvature Update → Holonomy Transport
//! - Maintain invariants:
//!   ||κ(x)|| ≤ K_max, |∮ κ · dγ| ≤ H_max, F(s_{t+1}) ≤ F(s_t), ||J|| ≤ 1
//! - Respect FFI consistency: κ, H, F identical across Rust/Python
//! - Do NOT implement fossilized spectral constructs (see DO_NOT_IMPLEMENT.md)
//! ================================================================================

pub mod ffi;
pub mod spectral;
pub mod runtime;
pub mod membrane;
pub mod haptics;
pub mod thermodynamics;
pub mod utils;

// Re-export the runtime state for FFI use
pub use runtime::state::VectoriumState;
