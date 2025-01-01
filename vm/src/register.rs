use crate::error::Error;
use crate::processor::Processor;

#[derive(Debug, Default, Eq, PartialEq)]
/// Meta-type containing the byte layout for a 64-bit type.
pub struct Register([u8; 8]);

#[repr(usize)]
#[derive(Debug, Eq, PartialEq)]
/// Enum containing the reserved register indices.
pub enum ReservedIndex {
    InstructionCounter = 15,
}

#[derive(Debug, PartialEq, Eq)]
/// Enum containing the possible widths of a [`Register`].
pub enum Width {
    Byte(usize),
    Word(usize),
    DWord(usize),
    QWord(usize),
}

impl Width {
    /// Converts the [`Width`] to a 8-bit value.
    pub fn as_u8(&self, processor: &Processor) -> Result<u8, Error> {
        match self {
            Width::Byte(index) => Ok(processor.register(*index)?.as_u8()),
            Width::Word(index) => Ok(processor.register(*index)?.as_u16() as u8),
            Width::DWord(index) => Ok(processor.register(*index)?.as_u32() as u8),
            Width::QWord(index) => Ok(processor.register(*index)?.as_u64() as u8),
        }
    }

    /// Converts the [`Width`] to a 16-bit value.
    pub fn as_u16(&self, processor: &Processor) -> Result<u16, Error> {
        match self {
            Width::Byte(index) => Ok(processor.register(*index)?.as_u8() as u16),
            Width::Word(index) => Ok(processor.register(*index)?.as_u16()),
            Width::DWord(index) => Ok(processor.register(*index)?.as_u32() as u16),
            Width::QWord(index) => Ok(processor.register(*index)?.as_u64() as u16),
        }
    }

    /// Converts the [`Width`] to a 32-bit value.
    pub fn as_u32(&self, processor: &Processor) -> Result<u32, Error> {
        match self {
            Width::Byte(index) => Ok(processor.register(*index)?.as_u8() as u32),
            Width::Word(index) => Ok(processor.register(*index)?.as_u16() as u32),
            Width::DWord(index) => Ok(processor.register(*index)?.as_u32()),
            Width::QWord(index) => Ok(processor.register(*index)?.as_u64() as u32),
        }
    }

    /// Converts the [`Width`] to a 64-bit value.
    pub fn as_u64(&self, processor: &Processor) -> Result<u64, Error> {
        match self {
            Width::Byte(index) => Ok(processor.register(*index)?.as_u8() as u64),
            Width::Word(index) => Ok(processor.register(*index)?.as_u16() as u64),
            Width::DWord(index) => Ok(processor.register(*index)?.as_u32() as u64),
            Width::QWord(index) => Ok(processor.register(*index)?.as_u64()),
        }
    }
}

impl Register {
    #[must_use]
    /// Constructs a new [`Register`] from a given 64-bit value.
    pub fn new(value: u64) -> Self {
        Register(value.to_le_bytes())
    }

    /// Sets the value of the [`Register`] to the given 8-bit value.
    pub fn assign_u8(&mut self, value: u8) {
        self.0[..1].copy_from_slice(value.to_le_bytes().as_slice());
    }

    /// Sets the value of the [`Register`] to the given 16-bit value.
    pub fn assign_u16(&mut self, value: u16) {
        self.0[..2].copy_from_slice(value.to_le_bytes().as_slice());
    }

    /// Sets the value of the [`Register`] to the given 32-bit value.
    pub fn assign_u32(&mut self, value: u32) {
        self.0[..4].copy_from_slice(value.to_le_bytes().as_slice());
    }

    /// Sets the value of the [`Register`] to the given 64-bit value.
    pub fn assign_u64(&mut self, value: u64) {
        self.0 = value.to_le_bytes();
    }

    #[must_use]
    /// Gets the value of the [`Register`] as an 8-bit value.
    pub fn as_u8(&self) -> u8 {
        u8::from_le_bytes(self.0[..1].try_into().unwrap())
    }

    #[must_use]
    /// Gets the value of the [`Register`] as a 16-bit value.
    pub fn as_u16(&self) -> u16 {
        u16::from_le_bytes(self.0[..2].try_into().unwrap())
    }

    #[must_use]
    /// Gets the value of the [`Register`] as a 32-bit value.
    pub fn as_u32(&self) -> u32 {
        u32::from_le_bytes(self.0[..4].try_into().unwrap())
    }

    #[must_use]
    /// Gets the value of the [`Register`] as a 64-bit value.
    pub fn as_u64(&self) -> u64 {
        u64::from_le_bytes(self.0)
    }
}
