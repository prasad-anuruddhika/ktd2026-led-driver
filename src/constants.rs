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

    pub fn from_value(value: u8) -> Option<Self>{
        match value {
            0x00 => Some(Register::RegEnableReset),
            0x01 => Some(Register::RegFlashPeriod),
            0x02 => Some(Register::RegFlashOnTime1),
            0x03 => Some(Register::RegFlashOnTime2),
            0x04 => Some(Register::RegChannelControl),
            0x05 => Some(Register::RegRampRate),
            0x06 => Some(Register::RegLED1CurrentOut),
            0x07 => Some(Register::RegLED2CurrentOut),
            0x08 => Some(Register::RegLED3CurrentOut),
            0x09 => Some(Register::RegLED4CurrentOut),
            _ => None,
        }
    }
}

pub trait RegMask {
    fn mask(self) -> u8;
}

pub trait RegBitPos{
    fn position(self) -> u8;
}

pub trait RegValue{
    fn value(self) -> u8;
}

pub trait Value{
    fn value(self) -> u8;
}

pub enum RegEnableResetMask{
    MaskTCtrlReset              = 0x07,             // Bit [2:0]
}

impl RegMask for RegEnableResetMask {
    fn mask(self) -> u8 {
        self as u8
    }
}

pub enum RegEnableResetBitPos{
    PosTCtrlReset               = 0,
}

impl RegBitPos for RegEnableResetBitPos {
    fn position(self) -> u8 {
        self as u8
    }
}

pub enum TCtrlResetModes{
    Tslot1Ctrl                  = 0x00,
    Tslot2Ctrl                  = 0x01,
    Tslot3Ctrl                  = 0x02,
    Tslot4Ctrl                  = 0x03,
    DoNothing                   = 0x04,
    ResetRegistersOnly          = 0x05,
    ResetMainDigitalOnly        = 0x06,
    ResetCompleteChip           = 0x07,
}

impl RegValue for TCtrlResetModes {
    fn value(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Copy)]
pub enum Ktd2026LED {
    LedG                        = 0,
    LedB                        = 2,
    LedR                        = 4,
}

impl Value for Ktd2026LED {
    fn value(self) -> u8 {
        self as u8
    }
}

pub enum Ktd2026Mode {
    LedAlwaysOff                = 0x00,
    LedAlwaysOn                 = 0x01,
    LedSetPwm1                  = 0x02,
    LedSetPwm2                  = 0x03,
}

impl Value for Ktd2026Mode {
    fn value(self) -> u8 {
        self as u8
    }
}