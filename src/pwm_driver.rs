use crate::{
    device::{ChannelOnePwm, ChannelZeroPwm},
    types::Percentage,
};

pub struct PwmDriver {
    pub channel_0_pwm: ChannelZeroPwm,
    pub channel_1_pwm: ChannelOnePwm,
}

impl PwmDriver {
    pub fn set_channel_0_duty(&mut self, duty: Percentage) {
        let scaled_value = duty.scale_to(self.channel_0_pwm.get_max_duty());
        self.channel_0_pwm.set_duty(scaled_value);
    }

    pub fn set_channel_1_duty(&mut self, duty: Percentage) {
        let scaled_value = duty.scale_to(self.channel_1_pwm.get_max_duty());
        self.channel_1_pwm.set_duty(scaled_value);
    }
}
