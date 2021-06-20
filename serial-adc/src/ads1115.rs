use core::time::Duration;
use embedded_hal::blocking::i2c;
use embedded_hal::prelude::_embedded_hal_blocking_i2c_WriteRead;
use teensy4_bsp::hal::i2c::{
    BusIdleTimeoutError, ClockSpeed, ClockSpeedError, PinLowTimeoutError, I2C,
};
use teensy4_bsp::hal::iomuxc::consts::Unsigned;

/// Point mask
const ADS1X15_REG_POINTER_MASK: u8 = 0x03;
/// Conversion
const ADS1X15_REG_POINTER_CONVERT: u8 = 0x00;
/// Configuration
const ADS1X15_REG_POINTER_CONFIG: u8 = 0x01;
/// Low threshold
const ADS1X15_REG_POINTER_LOWTHRESH: u8 = 0x02;
/// High threshold
const ADS1X15_REG_POINTER_HITHRESH: u8 = 0x03;

/// 8 samples per second
const RATE_ADS1115_8SPS: u16 = 0x0000;
/// 16 samples per second
const RATE_ADS1115_16SPS: u16 = 0x0020;
/// 32 samples per second
const RATE_ADS1115_32SPS: u16 = 0x0040;
/// 64 samples per second
const RATE_ADS1115_64SPS: u16 = 0x0060;
/// 128 samples per second (default)
const RATE_ADS1115_128SPS: u16 = 0x0080;
/// 250 samples per second
const RATE_ADS1115_250SPS: u16 = 0x00A0;
/// 475 samples per second
const RATE_ADS1115_475SPS: u16 = 0x00C0;
/// 860 samples per second
const RATE_ADS1115_860SPS: u16 = 0x00E0;

/// OS Mask
const ADS1X15_REG_CONFIG_OS_MASK: u16 = 0x8000;
/// Write: Set to start a single-conversion
const ADS1X15_REG_CONFIG_OS_SINGLE: u16 = 0x8000;
/// Read: Bit = 0 when conversion is in progress
const ADS1X15_REG_CONFIG_OS_BUSY: u16 = 0x0000;
/// Read: Bit = 1 when device is not performing a conversion
const ADS1X15_REG_CONFIG_OS_NOTBUSY: u16 = 0x8000;

/// Mux Mask
const ADS1X15_REG_CONFIG_MUX_MASK: u16 = 0x7000;
/// Differential P = AIN0, N = AIN1 (default)
const ADS1X15_REG_CONFIG_MUX_DIFF_0_1: u16 = 0x0000;
/// Differential P = AIN0, N = AIN3
const ADS1X15_REG_CONFIG_MUX_DIFF_0_3: u16 = 0x1000;
/// Differential P = AIN1, N = AIN3
const ADS1X15_REG_CONFIG_MUX_DIFF_1_3: u16 = 0x2000;
/// Differential P = AIN2, N = AIN3
const ADS1X15_REG_CONFIG_MUX_DIFF_2_3: u16 = 0x3000;
/// Single-ended AIN0
const ADS1X15_REG_CONFIG_MUX_SINGLE_0: u16 = 0x4000;
/// Single-ended AIN1
const ADS1X15_REG_CONFIG_MUX_SINGLE_1: u16 = 0x5000;
/// Single-ended AIN2
const ADS1X15_REG_CONFIG_MUX_SINGLE_2: u16 = 0x6000;
/// Single-ended AIN3
const ADS1X15_REG_CONFIG_MUX_SINGLE_3: u16 = 0x7000;

/// PGA Mask
const ADS1X15_REG_CONFIG_PGA_MASK: u16 = 0x0E00;
/// +/-6.144V range = Gain 2/3
const ADS1X15_REG_CONFIG_PGA_6_144V: u16 = 0x0000;
/// +/-4.096V range = Gain 1
const ADS1X15_REG_CONFIG_PGA_4_096V: u16 = 0x0200;
/// +/-2.048V range = Gain 2 (default)
const ADS1X15_REG_CONFIG_PGA_2_048V: u16 = 0x0400;
/// +/-1.024V range = Gain 4
const ADS1X15_REG_CONFIG_PGA_1_024V: u16 = 0x0600;
/// +/-0.512V range = Gain 8
const ADS1X15_REG_CONFIG_PGA_0_512V: u16 = 0x0800;
/// +/-0.256V range = Gain 16
const ADS1X15_REG_CONFIG_PGA_0_256V: u16 = 0x0A00;

