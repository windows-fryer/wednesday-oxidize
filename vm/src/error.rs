#[derive(Debug, Default, Eq, PartialEq)]
pub enum Error {
    #[default]
    /// An unknown error occurred; Occurs when error is not set.
    Unknown,

    /// The given [`Register`](crate::register::Register) index is out of bounds.
    RegisterIndexOutOfBounds,

    /// The given processor index is out of bounds.
    ProcessorIndexOutOfBounds,

    /// The [`VmCtx`](crate::VmCtx) instructions are poisoned.
    InstructionsPoisoned,

    /// The [`VmCtx`](crate::VmCtx) memory is poisoned.
    MemoryPoisoned,

    /// [`Execute`](crate::instructions::Execute) threw that [`Operand`](crate::instructions::Operand) wasn't valid.
    InvalidOperand,
}
