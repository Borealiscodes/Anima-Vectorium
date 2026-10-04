#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MembraneState {
    Neutral = 0,   // ○
    Active = 1,    // ⚡
    Clamp = 2,     // ⛒
    Reset = 3,     // ↻
}

impl MembraneState {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => MembraneState::Active,
            2 => MembraneState::Clamp,
            3 => MembraneState::Reset,
            _ => MembraneState::Neutral,
        }
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }
}
