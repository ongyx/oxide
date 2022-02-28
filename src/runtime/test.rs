use crate::runtime::{Bytecode, Opcode, Value, VM};

#[test]
fn push_const() {
    let mut vm = VM::new();
    let result = vm.execute(vec![Bytecode {
        op: Opcode::PushConst,
        args: vec![Value::Nil(())],
    }]);

    match result {
        Ok(_) => {}
        Err(e) => panic!("{:?}", e),
    }

    let frame = vm.stack.current_frame().expect("no global stack frame");

    println!("{:?}", frame.eval);
    assert!(matches!(frame.eval[0], Value::Nil(_)))
}
