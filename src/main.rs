//! ======================= Vectorium Runtime Entry — Copilot Guidance ==============
//! This file orchestrates high-level runtime behavior for the Vectorium engine.
//!
//! Copilot must:
//! - Use the update rule exactly as defined in v5.0
//! - Ensure all state evolution preserves invariants and operator ordering
//! - Avoid generating deprecated spectral constructs
//! - Consult COPILOT_MAP.md before generating logic
//! ================================================================================

use std::time::{Duration, Instant};
use vectorium::{
    initialize_vectorium,
    tick_vectorium,
    free_vectorium,
    VectoriumState,
};

fn main() {
    // Initialize the Vectorium runtime
    let state_ptr = initialize_vectorium();
    
    if state_ptr.is_null() {
        eprintln!("Failed to initialize Vectorium runtime");
        return;
    }

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    println!("Vectorium runtime initialized. Running ticks at ~60 FPS...");

    loop {
        if last_tick.elapsed() >= tick_rate {
            tick_vectorium(state_ptr);
            last_tick = Instant::now();
        }
    }
    
    // Cleanup (unreachable in infinite loop, but good for completeness)
    free_vectorium(state_ptr);
}
