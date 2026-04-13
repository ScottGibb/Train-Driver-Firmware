use defmt::Format;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Format)]
pub struct Percentage(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PercentageError {
    InvalidValue,
}

impl Percentage {
    pub fn new(value: u8) -> Result<Self, PercentageError> {
        if value > 100 {
            return Err(PercentageError::InvalidValue);
        }
        Ok(Self(value))
    }
}

impl From<Percentage> for u8 {
    fn from(value: Percentage) -> Self {
        value.0
    }
}

impl TryFrom<u8> for Percentage {
    type Error = PercentageError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Percentage::new(value)
    }
}

impl From<Percentage> for u16 {
    fn from(value: Percentage) -> Self {
        value.0 as u16
    }
}

impl TryFrom<u16> for Percentage {
    type Error = PercentageError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 100 {
            return Err(PercentageError::InvalidValue);
        }
        Ok(Self(value as u8))
    }
}
