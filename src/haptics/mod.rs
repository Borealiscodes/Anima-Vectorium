//! ======================= Haptics Module — Copilot Guidance =======================
//! Implements interface-layer transformations and safe external interactions.
//!
//! Copilot must:
//! - Preserve invariants when exposing state externally
//! - Maintain operator ordering when interfacing with runtime
//! - Avoid deprecated spectral constructs
//! ================================================================================

pub mod envelope;
pub mod device_class;
pub mod ndh_safety;
