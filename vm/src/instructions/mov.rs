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

            _ => return Err(Error::InvalidOperand),
        };

        match self.destination {
            Operand::Register(ref register) => match register {
                Width::Byte(index) => processor.register_mut(*index)?.assign_u8(source as u8),
                Width::Word(index) => processor.register_mut(*index)?.assign_u16(source as u16),
                Width::DWord(index) => processor.register_mut(*index)?.assign_u32(source as u32),
                Width::QWord(index) => processor.register_mut(*index)?.assign_u64(source),
            },

            _ => return Err(Error::InvalidOperand),
        };

        Ok(())
    }
}
