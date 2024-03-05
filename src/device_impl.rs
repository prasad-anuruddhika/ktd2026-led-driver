use embedded_hal::blocking::i2c;

use crate::Ktd2026Data;
use crate::Ktd2026LedParam;
use crate::KTD2026;
use crate::constants::*;

impl <I2C, E> KTD2026<I2C> 
where
I2C: i2c::Write<Error = E> + i2c::WriteRead <Error = E>
{
    // create a new instance of KTD2026 device
    pub fn new(i2c: I2C, address: u8) -> Self{
        KTD2026{
            i2c,
            address,
            ktd2026_data: Default::default(),
        }
    }

    // destroy driver instace, return I2C bus instance
    pub fn destroy(self) -> I2C{
        self.i2c
    }
}


impl <I2C, E> KTD2026<I2C> 
where
I2C: i2c::Write<Error = E> + i2c::WriteRead <Error = E>
{
    // method for read register data
    pub fn read_register(&mut self, reg: Register) -> Result<u8, E>{
        let mut data = [0x00; 1];
        self.i2c.write_read(self.address, &[reg.addr()], &mut data)?;

        Ok(data[0])
    }

    // method for write data to a register
    pub fn write_register(&mut self, reg: Register, data: u8) -> Result<(), E>{
        self.i2c.write(self.address, &[reg.addr(), data])?;

        Ok(())
    }

    // method for modify multiple bits of a register by using bitmask
    pub fn modify_register(&mut self, reg: Register,
                            mask: u8, position: u8, value: u8) -> Result<(), E>{
        
        // read required register value
        let reg_data = self.read_register(reg)?;
        
        // update the value of register using a mask
        let updated_value = (reg_data & !mask) | ((value << position) & mask);
        
        // write the updated value to the register
        self.write_register(reg, updated_value)?;

        Ok(())
    }

    // method for initialize the driver
    // as this led chip has i2c read issue, this driver keep a shadow memory
    // this will create a shadow memory and populate default values to it
    pub fn init(&mut self, ch: (u8, u8, u8), brightness: (u8, u8, u8)){
        let led_red = Ktd2026LedParam{
            channel: ch.0,
            brightness: brightness.0,
        };

        let led_green: Ktd2026LedParam = Ktd2026LedParam{
            channel: ch.1,
            brightness: brightness.1,
        };

        let led_blue = Ktd2026LedParam{
            channel: ch.2,
            brightness: brightness.2,
        };

        self.ktd2026_data = Ktd2026Data{
            leds: [led_red, led_green, led_blue],
            shadow_register: [0; KTD2026_NUM_REGISTERS],
        };
    }

    // this method used to update shadow memory data
    pub fn update_shadow_memory(&mut self, reg_addr: Register, value: u8){
        let addr = reg_addr.addr() as usize;
        self.ktd2026_data.shadow_register[addr] = value;
    }

    // this method used to read data from shadow memory
    pub fn read_shadow_memory(&mut self, reg_addr: Register) -> u8{
        let addr = reg_addr.addr() as usize;

        // load data from given register of shadow memory
        let value = self.ktd2026_data.shadow_register[addr];
        
        // return value
        value
    }

    // this method used to update KTD2026 device memory through shadow memory
    pub fn update_device_memory(&mut self, reg_addr: Register, value: u8) -> Result<(), E>{        
        // first write data to shadow memory
        self.update_shadow_memory(reg_addr, value);

        // load new value from shadow to tempory variable 
        let temp_value = self.read_shadow_memory(reg_addr);

        // update device memory through shadow memory
        self.write_register(reg_addr, temp_value) ?;

        Ok(())
    }

    // this method used to convert led name to channel number
    pub fn get_led_channel(&mut self, led: Ktd2026LED) -> u8{
        // load channel number for the led color
        let led_channel_number = self.ktd2026_data.leds[led as usize].channel;

        led_channel_number
    }

    // this method used to set a led to a user define brightness value with user define mode
    pub fn led_on(&mut self, led: Ktd2026LED, mode: Ktd2026Mode, bright: u8) -> Result<(), E>{
        // get channel number
        let led_channel_num = self.get_led_channel(led);

        // calculate the register address which need to modify
        let bright_ctrl_reg = (Register::RegLED1CurrentOut as u8) + led_channel_num;

        // update brightess data on device memory through shadow memory
        if let Some(reg_addr) = Register::from_value(bright_ctrl_reg){
            self.update_device_memory(reg_addr, bright) ?;
        }

        // read existing ctrl register value (through shadow memory)
        let ctrl_reg_value = self.read_shadow_memory(Register::RegChannelControl);
        
        // set led mode
        let led_mode = match mode {
            Ktd2026Mode::LedAlwaysOff => {
                // prepare the mask and apply the mask to read value for clear required bits
                ctrl_reg_value & (!(!(Ktd2026Mode::LedAlwaysOff as u8) << (led_channel_num * 2)))
            }
            _ => {
                // prepare the mask and apply the mask to read value for set required bits
                ctrl_reg_value | (mode as u8) << (led_channel_num * 2)
            }
        };

        // update the calculated channel control register values to the device memory through shadow
        self.update_device_memory(Register::RegChannelControl, led_mode)?;

        Ok(())
    }


    pub fn timerslot_control(&mut self, t_slot: TimeSlotControl) -> Result<(), E>{
        // read time slot details from shadow memory
        let mut t_slot_reg_value = self.read_shadow_memory(Register::RegEnableReset);

        println!("existing timer slot: {}", t_slot_reg_value);
        // clear existing time slot control settings
        t_slot_reg_value &= MASK_TIMER_SLOT_CONTROL;

        // set new time slot data
        t_slot_reg_value |= t_slot as u8;

        self.update_device_memory(Register::RegEnableReset, t_slot_reg_value) ?;

        Ok(())

    }

    // flash period = period_multiplier * 0.128 + 0.256s;
    // except 0, period = 0.128s
    pub fn set_period(&mut self, period_multiplier: u8) -> Result<(), E>{
        self.update_device_memory(Register::RegFlashPeriod, period_multiplier) ?;

        Ok(())
    }

    // this method used to set pwm duty
    // pwm duty set as a percentage value of pwm period
    // pwm_duty = pwm_duty_multiplier * 0.4%
    pub fn set_pwm_duty(&mut self, pwm_channel: Ktd2026Pwm, pwm_duty_multiplier: u8) -> Result<(), E>{
        let pwm_duty_reg = Register::RegFlashOnTime1 as u8 + pwm_channel as u8;

        // update pwm duty data on device memory through shadow memory
        if let Some(reg_addr) = Register::from_value(pwm_duty_reg){
            self.update_device_memory(reg_addr, pwm_duty_multiplier) ?;
        }

        Ok(())
    }


}