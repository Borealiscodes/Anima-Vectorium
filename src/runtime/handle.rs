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
        tick(&mut self.state, Some(op), 16);
    }

    pub fn tick(&mut self) {
        tick(&mut self.state, None, 16);
    }

    pub fn get_state(&self) -> &RuntimeState {
        &self.state
    }
}
