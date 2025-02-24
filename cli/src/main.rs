use wednesday_vm::Vm;
use wednesday_vm::assembler::Assembler;
use wednesday_vm::error::Error;
use wednesday_vm::instructions::Operand;
use wednesday_vm::register::Width;

fn main() -> Result<(), Error> {
    let mut vm = Vm::new();

    // Fibonacci sequence: F(93)
    // mov 0, rq0                   ; Setup counter registers
    // mov 1, rq1
    // mov rq1, rq2                 ; Move the old value of rq1 to rq2
    // add rq0, rq1, rq1            ; Add rq0 onto rq1 and store in rq1
    // mov rq2, rq0                 ; Restore the old value of rq1 into rq0
    // cmp rq1, 7540113804746346429 ; See if rq1 is at the 93rd term
    // jnz 3                        ; If not, jump to the move of rq1 to rq2

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
        .jnz(Operand::Value(2u64))
        .compile();

    vm.load_instructions(compiled)?;

    let handle = vm.new_processor();
    let processor = vm.processor_mut(handle)?; // fuck

    processor.start()?; // processor: yes king 🙇‍♂️

    println!("{:?}", std::time::Instant::now() - start);

    Ok(())
}
