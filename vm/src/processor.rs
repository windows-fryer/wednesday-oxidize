use crate::VmCtx;

use crate::error::Error;

use std::sync::Arc;

#[derive(Debug, Default)]
/// Meta-type containing the byte layout for a 64-bit type.
pub struct Register([u8; 8]);

impl Into<u64> for Register {
    fn into(self) -> u64 {
        u64::from_le_bytes(self.0)
    }
}

#[derive(Debug, Default)]
/// Single-threaded object running code given by the [`Vm`][crate::Vm].
pub struct Processor {
    pub vm_ctx: Arc<VmCtx>,

    pub registers: [Register; 16],
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

    /// Starts execution on self, locking [`VmCtx`]'s instructions for readonly.
    ///
    /// # Errors
    /// When the [`VmCtx`]'s instructions is poisoned, [`InstructionsPoisoned`] is returned.
    pub fn start(&mut self) -> Result<(), Error> {
        let ctx = Arc::clone(&self.vm_ctx);
        let executable_slice = ctx
            .instructions
            .read()
            .map_err(|_| Error::InstructionsPoisoned)?;

        executable_slice.iter().for_each(|instruction| {
            instruction.execute(self);
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::assembler::Assembler;
    use crate::Vm;

    #[test]
    pub fn processor_execute() {
        let mut vm = Vm::new();

        let assembler = Assembler::new().call(0).call(1);
        let compiled = assembler.compile();

        vm.load_instructions(compiled).unwrap();

        let handle = vm.new_processor();
        let processor = vm.processors.get_mut(&handle).unwrap();

        processor.start().unwrap();
    }
}
