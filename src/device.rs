use defmt::Format;
use stm32f1xx_hal::adc::Adc;
use stm32f1xx_hal::gpio::Analog;
use stm32f1xx_hal::gpio::Output;
use stm32f1xx_hal::gpio::PushPull;
use stm32f1xx_hal::gpio::gpioa;
use stm32f1xx_hal::gpio::gpioc;
use stm32f1xx_hal::pac::ADC1;
use stm32f1xx_hal::pac::SYST;
use stm32f1xx_hal::pac::TIM3;
use stm32f1xx_hal::timer::C1;
use stm32f1xx_hal::timer::C2;
use stm32f1xx_hal::timer::C3;
use stm32f1xx_hal::timer::C4;
use stm32f1xx_hal::timer::PwmChannel;

pub type ChannelZeroPwm = PwmChannel<TIM3, C3>;
pub type ChannelOnePwm = PwmChannel<TIM3, C4>;
pub type ChannelZeroLed = PwmChannel<TIM3, C1>;
pub type ChannelOneLed = PwmChannel<TIM3, C2>;

pub type ChannelZeroAdc = gpioa::PA0<Analog>;
pub type ChannelOneAdc = gpioa::PA1<Analog>;

pub type HealthLed = gpioc::PC13<Output<PushPull>>;
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
    // SysTick timer with 1 ms resolution (1 kHz)
    pub sys_tick_timer: SYST,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Format)]
pub enum DeviceError {
    AdcError,
    ConversionError,
}
