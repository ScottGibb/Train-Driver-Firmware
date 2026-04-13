use cortex_m::prelude::_embedded_hal_adc_OneShot;
use stm32f1xx_hal::{adc::Adc, pac::ADC1};

use crate::{device::DeviceError, types::Percentage};

//TODO: Replace this code with generic code that can be used with any ADC and any pins, instead of hardcoding the specific pins and ADC instance. This will make it more reusable and adaptable to different hardware configurations.
pub const LOW_THRESHOLD: u16 = 0;
pub const HIGH_THRESHOLD: u16 = 4095;
pub struct PotScanner {
    adc: Adc<ADC1>,
    adc_channel_0: stm32f1xx_hal::gpio::gpioa::PA0<stm32f1xx_hal::gpio::Analog>,
    adc_channel_1: stm32f1xx_hal::gpio::gpioa::PA1<stm32f1xx_hal::gpio::Analog>,
}

impl PotScanner {
    pub fn new(
        adc: Adc<ADC1>,
        adc_channel_0: stm32f1xx_hal::gpio::gpioa::PA0<stm32f1xx_hal::gpio::Analog>,
        adc_channel_1: stm32f1xx_hal::gpio::gpioa::PA1<stm32f1xx_hal::gpio::Analog>,
    ) -> Self {
        PotScanner {
            adc,
            adc_channel_0,
            adc_channel_1,
        }
    }

    pub fn scan(&mut self) -> Result<(Percentage, Percentage), DeviceError> {
        let mut channel_0_value: u16 = self
            .adc
            .read(&mut self.adc_channel_0)
            .map_err(|_| DeviceError::AdcError)?;
        let mut channel_1_value: u16 = self
            .adc
            .read(&mut self.adc_channel_1)
            .map_err(|_| DeviceError::AdcError)?;

        // Clamp inside the valid range to avoid issues with out-of-range values
        channel_0_value = channel_0_value.clamp(LOW_THRESHOLD, HIGH_THRESHOLD);
        channel_1_value = channel_1_value.clamp(LOW_THRESHOLD, HIGH_THRESHOLD);

        // Calculate the percentage values
        channel_0_value = ((channel_0_value - LOW_THRESHOLD) as u32 * 100
            / (HIGH_THRESHOLD - LOW_THRESHOLD) as u32) as u16;
        channel_1_value = ((channel_1_value - LOW_THRESHOLD) as u32 * 100
            / (HIGH_THRESHOLD - LOW_THRESHOLD) as u32) as u16;

        let channel_0_percentage =
            Percentage::new(channel_0_value as u8).map_err(|_| DeviceError::ConversionError)?;
        let channel_1_percentage =
            Percentage::new(channel_1_value as u8).map_err(|_| DeviceError::ConversionError)?;

        Ok((channel_0_percentage, channel_1_percentage))
    }
}
