use vm::assembler::Assembler;
use vm::error::Error;
use vm::instructions::Operand;
use vm::register::ReservedIndex;
use vm::Vm;

fn main() -> Result<(), Error> {
    let mut vm = Vm::new();

    let compiled = Assembler::new()
        .call(Operand::Immediate(0))
        .mov(
            Operand::Immediate(2),
            Operand::Register(ReservedIndex::InstructionCounter as u64),
        )
        .mov(Operand::Immediate(1), Operand::Register(0))
        .call(Operand::Immediate(0))
        .compile();

    vm.load_instructions(compiled)?;

    let handle = vm.new_processor();
    let processor = vm.processor_mut(handle)?; // fuck

    processor.start()?; // processor: yes king 🙇‍♂️

    Ok(())
}
