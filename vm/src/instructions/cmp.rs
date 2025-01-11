use crate::error::Error;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;
use crate::register::{Flag, Width};
use crate::{get_memory_value, get_memory_value_by_width, get_register_value};

#[derive(Debug, Default)]
/// Compare two operands and set the flags accordingly.
pub struct Cmp {
    value: Operand,
    comparator: Operand,
}

impl Cmp {
    #[must_use]
    /// Constructs a new [`Cmp`].
    pub fn new(value: Operand, comparator: Operand) -> Self {
        Cmp { value, comparator }
    }
}

impl Execute for Cmp {
    fn execute(&self, processor: &mut Processor) -> Result<(), Error> {
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

        let comparator = match self.comparator {
            Operand::Value(comparator) => comparator,
            Operand::Register(ref register) => get_register_value!(processor, register),
            Operand::Memory(ref memory) => get_memory_value_by_width!(processor, memory),
            Operand::MemoryRegister(ref memory_register) => {
                let address = get_register_value!(processor, memory_register);

                get_memory_value!(processor, memory_register, address as usize)
            }

            _ => return Err(Error::InvalidOperand),
        };

        processor.set_flag(Flag::Zero, value.wrapping_sub(comparator) == 0);
        processor.set_flag(Flag::Greater, value > comparator);

        Ok(())
    }
}
