use defmt::Format;

/// A value in the range 0–100 representing a percentage.
///
/// Constructed via [`Percentage::new`] or [`Percentage::from_range`],
/// both of which enforce the 0–100 invariant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Format)]
pub struct Percentage(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Format)]
pub enum PercentageError {
    OutOfRange,
    IncorrectThresholds,
}

impl Percentage {
    pub const MAX: u8 = 100;

    pub fn new(value: u8) -> Result<Self, PercentageError> {
        if value <= Self::MAX {
            Ok(Self(value))
        } else {
            Err(PercentageError::OutOfRange)
        }
    }

    pub fn get(self) -> u8 {
        self.0
    }

    /// Maps `value` from the range `[min, max]` into a [`Percentage`].
    ///
    /// Values outside the range are clamped before scaling.
    /// Returns [`PercentageError::IncorrectThresholds`] if `min >= max`.
    pub fn from_range(value: u16, min: u16, max: u16) -> Result<Self, PercentageError> {
        if min >= max {
            return Err(PercentageError::IncorrectThresholds);
        }
        let clamped = value.clamp(min, max);
        let scaled = ((clamped - min) as u32 * 100) / (max - min) as u32;
        Percentage::new(scaled as u8)
    }
}
