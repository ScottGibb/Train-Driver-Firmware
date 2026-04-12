//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use firmware::{log_metadata, setup_device};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let mut device = setup_device();
    log_metadata();

    loop {
        device.onboard_led.set_low();
        cortex_m::asm::delay(8_000_000);
        device.onboard_led.set_high();
        cortex_m::asm::delay(8_000_000);
    }
}
