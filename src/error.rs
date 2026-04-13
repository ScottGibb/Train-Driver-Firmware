use defmt::Format;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Format)]
pub enum DeviceError {
    AdcError(),
    ConversionError,
}
