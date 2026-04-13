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

    pub fn scale_to(self, max: u16) -> u16 {
        let value = self.0 as u32;
        let max = max as u32;
        ((value * max + (Self::MAX as u32 / 2)) / Self::MAX as u32) as u16
    }
}
