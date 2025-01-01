use vm::assembler::Assembler;
use vm::error::Error;
use vm::instructions::call::CallIndex;
use vm::instructions::Operand;
use vm::register::Width;
use vm::Vm;

fn main() -> Result<(), Error> {
    let mut vm = Vm::new();

    let compiled = Assembler::new()
        .mov(Operand::Value(257), Operand::Register(Width::Word(1)))
        .mov(
            Operand::Register(Width::Word(1)),
            Operand::Register(Width::Byte(0)),
        )
        .call(Operand::Value(CallIndex::PrintProcessor as u64))
        .compile();

    vm.load_instructions(compiled)?;

    let handle = vm.new_processor();
    let processor = vm.processor_mut(handle)?; // fuck

    processor.start()?; // processor: yes king 🙇‍♂️

    Ok(())
}
