use crate::runtime::{Bytecode, Frame, Instruction, VM};
use crate::types::Object;

use Instruction::*;

#[test]
fn push_const() {
    let mut vm = VM::new();
    let bc = Bytecode::new(vec![PushConst(Object::Nil(()).ptr())]);

    vm.execute(bc).unwrap();

    let frame = vm.stack.frames().unwrap().0;

    println!("{:?}", frame.eval);
    assert!(matches!(*frame.eval[0].borrow_mut(), Object::Nil(_)));
}

#[test]
fn add() {
    let mut vm = VM::new();
    let bc = Bytecode::new(vec![
        PushConst(Object::from(1).ptr()),
        PushConst(Object::from(2).ptr()),
        Add,
    ]);

    vm.execute(bc).unwrap();

    let frame = vm.stack.frames().unwrap().0;

    if let Object::Integer(i) = *frame.eval[0].borrow() {
        assert!(i == 3)
    } else {
        panic!("not an integer")
    };
}

#[test]
fn frames() {
    let mut vm = VM::new();
    vm.stack.push(Frame::new());

    let (_, local) = vm.stack.frames().unwrap();

    assert!(!local.is_none());
    assert!(vm.stack.len() == 2);
}
