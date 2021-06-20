#![no_std]
#![no_main]
#![allow(dead_code)]

use teensy4_bsp as bsp;
use teensy4_panic as _;

use crate::ads1115::{Ads1115Uninit, SingleEndedChannel};
use embedded_hal::digital::ToggleableOutputPin;
use embedded_hal::digital::v2::OutputPin;

mod ads1115;
mod logging;

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

    log::info!("Setting up led");
    let mut r = bsp::hal::gpio::GPIO::new(pins.p21);
    r.set_fast(true);
    let mut r = r.output();
    let mut g = bsp::hal::gpio::GPIO::new(pins.p22);
    g.set_fast(true);
    let mut g = g.output();
    let mut b = bsp::hal::gpio::GPIO::new(pins.p23);
    b.set_fast(true);
    let mut b = b.output();

    log::info!("Constructing I2C1 instance on pins 18 and 19...");
    let i2c4 = i2c4_builder.build(pins.p24, pins.p25);
    let adc = Ads1115Uninit::new(i2c4);
    let mut adc = adc.init().unwrap();

    log::info!("Starting I/O loop...");
    loop {
        systick.delay(10);
        let val = adc.read_single(SingleEndedChannel::One).expect("could not read value!");
        log::info!("1: {:?}", val);
        if val > (15000 / 2) {
            b.set_high();
        } else {
            b.set_low();
        };
        let val = adc.read_single(SingleEndedChannel::Two).expect("could not read value!");
        log::info!("2: {:?}", val);
        if val > (15000 / 2) {
            r.set_high();
        } else {
            r.set_low();
        };

    }
}
