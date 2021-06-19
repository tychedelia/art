//! Demonstrates an I2C master. We try to read data from
//! a MPU9250 9-DOF IMU.
//!
//! Teensy pin 16 => SCL (I2C3)
//! Teensy pin 17 => SDA (I2C3)
//!
//! Success criteria:
//!
//! - The MPU correctly reports its `WHO_AM_I` address. The slave
//!   address is printed over USB logging.
//! - The clock is running at its selected bit rate; either 100KHz
//!   or 400KHz. Measure it with a logic analyzer.
//! - There's a repeated start in the `write_read` call; observable
//!   via a logic analyzer. Changing it to a `write`, followed by a
//!   `read`, should show that there is are two transactions.

#![no_std]
#![no_main]

mod ads1115;
mod logging;

use teensy4_panic as _;

use crate::ads1115::Ads1115;
use bsp::hal::i2c::ClockSpeed;
use embedded_hal::blocking::i2c;
use teensy4_bsp as bsp;

/// Returns the MPU's WHO_AM_I value. This should be a static
/// value that's specific for a MPU variant.

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let pins = bsp::t41::into_pins(peripherals.iomuxc);
    logging::init().unwrap();
    systick.delay(5000);

    log::info!("Enabling I2C clocks...");
    let (_, _, _, i2c4_builder) = peripherals.i2c.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::i2c::ClockSelect::OSC, // 24MHz clock...
        bsp::hal::ccm::i2c::PrescalarSelect::DIVIDE_3, // Divide by 3
    );

    log::info!("Constructing I2C1 instance on pins 18 and 19...");
    let mut i2c4 = i2c4_builder.build(pins.p24, pins.p25);
    let mut adc = Ads1115::new(i2c4);

    log::info!("Starting I/O loop...");
    loop {
        systick.delay(1000);
        log::info!("Querying WHO_AM_I...");
        match adc.who_am_i() {
            Ok(who) => log::info!("Received 0x{:X} for WHO_AM_I", who),
            Err(err) => {
                log::warn!("Error reading WHO_AM_I: {:?}", err);
                continue;
            }
        }
    }
}
