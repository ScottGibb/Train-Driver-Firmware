//! Peripheral type aliases and the [`Device`] struct that owns all hardware handles.

use stm32f1xx_hal::adc::Adc;
use stm32f1xx_hal::gpio::Analog;
use stm32f1xx_hal::gpio::Output;
use stm32f1xx_hal::gpio::PushPull;
use stm32f1xx_hal::gpio::gpioa;
use stm32f1xx_hal::gpio::gpioc;
use stm32f1xx_hal::pac::ADC1;
use stm32f1xx_hal::pac::TIM3;
use stm32f1xx_hal::timer::C1;
use stm32f1xx_hal::timer::C2;
use stm32f1xx_hal::timer::C3;
use stm32f1xx_hal::timer::C4;
use stm32f1xx_hal::timer::PwmChannel;

/// Motor PWM output on TIM3 CH3 (PB0).
pub type ChannelZeroPwm = PwmChannel<TIM3, C3>;
/// Motor PWM output on TIM3 CH4 (PB1).
pub type ChannelOnePwm = PwmChannel<TIM3, C4>;
/// LED PWM output on TIM3 CH1 (PA6).
pub type ChannelZeroLed = PwmChannel<TIM3, C1>;
/// LED PWM output on TIM3 CH2 (PA7).
pub type ChannelOneLed = PwmChannel<TIM3, C2>;

/// ADC input on PA0.
pub type ChannelZeroAdc = gpioa::PA0<Analog>;
/// ADC input on PA1.
pub type ChannelOneAdc = gpioa::PA1<Analog>;

/// Onboard LED on PC13 (active low on Blue Pill).
pub type HealthLed = gpioc::PC13<Output<PushPull>>;
/// Owns all peripheral handles returned by [`crate::setup_device`].
pub struct Device {
    pub onboard_led: HealthLed,

    // pwm channels
    pub channel_0_pwm: ChannelZeroPwm,
    pub channel_1_pwm: ChannelOnePwm,

    // led pwm channels
    pub channel_0_led: ChannelZeroLed,
    pub channel_1_led: ChannelOneLed,

    // ADC channels
    pub channel_0_adc: ChannelZeroAdc,
    pub channel_1_adc: ChannelOneAdc,

    // ADC
    pub adc: Adc<ADC1>,
}
