pub mod packet_serialization;
pub mod bindings;

use crate::runtime::state::VectoriumState;
use crate::runtime::operator_dispatch::apply_operator;
use crate::ffi::packet_serialization::{
    ExpressiveStatePacket,
    ThermodynamicTelemetry,
    HapticEnvelopePacket,
};

use std::ffi::CString;
use std::os::raw::c_char;

/// Create a new Vectorium runtime and return a raw pointer.
#[no_mangle]
pub extern "C" fn initialize_vectorium() -> *mut VectoriumState {
    let state = VectoriumState::new();
    Box::into_raw(Box::new(state))
}

/// Advance the runtime by one tick.
#[no_mangle]
pub extern "C" fn tick_vectorium(state_ptr: *mut VectoriumState) {
    if state_ptr.is_null() {
        return;
    }

    let state = unsafe { &mut *state_ptr };

    // Your tick logic lives here
    state.timestamp_ms += 16;
}

/// Free the runtime state.
#[no_mangle]
pub extern "C" fn free_vectorium(state_ptr: *mut VectoriumState) {
    if state_ptr.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(state_ptr));
    }
}

/// Get expressive packet as JSON.
#[no_mangle]
pub extern "C" fn get_expressive_state_json(state_ptr: *mut VectoriumState) -> *mut c_char {
    if state_ptr.is_null() {
        return CString::new("{}").unwrap().into_raw();
    }

    let state = unsafe { &mut *state_ptr };
    let packet = ExpressiveStatePacket::from_state(state);
    CString::new(serde_json::to_string(&packet).unwrap()).unwrap().into_raw()
}

/// Get thermodynamic telemetry as JSON.
#[no_mangle]
pub extern "C" fn get_telemetry_json(state_ptr: *mut VectoriumState) -> *mut c_char {
    if state_ptr.is_null() {
        return CString::new("{}").unwrap().into_raw();
    }

    let state = unsafe { &mut *state_ptr };
    let packet = ThermodynamicTelemetry::from_state(state);
    CString::new(serde_json::to_string(&packet).unwrap()).unwrap().into_raw()
}

/// Get haptic envelope packet as JSON.
#[no_mangle]
pub extern "C" fn get_haptic_envelope_json(state_ptr: *mut VectoriumState) -> *mut c_char {
    if state_ptr.is_null() {
        return CString::new("{}").unwrap().into_raw();
    }

    let state = unsafe { &mut *state_ptr };
    let packet = HapticEnvelopePacket::from_state(state);
    CString::new(serde_json::to_string(&packet).unwrap()).unwrap().into_raw()
}

//
// ────────────────────────────────────────────────────────────────
//   Artifact 13 — Operator Injection (Stabilized Routing)
// ────────────────────────────────────────────────────────────────
//

use crate::spectral::operator_algebra::Operator;

#[no_mangle]
pub extern "C" fn inject_operator_json(
    state_ptr: *mut VectoriumState,
    json: *const libc::c_char
) {
    if state_ptr.is_null() || json.is_null() {
        return;
    }

    let state = unsafe { &mut *state_ptr };
    let c_str = unsafe { std::ffi::CStr::from_ptr(json) };
    let json_str = c_str.to_str().unwrap_or("");

    if let Ok(op) = serde_json::from_str::<Operator>(json_str) {
        // Route through the runtime's operator dispatch path
        apply_operator(&mut state.expressive, &mut state.drift, &op);
    }
}
