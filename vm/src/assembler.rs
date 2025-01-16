use crate::instructions::{Execute, Instruction, Operand};

#[derive(Debug, Default)]
/// Self-contained type for the creation and processing of instructions.
pub struct Assembler {
    instructions: Vec<Instruction>,
}

impl Assembler {
    #[must_use]
    /// Constructs a new [`Assembler`].
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    ///
    /// let assembler = Assembler::new();
    /// ```
    pub fn new() -> Self {
        Assembler::default()
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Call`](Instruction::Call) into self.
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    /// use vm::instructions::Operand;
    ///
    /// let assembler = Assembler::new().call(Operand::Value(0));
    /// ```
    pub fn call(mut self, index: Operand) -> Self {
        self.instructions.push(Instruction::Call(index));

        self
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Mov`](Instruction::Mov) into self.
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    /// use vm::instructions::Operand;
    /// use vm::register::Width;
    ///
    /// let assembler = Assembler::new().mov(Operand::Value(0), Operand::Register(Width::QWord(0)));
    /// ```
    pub fn mov(mut self, source: Operand, destination: Operand) -> Self {
        self.instructions
            .push(Instruction::Mov(source, destination));

        self
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Jmp`](Instruction::Jmp) into self.
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    /// use vm::instructions::Operand;
    ///
    /// let assembler = Assembler::new().jmp(Operand::Value(0));
    /// ```
    pub fn jmp(mut self, source: Operand) -> Self {
        self.instructions.push(Instruction::Jmp(source));

        self
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Jnz`](Instruction::Jnz) into self.
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    /// use vm::instructions::Operand;
    ///
    /// let assembler = Assembler::new().jnz(Operand::Value(0));
    /// ```
    pub fn jnz(mut self, source: Operand) -> Self {
        self.instructions.push(Instruction::Jnz(source));

        self
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Jz`](Instruction::Jz) into self.
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    /// use vm::instructions::Operand;
    ///
    /// let assembler = Assembler::new().jz(Operand::Value(0));
    /// ```
    pub fn jz(mut self, source: Operand) -> Self {
        self.instructions.push(Instruction::Jz(source));

        self
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Cmp`](Instruction::Cmp) into self.
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    /// use vm::instructions::Operand;
    /// use vm::register::Width;
    ///
    /// let assembler = Assembler::new().cmp(Operand::Value(0), Operand::Register(Width::QWord(0)));
    /// ```
    pub fn cmp(mut self, source: Operand, destination: Operand) -> Self {
        self.instructions
            .push(Instruction::Cmp(source, destination));

        self
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Add`](Instruction::Add) into self.
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    /// use vm::instructions::Operand;
    /// use vm::register::Width;
    ///
    /// let assembler = Assembler::new().add(
    ///     Operand::Value(0),
    ///     Operand::Register(Width::QWord(0)),
    ///     Operand::Register(Width::QWord(0))
    /// );
    /// ```
    pub fn add(mut self, value: Operand, source: Operand, destination: Operand) -> Self {
        self.instructions
            .push(Instruction::Add(value, source, destination));

        self
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Sub`](Instruction::Sub) into self.
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    /// use vm::instructions::Operand;
    /// use vm::register::Width;
    ///
    /// let assembler = Assembler::new().sub(
    ///    Operand::Value(0),
    ///    Operand::Register(Width::QWord(0)),
    ///    Operand::Register(Width::QWord(0))
    /// );
    /// ```
    pub fn sub(mut self, value: Operand, source: Operand, destination: Operand) -> Self {
        self.instructions
            .push(Instruction::Sub(value, source, destination));

        self
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Jg`](Instruction::Jg) into self.
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    /// use vm::instructions::Operand;
    ///
    /// let assembler = Assembler::new().jg(Operand::Value(0));
    /// ```
    pub fn jg(mut self, source: Operand) -> Self {
        self.instructions.push(Instruction::Jg(source));

        self
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Jl`](Instruction::Jl) into self.
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    /// use vm::instructions::Operand;
    ///
    /// let assembler = Assembler::new().jl(Operand::Value(0));
    /// ```
    pub fn jl(mut self, source: Operand) -> Self {
        self.instructions.push(Instruction::Jl(source));

        self
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Jng`](Instruction::Jng) into self.
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    /// use vm::instructions::Operand;
    ///
    /// let assembler = Assembler::new().jng(Operand::Value(0));
    /// ```
    pub fn jng(mut self, source: Operand) -> Self {
        self.instructions.push(Instruction::Jng(source));

        self
    }

    #[must_use]
    /// Consumes [`self`](Assembler) pushing a new [`Jnl`](Instruction::Jnl) into self.
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    /// use vm::instructions::Operand;
    ///
    /// let assembler = Assembler::new().jnl(Operand::Value(0));
    /// ```
    pub fn jnl(mut self, source: Operand) -> Self {
        self.instructions.push(Instruction::Jnl(source));

        self
    }

    #[must_use]
    /// Returns a list of [`Execute`] traits derived from self's instruction list.
    ///
    /// # Example
    /// ```
    /// use vm::assembler::Assembler;
    /// use vm::instructions::Operand;
    /// use vm::register::Width;
    ///
    /// let instructions = Assembler::new()
    ///    .add(
    ///         Operand::Value(0),
    ///         Operand::Register(Width::QWord(0)),
    ///         Operand::Register(Width::QWord(0)),
    ///    )
    ///    .compile();
    ///```
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
    use crate::register::Width;

    #[test]
    fn test_new_assembler() {
        let assembler = Assembler::new();
        assert_eq!(assembler.instructions.len(), 0);
    }

    #[test]
    fn test_call() {
        let assembler = Assembler::new().call(Operand::Value(0));
        assert_eq!(assembler.instructions.len(), 1);
    }

    #[test]
    fn test_mov() {
        let assembler = Assembler::new().mov(Operand::Value(0), Operand::Register(Width::QWord(0)));
        assert_eq!(assembler.instructions.len(), 1);
    }

    #[test]
    fn test_jmp() {
        let assembler = Assembler::new().jmp(Operand::Value(0));
        assert_eq!(assembler.instructions.len(), 1);
    }

    #[test]
    fn test_jnz() {
        let assembler = Assembler::new().jnz(Operand::Value(0));
        assert_eq!(assembler.instructions.len(), 1);
    }

    #[test]
    fn test_jz() {
        let assembler = Assembler::new().jz(Operand::Value(0));
        assert_eq!(assembler.instructions.len(), 1);
    }

    #[test]
    fn test_cmp() {
        let assembler = Assembler::new().cmp(Operand::Value(0), Operand::Register(Width::QWord(0)));
        assert_eq!(assembler.instructions.len(), 1);
    }

    #[test]
    fn test_add() {
        let assembler = Assembler::new().add(
            Operand::Value(0),
            Operand::Register(Width::QWord(0)),
            Operand::Register(Width::QWord(0)),
        );
        assert_eq!(assembler.instructions.len(), 1);
    }

    #[test]
    fn test_compile() {
        let assembler = Assembler::new().add(
            Operand::Value(0),
            Operand::Register(Width::QWord(0)),
            Operand::Register(Width::QWord(0)),
        );
        let instructions = assembler.compile();
        assert_eq!(instructions.len(), 1);
    }
}
