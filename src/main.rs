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
use defmt::info;
use firmware::{
    health_checker::HealthChecker, log_metadata, pot_scanner, pwm_driver::PwmDriver, setup_device,
};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let mut device = setup_device();
    log_metadata();

    let mut health_checker = HealthChecker::new(
        device.onboard_led,
        Duration::from_millis(
            env!("HEALTH_CHECKER_INTERVAL_MS")
                .parse::<u64>()
                .expect("HEALTH_CHECKER_INTERVAL_MS must be a valid u64"),
        ),
    );
    let mut pot_scanner =
        pot_scanner::PotScanner::new(device.adc, device.channel_0_adc, device.channel_1_adc);
    let mut pwm_driver = PwmDriver {
        channel_0_pwm: device.channel_0_pwm,
        channel_1_pwm: device.channel_1_pwm,
    };
    loop {
        health_checker.check();
        match pot_scanner.scan() {
            Ok((channel_0, channel_1)) => {
                info!(
                    "Potentiometer values: channel 0 = {}%, channel 1 = {}%",
                    channel_0, channel_1
                );
                pwm_driver.set_channel_0_duty(channel_0);
                pwm_driver.set_channel_1_duty(channel_1);
            }
            Err(err) => {
                info!("Potentiometer scan failed: {}", err);
            }
        }
    }
}
