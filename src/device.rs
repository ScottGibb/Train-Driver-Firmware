use stm32f1xx_hal::gpio;
use stm32f1xx_hal::gpio::gpioa;
use stm32f1xx_hal::gpio::gpiob;
use stm32f1xx_hal::gpio::gpioc;
pub struct Device {
    pub onboard_led: gpioc::PC13<gpio::Output<gpio::PushPull>>,

    // pwm channels
    pub channel_0_pwm: gpiob::PB0<gpio::Alternate<gpio::PushPull>>,
    pub channel_1_pwm: gpiob::PB1<gpio::Alternate<gpio::PushPull>>,

    // led pwm channels
    pub channel_0_led_pwm: gpioa::PA6<gpio::Alternate<gpio::PushPull>>,
    pub channel_1_led_pwm: gpioa::PA7<gpio::Alternate<gpio::PushPull>>,

    // ADC channels
    pub channel_0_adc: gpioa::PA0<gpio::Analog>,
    pub channel_1_adc: gpioa::PA1<gpio::Analog>,
}
