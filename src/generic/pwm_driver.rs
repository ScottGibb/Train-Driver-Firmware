use crate::types::Percentage;
use core::convert::Infallible;
use embedded_hal::pwm::SetDutyCycle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PwmDriverError {
    InvalidChannel,
}

/// A generic PWM driver that can control multiple PWM channels using a common interface.
/// Uses a Percentage type for duty cycle values, which can be easily converted to the appropriate duty cycle values for each channel.
pub struct PwmDriver<'a, const NUM_CHANNELS: usize> {
    channels: [&'a mut dyn SetDutyCycle<Error = Infallible>; NUM_CHANNELS],
    channel_duties: [Percentage; NUM_CHANNELS],
}

impl<'a, const NUM_CHANNELS: usize> PwmDriver<'a, NUM_CHANNELS> {
    pub fn new(
        // The lifetime of the channel references must be the same as the lifetime of the PwmDriver struct,
        // since the channels are stored as references inside the struct. This ensures that the channels will
        // not be dropped while the PwmDriver is still using them. Using a vtable with the dyn keyword to allow
        // a single PwmDriver struct that can work with any number of channels, without needing to create separate
        // structs for each possible number of channels.
        channels: [&'a mut dyn SetDutyCycle<Error = Infallible>; NUM_CHANNELS],
    ) -> Self {
        Self {
            channels,
            channel_duties: [Percentage::new(0).expect("This should not fail"); NUM_CHANNELS],
        }
    }

    pub fn set_duty(
        &mut self,
        channel_index: usize,
        duty: Percentage,
    ) -> Result<(), PwmDriverError> {
        let channel = self
            .channels
            .get_mut(channel_index)
            .ok_or(PwmDriverError::InvalidChannel)?;
        channel
            .set_duty_cycle_percent(duty.get())
            .expect("PWM is infallible");
        self.channel_duties[channel_index] = duty;
        Ok(())
    }

    pub fn get_duty(&self, channel_index: usize) -> Result<Percentage, PwmDriverError> {
        self.channel_duties
            .get(channel_index)
            .copied()
            .ok_or(PwmDriverError::InvalidChannel)
    }

    pub fn turn_all_off(&mut self) {
        for (i, channel) in self.channels.iter_mut().enumerate() {
            channel
                .set_duty_cycle_percent(0)
                .expect("PWM is infallible");
            self.channel_duties[i] = Percentage::new(0).expect("This should not fail");
        }
    }
}
