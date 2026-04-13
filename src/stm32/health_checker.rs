use core::time::Duration;

use crate::stm32::sys_timer;
use embedded_hal::digital::StatefulOutputPin;

pub struct HealthChecker<P>
where
    P: StatefulOutputPin,
{
    led_pin: P,
    led_toggle_interval: Duration,
    last_led_toggle_time: Duration,
}

impl<P> HealthChecker<P>
where
    P: StatefulOutputPin,
{
    pub fn new(led_pin: P, led_toggle_interval: Duration) -> Self {
        Self {
            led_pin,
            led_toggle_interval,
            last_led_toggle_time: Duration::from_millis(0),
        }
    }

    pub fn check(&mut self) -> Result<(), P::Error> {
        let now = sys_timer::millis();
        let elapsed = now
            .checked_sub(self.last_led_toggle_time)
            .unwrap_or(self.led_toggle_interval);
        if elapsed >= self.led_toggle_interval {
            self.led_pin.toggle()?;
            self.last_led_toggle_time = now;
        }
        Ok(())
    }
}
