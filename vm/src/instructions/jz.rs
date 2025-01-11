use crate::error::Error;
use crate::instructions::jmp::Jmp;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;
use crate::register::Flag;

#[derive(Debug, Default)]
/// Jump to the specified location in the instruction memory if the zero flag is set.
pub struct Jz {
    source: Operand,
}

impl Jz {
    #[must_use]
    /// Constructs a new [`Jz`].
    pub fn new(source: Operand) -> Self {
        Jz { source }
    }
}

impl Execute for Jz {
    fn execute(&self, processor: &mut Processor) -> Result<(), Error> {
        if !processor.flag(Flag::Zero) {
            return Ok(());
        }

        processor.set_flag(Flag::Zero, false);

        // TODO: Figure out if clone nature is fine! There was a fatal flaw in my design that makes it
        //       Impossible for me to move source even though it does quite literally nothing but be moved.
        Jmp::new(self.source.clone()).execute(processor)
    }
}
