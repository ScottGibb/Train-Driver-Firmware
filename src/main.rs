//! Train Driver firmware entry point.
//!
//! Reads two potentiometers via ADC, maps the values to PWM duty cycles,
//! and blinks the onboard LED (PC13) as a health indicator.

#![no_std]
#![no_main]

use core::time::Duration;

use cortex_m_rt::entry;
use defmt::info;
use firmware::generic::pwm_driver::PwmDriver;
use firmware::stm32::health_checker::HealthChecker;
use firmware::stm32::pot_scanner::PotScanner;
use firmware::{log_metadata, setup_device};
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
    let mut pot_scanner = PotScanner::new(device.adc, device.channel_0_adc, device.channel_1_adc);
    let mut channel_0_pwm = device.channel_0_pwm;
    let mut channel_1_pwm = device.channel_1_pwm;
    let mut pwm_driver = PwmDriver::new([&mut channel_0_pwm, &mut channel_1_pwm]);
    loop {
        health_checker.check().expect("This should not fail");
        match pot_scanner.scan() {
            Ok((channel_0, channel_1)) => {
                info!(
                    "Potentiometer values: channel 0 = {}%, channel 1 = {}%",
                    channel_0, channel_1
                );
                pwm_driver
                    .set_duty(0, channel_0)
                    .expect("This should not fail");
                pwm_driver
                    .set_duty(1, channel_1)
                    .expect("This should not fail");
            }
            Err(err) => {
                info!("Potentiometer scan failed: {}", err);
            }
        }
    }
}
