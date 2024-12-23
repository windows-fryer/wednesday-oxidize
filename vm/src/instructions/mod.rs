mod call;

use crate::instructions::call::Call;
use crate::processor::Processor;

use std::fmt::Debug;

/// Polymorphic self-containing data-type for executing an instruction on a [`Processor`].
pub trait Execute: Debug {
    /// Executes the [`Instruction`] modifying the state of the [`Processor`].
    fn execute(&self, processor: &mut Processor);
}

#[derive(Debug, PartialEq, Eq)]
/// Abstracted pseudo-type for [`Execute`].
pub enum Instruction {
    /// Depending on call index range, calls either user defined or vm defined function.
    Call(u64),
}

impl Instruction {
    pub fn executable(&self) -> Box<dyn Execute> {
        match self {
            Instruction::Call(idx) => Box::from(Call::new(*idx)),
        }
    }
}
