pub mod ffi;
pub mod spectral;
pub mod runtime;
pub mod membrane;
pub mod haptics;
pub mod thermodynamics;
pub mod utils;

// Re-export the runtime state for FFI use
pub use runtime::state::VectoriumState;
