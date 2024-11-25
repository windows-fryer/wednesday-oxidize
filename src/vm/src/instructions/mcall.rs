use crate::instructions::Execute;
use crate::processor::Processor;

#[derive(Debug, Default)]
/// Modify the [`Processor`] in a mutable state.
pub struct MCall {}

impl Execute for MCall {
    fn new() -> Self {
        MCall::default()
    }

    fn execute(&self, processor: &Processor) {
        todo!()
    }

    fn execute_mut(&self, processor: &mut Processor) {
        todo!()
    }
}
