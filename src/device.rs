use defmt::Format;
use stm32f1xx_hal::adc::Adc;
use stm32f1xx_hal::gpio::Alternate;
use stm32f1xx_hal::gpio::Analog;
use stm32f1xx_hal::gpio::Output;
use stm32f1xx_hal::gpio::PushPull;
use stm32f1xx_hal::gpio::gpioa;
use stm32f1xx_hal::gpio::gpiob;
use stm32f1xx_hal::gpio::gpioc;
use stm32f1xx_hal::pac::ADC1;
use stm32f1xx_hal::pac::SYST;
use stm32f1xx_hal::pac::TIM3;
use stm32f1xx_hal::timer::C1;
use stm32f1xx_hal::timer::C2;
use stm32f1xx_hal::timer::PwmChannel;

pub struct Device {
    pub onboard_led: gpioc::PC13<Output<PushPull>>,

    // pwm channels
    pub channel_0_pwm: gpiob::PB0<Alternate<PushPull>>,
    pub channel_1_pwm: gpiob::PB1<Alternate<PushPull>>,

    // led pwm channels
    pub channel_0_led: PwmChannel<TIM3, C1>,
    pub channel_1_led: PwmChannel<TIM3, C2>,

    // ADC channels
    pub channel_0_adc: gpioa::PA0<Analog>,
    pub channel_1_adc: gpioa::PA1<Analog>,

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
