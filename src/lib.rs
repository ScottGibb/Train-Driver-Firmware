#![no_std]

use crate::device::Device;
use cortex_m::peripheral::syst::SystClkSource;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use stm32f1xx_hal::adc::Adc;
use stm32f1xx_hal::afio;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::rcc;
use stm32f1xx_hal::timer::Timer;

mod device;
pub mod health_checker;
pub mod pot_scanner;
pub mod sys_timer;
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
    let peripherals = pac::Peripherals::take().unwrap();
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut flash = peripherals.FLASH.constrain();
    let mut rcc = peripherals
        .RCC
        .freeze(rcc::Config::hse(8.MHz()).sysclk(72.MHz()), &mut flash.acr);
    let mut afio = peripherals.AFIO.constrain(&mut rcc);

    let mut gpioa = peripherals.GPIOA.split(&mut rcc);
    let mut gpiob = peripherals.GPIOB.split(&mut rcc);
    let mut gpioc = peripherals.GPIOC.split(&mut rcc);

    let onboard_led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // pwm channels
    let channel_0_pwm = gpiob.pb0.into_alternate_push_pull(&mut gpiob.crl);
    let channel_1_pwm = gpiob.pb1.into_alternate_push_pull(&mut gpiob.crl);

    // led pwm channels
    let channel_0_led_pin = gpioa.pa6.into_alternate_push_pull(&mut gpioa.crl);
    let channel_1_led_pin = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let tim3 = Timer::new(peripherals.TIM3, &mut rcc);

    let led_pwm_timer = tim3.pwm_hz(
        (channel_0_led_pin, channel_1_led_pin),
        &mut afio.mapr,
        10.kHz(),
    );
    let (mut channel_0_led, mut channel_1_led) = led_pwm_timer.split();
    channel_0_led.set_duty(0);
    channel_0_led.enable();
    channel_1_led.set_duty(0);
    channel_1_led.enable();

    // ADC channels
    let channel_0_adc = gpioa.pa0.into_analog(&mut gpioa.crl);
    let channel_1_adc = gpioa.pa1.into_analog(&mut gpioa.crl);

    let adc = Adc::new(peripherals.ADC1, &mut rcc);

    let mut syst = core_peripherals.SYST;

    syst.set_clock_source(SystClkSource::Core);

    syst.set_reload(72_000 - 1);
    syst.clear_current();
    syst.enable_interrupt();
    syst.enable_counter();

    Device {
        onboard_led,
        channel_0_pwm,
        channel_1_pwm,
        channel_0_led,
        channel_1_led,
        channel_0_adc,
        channel_1_adc,
        adc,
        sys_tick_timer: syst,
    }
}
