use crate::error::Error;
use crate::instructions::jmp::Jmp;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;
use crate::register::Flag;

#[derive(Debug, Default)]
/// Jump to the specified location in the instruction memory if the zero flag is set.
pub struct Jg {
    source: Operand,
}

impl Jg {
    #[must_use]
    /// Constructs a new [`Jg`].
    pub fn new(source: Operand) -> Self {
        Jg { source }
    }
}

impl Execute for Jg {
    fn execute(&self, processor: &mut Processor) -> Result<(), Error> {
        if !processor.flag(Flag::Greater) {
            return Ok(());
        }

        processor.set_flag(Flag::Greater, false);

        Jmp::new(self.source.clone()).execute(processor)
    }
}
