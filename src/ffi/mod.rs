pub mod packet_serialization;
pub mod bindings;

use crate::runtime::handle::RuntimeHandle;
use crate::spectral::operator_algebra::Operator;
use crate::ffi::packet_serialization::{
    ExpressiveStatePacket,
    ThermodynamicTelemetry,
    HapticEnvelopePacket,
};

static mut RUNTIME: Option<RuntimeHandle> = None;

#[no_mangle]
pub extern "C" fn initialize_vectorium() {
    unsafe {
        RUNTIME = Some(RuntimeHandle::new());
    }
}

#[no_mangle]
pub extern "C" fn inject_operator_json(json: *const libc::c_char) {
    let c_str = unsafe { std::ffi::CStr::from_ptr(json) };
    let json_str = c_str.to_str().unwrap_or("");

    if let Ok(op) = serde_json::from_str::<Operator>(json_str) {
        unsafe {
            if let Some(handle) = &mut RUNTIME {
                handle.inject_operator(op);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn tick_vectorium() {
    unsafe {
        if let Some(handle) = &mut RUNTIME {
            handle.tick();
        }
    }
}

#[no_mangle]
pub extern "C" fn get_expressive_state_json() -> *mut libc::c_char {
    unsafe {
        if let Some(handle) = &RUNTIME {
            let packet = ExpressiveStatePacket::from_state(handle.get_state());
            let json = serde_json::to_string(&packet).unwrap();
            return std::ffi::CString::new(json).unwrap().into_raw();
        }
    }
    std::ffi::CString::new("{}").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn get_telemetry_json() -> *mut libc::c_char {
    unsafe {
        if let Some(handle) = &RUNTIME {
            let packet = ThermodynamicTelemetry::from_state(handle.get_state());
            let json = serde_json::to_string(&packet).unwrap();
            return std::ffi::CString::new(json).unwrap().into_raw();
        }
    }
    std::ffi::CString::new("{}").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn get_haptic_envelope_json() -> *mut libc::c_char {
    unsafe {
        if let Some(handle) = &RUNTIME {
            let packet = HapticEnvelopePacket::from_state(handle.get_state());
            let json = serde_json::to_string(&packet).unwrap();
            return std::ffi::CString::new(json).unwrap().into_raw();
        }
    }
    std::ffi::CString::new("{}").unwrap().into_raw()
}
