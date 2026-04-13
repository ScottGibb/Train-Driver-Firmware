use defmt::Format;

use crate::types::PercentageError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Format)]
pub enum DeviceError {
    ConversionError(PercentageError),
}
