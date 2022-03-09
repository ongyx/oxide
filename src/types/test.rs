use crate::types::Object;

#[test]
fn integer_add() {
    let a: i64 = 1;
    let b: i64 = 2;
    let ao = Object::from(a).ptr();
    let bo = Object::from(b).ptr();

    let add_fn = ao.type_.add.expect("add not implemented");
    let co = add_fn(ao, bo).expect("add error");

    match &*co.borrow() {
        Object::Integer(i) => assert!(*i == 3),
        _ => panic!("not an integer"),
    };
}
