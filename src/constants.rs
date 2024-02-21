#[derive(Clone, Copy)]
pub enum Register{
    RegEnableReset              = 0x00,
    RegFlashPeriod              = 0x01,
    RegFlashOnTime1             = 0x02,
    RegFlashOnTime2             = 0x03,
    RegChannelControl           = 0x04,
    RegRampRate                 = 0x05,
    RegLED1CurrentOut           = 0x06,
    RegLED2CurrentOut           = 0x07,
    RegLED3CurrentOut           = 0x08,
    RegLED4CurrentOut           = 0x09,
}

impl Register {
    pub fn addr(self) -> u8{
        self as u8
    }
}