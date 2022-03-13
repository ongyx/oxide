use crate::ovec;
use crate::runtime::{Bytecode, Frame, Instruction, VM};
use crate::types::{Nil, Object};

use Instruction::*;

fn local_frame(vm: &mut VM) -> &mut Frame {
    vm.stack.frames().unwrap().0
}

#[test]
fn push_const() {
    let mut vm = VM::new();
    let mut bc = Bytecode::new(vec![PushConst(0)]);

    bc.consts = ovec![Nil];

    vm.execute(&mut bc).unwrap();

    let frame = local_frame(&mut vm);

    println!("{:?}", frame.eval);
    assert!(matches!(*frame.eval[0].borrow_mut(), Object::Nil(_)));
}

#[test]
fn load_store() {
    let mut vm = VM::new();
    let mut bc = Bytecode::new(vec![PushConst(0), Store(0), Load(0)]);

    bc.locals = vec!["test"].iter().map(|&s| s.into()).collect();
    bc.consts = ovec!["Hello World!"];

    vm.execute(&mut bc).unwrap();

    let frame = local_frame(&mut vm);

    if let Object::String(s) = &*frame.eval[0].borrow() {
        assert_eq!(s, "Hello World!")
    } else {
        panic!("not a string")
    };
}

#[test]
fn global_load_store() {
    let mut vm = VM::new();
    let mut bc = Bytecode::new(vec![
        PushConst(0),
        StoreGlobal("test".into()),
        LoadGlobal("test".into()),
    ]);

    bc.consts = ovec!["Hello World!"];

    vm.execute(&mut bc).unwrap();

    let frame = local_frame(&mut vm);

    if let Object::String(s) = &*frame.eval[0].borrow() {
        assert_eq!(s, "Hello World!")
    } else {
        panic!("not a string")
    };
}

#[test]
fn add() {
    let mut vm = VM::new();
    let mut bc = Bytecode::new(vec![PushConst(0), PushConst(1), Add]);

    bc.consts = ovec![1, 2];

    vm.execute(&mut bc).unwrap();

    let frame = local_frame(&mut vm);

    if let Object::Integer(i) = *frame.eval[0].borrow() {
        assert_eq!(i, 3)
    } else {
        panic!("not an integer")
    };
}

#[test]
fn and() {
    let mut vm = VM::new();
    let mut bc = Bytecode::new(vec![PushConst(0), PushConst(1), And]);

    bc.consts = ovec![true, false];

    vm.execute(&mut bc).unwrap();

    let frame = local_frame(&mut vm);

    if let Object::Boolean(b) = *frame.eval[0].borrow() {
        assert!(!b)
    } else {
        panic!("not a boolean")
    };
}

#[test]
fn jump() {
    let mut vm = VM::new();
    let mut bc = Bytecode::new(vec![PushConst(0), JumpIf(3), PushConst(1), Nop]);

    bc.consts = ovec![true, Nil];

    vm.execute(&mut bc).unwrap();

    assert_eq!(bc.counter, 4);
}

#[test]
fn frames() {
    let mut vm = VM::new();
    vm.stack.push(Frame::new());

    let (_, local) = vm.stack.frames().unwrap();

    assert!(!local.is_none());
    assert_eq!(vm.stack.len(), 2);
}
