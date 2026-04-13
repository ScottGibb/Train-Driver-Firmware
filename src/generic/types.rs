use defmt::Format;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Format)]
pub struct Percentage(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Format)]
pub enum PercentageError {
    InvalidValue,
}

impl Percentage {
    pub const MAX: u8 = 100;

    pub fn new(value: u8) -> Result<Self, PercentageError> {
        if value <= Self::MAX {
            Ok(Self(value))
        } else {
            Err(PercentageError::InvalidValue)
        }
    }

    pub fn get(self) -> u8 {
        self.0
    }

    pub fn to_percentage(value: u16, max: u16, min: u16) -> Result<Self, PercentageError> {
        let clamped = value.clamp(min, max);
        let scaled = ((clamped - min) as u32 * 100) / (max - min) as u32;
        Percentage::new(scaled as u8)
    }
}
