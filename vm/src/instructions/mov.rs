use crate::error::Error;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;
use crate::register::Width;
use crate::{
    assign_memory_value, assign_memory_value_by_width, assign_register_value, get_memory_value,
    get_memory_value_by_width, get_register_value,
};

#[derive(Debug, Default)]
/// Move data from a [`Register`](Operand::Register) or [`Immediate`](Operand::Immediate) to another [`Register`](Operand::Register).
pub struct Mov {
    source: Operand,
    destination: Operand,
}

impl Mov {
    #[must_use]
    /// Constructs a new [`Mov`].
    pub fn new(source: Operand, destination: Operand) -> Self {
        Mov {
            source,
            destination,
        }
    }
}

impl Execute for Mov {
    fn execute(&self, processor: &mut Processor) -> Result<(), Error> {
        let source = match self.source {
            Operand::Value(value) => value,
            Operand::Register(ref register) => get_register_value!(processor, register),
            Operand::Memory(ref memory) => get_memory_value_by_width!(processor, memory),
            Operand::MemoryRegister(ref memory_register) => {
                let address = get_register_value!(processor, memory_register);

                get_memory_value!(processor, memory_register, address as usize)
            }

            _ => return Err(Error::InvalidOperand),
        };

        match self.destination {
            Operand::Register(ref register) => assign_register_value!(processor, register, source),

            Operand::Memory(ref memory) => assign_memory_value_by_width!(processor, memory, source),
            Operand::MemoryRegister(ref memory_register) => {
                let address = get_register_value!(processor, memory_register);

                assign_memory_value!(processor, memory_register, address as usize, source);
            }

            _ => return Err(Error::InvalidOperand),
        };

        Ok(())
    }
}
