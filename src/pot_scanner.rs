use crate::device::DeviceError;
use crate::types::Percentage;
use stm32f1xx_hal::adc::Adc;
use stm32f1xx_hal::adc::Instance;
use stm32f1xx_hal::hal_02::adc::Channel;
use stm32f1xx_hal::hal_02::adc::OneShot;

pub struct PotScanner<ADC: Instance, P0, P1> {
    adc: Adc<ADC>,
    channel_0: P0,
    channel_1: P1,
}

impl<ADC, P0, P1> PotScanner<ADC, P0, P1>
where
    ADC: Instance,
    P0: Channel<ADC, ID = u8>,
    P1: Channel<ADC, ID = u8>,
    Adc<ADC>: OneShot<ADC, u16, P0, Error = ()> + OneShot<ADC, u16, P1, Error = ()>,
{
    pub const LOW_THRESHOLD: u16 = 0;
    pub const HIGH_THRESHOLD: u16 = 4095;

    pub fn new(adc: Adc<ADC>, channel_0: P0, channel_1: P1) -> Self {
        Self {
            adc,
            channel_0,
            channel_1,
        }
    }

    pub fn scan(&mut self) -> Result<(Percentage, Percentage), DeviceError> {
        let raw_0: u16 = self
            .adc
            .read(&mut self.channel_0)
            .map_err(|_| DeviceError::AdcError)?;
        let raw_1: u16 = self
            .adc
            .read(&mut self.channel_1)
            .map_err(|_| DeviceError::AdcError)?;

        Ok((
            Percentage::to_percentage(raw_0, Self::HIGH_THRESHOLD, Self::LOW_THRESHOLD)
                .map_err(|_| DeviceError::ConversionError)?,
            Percentage::to_percentage(raw_1, Self::HIGH_THRESHOLD, Self::LOW_THRESHOLD)
                .map_err(|_| DeviceError::ConversionError)?,
        ))
    }
}
