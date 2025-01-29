use crate::error::Error;
use crate::register::{Flag, Register, ReservedIndex};
use crate::VmCtx;

use crate::memory::Memory;
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Default)]
/// Single-threaded object running code given by the [`Vm`][crate::Vm].
pub struct Processor {
    vm_ctx: Arc<VmCtx>,

    /// "16, why 16!?" - The ISA for the Wednesday VM only permits for 16 registers.
    ///                  16 comes from the lower bound of the 4-bit register index.
    registers: [Register; 16],
}

impl Processor {
    #[must_use]
    /// Constructs a new [`Processor`] creating a new reference to [`VmCtx`].
    ///
    /// # Example
    /// ```
    /// use vm::Processor;
    /// use vm::VmCtx;
    /// use std::sync::Arc;
    ///
    /// let vm_ctx = Arc::new(VmCtx::default());
    /// let processor = Processor::new(&vm_ctx);
    /// ```
    pub fn new(vm_ctx: &Arc<VmCtx>) -> Self {
        Processor {
            vm_ctx: Arc::clone(vm_ctx),
            ..Self::default()
        }
    }

    /// Starts execution on self, locking [`VmCtx's`](VmCtx) instructions for readonly.
    ///
    /// # Example
    /// ```
    /// use vm::Processor;
    /// use vm::VmCtx;
    /// use std::sync::Arc;
    /// use vm::error::Error;
    ///
    /// let vm_ctx = Arc::new(VmCtx::default());
    /// let mut processor = Processor::new(&vm_ctx);
    /// let _ = processor.start();
    /// ```
    ///
    /// # Errors
    /// When the [`VmCtx`](VmCtx) instructions are poisoned, [`InstructionsPoisoned`](Error::InstructionsPoisoned) is returned.
    pub fn start(&mut self) -> Result<(), Error> {
        let ctx = Arc::clone(&self.vm_ctx);
        let executable_slice = ctx
            .instructions
            .read()
            .map_err(|_| Error::InstructionsPoisoned)?;

        loop {
            let register_index = self
                .register(ReservedIndex::InstructionCounter as usize)?
                .as_u64() as usize;
            let instruction = executable_slice.get(register_index);

            match instruction {
                Some(instruction) => {
                    let counter = self.register_mut(ReservedIndex::InstructionCounter as usize)?;

                    counter.assign_u64(counter.as_u64() + 1);

                    instruction.execute(self)?;
                }

                None => break,
            }
        }

        Ok(())
    }

    /// Returns a reference to the [`Register`] at the given index.
    ///
    /// # Example
    /// ```
    /// use vm::Processor;
    /// use vm::VmCtx;
    /// use std::sync::Arc;
    ///
    /// let vm_ctx = Arc::new(VmCtx::default());
    /// let processor = Processor::new(&vm_ctx);
    /// let register = processor.register(0);
    /// ```
    pub fn register(&self, index: usize) -> Result<&Register, Error> {
        self.registers
            .get(index)
            .ok_or(Error::RegisterIndexOutOfBounds)
    }

    /// Returns a mutable reference to the [`Register`] at the given index.
    ///
    /// # Example
    /// ```
    /// use vm::Processor;
    /// use vm::VmCtx;
    /// use std::sync::Arc;
    ///
    /// let vm_ctx = Arc::new(VmCtx::default());
    /// let mut processor = Processor::new(&vm_ctx);
    /// let register = processor.register_mut(0);
    /// ```
    pub fn register_mut(&mut self, index: usize) -> Result<&mut Register, Error> {
        self.registers
            .get_mut(index)
            .ok_or(Error::RegisterIndexOutOfBounds)
    }

    /// Returns a reference to the [`Memory`] contained in the [`VmCtx`].
    ///
    /// # Example
    /// ```
    /// use vm::Processor;
    /// use vm::VmCtx;
    /// use std::sync::Arc;
    ///
    /// let vm_ctx = Arc::new(VmCtx::default());
    /// let processor = Processor::new(&vm_ctx);
    /// let _ = processor.memory();
    /// ```
    pub fn memory(&self) -> Result<RwLockReadGuard<Memory>, Error> {
        self.vm_ctx.memory.read().map_err(|_| Error::MemoryPoisoned)
    }

