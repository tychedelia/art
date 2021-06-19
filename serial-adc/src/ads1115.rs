use core::time::Duration;
use embedded_hal::blocking::i2c;
use teensy4_bsp::hal::i2c::{
    BusIdleTimeoutError, ClockSpeed, ClockSpeedError, PinLowTimeoutError, I2C,
};
use teensy4_bsp::hal::iomuxc::consts::Unsigned;
use embedded_hal::prelude::_embedded_hal_blocking_i2c_WriteRead;

///< 8 samples per second
const RATE_ADS1115_8SPS: u8 = 0x0000;
///< 16 samples per second
const RATE_ADS1115_16SPS: u8 = 0x0020;
///< 32 samples per second
const RATE_ADS1115_32SPS: u8 = 0x0040;
///< 64 samples per second
const RATE_ADS1115_64SPS: u8 = 0x0060;
///< 128 samples per second (default)
const RATE_ADS1115_128SPS: u8 = 0x0080;
///< 250 samples per second
const RATE_ADS1115_250SPS: u8 = 0x00A0;
///< 475 samples per second
const RATE_ADS1115_475SPS: u8 = 0x00C0;
///< 860 samples per second
const RATE_ADS1115_860SPS: u8 = 0x00E0;

enum ErrorKind {
    BusIdleTimeout(BusIdleTimeoutError),
    PinLowTimeout(PinLowTimeoutError),
    ClockSpeed(ClockSpeedError),
}

pub struct Ads1115<M>
where
    M: Unsigned,
{
    i2c: I2C<M>,
    addr: u8,
    clock_speed: ClockSpeed,
}

impl<M> Ads1115<M>
where
    M: Unsigned,
{
    pub fn new(i2c: I2C<M>) -> Self {
        Ads1115 {
            i2c,
            addr: 0x48,
            clock_speed: ClockSpeed::KHz400,
        }
    }

    pub fn init(&mut self) -> Result<(), ErrorKind> {
        if let Err(err) = self.i2c.set_bus_idle_timeout(Duration::from_micros(200)) {
            log::warn!("Error when setting bus idle timeout: {:?}", err);
            return Err(ErrorKind::BusIdleTimeout(err));
        }
        if let Err(err) = self.i2c.set_pin_low_timeout(Duration::from_millis(1)) {
            log::warn!("Error when setting pin low timeout: {:?}", err);
            return Err(ErrorKind::PinLowTimeout(err));
        }
        if let Err(err) = self.i2c.set_clock_speed(self.clock_speed) {
            log::warn!(
                "Error when setting I1C clock speed to {:?}: {:?}",
                self.clock_speed,
                err
            );
            return Err(ErrorKind::ClockSpeed(err));
        }

        Ok(())
    }

    pub fn who_am_i(&mut self) -> Result<u8, <I2C<M> as i2c::WriteRead>::Error> {
        const WHO_AM_I: u8 = 0x75;
        let mut out = [0; 1];
        self.i2c.write_read(self.addr, &[WHO_AM_I], &mut out)?;
        Ok(out[0])
    }
}
