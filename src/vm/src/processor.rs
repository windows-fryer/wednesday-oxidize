use crate::VmCtx;

use crate::instructions::Instruction;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Debug, Default)]
/// Single-threaded object running code given by the [`Vm`][crate::Vm].
pub struct Processor {
    vm_ctx: Arc<RwLock<VmCtx>>,
}

impl Processor {
    #[must_use]
    /// Constructs a new [`Processor`] creating a new reference to [`VmCtx`].
    pub fn new(vm_ctx: &Arc<RwLock<VmCtx>>) -> Self {
        Processor {
            vm_ctx: Arc::clone(vm_ctx),
            // Keep for future; I'll probably forget
            // ..Self::default()
        }
    }

    pub fn start(&mut self, instructions: &[Instruction]) {
        let ctx = &mut self.vm_ctx.read().expect("VM lock is poisoned");

        for instruction in instructions {
            // TODO: Implement
        }
    }
}