/// Mode Mask
const ADS1X15_REG_CONFIG_MODE_MASK: u16 = 0x0100;
/// Continuous conversion mode
const ADS1X15_REG_CONFIG_MODE_CONTIN: u16 = 0x0000;
/// Power-down single-shot mode (default)
const ADS1X15_REG_CONFIG_MODE_SINGLE: u16 = 0x0100;

/// Data Rate Mask
const ADS1X15_REG_CONFIG_RATE_MASK: u16 = 0x00E0;

/// CMode Mask
const ADS1X15_REG_CONFIG_CMODE_MASK: u16 = 0x0010;
/// Traditional comparator with hysteresis (default)
const ADS1X15_REG_CONFIG_CMODE_TRAD: u16 = 0x0000;
/// Window comparator
const ADS1X15_REG_CONFIG_CMODE_WINDOW: u16 = 0x0010;

/// CPol Mask
const ADS1X15_REG_CONFIG_CPOL_MASK: u16 = 0x0008;
/// ALERT/RDY pin is low when active (default)
const ADS1X15_REG_CONFIG_CPOL_ACTVLOW: u16 = 0x0000;
/// ALERT/RDY pin is high when active
const ADS1X15_REG_CONFIG_CPOL_ACTVHI: u16 = 0x0008;

/// Determines if ALERT/RDY pin latches once asserted
const ADS1X15_REG_CONFIG_CLAT_MASK: u16 = 0x0004;
/// Non-latching comparator (default)
const ADS1X15_REG_CONFIG_CLAT_NONLAT: u16 = 0x0000;
/// Latching comparator
const ADS1X15_REG_CONFIG_CLAT_LATCH: u16 = 0x0004;

/// CQue Mask
const ADS1X15_REG_CONFIG_CQUE_MASK: u16 = 0x0003;
/// Assert ALERT/RDY after one conversions
const ADS1X15_REG_CONFIG_CQUE_1CONV: u16 = 0x0000;
/// Assert ALERT/RDY after two conversions
const ADS1X15_REG_CONFIG_CQUE_2CONV: u16 = 0x0001;
/// Assert ALERT/RDY after four conversions
const ADS1X15_REG_CONFIG_CQUE_4CONV: u16 = 0x0002;
/// Disable the comparator and put ALERT/RDY in high state (default)
const ADS1X15_REG_CONFIG_CQUE_NONE: u16 = 0x0003;

#[derive(Debug, Copy, Clone)]
pub enum DataRate {
    SPS8 = RATE_ADS1115_8SPS as isize,
    SPS16 = RATE_ADS1115_16SPS as isize,
    SPS32 = RATE_ADS1115_32SPS as isize,
    SPS64 = RATE_ADS1115_64SPS as isize,
    SPS128 = RATE_ADS1115_128SPS as isize,
    SPS250 = RATE_ADS1115_250SPS as isize,
    SPS475 = RATE_ADS1115_475SPS as isize,
    SPS860 = RATE_ADS1115_860SPS as isize,
}

#[derive(Debug, Copy, Clone)]
pub enum Gain {
    TwoThirds = ADS1X15_REG_CONFIG_PGA_6_144V as isize,
    One = ADS1X15_REG_CONFIG_PGA_4_096V as isize,
    Two = ADS1X15_REG_CONFIG_PGA_2_048V as isize,
    Four = ADS1X15_REG_CONFIG_PGA_1_024V as isize,
    Eight = ADS1X15_REG_CONFIG_PGA_0_512V as isize,
    Sixteen = ADS1X15_REG_CONFIG_PGA_0_256V as isize,
}

#[derive(Debug)]
pub enum ErrorKind {
    BusIdleTimeout(BusIdleTimeoutError),
    PinLowTimeout(PinLowTimeoutError),
    ClockSpeed(ClockSpeedError),
}

pub enum SingleEndedChannel {
    One,
    Two,
    Three,
    Four,
}

pub enum DifferentialChannel {
    One,
    Two,
}

pub struct Ads1115Uninit<M>
where
    M: Unsigned,
{
    i2c: I2C<M>,
    addr: u8,
    clock_speed: ClockSpeed,
}

