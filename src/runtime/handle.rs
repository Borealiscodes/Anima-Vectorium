use super::state::RuntimeState;
use super::tick::tick;
use crate::spectral::operator_algebra::Operator;

pub struct RuntimeHandle {
    pub state: RuntimeState,
}

impl RuntimeHandle {
    pub fn new() -> Self {
        Self {
            state: RuntimeState::new(),
        }
    }

    pub fn inject_operator(&mut self, op: Operator) {
        self.state.expressive.data[0] += op.magnitude * op.direction[0];
        tick(&mut self.state);
    }

    pub fn tick(&mut self) {
        tick(&mut self.state);
    }

    pub fn get_state(&self) -> &RuntimeState {
        &self.state
    }
}
