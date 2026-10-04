pub mod ffi;
pub mod spectral;
pub mod runtime;
pub mod membrane;
pub mod haptics;
pub mod thermodynamics;
pub mod utils;

use runtime::state::VectoriumState;
use std::ptr;
use std::boxed::Box;

/// Initialize the Vectorium runtime and return a raw pointer to the state.
#[no_mangle]
pub extern "C" fn initialize_vectorium() -> *mut VectoriumState {
    let state = VectoriumState::new();
    Box::into_raw(Box::new(state))
}

/// Advance the Vectorium runtime by one tick.
#[no_mangle]
pub extern "C" fn tick_vectorium(state_ptr: *mut VectoriumState) {
    if state_ptr.is_null() {
        return;
    }

    // SAFETY: caller guarantees the pointer is valid
    let state = unsafe { &mut *state_ptr };

    // Example tick logic — replace with your actual runtime update
    state.timestamp_ms += 16; // simulate ~60 FPS

    // Update expressive vector (placeholder)
    for v in state.expressive.data.iter_mut() {
        *v += 0.01;
    }

    // Update drift vector (placeholder)
    for d in state.drift.data.iter_mut() {
        *d *= 0.95;
    }

    // Update telemetry (placeholder)
    state.telemetry.power_mw += 0.1;
    state.telemetry.spectral_variance *= 0.99;
    state.telemetry.laplacian_load += 0.05;

    // Update stability score (placeholder)
    state.stability_score = 1.0 - (state.telemetry.spectral_variance * 0.1);

    // Update fog density (placeholder)
    state.fog_density = (state.fog_density + 0.01).min(1.0);

    // Update glyph hint (placeholder)
    state.glyph_hint = (state.glyph_hint + 1) % 512;

    // Update envelope violation (placeholder)
    state.envelope_violation = state.telemetry.power_mw > 10.0;

    // Update haptic envelope (placeholder)
    state.haptic_envelope.amplitude *= 0.98;
    state.haptic_envelope.frequency += 0.05;

    // NDH safety (placeholder)
    state.ndh_safe = state.stability_score > 0.5;
}

/// Free the Vectorium runtime state.
#[no_mangle]
pub extern "C" fn free_vectorium(state_ptr: *mut VectoriumState) {
    if state_ptr.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(state_ptr));
    }
}
