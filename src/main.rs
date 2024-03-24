mod stack;
mod vm;
use crate::vm::VM;
use crate::vm::Instruction::*;

// fibbonacci nums
fn main() {
    let mut vm = VM::new(
        vec![
            LOAD(1),
            LOAD(2),
            ADD,
            LOAD(1),
            STORE(2),
            STORE(1),
            RET,
            CALL(0),
            PUSH(1),
            LOAD(0),
            SUB,
            STORE(0),
            LOAD(0),
            NOT,
            JMPIF(16),
            CALL(7),
            RET,
            PUSH(0),
            STORE(2),
            PUSH(1),
            STORE(1),
            PUSH(9), /* num index (first one is zeroth index) */
            STORE(0),
            CALL(7),
            LOAD(1),
            OUT,
            HALT(0)
        ],
        17
    );
    vm.run();
}
