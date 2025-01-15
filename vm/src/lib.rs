pub mod assembler;
pub mod error;
pub mod instructions;
mod memory;
mod processor;
pub mod register;

use crate::error::Error;
use crate::instructions::Execute;
use crate::memory::Memory;
use crate::processor::Processor;

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Default)]
/// An encapsulated struct containing the vital processor data and intercommunication.
pub struct VmCtx {
    memory: RwLock<Memory>,

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
    use crate::instructions::Operand;
    use crate::register::Width;

    #[test]
    fn test_new_vm() {
        let vm = Vm::new();
        assert_eq!(vm.processors.len(), 0);
    }

    #[test]
    fn test_load_instructions() {
        let mut vm = Vm::new();
        let instructions = vec![instructions::Instruction::Mov(
            Operand::Value(0),
            Operand::Register(Width::QWord(0)),
        )
        .executable()];
        let _ = vm.load_instructions(instructions);
        assert_eq!(vm.ctx.instructions.read().unwrap().len(), 1);
    }

    #[test]
    fn test_new_processor() {
        let mut vm = Vm::new();
        let _ = vm.new_processor();
        assert_eq!(vm.processors.len(), 1);
    }

    #[test]
    fn test_destroy_processor() {
        let mut vm = Vm::new();
        let prod_idx = vm.new_processor();
        vm.destroy_processor(prod_idx);
        assert_eq!(vm.processors.len(), 0);
    }

    #[test]
    fn test_processor() {
        let mut vm = Vm::new();
        let prod_idx = vm.new_processor();
        let processor = vm.processor(prod_idx).unwrap();

        assert_eq!(processor.register(0usize).unwrap().as_u64(), 0);
    }

    #[test]
    fn test_processor_mut() {
        let mut vm = Vm::new();
        let prod_idx = vm.new_processor();
        let processor = vm.processor_mut(prod_idx).unwrap();

        processor.register_mut(0usize).unwrap().assign_u64(10);
        assert_eq!(processor.register(0usize).unwrap().as_u64(), 10);
    }
}