    /// Returns a mutable reference to the [`Memory`] contained in the [`VmCtx`].
    ///
    /// # Example
    /// ```
    /// use vm::Processor;
    /// use vm::VmCtx;
    /// use std::sync::Arc;
    ///
    /// let vm_ctx = Arc::new(VmCtx::default());
    /// let mut processor = Processor::new(&vm_ctx);
    /// let _ = processor.memory_mut();
    /// ```
    pub fn memory_mut(&self) -> Result<RwLockWriteGuard<Memory>, Error> {
        self.vm_ctx
            .memory
            .write()
            .map_err(|_| Error::MemoryPoisoned)
    }

    /// Sets the given [`Flag`] to the given state.
    ///
    /// # Example
    /// ```
    /// use vm::Processor;
    /// use vm::VmCtx;
    /// use std::sync::Arc;
    /// use vm::register::Flag;
    ///
    /// let vm_ctx = Arc::new(VmCtx::default());
    /// let mut processor = Processor::new(&vm_ctx);
    /// processor.set_flag(Flag::Overflow, true);
    /// ```
    pub fn set_flag(&mut self, flag: Flag, state: bool) {
        let mut flags = self
            .register_mut(ReservedIndex::Flags as usize)
            .unwrap()
            .as_u64();

        if state {
            flags |= flag as u64;
        } else {
            flags &= !(flag as u64);
        }

        self.registers[ReservedIndex::Flags as usize].assign_u64(flags);
    }

    #[must_use]
    /// Returns the state of the given [`Flag`].
    ///
    /// # Example
    /// ```
    /// use vm::Processor;
    /// use vm::VmCtx;
    /// use std::sync::Arc;
    /// use vm::register::Flag;
    ///
    /// let vm_ctx = Arc::new(VmCtx::default());
    /// let processor = Processor::new(&vm_ctx);
    /// let _ = processor.flag(Flag::Overflow);
    /// ```
    pub fn flag(&self, flag: Flag) -> bool {
        let flags = self
            .register(ReservedIndex::Flags as usize)
            .unwrap()
            .as_u64();

        flags & (flag as u64) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vm;

    #[test]
    fn test_new_processor() {
        let mut vm = Vm::new();

        let _ = vm.new_processor();
    }

    #[test]
    fn test_register() {
        let mut vm = Vm::new();

        let processor_handle = vm.new_processor();
        let processor = vm.processor(processor_handle).unwrap();

        let register = processor.register(0usize).unwrap();
        assert_eq!(register.as_u64(), 0);
    }

    #[test]
    fn test_register_mut() {
        let mut vm = Vm::new();

        let processor_handle = vm.new_processor();
        let processor = vm.processor_mut(processor_handle).unwrap();

        let register = processor.register_mut(0usize).unwrap();
        register.assign_u64(1);

        assert_eq!(register.as_u64(), 1);
    }

    #[test]
    fn test_memory() {
        let mut vm = Vm::new();

        let processor_handle = vm.new_processor();
        let processor = vm.processor(processor_handle).unwrap();

        let _unused = processor.memory().unwrap();
    }

    #[test]
    fn test_memory_mut() {
        let mut vm = Vm::new();

        let processor_handle = vm.new_processor();
        let processor = vm.processor(processor_handle).unwrap();

        let mut memory = processor.memory_mut().unwrap();
        memory.put_u8(0, 1);

        assert_eq!(memory.get_u8(0), 1);
    }

    #[test]
    fn test_set_flag() {
        let mut vm = Vm::new();

        let processor_handle = vm.new_processor();
        let processor = vm.processor_mut(processor_handle).unwrap();

        processor.set_flag(Flag::Overflow, true);
        assert!(processor.flag(Flag::Overflow));
    }
}
