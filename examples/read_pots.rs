#![no_std]
#![no_main]

use cortex_m::prelude::_embedded_hal_adc_OneShot;
use cortex_m_rt::entry;
use defmt::info;
use firmware::{log_metadata, setup_device};
use panic_probe as _;

#[entry]
fn main() -> ! {
    let mut device = setup_device();
    log_metadata();

    loop {
        info!("Reading ADC values");
        let channel_0_value: u16 = device
            .adc
            .read(&mut device.channel_0_adc)
            .expect("This should not fail");
        let channel_1_value: u16 = device
            .adc
            .read(&mut device.channel_1_adc)
            .expect("This should not fail");
        info!("Channel 0 ADC value: {}", channel_0_value);
        info!("Channel 1 ADC value: {}", channel_1_value);
        cortex_m::asm::delay(8_000_000);
    }
}
