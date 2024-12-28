pub mod assembler;
pub mod error;
pub mod instructions;
mod memory;
mod processor;
pub mod register;

use crate::error::Error;
use crate::instructions::Execute;
use crate::processor::Processor;

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Default)]
/// An encapsulated struct containing the vital processor data and intercommunication.
pub struct VmCtx {
    memory: RwLock<Vec<Box<[u8]>>>,

    instructions: RwLock<Vec<Box<dyn Execute>>>,
}

#[derive(Debug, Default)]
/// A unique struct containing the processors and VM context.
pub struct Vm {
    processors: BTreeMap<usize, Processor>,
    ctx: Arc<VmCtx>,
}

impl Vm {
    #[must_use]
    /// Constructs a new [`Vm`].
    ///
    /// # Example
    /// ```
    /// use vm::Vm;
    /// let vm = Vm::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Moves the given [`Instruction`](instructions::Instruction) slice into [`VmCtx`] memory.
    ///
    /// # Example
    /// ```
    /// use vm::Vm;
    /// let mut vm_inst = Vm::new();
    /// let instructions = Vec::from([/* ... */]);
    /// _ = vm_inst.load_instructions(instructions);
    /// ```
    ///
    /// # Errors
    /// When the [`VmCtx`].instructions is poisoned, [`InstructionsPoisoned`](Error::InstructionsPoisoned) is returned.
    pub fn load_instructions(&mut self, instructions: Vec<Box<dyn Execute>>) -> Result<(), Error> {
        let ctx = Arc::clone(&self.ctx);
        let mut guard = ctx
            .instructions
            .write()
            .map_err(|_| Error::InstructionsPoisoned)?;

        *guard = instructions;

        Ok(())
    }

    #[must_use]
    /// Finds a new handle for the user climbing incrementally.
    /// # Vulnerabilities
    /// When you're given a handle, it is **expected you know the lifetime of the handle**.
    /// Failure to do so will result in nature that is unwarranted. Future implementations may
    /// include a random handle generator to mitigate this issue. However, it still doesn't ignore
    /// the issue of lifetime.
    fn find_next_handle(&self) -> usize {
        if self.processors.is_empty() {
            return 0;
        }

        // Go through the processors, create another iterator that skips 1 element, find a gap between indices.
        // Algorithm is quite slow, reaching O(2n) complexity.
        let index = self
            .processors
            .iter()
            .zip(self.processors.iter().skip(1))
            .find_map(|(iter, iter_skip)| -> Option<usize> {
                if iter_skip.0 - iter.0 > 1 {
                    return Some(iter.0 + 1);
                }

                None
            });

        index.unwrap_or(self.processors.len())
    }

    #[must_use]
    /// Constructs a new [`Processor`] and returns a unique handle to the [`Processor`].
    ///
    /// The handle exists with the [`Processor`]. Hence, it shares lifetimes with the [`Vm`].
    ///
    /// # Example
    /// ```
    /// use vm::Vm;
    /// let mut vm_inst = Vm::new();
    /// let mut _prod_idx = vm_inst.new_processor();
    /// ```
    pub fn new_processor(&mut self) -> usize {
        let index = self.find_next_handle();

        self.processors.insert(index, Processor::new(&self.ctx));

        index
    }

    /// Destroys the [`Processor`] at the given index.
    ///
    /// # Example
    /// ```
    /// use vm::Vm;
    /// let mut vm_inst = Vm::new();
    /// let mut prod_idx = vm_inst.new_processor();
    /// vm_inst.destroy_processor(prod_idx);
    /// ```
    pub fn destroy_processor(&mut self, index: usize) {
        self.processors.remove(&index);
    }

    /// Returns a reference to the [`Processor`] at the given index.
    pub fn processor(&self, index: usize) -> Result<&Processor, Error> {
        self.processors
            .get(&index)
            .ok_or(Error::ProcessorIndexOutOfBounds)
    }

    /// Returns a mutable reference to the [`Processor`] at the given index.
    pub fn processor_mut(&mut self, index: usize) -> Result<&mut Processor, Error> {
        self.processors
            .get_mut(&index)
            .ok_or(Error::ProcessorIndexOutOfBounds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn vm_construct() {
        let _ = Vm::new();
    }

    #[test]
    pub fn vm_processor_construct_once() {
        let mut vm = Vm::new();

        let processor_handle = vm.new_processor();

        assert_eq!(vm.processors.len(), 1);
        assert_eq!(vm.processors.len() - 1, processor_handle);
    }

    #[test]
    pub fn vm_processor_construct_multi() {
        let mut vm = Vm::new();

        let _ = vm.new_processor();
        let second_processor_handle = vm.new_processor();

        assert_eq!(vm.processors.len(), 2);
        assert_eq!(vm.processors.len() - 1, second_processor_handle);
    }

    #[test]
    pub fn vm_processor_construct_override_once() {
        let mut vm = Vm::new();

        let processor_handle = vm.new_processor();
        vm.destroy_processor(processor_handle);

        let second_processor_handle = vm.new_processor();

        assert_eq!(vm.processors.len(), 1);
        assert_eq!(vm.processors.len() - 1, second_processor_handle);
    }

    #[test]
    pub fn vm_processor_construct_override_multi() {
        let mut vm = Vm::new();

        let _first_processor_handle = vm.new_processor();
        let second_processor_handle = vm.new_processor();
        let _third_processor_handle = vm.new_processor();

        vm.destroy_processor(second_processor_handle);

        let fourth_processor_handle = vm.new_processor();

        assert_eq!(vm.processors.len(), 3);
        assert_eq!(1, fourth_processor_handle);
    }

    #[test]
    pub fn vm_processor_deconstruct_once() {
        let mut vm = Vm::new();

        let processor_handle = vm.new_processor();

        vm.destroy_processor(processor_handle);

        assert_eq!(vm.processors.len(), 0);
    }

    #[test]
    pub fn vm_processor_deconstruct_multi() {
        let mut vm = Vm::new();

        let first_processor_handle = vm.new_processor();
        let second_processor_handle = vm.new_processor();

        vm.destroy_processor(first_processor_handle);

        assert_eq!(vm.processors.len(), 1);
        assert_eq!(vm.processors.len(), second_processor_handle);
    }
}
