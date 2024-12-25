use crate::instructions::{Execute, Instruction};

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
    pub fn call(mut self, index: u64) -> Self {
        self.instructions.push(Instruction::Call(index));

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
        let assembler = Assembler::new().call(0);

        assert_eq!(assembler.instructions.len(), 1);
        assert_eq!(assembler.instructions[0], Instruction::Call(0));
    }
}
