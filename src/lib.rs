#![no_std]

use crate::device::Device;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::*;
mod device;
commitment_issues::include_metadata!();

pub fn log_metadata() {
    info!("Train Driver Metadata:");
    info!("Schema version:  {}", metadata::schema());
    info!("Compile time:    {}", metadata::compile_time());
    info!("Commit hash:     {}", metadata::short_hash());
    info!("Is dirty build:  {}", metadata::is_dirty());
    info!("Tag description: {}", metadata::tag_describe());
    info!("Last author:     {}", metadata::last_author());
}

pub fn setup_device() -> Device {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut gpiob = dp.GPIOB.split(&mut rcc);
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    let onboard_led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // pwm channels
    let channel_0_pwm = gpiob.pb0.into_alternate_push_pull(&mut gpiob.crl);
    let channel_1_pwm = gpiob.pb1.into_alternate_push_pull(&mut gpiob.crl);

    // led pwm channels
    let channel_0_led_pwm = gpioa.pa6.into_alternate_push_pull(&mut gpioa.crl);
    let channel_1_led_pwm = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    // ADC channels

    let channel_0_adc = gpioa.pa0.into_analog(&mut gpioa.crl);
    let channel_1_adc = gpioa.pa1.into_analog(&mut gpioa.crl);

    Device {
        onboard_led,
        channel_0_pwm,
        channel_1_pwm,
        channel_0_led_pwm,
        channel_1_led_pwm,
        channel_0_adc,
        channel_1_adc,
    }
}
