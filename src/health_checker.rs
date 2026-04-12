use core::time::Duration;

use stm32f1xx_hal::hal_02::digital::v2::ToggleableOutputPin;

pub struct HealthChecker<P>
where
    P: ToggleableOutputPin,
{
    led_pin: P,
    led_toggle_interval: Duration,
    last_led_toggle_time: Duration,
}

impl<P> HealthChecker<P>
where
    P: ToggleableOutputPin,
{
    pub fn new(led_pin: P, led_toggle_interval: Duration) -> Self {
        Self {
            led_pin,
            led_toggle_interval,
            last_led_toggle_time: Duration::from_millis(0),
        }
    }

    pub fn check(&mut self) {
        let now = super::sys_timer::millis();
        if now - self.last_led_toggle_time >= self.led_toggle_interval {
            let _ = self.led_pin.toggle();
            self.last_led_toggle_time = now;
        }
    }
}
