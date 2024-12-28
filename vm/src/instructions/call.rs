use crate::error::Error;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;

#[derive(Debug, Default)]
/// Call into internal or external code to access the [`Processor`] in a mutable state.
pub struct Call {
    call_index: Operand,
}

impl Call {
    #[must_use]
    /// Constructs a new [`Call`].
    pub fn new(call_index: Operand) -> Self {
        Call { call_index }
    }
}

impl Execute for Call {
    fn execute(&self, processor: &mut Processor) -> Result<(), Error> {
        let call_index = match self.call_index {
            Operand::Immediate(value) => value,
            Operand::Register(value) => processor.register(value as usize)?.as_u64(),

            _ => return Err(Error::InvalidOperand),
        };

        match call_index {
            0u64 => println!("{processor:?}"),

            _ => todo!(),
        }

        Ok(())
    }
}
