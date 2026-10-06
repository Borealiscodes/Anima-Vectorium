#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass {
    Mobile = 0,
    Desktop = 1,
    Wearable = 2,
    Haptic = 3,
}

impl DeviceClass {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => DeviceClass::Desktop,
            2 => DeviceClass::Wearable,
            3 => DeviceClass::Haptic,
            _ => DeviceClass::Mobile,
        }
    }
}
