#[derive(Debug, Clone)]
pub struct ExpressiveVector {
    pub data: [f32; 9],
}

impl ExpressiveVector {
    pub fn new(data: [f32; 9]) -> Self {
        let mut v = Self { data };
        v.normalize();
        v
    }

    pub fn magnitude(&self) -> f32 {
        self.data.iter().map(|x| x * x).sum::<f32>().sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > 1.0 {
            for i in 0..9 {
                self.data[i] /= mag;
            }
        }
    }

    pub fn add_scaled(&mut self, direction: [f32; 3], magnitude: f32) {
        for i in 0..3 {
            self.data[i] += direction[i] * magnitude;
        }
        self.normalize();
    }
}
