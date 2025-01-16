use crate::error::Error;
use crate::instructions::jmp::Jmp;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;
use crate::register::Flag;

#[derive(Debug, Default)]
/// Jump to the specified location in the instruction memory if the zero flag is not set.
pub struct Jng {
    source: Operand,
}

impl Jng {
    #[must_use]
    /// Constructs a new [`Jng`].
    pub fn new(source: Operand) -> Self {
        Jng { source }
    }
}

impl Execute for Jng {
    fn execute(&self, processor: &mut Processor) -> Result<(), Error> {
        if processor.flag(Flag::Greater) {
            return Ok(());
        }

        Jmp::new(self.source.clone()).execute(processor)
    }
}
