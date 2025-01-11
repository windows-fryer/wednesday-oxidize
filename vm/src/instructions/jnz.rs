use crate::error::Error;
use crate::instructions::jmp::Jmp;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;
use crate::register::Flag;

#[derive(Debug, Default)]
/// Jump to the specified location in the instruction memory if the zero flag is not set.
pub struct Jnz {
    source: Operand,
}

impl Jnz {
    #[must_use]
    /// Constructs a new [`Jnz`].
    pub fn new(source: Operand) -> Self {
        Jnz { source }
    }
}

impl Execute for Jnz {
    fn execute(&self, processor: &mut Processor) -> Result<(), Error> {
        if processor.flag(Flag::Zero) {
            return Ok(());
        }

        // TODO: Figure out if clone nature is fine! There was a fatal flaw in my design that makes it
        //       Impossible for me to move source even though it does quite literally nothing but be moved.
        Jmp::new(self.source.clone()).execute(processor)
    }
}
