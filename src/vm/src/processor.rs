#[derive(Debug, Default)]
/// Single-threaded object running code given by the [`Vm`][crate::Vm].
pub struct Processor {
    _hi: usize,
}

impl Processor {
    #[must_use]
    /// Constructs a new [`Processor`].
    pub fn new() -> Self {
        Self::default()
    }
}
