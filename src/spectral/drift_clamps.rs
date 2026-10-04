#[derive(Debug, Clone)]
pub struct DriftVector {
    pub data: [f32; 3],
}

impl DriftVector {
    pub fn magnitude(&self) -> f32 {
        self.data.iter().map(|x| x * x).sum::<f32>().sqrt()
    }

    pub fn rotational_angle(&self, expressive: &[f32; 9]) -> f32 {
        let e = &expressive[0..3];
        let dot = e[0] * self.data[0] + e[1] * self.data[1] + e[2] * self.data[2];

        let mag_e = (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt();
        let mag_d = self.magnitude();

        if mag_e == 0.0 || mag_d == 0.0 {
            return 0.0;
        }

        (dot / (mag_e * mag_d)).acos()
    }

    pub fn clamp_triggered(&self, expressive: &[f32; 9], drift_max: f32, angle_max: f32) -> bool {
        self.magnitude() > drift_max ||
        self.rotational_angle(expressive) > angle_max
    }
}
