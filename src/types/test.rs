use crate::types::{Object, Type};

#[test]
fn integer_add() {
    let a: i64 = 1;
    let b: i64 = 2;
    let ao = Object::from(a).ptr();
    let bo = Object::from(b).ptr();

    let co = Type::add(&a, ao, bo).unwrap();

    match &*co.borrow() {
        Object::Integer(i) => assert!(*i == 3),
        _ => panic!("not an integer"),
    };
}
