use super::state::RuntimeState;
use super::update_rule::{update, jacobian_stability};

pub fn tick(state: &mut RuntimeState) {
    let next = update(state);
    let stability = jacobian_stability(&next);
    
    if stability <= 1.0 {
        *state = next;
    } else {
        state.envelope_violation = true;
    }
    
    state.timestamp_ms = state.timestamp_ms.saturating_add(16);
}
