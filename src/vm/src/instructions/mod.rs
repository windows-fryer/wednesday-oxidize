mod mcall;
mod ncall;

use crate::instructions::mcall::MCall;
use crate::instructions::ncall::NCall;
use crate::processor::Processor;

/// Polymorphic self-containing non-mutable data-type.
pub trait Execute {
    #[must_use]
    /// Constructs a new [`Instruction`].
    fn new() -> Self;

    /// Executes the [`Instruction`] without modifying the state of the [`Processor`].
    fn execute(&self, processor: &Processor);

    /// Executes the [`Instruction`] modifying the state of the [`Processor`].
    fn execute_mut(&self, processor: &mut Processor);
}

#[derive(Debug)]
/// Encapsulated v-table holder for [`Execute`].
pub enum Instruction {
    MCall(MCall),
    NCall(NCall),
}
