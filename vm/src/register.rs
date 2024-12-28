#[derive(Debug, Default, Eq, PartialEq)]
/// Meta-type containing the byte layout for a 64-bit type.
pub struct Register([u8; 8]);

#[repr(usize)]
#[derive(Debug, Eq, PartialEq)]
/// Enum containing the reserved register indices.
pub enum ReservedIndex {
    InstructionCounter = 15,
}

impl Register {
    #[must_use]
    /// Constructs a new [`Register`] from a given 64-bit value.
    pub fn new(value: u64) -> Self {
        Register(value.to_le_bytes())
    }

    /// Sets the value of the [`Register`] to the given 64-bit value.
    pub fn assign_u64(&mut self, value: u64) {
        self.0 = value.to_le_bytes();
    }

    #[must_use]
    /// Gets the value of the [`Register`] as a 64-bit value.
    pub fn as_u64(&self) -> u64 {
        u64::from_le_bytes(self.0)
    }
}
