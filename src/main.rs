//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![no_std]
#![no_main]

use core::time::Duration;

use cortex_m_rt::entry;
use firmware::{health_checker::HealthChecker, log_metadata, setup_device};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let device = setup_device();
    log_metadata();

    let mut health_checker = HealthChecker::new(
        device.onboard_led,
        Duration::from_millis(
            env!("HEALTH_CHECKER_INTERVAL_MS")
                .parse::<u64>()
                .expect("HEALTH_CHECKER_INTERVAL_MS must be a valid u64"),
        ),
    );

    loop {
        health_checker.check();
    }
}
