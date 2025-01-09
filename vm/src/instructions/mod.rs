pub mod call;
mod cmp;
mod jmp;
mod mov;

use crate::error::Error;
use crate::processor::Processor;
use crate::register::Width;

use std::fmt::Debug;

/// Polymorphic self-containing data-type for executing an instruction on a [`Processor`].
pub trait Execute: Debug {
    /// Executes the [`Instruction`] modifying the state of the [`Processor`].
    fn execute(&self, processor: &mut Processor) -> Result<(), Error>;
}

#[derive(Debug, Default, PartialEq, Eq)]
/// Abstraction type for instruction operands to contain multiple views of data.
pub enum Operand {
    #[default]
    None,

    Value(u64),
    Register(Width),

    Memory(Width),
    MemoryRegister(Width),
}

#[derive(Debug, PartialEq, Eq)]
/// Abstracted pseudo-type for [`Execute`].
pub enum Instruction {
    /// Depending on call index range, calls either user defined or vm defined function.
    Call(Operand),

    /// Moves data from one location to another.
    Mov(Operand, Operand),

    /// Jumps to the specified location in the instruction memory.
    Jmp(Operand),
}

impl Instruction {
    pub fn executable(self) -> Box<dyn Execute> {
        match self {
            Instruction::Call(index) => Box::from(call::Call::new(index)),
            Instruction::Mov(source, destination) => Box::from(mov::Mov::new(source, destination)),
            Instruction::Jmp(source) => Box::from(jmp::Jmp::new(source)),
        }
    }
}

#[macro_export]
/// Macro for matching the [`Memory`] type and getting the value.
macro_rules! get_memory_value_by_width {
    ($processor:expr, $memory:expr) => {
        match $memory {
            Width::Byte(index) => $processor.memory()?.get_u8(*index) as u64,
            Width::Word(index) => $processor.memory()?.get_u16(*index) as u64,
            Width::DWord(index) => $processor.memory()?.get_u32(*index) as u64,
            Width::QWord(index) => $processor.memory()?.get_u64(*index),
        }
    };
}

#[macro_export]
/// Macro for matching the [`Register`] type and getting the value. Omits the value of [`Width`].
macro_rules! get_memory_value {
    ($processor:expr, $memory:expr, $index:expr) => {
        match $memory {
            Width::Byte(_) => $processor.memory()?.get_u8($index) as u64,
            Width::Word(_) => $processor.memory()?.get_u16($index) as u64,
            Width::DWord(_) => $processor.memory()?.get_u32($index) as u64,
            Width::QWord(_) => $processor.memory()?.get_u64($index),
        }
    };
}

#[macro_export]
/// Macro for matching the [`Memory`] type and setting the value.
macro_rules! assign_memory_value_by_width {
    ($processor:expr, $memory:expr, $source:expr) => {
        match $memory {
            Width::Byte(index) => $processor.memory_mut()?.put_u8(*index, $source as u8),
            Width::Word(index) => $processor.memory_mut()?.put_u16(*index, $source as u16),
            Width::DWord(index) => $processor.memory_mut()?.put_u32(*index, $source as u32),
            Width::QWord(index) => $processor.memory_mut()?.put_u64(*index, $source),
        }
    };
}

#[macro_export]
/// Macro for matching the [`Memory`] type and setting the value. Omits the value of [`Width`].
macro_rules! assign_memory_value {
    ($processor:expr, $memory:expr, $index:expr, $source:expr) => {
        match $memory {
            Width::Byte(_) => $processor.memory_mut()?.put_u8($index, $source as u8),
            Width::Word(_) => $processor.memory_mut()?.put_u16($index, $source as u16),
            Width::DWord(_) => $processor.memory_mut()?.put_u32($index, $source as u32),
            Width::QWord(_) => $processor.memory_mut()?.put_u64($index, $source),
        }
    };
}
