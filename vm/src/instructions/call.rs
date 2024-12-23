use crate::instructions::Execute;
use crate::processor::Processor;

#[derive(Debug, Default)]
/// Call into internal or external code to access the [`Processor`] in a mutable state.
pub struct Call {
    call_index: u64
}

impl Call {
    #[must_use]
    /// Constructs a new [`Call`].
    pub fn new(call_index: u64) -> Self {
        Call {
            call_index
        }
    }
}

impl Execute for Call {
    fn execute(&self, processor: &mut Processor) {
        println!("Call to index: {}, {:?}", self.call_index, processor);
    }
}
