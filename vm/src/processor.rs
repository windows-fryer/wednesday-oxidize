use crate::error::Error;
use crate::register::{Register, ReservedIndex};
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
    pub fn new(vm_ctx: &Arc<VmCtx>) -> Self {
        Processor {
            vm_ctx: Arc::clone(vm_ctx),
            ..Self::default()
        }
    }

    /// Starts execution on self, locking [`VmCtx's`](VmCtx) instructions for readonly.
    ///
    /// # Errors
    /// When the [`VmCtx's`](VmCtx) instructions is poisoned, [`InstructionsPoisoned`](Error::InstructionsPoisoned) is returned.
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
                    instruction.execute(self)?;

                    let counter = self.register_mut(ReservedIndex::InstructionCounter as usize)?;

                    counter.assign_u64(counter.as_u64() + 1);
                }

                None => break,
            }
        }

        Ok(())
    }

    /// Returns a reference to the [`Register`] at the given index.
    pub fn register(&self, index: usize) -> Result<&Register, Error> {
        self.registers
            .get(index)
            .ok_or(Error::RegisterIndexOutOfBounds)
    }

    /// Returns a mutable reference to the [`Register`] at the given index.
    pub fn register_mut(&mut self, index: usize) -> Result<&mut Register, Error> {
        self.registers
            .get_mut(index)
            .ok_or(Error::RegisterIndexOutOfBounds)
    }

    /// Returns a reference to the [`Memory`] contained in the [`VmCtx`].
    pub fn memory(&self) -> Result<RwLockReadGuard<Memory>, Error> {
        self.vm_ctx.memory.read().map_err(|_| Error::MemoryPoisoned)
    }

    /// Returns a mutable reference to the [`Memory`] contained in the [`VmCtx`].
    pub fn memory_mut(&self) -> Result<RwLockWriteGuard<Memory>, Error> {
        self.vm_ctx
            .memory
            .write()
            .map_err(|_| Error::MemoryPoisoned)
    }
}

#[cfg(test)]
mod tests {
    use crate::assembler::Assembler;
    use crate::instructions::Operand;
    use crate::Vm;

    #[test]
    pub fn processor_execute() {
        let mut vm = Vm::new();

        let assembler = Assembler::new()
            .call(Operand::Value(0))
            .call(Operand::Value(0));
        let compiled = assembler.compile();

        vm.load_instructions(compiled).unwrap();

        let handle = vm.new_processor();
        let processor = vm.processors.get_mut(&handle).unwrap();

        processor.start().unwrap();
    }
}
