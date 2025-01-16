use crate::error::Error;
use crate::instructions::jmp::Jmp;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;
use crate::register::Flag;

#[derive(Debug, Default)]
/// Jump to the specified location in the instruction memory if the zero flag is not set.
pub struct Jnl {
    source: Operand,
}

impl Jnl {
    #[must_use]
    /// Constructs a new [`Jnl`].
    pub fn new(source: Operand) -> Self {
        Jnl { source }
    }
}

impl Execute for Jnl {
    fn execute(&self, processor: &mut Processor) -> Result<(), Error> {
        if processor.flag(Flag::Less) {
            return Ok(());
        }

        Jmp::new(self.source.clone()).execute(processor)
    }
}
