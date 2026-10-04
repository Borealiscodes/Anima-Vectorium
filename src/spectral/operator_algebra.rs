use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operator {
    pub op_code: u16,
    pub magnitude: f32,
    pub direction: [f32; 3],
    pub reversible: bool,
}

impl Operator {
    pub fn apply(&self, vector: &mut [f32; 9]) {
        for i in 0..3 {
            vector[i] += self.direction[i] * self.magnitude;
        }
    }

    pub fn reverse(&self, vector: &mut [f32; 9]) {
        if self.reversible {
            for i in 0..3 {
                vector[i] -= self.direction[i] * self.magnitude;
            }
        }
    }
}
