#![no_std]
#![no_main]

use cortex_m_rt::entry;
use firmware::setup_device;
use panic_probe as _;

#[entry]
fn main() -> ! {
    let device = setup_device();
    let mut led_0 = device.channel_0_led;
    let mut led_1 = device.channel_1_led;
    loop {
        // Fade in LED 0 and fade out LED 1
        for duty in 0..=led_0.get_max_duty() {
            led_0.set_duty(duty);
            led_1.set_duty(led_1.get_max_duty() - duty);
            cortex_m::asm::delay(10_000);
        }
    }
}
