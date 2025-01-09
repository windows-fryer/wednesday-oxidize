use crate::instructions::{Execute, Instruction, Operand};

#[derive(Debug, Default)]
/// Self-contained type for the creation and processing of instructions.
pub struct Assembler {
    instructions: Vec<Instruction>,
}

impl Assembler {
    #[must_use]
    /// Constructs a new [`Assembler`].
    pub fn new() -> Self {
        Assembler::default()
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Call`](Instruction::Call) into self.
    pub fn call(mut self, index: Operand) -> Self {
        self.instructions.push(Instruction::Call(index));

        self
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Mov`](Instruction::Mov) into self.
    pub fn mov(mut self, source: Operand, destination: Operand) -> Self {
        self.instructions
            .push(Instruction::Mov(source, destination));

        self
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Jmp`](Instruction::Jmp) into self.
    pub fn jmp(mut self, source: Operand) -> Self {
        self.instructions.push(Instruction::Jmp(source));

        self
    }

    #[must_use]
    /// Returns a list of [`Execute`] traits derived from self's instruction list.
    pub fn compile(self) -> Vec<Box<dyn Execute>> {
        self.instructions
            .into_iter()
            .map(|instruction| instruction.executable())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn assembler_construct() {
        let _ = Assembler::new();
    }

    #[test]
    pub fn assembler_call() {
        let assembler = Assembler::new().call(Operand::Value(0));

        assert_eq!(assembler.instructions.len(), 1);
        assert_eq!(
            assembler.instructions[0],
            Instruction::Call(Operand::Value(0))
        );
    }

    #[test]
    pub fn assembler_mov() {
        let assembler = Assembler::new().mov(Operand::Value(0), Operand::Value(1));

        assert_eq!(assembler.instructions.len(), 1);
        assert_eq!(
            assembler.instructions[0],
            Instruction::Mov(Operand::Value(0), Operand::Value(1))
        );
    }
}
