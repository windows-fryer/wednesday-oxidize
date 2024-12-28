mod call;
mod mov;

use crate::error::Error;
use crate::instructions::call::Call;
use crate::processor::Processor;

use std::fmt::Debug;

/// Polymorphic self-containing data-type for executing an instruction on a [`Processor`].
pub trait Execute: Debug {
    /// Executes the [`Instruction`] modifying the state of the [`Processor`].
    fn execute(&self, processor: &mut Processor) -> Result<(), Error>;
}

#[derive(Debug, Default, PartialEq, Eq)]
/// Abstraction type for allowing [`Operands`](Operand) to be either [`Register`](Operand::Register) or [`Immediate`](Operand::Register).
pub enum Operand {
    #[default]
    None,

    Register(u64),
    Immediate(u64),
}

#[derive(Debug, PartialEq, Eq)]
/// Abstracted pseudo-type for [`Execute`].
pub enum Instruction {
    /// Depending on call index range, calls either user defined or vm defined function.
    Call(Operand),

    /// Moves data from one [`Register`](crate::processor::Register) or [`Immediate`](Operand::Immediate) to another [`Register`](crate::processor::Register).
    Mov(Operand, Operand),
}

impl Instruction {
    pub fn executable(self) -> Box<dyn Execute> {
        match self {
            Instruction::Call(idx) => Box::from(Call::new(idx)),
            Instruction::Mov(src, dst) => Box::from(mov::Mov::new(src, dst)),
        }
    }
}
