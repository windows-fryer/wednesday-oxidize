use crate::error::Error;
use crate::instructions::jmp::Jmp;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;
use crate::register::Flag;

#[derive(Debug, Default)]
/// Jump to the specified location in the instruction memory if the zero flag is set.
pub struct Jl {
    source: Operand,
}

impl Jl {
    #[must_use]
    /// Constructs a new [`Jl`].
    pub fn new(source: Operand) -> Self {
        Jl { source }
    }
}

impl Execute for Jl {
    fn execute(&self, processor: &mut Processor) -> Result<(), Error> {
        if !processor.flag(Flag::Less) {
            return Ok(());
        }

        processor.set_flag(Flag::Less, false);

        Jmp::new(self.source.clone()).execute(processor)
    }
}
