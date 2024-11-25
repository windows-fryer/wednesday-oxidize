use crate::instructions::Execute;
use crate::processor::Processor;

#[derive(Debug, Default)]
/// Modify the [`Processor`] in a non-mutable state.
pub struct NCall {}

impl Execute for NCall {
    fn new() -> Self {
        NCall::default()
    }

    fn execute(&self, processor: &Processor) {
        todo!()
    }

    fn execute_mut(&self, processor: &mut Processor) {
        todo!()
    }
}
