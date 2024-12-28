use crate::error::Error;
use crate::instructions::{Execute, Operand};
use crate::processor::Processor;

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
            Operand::Immediate(value) => value,
            Operand::Register(value) => processor.register(value as usize)?.as_u64(),

            _ => return Err(Error::InvalidOperand),
        };

        let destination = match self.destination {
            Operand::Immediate(_) => return Err(Error::InvalidOperand),
            Operand::Register(value) => value,

            _ => return Err(Error::InvalidOperand),
        };

        processor
            .register_mut(destination as usize)?
            .assign_u64(source);

        Ok(())
    }
}
