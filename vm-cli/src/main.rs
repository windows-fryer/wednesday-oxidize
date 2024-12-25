use vm::error::Error;
use vm::{assembler::Assembler, Vm};

fn main() -> Result<(), Error> {
    let mut vm = Vm::new();

    let compiled = Assembler::new().call(0).call(1).compile();

    vm.load_instructions(compiled)?;

    let handle = vm.new_processor();
    let processor = vm.processors.get_mut(&handle).unwrap(); // fuck

    processor.start()?;

    Ok(())
}