impl<M> Ads1115Uninit<M>
where
    M: Unsigned,
{
    pub fn new(i2c: I2C<M>) -> Self {
        Ads1115Uninit {
            i2c,
            addr: 0x48,
            clock_speed: ClockSpeed::KHz400,
        }
    }

    pub fn init(mut self) -> Result<Ads1115<M>, ErrorKind> {
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

        Ok(Ads1115 {
            i2c: self.i2c,
            addr: self.addr,
            gain: Gain::TwoThirds,
            data_rate: DataRate::SPS128,
        })
    }
}

pub struct Ads1115<M>
where
    M: Unsigned,
{
    i2c: I2C<M>,
    addr: u8,
    gain: Gain,
    data_rate: DataRate,
}

impl<M> Ads1115<M>
where
    M: Unsigned,
{
    pub fn who_am_i(&mut self) -> Result<u8, <I2C<M> as i2c::WriteRead>::Error> {
        const WHO_AM_I: u8 = 0x75;
        let mut out = [0; 1];
        self.i2c.write_read(self.addr, &[WHO_AM_I], &mut out)?;
        Ok(out[0])
    }

    pub fn set_gain(&mut self, gain: Gain) {
        self.gain = gain;
    }

    pub fn gain(&self) -> Gain {
        self.gain
    }

    pub fn set_data_rate(&mut self, data_rate: DataRate) {
        self.data_rate = data_rate;
    }

    pub fn data_rate(&self) -> DataRate {
        self.data_rate
    }

    pub fn read_single(&mut self, channel: SingleEndedChannel) -> Result<u16, <I2C<M> as i2c::WriteRead>::Error> {
        let mut config = self.get_config();
        match channel {
            SingleEndedChannel::One => config |= ADS1X15_REG_CONFIG_MUX_SINGLE_0,
            SingleEndedChannel::Two => config |= ADS1X15_REG_CONFIG_MUX_SINGLE_1,
            SingleEndedChannel::Three => config |= ADS1X15_REG_CONFIG_MUX_SINGLE_2,
            SingleEndedChannel::Four => config |= ADS1X15_REG_CONFIG_MUX_SINGLE_3,
        };

        // Set 'start single-conversion' bit
        config |= ADS1X15_REG_CONFIG_OS_SINGLE;

        self.write_register(ADS1X15_REG_POINTER_CONFIG, config)?;

        while !self.conversion_complete()? {
            log::trace!("Conversion not complete!");
        }

        self.last_conversion_results()
    }

    fn conversion_complete(&mut self) -> Result<bool, <I2C<M> as i2c::WriteRead>::Error>  {
        let conf = self.read_register(ADS1X15_REG_POINTER_CONFIG)? >> 0;
        log::trace!("Read config {:#06x}", conf);
        let complete = (conf & 0x8000) != 0;
        Ok(complete)
    }

    fn last_conversion_results(&mut self) -> Result<u16, <I2C<M> as i2c::WriteRead>::Error> {
        self.read_register(ADS1X15_REG_POINTER_CONVERT)
    }

    fn get_config(&self) -> u16 {
        let mut config = ADS1X15_REG_CONFIG_CQUE_NONE |    // Disable the comparator (default val)
                ADS1X15_REG_CONFIG_CLAT_NONLAT |  // Non-latching (default val)
                ADS1X15_REG_CONFIG_CPOL_ACTVLOW | // Alert/Rdy active low   (default val)
                ADS1X15_REG_CONFIG_CMODE_TRAD |   // Traditional comparator (default val)
                ADS1X15_REG_CONFIG_MODE_SINGLE;   // Single-shot mode (default)
        config |= self.gain as u16;
        config |= self.data_rate as u16;
        config as u16
    }

    fn write_register(&mut self, register: u8, value: u16) -> Result<u16, <I2C<M> as i2c::WriteRead>::Error> {
        let mut buf = [0; 3];
        buf[0] = register;
        let value = value.to_be_bytes();
        buf[1] = value[0];
        buf[2] = value[1];

        let mut out = [0; 2];
        self.i2c.write_read(self.addr, &buf, &mut out)?;
        Ok(u16::from_be_bytes(out))
    }

    fn read_register(&mut self, register: u8) -> Result<u16, <I2C<M> as i2c::WriteRead>::Error> {
        let mut buf = [0; 1];
        buf[0] = register;
        let mut out = [0; 2];
        self.i2c.write_read(self.addr, &buf, &mut out)?;
        Ok(u16::from_be_bytes(out))
    }
}
