//! ======================= FFI Module — Copilot Guidance ===========================
//! Implements cross-language invariants and Rust/Python boundary conditions.
//!
//! Copilot must ensure:
//! - κ(s_rust) == κ(s_python)
//! - H(s_rust) == H(s_python)
//! - F(s_rust) == F(s_python)
//! - Operator ordering is preserved across the boundary
//! - No deprecated spectral constructs cross the FFI boundary
//! ================================================================================

pub mod bindings;
pub mod packet_serialization;

pub use bindings::*;
pub use packet_serialization::*;

use crate::runtime::state::VectoriumState;
use crate::runtime::operator_dispatch::apply_operator;
use crate::runtime::tick::tick;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn initialize_vectorium() -> *mut VectoriumState {
    let state = VectoriumState::new();
    Box::into_raw(Box::new(state))
}

#[no_mangle]
pub extern "C" fn tick_vectorium(state_ptr: *mut VectoriumState) {
    if state_ptr.is_null() {
        return;
    }
    let state = unsafe { &mut *state_ptr };
    tick(state);
}

#[no_mangle]
pub extern "C" fn free_vectorium(state_ptr: *mut VectoriumState) {
    if state_ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(state_ptr));
    }
}

#[no_mangle]
pub extern "C" fn get_expressive_state_json(state_ptr: *mut VectoriumState) -> *mut c_char {
    if state_ptr.is_null() {
        return CString::new("{}").unwrap().into_raw();
    }
    let state = unsafe { &*state_ptr };
    let packet = ExpressiveStatePacket::from_state(state);
    let json = serde_json::to_string(&packet).unwrap_or_default();
    CString::new(json).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn get_telemetry_json(state_ptr: *mut VectoriumState) -> *mut c_char {
    if state_ptr.is_null() {
        return CString::new("{}").unwrap().into_raw();
    }
    let state = unsafe { &*state_ptr };
    let packet = ThermodynamicTelemetry::from_state(state);
    let json = serde_json::to_string(&packet).unwrap_or_default();
    CString::new(json).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn get_haptic_envelope_json(state_ptr: *mut VectoriumState) -> *mut c_char {
    if state_ptr.is_null() {
        return CString::new("{}").unwrap().into_raw();
    }
    let state = unsafe { &*state_ptr };
    let packet = HapticEnvelopePacket::from_state(state);
    let json = serde_json::to_string(&packet).unwrap_or_default();
    CString::new(json).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn inject_operator_json(
    state_ptr: *mut VectoriumState,
    json: *const c_char
) {
    if state_ptr.is_null() || json.is_null() {
        return;
    }
    let state = unsafe { &mut *state_ptr };
    let c_str = unsafe { std::ffi::CStr::from_ptr(json) };
    let json_str = c_str.to_str().unwrap_or("");
    
    if let Ok(op) = serde_json::from_str::<crate::spectral::operator_algebra::Operator>(json_str) {
        apply_operator(&mut state.expressive, &mut state.drift, &op);
    }
}
