pub mod call;
mod mov;

use crate::error::Error;
use crate::instructions::call::Call;
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
}

impl Instruction {
    pub fn executable(self) -> Box<dyn Execute> {
        match self {
            Instruction::Call(index) => Box::from(Call::new(index)),
            Instruction::Mov(source, destination) => Box::from(mov::Mov::new(source, destination)),
        }
    }
}
