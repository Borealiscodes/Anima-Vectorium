use libc::{c_char, c_uint};
use std::ffi::{CStr, CString};

use crate::ffi::packet_serialization::{
    ExpressiveStatePacket,
    OperatorInjectionPacket,
    ThermodynamicTelemetry,
    HapticEnvelopePacket,
};

static mut LAST_EXPRESSIVE_STATE: Option<ExpressiveStatePacket> = None;
static mut LAST_TELEMETRY: Option<ThermodynamicTelemetry> = None;
static mut LAST_HAPTIC: Option<HapticEnvelopePacket> = None;

#[no_mangle]
pub extern "C" fn inject_operator_json(json_ptr: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(json_ptr) };
    let json = c_str.to_str().unwrap();

    let packet: OperatorInjectionPacket =
        serde_json::from_str(json).expect("Invalid OperatorInjectionPacket JSON");

    // Store or dispatch packet (runtime cluster will implement this)
    println!("[FFI] Received operator packet: {:?}", packet);
}

#[no_mangle]
pub extern "C" fn get_expressive_state_json() -> *mut c_char {
    let packet = unsafe {
        LAST_EXPRESSIVE_STATE
            .clone()
            .unwrap_or(default_expressive_state())
    };

    let json = serde_json::to_string(&packet).unwrap();
    CString::new(json).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn get_thermodynamic_telemetry_json() -> *mut c_char {
    let packet = unsafe {
        LAST_TELEMETRY
            .clone()
            .unwrap_or(default_telemetry())
    };

    let json = serde_json::to_string(&packet).unwrap();
    CString::new(json).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn get_haptic_envelope_json() -> *mut c_char {
    let packet = unsafe {
        LAST_HAPTIC
            .clone()
            .unwrap_or(default_haptic())
    };

    let json = serde_json::to_string(&packet).unwrap();
    CString::new(json).unwrap().into_raw()
}

fn default_expressive_state() -> ExpressiveStatePacket {
    ExpressiveStatePacket {
        timestamp_ms: 0,
        expressive_vector: [0.0; 9],
        drift_vector: [0.0; 3],
        stability_score: 1.0,
        membrane_state: 0,
        fog_density: 0.0,
        glyph_hint: 0,
    }
}

fn default_telemetry() -> ThermodynamicTelemetry {
    ThermodynamicTelemetry {
        power_mw: 0.0,
        envelope_violation: false,
        laplacian_load: 0.0,
        spectral_variance: 0.0,
    }
}

fn default_haptic() -> HapticEnvelopePacket {
    HapticEnvelopePacket {
        amplitude: 0.0,
        frequency: 0.0,
        temporal_curve: [0.0; 4],
        device_class: 0,
        ndh_safe: true,
    }
}
