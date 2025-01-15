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

/// Assigns a given type value to the memory cell at the given index.
/// Retrieves a given type value from the memory cell at the given index.
macro_rules! primitive_impl {
    ($put_fn:ident, $get_fn:ident, $type:ty) => {
        pub fn $put_fn(&mut self, index: usize, value: $type) {
            self.cells
                .insert(index, MemoryCell::ByteArray(value.to_le_bytes().into()));
        }

        #[must_use]
        pub fn $get_fn(&self, index: usize) -> $type {
            match &self.cells[&index] {
                MemoryCell::ByteArray(value) => {
                    <$type>::from_le_bytes(value[..size_of::<$type>()].try_into().unwrap())
                }
            }
        }
    };
}

impl Memory {
    #[must_use]
    /// Constructs a new [`MemoryCell`].
    pub fn new() -> Self {
        Self::default()
    }

    primitive_impl!(put_u8, get_u8, u8);
    primitive_impl!(put_u16, get_u16, u16);
    primitive_impl!(put_u32, get_u32, u32);
    primitive_impl!(put_u64, get_u64, u64);

    primitive_impl!(put_i8, get_i8, i8);
    primitive_impl!(put_i16, get_i16, i16);
    primitive_impl!(put_i32, get_i32, i32);
    primitive_impl!(put_i64, get_i64, i64);

    primitive_impl!(put_f32, get_f32, f32);
    primitive_impl!(put_f64, get_f64, f64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8() {
        let mut memory = Memory::new();
        memory.put_u8(0, 0x01);

        assert_eq!(memory.get_u8(0), 0x01);
    }

    #[test]
    fn test_u16() {
        let mut memory = Memory::new();
        memory.put_u16(0, 0x0102);

        assert_eq!(memory.get_u16(0), 0x0102);
    }

    #[test]
    fn test_u32() {
        let mut memory = Memory::new();
        memory.put_u32(0, 0x01020304);

        assert_eq!(memory.get_u32(0), 0x01020304);
    }

    #[test]
    fn test_u64() {
        let mut memory = Memory::new();
        memory.put_u64(0, 0x0102030405060708);

        assert_eq!(memory.get_u64(0), 0x0102030405060708);
    }

    #[test]
    fn test_i8() {
        let mut memory = Memory::new();
        memory.put_i8(0, 0x01);

        assert_eq!(memory.get_i8(0), 0x01);
    }

    #[test]
    fn test_i16() {
        let mut memory = Memory::new();
        memory.put_i16(0, 0x0102);

        assert_eq!(memory.get_i16(0), 0x0102);
    }

    #[test]
    fn test_i32() {
        let mut memory = Memory::new();
        memory.put_i32(0, 0x01020304);

        assert_eq!(memory.get_i32(0), 0x01020304);
    }

    #[test]
    fn test_i64() {
        let mut memory = Memory::new();
        memory.put_i64(0, 0x0102030405060708);

        assert_eq!(memory.get_i64(0), 0x0102030405060708);
    }

    #[test]
    fn test_f32() {
        let mut memory = Memory::new();
        memory.put_f32(0, 0.1);

        assert_eq!(memory.get_f32(0), 0.1);
    }

    #[test]
    fn test_f64() {
        let mut memory = Memory::new();
        memory.put_f64(0, 0.1);

        assert_eq!(memory.get_f64(0), 0.1);
    }
}
