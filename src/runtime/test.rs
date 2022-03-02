use crate::runtime::{Bytecode, Instruction, Value, VM};
use Instruction::*;

#[test]
fn push_const() {
    let mut vm = VM::new();
    let bc = Bytecode::new(vec![PushConst(Value::Nil(()))]);

    let result = vm.execute(bc);

    match result {
        Ok(_) => {}
        Err(e) => panic!("{:?}", e),
    }

    let frame = vm.stack.current_frame().expect("no global stack frame");

    println!("{:?}", frame.eval);
    assert!(matches!(frame.eval[0], Value::Nil(_)))
}
