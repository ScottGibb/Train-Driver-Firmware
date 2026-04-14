//! Top-level error types for the firmware.

use defmt::Format;

use crate::types::PercentageError;

/// Errors that can occur during device operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Format)]
pub enum DeviceError {
    /// A raw ADC or sensor value could not be converted to [`Percentage`](crate::types::Percentage).
    ConversionError(PercentageError),
    /// An ADC read failed.
    AdcError,
}
