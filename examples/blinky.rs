#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use firmware::{log_metadata, setup_device};
use panic_probe as _;

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let mut device = setup_device();
    info!("Starting blinky example");
    log_metadata();

    loop {
        device.onboard_led.set_low();
        cortex_m::asm::delay(8_000_000);
        info!("LED on");
        device.onboard_led.set_high();
        cortex_m::asm::delay(8_000_000);
        info!("LED off");
    }
}
