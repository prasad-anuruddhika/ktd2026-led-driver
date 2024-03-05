use constants::*;

mod device_impl;
pub mod constants;

pub struct KTD2026<I2C>{
    i2c: I2C,
    address: u8,
    ktd2026_data: Ktd2026Data,
}

#[derive(Default, Debug)]
pub struct Ktd2026LedParam {
    channel: u8,
    brightness: u8,
    // delay_on_time_ms: u64,
    // delay_off_time_ms: u64,
}

#[derive(Default, Debug)]
pub struct Ktd2026Data {
    leds: [Ktd2026LedParam; MAX_LED_COUNT],
    shadow_register: [u8; KTD2026_NUM_REGISTERS],
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
