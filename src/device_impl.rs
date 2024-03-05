use embedded_hal::blocking::i2c;

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

    pub fn led_on(&mut self, led: Ktd2026LED, mode: Ktd2026Mode, bright: u8) -> Result<(), E>{
        
        let led_reg = led as u8;
        let default_bright_ctrl_reg = Register::addr(Register::RegLED1CurrentOut);

        println!("Led: {}, default: {},  Write: {}, Bright: {}",
                led_reg, default_bright_ctrl_reg, 
                (default_bright_ctrl_reg + led_reg/2),
                bright
                );

        if let Some(bright_ctrl_reg) = Register::from_value(default_bright_ctrl_reg + led_reg/2){
            // set led brightness
            self.write_register(bright_ctrl_reg, bright)?;
        }

        // set led mode
        let led_mode = match mode {
            Ktd2026Mode::LedAlwaysOff => {
                // this need to refactor
                // shadow memory need to perform this op
                !(!(Ktd2026Mode::LedAlwaysOff as u8) << (led as u8))
            }
            _ => {
                (mode as u8) << (led as u8)
            }
        };

        println!("Mode : {:b}", led_mode);

        // let led_mode = (mode as u8) << (led as u8);
        self.write_register(Register::RegChannelControl, led_mode)?;
        

        Ok(())
    }


}