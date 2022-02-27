use crate::runtime::{Bytecode, Opcode, VM};
use crate::types::Value;

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

    println!("{:?}", vm.stack);
    assert!(matches!(vm.stack[0], Value::Nil(_)))
}
