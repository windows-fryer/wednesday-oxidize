#[derive(Debug, Default, Eq, PartialEq)]
pub enum Error {
    #[default]
    Unknown,

    RegisterIndexOutOfBounds,
    ProcessorIndexOutOfBounds,

    InstructionsPoisoned,
    MemoryPoisoned,

    InvalidOperand,
}
