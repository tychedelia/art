#![no_std]
#![no_main]
#![allow(dead_code)]

use core::fmt::Write;

use embedded_hal::digital::ToggleableOutputPin;
use embedded_hal::digital::v2::OutputPin;
use teensy4_bsp as bsp;
use teensy4_panic as _;

use crate::ads1115::{Ads1115Uninit, SingleEndedChannel};

mod ads1115;
// mod logging;
mod usb_io;

/// Returns the MPU's WHO_AM_I value. This should be a static
/// value that's specific for a MPU variant.

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let pins = bsp::t40::into_pins(peripherals.iomuxc);
    let (usb_r, mut usb_w) = usb_io::split().unwrap();
    systick.delay(5000);

    let (i2c1_builder, _, _, _) = peripherals.i2c.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::i2c::ClockSelect::OSC, // 24MHz clock...
        bsp::hal::ccm::i2c::PrescalarSelect::DIVIDE_3, // Divide by 3
    );

    let mut r = bsp::hal::gpio::GPIO::new(pins.p14);
    r.set_fast(true);
    let mut r = r.output();
    let mut g = bsp::hal::gpio::GPIO::new(pins.p15);
    g.set_fast(true);
    let mut g = g.output();
    let mut b = bsp::hal::gpio::GPIO::new(pins.p17);
    b.set_fast(true);
    let mut b = b.output();

    let i2c4 = i2c1_builder.build(pins.p19, pins.p18);
    let adc = Ads1115Uninit::new(i2c4);
    let mut adc = adc.init().unwrap();

    loop {
        systick.delay(10);
        let val = adc.read_single(SingleEndedChannel::Four).expect("could not read value!");
        writeln!(usb_w, "{}", val).unwrap();
        usb_w.flush().unwrap();

        // if val > (15000 / 2) {
        //     b.set_high();
        // } else {
        //     b.set_low();
        // };
        // let val = adc.read_single(SingleEndedChannel::Four).expect("could not read value!");
        // log::info!("2: {:?}", val);
        // if val > (15000 / 2) {
        //     r.set_high();
        // } else {
        //     r.set_low();
        // };

    }
}
