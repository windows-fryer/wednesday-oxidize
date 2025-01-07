use crate::error::Error;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;
use crate::register::Width;

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
            Operand::Register(ref register) => register.as_u64(processor)?,

            Operand::Memory(ref memory) => match memory {
                Width::Byte(index) => processor.memory()?.get_u8(*index) as u64,
                Width::Word(index) => processor.memory()?.get_u16(*index) as u64,
                Width::DWord(index) => processor.memory()?.get_u32(*index) as u64,
                Width::QWord(index) => processor.memory()?.get_u64(*index),
            },

            Operand::MemoryRegister(ref memory_register) => {
                let address = match memory_register {
                    Width::Byte(index) => processor.register(*index)?.as_u8() as usize,
                    Width::Word(index) => processor.register(*index)?.as_u16() as usize,
                    Width::DWord(index) => processor.register(*index)?.as_u32() as usize,
                    Width::QWord(index) => processor.register(*index)?.as_u64() as usize,
                };

                match memory_register {
                    Width::Byte(_) => processor.memory()?.get_u8(address) as u64,
                    Width::Word(_) => processor.memory()?.get_u16(address) as u64,
                    Width::DWord(_) => processor.memory()?.get_u32(address) as u64,
                    Width::QWord(_) => processor.memory()?.get_u64(address),
                }
            }

            _ => return Err(Error::InvalidOperand),
        };

        match self.destination {
            Operand::Register(ref register) => match register {
                Width::Byte(index) => processor.register_mut(*index)?.assign_u8(source as u8),
                Width::Word(index) => processor.register_mut(*index)?.assign_u16(source as u16),
                Width::DWord(index) => processor.register_mut(*index)?.assign_u32(source as u32),
                Width::QWord(index) => processor.register_mut(*index)?.assign_u64(source),
            },

            Operand::Memory(ref memory) => match memory {
                Width::Byte(index) => processor.memory_mut()?.put_u8(*index, source as u8),
                Width::Word(index) => processor.memory_mut()?.put_u16(*index, source as u16),
                Width::DWord(index) => processor.memory_mut()?.put_u32(*index, source as u32),
                Width::QWord(index) => processor.memory_mut()?.put_u64(*index, source),
            },

            Operand::MemoryRegister(ref memory_register) => {
                let address = match memory_register {
                    Width::Byte(index) => processor.register(*index)?.as_u8() as usize,
                    Width::Word(index) => processor.register(*index)?.as_u16() as usize,
                    Width::DWord(index) => processor.register(*index)?.as_u32() as usize,
                    Width::QWord(index) => processor.register(*index)?.as_u64() as usize,
                };

                match memory_register {
                    Width::Byte(_) => processor.memory_mut()?.put_u8(address, source as u8),
                    Width::Word(_) => processor.memory_mut()?.put_u16(address, source as u16),
                    Width::DWord(_) => processor.memory_mut()?.put_u32(address, source as u32),
                    Width::QWord(_) => processor.memory_mut()?.put_u64(address, source),
                }
            }

            _ => return Err(Error::InvalidOperand),
        };

        Ok(())
    }
}
