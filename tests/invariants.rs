//! ======================= Vectorium v5.0 — Invariant Test Harness =================
//! These tests enforce the core invariants defined in the v5.0 math backbone.
//! ==============================================================================

use vectorium::curvature::{curvature_bound, laplacian};
use vectorium::holonomy::{holonomy_bound, transport::holonomy};
use vectorium::runtime::{RuntimeState, update_rule::{jacobian_stability, update}};

#[test]
fn curvature_bound_is_respected() {
    let sample = [0.5f32; 9];
    let curvature = laplacian(&sample);
    assert!(curvature_bound(&curvature));
}

#[test]
fn holonomy_bound_is_respected() {
    let sample = [0.25f32; 9];
    let curvature = laplacian(&sample);
    let loop_state = holonomy(&sample, &curvature);
    assert!(holonomy_bound(&loop_state));
}

#[test]
fn free_energy_dissipates() {
    let initial = RuntimeState::new();
    let next = update(&initial);
    assert!(next.telemetry.power_mw <= 500.0 + 1e-4);
}

#[test]
fn jacobian_stability_is_preserved() {
    let state = RuntimeState::new();
    let jacobian = jacobian_stability(&state);
    assert!(jacobian <= 1.0);
}

#[test]
fn ffi_consistency_is_preserved() {
    let state = RuntimeState::new();
    let packet = vectorium::ffi::packet_serialization::ExpressiveStatePacket::from_state(&state);
    let json = serde_json::to_string(&packet).unwrap();
    let decoded: vectorium::ffi::packet_serialization::ExpressiveStatePacket =
        serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.expressive_vector, packet.expressive_vector);
}
