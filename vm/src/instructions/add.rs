use crate::error::Error;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;
use crate::register::{Flag, Width};
use crate::{
    assign_memory_value, assign_memory_value_by_width, assign_register_value, get_memory_value,
    get_memory_value_by_width, get_register_value,
};

#[derive(Debug, Default)]
/// Add two operands and store the result in the destination.
pub struct Add {
    value: Operand,
    source: Operand,
    destination: Operand,
}

impl Add {
    #[must_use]
    /// Constructs a new [`Add`].
    pub fn new(value: Operand, source: Operand, destination: Operand) -> Self {
        Add {
            value,
            source,
            destination,
        }
    }
}

impl Execute for Add {
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

        let value = match self.value {
            Operand::Value(value) => value,
            Operand::Register(ref register) => get_register_value!(processor, register),
            Operand::Memory(ref memory) => get_memory_value_by_width!(processor, memory),
            Operand::MemoryRegister(ref memory_register) => {
                let address = get_register_value!(processor, memory_register);

                get_memory_value!(processor, memory_register, address as usize)
            }

            _ => return Err(Error::InvalidOperand),
        };

        let result = source.overflowing_add(value);

        if let (_, true) = result {
            processor.set_flag(Flag::Overflow, true);
        }

        match self.destination {
            Operand::Register(ref register) => {
                assign_register_value!(processor, register, result.0)
            }
            Operand::Memory(ref memory) => {
                assign_memory_value_by_width!(processor, memory, result.0)
            }
            Operand::MemoryRegister(ref memory_register) => {
                let memory = get_register_value!(processor, memory_register);

                assign_memory_value!(processor, memory_register, memory as usize, result.0)
            }

            _ => return Err(Error::InvalidOperand),
        }

        Ok(())
    }
}
