use vm::assembler::Assembler;
use vm::error::Error;
use vm::instructions::call::CallIndex;
use vm::instructions::Operand;
use vm::register::Width;
use vm::Vm;

fn main() -> Result<(), Error> {
    let mut vm = Vm::new();

    let compiled = Assembler::new()
        .mov(Operand::Value(1), Operand::Memory(Width::Byte(1)))
        .mov(Operand::Value(2), Operand::Memory(Width::Byte(2)))
        .jmp(Operand::Value(4))
        .mov(Operand::Value(3), Operand::Memory(Width::Byte(3)))
        .mov(Operand::Value(4), Operand::Memory(Width::Byte(4)))
        .mov(Operand::Value(5), Operand::Memory(Width::Byte(5)))
        .call(Operand::Value(CallIndex::PrintProcessor as u64))
        .compile();

    vm.load_instructions(compiled)?;

    let handle = vm.new_processor();
    let processor = vm.processor_mut(handle)?; // fuck

    processor.start()?; // processor: yes king 🙇‍♂️

    Ok(())
}
