mod lexer;

#[derive(Debug, Default)]
/// Basic type for language operations
pub struct Interpreter {}

impl Interpreter {
    #[must_use]
    /// Creates a new [`Interpreter`]
    fn _new() -> Self {
        Self::default()
    }
}
