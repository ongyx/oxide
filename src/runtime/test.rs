use crate::runtime::{Bytecode, Frame, Instruction, VM};
use crate::types::Object;

use Instruction::*;

fn local_frame(vm: &mut VM) -> &mut Frame {
    vm.stack.frames().unwrap().0
}

#[test]
fn push_const() {
    let mut vm = VM::new();
    let bc = Bytecode::new(vec![PushConst(Object::from(()).ptr())]);

    vm.execute(bc).unwrap();

    let frame = local_frame(&mut vm);

    println!("{:?}", frame.eval);
    assert!(matches!(*frame.eval[0].borrow_mut(), Object::Nil(_)));
}

#[test]
fn load_store() {
    let mut vm = VM::new();
    let mut bc = Bytecode::new(vec![
        PushConst(Object::from("Hello World!").ptr()),
        Store(0),
        Load(0),
    ]);

    bc.locals = vec!["test"].iter().map(|&s| s.into()).collect();

    vm.execute(bc).unwrap();

    let frame = local_frame(&mut vm);

    if let Object::String(s) = &*frame.eval[0].borrow() {
        assert!(s == "Hello World!")
    } else {
        panic!("not a string")
    };
}

#[test]
fn global_load_store() {
    let mut vm = VM::new();
    let bc = Bytecode::new(vec![
        PushConst(Object::from(String::from("Hello World!")).ptr()),
        StoreGlobal("test".into()),
        LoadGlobal("test".into()),
    ]);

    vm.execute(bc).unwrap();

    let frame = local_frame(&mut vm);

    if let Object::String(s) = &*frame.eval[0].borrow() {
        assert!(s == "Hello World!")
    } else {
        panic!("not a string")
    };
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

    let frame = local_frame(&mut vm);

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
