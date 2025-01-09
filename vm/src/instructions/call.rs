use crate::error::Error;
use crate::get_register_value;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;
use crate::register::Width;

#[repr(u64)]
#[derive(Debug, PartialEq, Eq)]
/// Enum containing the call indices for external code.
pub enum CallIndex {
    /// Prints the [`Processor`] using the Debug format.
    PrintProcessor = 0,
}

impl From<u64> for CallIndex {
    fn from(value: u64) -> Self {
        match value {
            0 => CallIndex::PrintProcessor,

            _ => todo!(),
        }
    }
}

#[derive(Debug, Default)]
/// Call into external code to access the [`Processor`] in a mutable state.
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
        let call_index: CallIndex = match self.call_index {
            Operand::Value(value) => value,
            Operand::Register(ref register) => get_register_value!(processor, register),

            _ => return Err(Error::InvalidOperand),
        }
        .into();

        match call_index {
            CallIndex::PrintProcessor => println!("{processor:#?}"),
        }

        Ok(())
    }
}
