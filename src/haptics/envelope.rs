#[derive(Debug, Clone, Copy)]
pub enum DeviceClass {
    Mobile = 0,
    Controller = 1,
    DesktopPeripheral = 2,
}

impl DeviceClass {
    pub fn max_amplitude(self) -> f32 {
        match self {
            DeviceClass::Mobile => 0.4,
            DeviceClass::Controller => 0.7,
            DeviceClass::DesktopPeripheral => 1.0,
        }
    }

    pub fn max_frequency(self) -> f32 {
        match self {
            DeviceClass::Mobile => 180.0,
            DeviceClass::Controller => 220.0,
            DeviceClass::DesktopPeripheral => 260.0,
        }
    }
}
