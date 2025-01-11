use crate::error::Error;
use crate::processor::Processor;

#[derive(Debug, Default, Eq, PartialEq)]
/// Meta-type containing the byte layout for a 64-bit type.
pub struct Register([u8; 8]);

#[repr(usize)]
#[derive(Debug, Eq, PartialEq)]
/// Enum containing the reserved register indices.
pub enum ReservedIndex {
    Flags = 14,
    InstructionCounter = 15,
}

#[repr(u64)]
#[derive(Debug, Eq, PartialEq)]
pub enum Flag {
    Zero = 1 << 0,
    Greater = 1 << 1,
    Overflow = 1 << 2,
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// Enum containing the possible widths of a [`Register`].
pub enum Width {
    Byte(usize),
    Word(usize),
    DWord(usize),
    QWord(usize),
}

impl Width {
    /// Converts the [`Width`] to an 8-bit value.
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

/// Macro to set the value of the [`Register`] to the given primitive value.
macro_rules! primitive_impl {
    ($assign_name: ident, $as_name: ident, $type:ty) => {
        pub fn $assign_name(&mut self, value: $type) {
            self.0[..std::mem::size_of::<$type>()].copy_from_slice(&value.to_le_bytes());
        }

        pub fn $as_name(&self) -> $type {
            <$type>::from_le_bytes(self.0[..std::mem::size_of::<$type>()].try_into().unwrap())
        }
    };
}

impl Register {
    #[must_use]
    /// Constructs a new [`Register`] from a given 64-bit value.
    pub fn new(value: u64) -> Self {
        Register(value.to_le_bytes())
    }

    primitive_impl!(assign_u8, as_u8, u8);
    primitive_impl!(assign_u16, as_u16, u16);
    primitive_impl!(assign_u32, as_u32, u32);
    primitive_impl!(assign_u64, as_u64, u64);

    primitive_impl!(assign_i8, as_i8, i8);
    primitive_impl!(assign_i16, as_i16, i16);
    primitive_impl!(assign_i32, as_i32, i32);
    primitive_impl!(assign_i64, as_i64, i64);

    primitive_impl!(assign_f32, as_f32, f32);
    primitive_impl!(assign_f64, as_f64, f64);
}

#[macro_export]
/// Macro to match a [`Width`] to a register index.
macro_rules! get_register_value {
    ($processor:expr, $memory_register:expr) => {
        match $memory_register {
            Width::Byte(index) => $processor.register(*index)?.as_u8() as u64,
            Width::Word(index) => $processor.register(*index)?.as_u16() as u64,
            Width::DWord(index) => $processor.register(*index)?.as_u32() as u64,
            Width::QWord(index) => $processor.register(*index)?.as_u64(),
        }
    };
}

#[macro_export]
/// Macro to match a [`Width`] to a mutable register index.
macro_rules! assign_register_value {
    ($processor:expr, $memory_register:expr, $source:expr) => {
        match $memory_register {
            Width::Byte(index) => $processor.register_mut(*index)?.assign_u8($source as u8),
            Width::Word(index) => $processor.register_mut(*index)?.assign_u16($source as u16),
            Width::DWord(index) => $processor.register_mut(*index)?.assign_u32($source as u32),
            Width::QWord(index) => $processor.register_mut(*index)?.assign_u64($source),
        }
    };
}
