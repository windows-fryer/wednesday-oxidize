use vm::assembler::Assembler;
use vm::error::Error;
use vm::instructions::Operand;
use vm::register::Width;
use vm::Vm;

fn main() -> Result<(), Error> {
    let mut vm = Vm::new();

    // Fibonacci sequence :D
    // mov 0, rq0
    // mov 1, rq1
    // mov rq1, rq2
    // add rq0, rq1, rq1
    // mov rq2, rq0
    // cmp rq1, 21
    // jnz 3
    // call 0

    let start = std::time::Instant::now();

    let compiled = Assembler::new()
        .mov(Operand::Value(0), Operand::Register(Width::QWord(0)))
        .mov(Operand::Value(1), Operand::Register(Width::QWord(1)))
        .mov(
            Operand::Register(Width::QWord(1)),
            Operand::Register(Width::QWord(2)),
        )
        .add(
            Operand::Register(Width::QWord(0)),
            Operand::Register(Width::QWord(1)),
            Operand::Register(Width::QWord(1)),
        )
        .mov(
            Operand::Register(Width::QWord(2)),
            Operand::Register(Width::QWord(0)),
        )
        // .call(Operand::Value(0))
        .cmp(
            Operand::Register(Width::QWord(1)),
            Operand::Value(7540113804746346429),
        )
        .jnz(Operand::Value(1))
        .compile();

    vm.load_instructions(compiled)?;

    let handle = vm.new_processor();
    let processor = vm.processor_mut(handle)?; // fuck

    processor.start()?; // processor: yes king 🙇‍♂️

    println!("{:?}", std::time::Instant::now() - start);

    Ok(())
}
