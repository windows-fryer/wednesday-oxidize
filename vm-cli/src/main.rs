use vm::assembler::Assembler;
use vm::error::Error;
use vm::instructions::call::CallIndex;
use vm::instructions::Operand;
use vm::register::Width;
use vm::Vm;

fn main() -> Result<(), Error> {
    let mut vm = Vm::new();

    let compiled = Assembler::new()
        .mov(Operand::Value(0), Operand::Register(Width::QWord(0))) // mov 0, rq0
        .mov(
            Operand::Value(1_000_000_000),
            Operand::Register(Width::QWord(1)),
        ) // mov 42, rq1
        .add(
            Operand::Value(1),
            Operand::Register(Width::QWord(0)),
            Operand::Register(Width::QWord(0)),
        ) // add 1, rq0, rq0
        .cmp(
            Operand::Register(Width::QWord(0)),
            Operand::Register(Width::QWord(1)),
        ) // cmp rq0, rq1
        .jnz(Operand::Value(1)) // jnz 2
        .call(Operand::Value(CallIndex::PrintProcessor as u64)) // call 0
        .compile();

    vm.load_instructions(compiled)?;

    let handle = vm.new_processor();
    let processor = vm.processor_mut(handle)?; // fuck

    processor.start()?; // processor: yes king 🙇‍♂️

    Ok(())
}
