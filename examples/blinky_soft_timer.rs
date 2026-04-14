#![no_std]
#![no_main]

use core::time::Duration;

use cortex_m_rt::entry;
use defmt::info;
use firmware::{log_metadata, setup_device, sys_timer::millis};
use panic_probe as _;

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let mut device = setup_device();
    info!("Starting blinky example");
    log_metadata();
    let mut last_led_time = millis();
    let led_toggle_interval = Duration::from_millis(500);

    loop {
        if millis() - last_led_time >= led_toggle_interval {
            last_led_time = millis();
            device.onboard_led.toggle();
            info!("Toggled LED at {} ms", last_led_time.as_millis());
        }
    }
}
