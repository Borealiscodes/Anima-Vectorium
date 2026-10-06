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
//! ==============================================================================

pub mod curvature;
pub mod ffi;
pub mod holonomy;
pub mod haptics;
pub mod membrane;
pub mod operators;
pub mod runtime;
pub mod spectral;
pub mod thermodynamics;
pub mod utils;

pub use ffi::{
    free_vectorium, get_expressive_state_json, get_haptic_envelope_json,
    get_telemetry_json, initialize_vectorium, inject_operator_json,
    tick_vectorium,
};
pub use runtime::{update_rule, RuntimeState, VectoriumState};
