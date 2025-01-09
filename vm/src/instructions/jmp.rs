use crate::error::Error;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;
use crate::register::{ReservedIndex, Width};
use crate::{get_memory_value, get_memory_value_by_width, get_register_value};

#[derive(Debug, Default)]
/// Jump to the specified location in the instruction memory.
pub struct Jmp {
    source: Operand,
}

impl Jmp {
    #[must_use]
    /// Constructs a new [`Jmp`].
    pub fn new(source: Operand) -> Self {
        Jmp { source }
    }
}

impl Execute for Jmp {
    fn execute(&self, processor: &mut Processor) -> Result<(), Error> {
        match self.source {
            Operand::Value(value) => processor
                .register_mut(ReservedIndex::InstructionCounter as usize)?
                .assign_u64(value),

            Operand::Register(ref register) => {
                let value = get_register_value!(processor, register);

                processor
                    .register_mut(ReservedIndex::InstructionCounter as usize)?
                    .assign_u64(value);
            }

            Operand::Memory(ref memory) => {
                let value = get_memory_value_by_width!(processor, memory);

                processor
                    .register_mut(ReservedIndex::InstructionCounter as usize)?
                    .assign_u64(value);
            }
            Operand::MemoryRegister(ref memory_register) => {
                let address = get_register_value!(processor, memory_register);
                let value = get_memory_value!(processor, memory_register, address as usize);

                processor
                    .register_mut(ReservedIndex::InstructionCounter as usize)?
                    .assign_u64(value);
            }

            _ => return Err(Error::InvalidOperand),
        };

        Ok(())
    }
}
