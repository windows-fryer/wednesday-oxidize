use std::collections::BTreeMap;

#[derive(Debug)]
/// Encapsulated abstraction for memory values.
enum MemoryCell {
    ByteArray(Box<[u8]>),
}

#[derive(Debug, Default)]
pub struct Memory {
    cells: BTreeMap<usize, MemoryCell>,
}

impl Memory {
    #[must_use]
    /// Constructs a new [`MemoryCell`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Assigns a [`u8`] value to the memory cell at the given index.
    pub fn put_u8(&mut self, index: usize, value: u8) {
        self.cells
            .insert(index, MemoryCell::ByteArray(value.to_le_bytes().into()));
    }

    /// Assigns a [`u16`] value to the memory cell at the given index.
    pub fn put_u16(&mut self, index: usize, value: u16) {
        self.cells
            .insert(index, MemoryCell::ByteArray(value.to_le_bytes().into()));
    }

    /// Assigns a [`u32`] value to the memory cell at the given index.
    pub fn put_u32(&mut self, index: usize, value: u32) {
        self.cells
            .insert(index, MemoryCell::ByteArray(value.to_le_bytes().into()));
    }

    /// Assigns a [`u64`] value to the memory cell at the given index.
    pub fn put_u64(&mut self, index: usize, value: u64) {
        self.cells
            .insert(index, MemoryCell::ByteArray(value.to_le_bytes().into()));
    }

    #[must_use]
    /// Retrieves a [`u8`] value from the memory cell at the given index.
    pub fn get_u8(&self, index: usize) -> u8 {
        match &self.cells[&index] {
            MemoryCell::ByteArray(value) => u8::from_le_bytes(value[..1].try_into().unwrap()),
        }
    }

    #[must_use]
    /// Retrieves a [`u16`] value from the memory cell at the given index.
    pub fn get_u16(&self, index: usize) -> u16 {
        match &self.cells[&index] {
            MemoryCell::ByteArray(value) => u16::from_le_bytes(value[..2].try_into().unwrap()),
        }
    }

    #[must_use]
    /// Retrieves a [`u32`] value from the memory cell at the given index.
    pub fn get_u32(&self, index: usize) -> u32 {
        match &self.cells[&index] {
            MemoryCell::ByteArray(value) => u32::from_le_bytes(value[..4].try_into().unwrap()),
        }
    }

    #[must_use]
    /// Retrieves a [`u64`] value from the memory cell at the given index.
    pub fn get_u64(&self, index: usize) -> u64 {
        match &self.cells[&index] {
            MemoryCell::ByteArray(value) => u64::from_le_bytes(value[..8].try_into().unwrap()),
        }
    }
}
