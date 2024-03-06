use linux_embedded_hal::I2cdev;
use ktd2026_led_driver::KTD2026;
use ktd2026_led_driver::constants::*;

use std::thread::sleep;
use std::time::Duration;

// device address
const DEVICE_ADDR:u8 = 0x30;
const TRANSITION_TIME: u64 = 1000;

fn main(){
    // create I2C instance
    let i2c_dev = I2cdev::new("/dev/i2c-5").unwrap();

    // create a led driver device
    let mut led_driver = KTD2026::new(i2c_dev, DEVICE_ADDR);

    // initialize led driver
    // RGB convention used in here
    // first value is RED, then Green, then Blue
    led_driver.init(Ktd2026Channel::value(Ktd2026Channel::Channel3),
                    Ktd2026Channel::value(Ktd2026Channel::Channel1),
                    Ktd2026Channel::value(Ktd2026Channel::Channel2));
    
    loop{
        // turn on Red color with full brightness
        led_driver.led_on(Ktd2026LED::LedRed, Ktd2026Mode::LedAlwaysOn, 0xff).unwrap();
        sleep(Duration::from_millis(TRANSITION_TIME));

        // turn off Red color
        led_driver.led_on(Ktd2026LED::LedRed, Ktd2026Mode::LedAlwaysOff, 0xff).unwrap();
        sleep(Duration::from_millis(TRANSITION_TIME));

        // turn on Blue color with full brightness
        led_driver.led_on(Ktd2026LED::LedBlue, Ktd2026Mode::LedAlwaysOn, 0xff).unwrap();
        sleep(Duration::from_millis(TRANSITION_TIME));

        // turn off Blue color
        led_driver.led_on(Ktd2026LED::LedBlue, Ktd2026Mode::LedAlwaysOff, 0xff).unwrap();
        sleep(Duration::from_millis(TRANSITION_TIME));


        // turn on Green color with full brightness
        led_driver.led_on(Ktd2026LED::LedGreen, Ktd2026Mode::LedAlwaysOn, 0xff).unwrap();
        sleep(Duration::from_millis(TRANSITION_TIME));

        // turn off Green color
        led_driver.led_on(Ktd2026LED::LedGreen, Ktd2026Mode::LedAlwaysOff, 0xff).unwrap();
        sleep(Duration::from_millis(TRANSITION_TIME));

        // fade in red color
        for bright in (0..=255).step_by(25){
            led_driver.led_on(Ktd2026LED::LedRed, Ktd2026Mode::LedAlwaysOn, bright).unwrap();
            sleep(Duration::from_millis(TRANSITION_TIME/10));
        }
        
        // fade out red color
        for bright in (0..=255).step_by(25).rev(){
            led_driver.led_on(Ktd2026LED::LedRed, Ktd2026Mode::LedAlwaysOn, bright).unwrap();
            sleep(Duration::from_millis(TRANSITION_TIME/10));
        }

        // turn on red color with 50% brightness
        // this has 192 brightness levels
        // 50% --> 96th level which is 0x60
        led_driver.led_on(Ktd2026LED::LedRed, Ktd2026Mode::LedAlwaysOn, 0x60).unwrap();
        sleep(Duration::from_millis(TRANSITION_TIME));

        // apend blue color with 50% brightness
        led_driver.led_on(Ktd2026LED::LedBlue, Ktd2026Mode::LedAlwaysOn, 0x60).unwrap();
        sleep(Duration::from_millis(TRANSITION_TIME));

        // turn off red color
        led_driver.led_on(Ktd2026LED::LedRed, Ktd2026Mode::LedAlwaysOff, 0x60).unwrap();
        sleep(Duration::from_millis(TRANSITION_TIME));

        // apend green color with 75% brightness
        led_driver.led_on(Ktd2026LED::LedGreen, Ktd2026Mode::LedAlwaysOn, 0x90).unwrap();
        sleep(Duration::from_millis(TRANSITION_TIME));


        // turn off blue color
        led_driver.led_on(Ktd2026LED::LedBlue, Ktd2026Mode::LedAlwaysOff, 0xff).unwrap();
        sleep(Duration::from_millis(TRANSITION_TIME));

        // turn off green color
        led_driver.led_on(Ktd2026LED::LedGreen, Ktd2026Mode::LedAlwaysOff, 0xff).unwrap();
        sleep(Duration::from_millis(TRANSITION_TIME * 2));
    }
}
