use crate::ast::*;

fn body(code: &str) -> Body {
    let mut ast = Ast::new(code);
    match ast.parse() {
        Err(e) => {
            panic!("{}", ast.format(e));
        }
        _ => {
            let root = ast.root.expect("ast error");
            // println!("{:#?}", root);
            root
        }
    }
}

fn expr(code: &str) -> StmtNode {
    let mut ast = Ast::new(code);
    match ast.parse_expr() {
        Err(e) => {
            panic!("{}", ast.format(e));
        }
        _ => {
            let mut root = ast.root.expect("ast error");
            let stmt = root.swap_remove(0);
            // println!("{:#?}", stmt);
            stmt
        }
    }
}

#[test]
fn array_literal() {
    assert_eq!(
        expr("[1, 2, 3]"),
        Expression::Array(vec![literal(1), literal(2), literal(3),])
            .node()
            .into()
    )
}

#[test]
fn add() {
    assert_eq!(
        expr("1 + 1"),
        Expression::Binop {
            op: Op::Add,
            lhs: Box::new(literal(1)),
            rhs: Box::new(literal(1)),
        }
        .node()
        .into()
    )
}

#[test]
fn nested_add() {
    assert_eq!(
        expr("(1 + (2 + (3)))"),
        Expression::Binop {
            op: Op::Add,
            lhs: Box::new(literal(1)),
            rhs: Box::new(
                Expression::Binop {
                    op: Op::Add,
                    lhs: Box::new(literal(2)),
                    rhs: Box::new(literal(3)),
                }
                .node()
            ),
        }
        .node()
        .into()
    );
}

#[test]
fn assign() {
    assert_eq!(
        body("a = b"),
        vec![Statement::from(Assign {
            targets: vec!["a"],
            expr: Expression::Id("b").node()
        })
        .node()]
    )
}

#[test]
fn multiple_assign() {
    assert_eq!(
        body("a, b = [1, 2]"),
        vec![Statement::from(Assign {
            targets: vec!["a", "b"],
            expr: Expression::Array(vec![literal(1), literal(2)]).node()
        })
        .node()]
    )
}

#[test]
fn function_call() {
    assert_eq!(
        body("print('Hello World!')"),
        vec![Expression::Call {
            name: "print",
            args: vec![literal("Hello World!")]
        }
        .node()
        .into()]
    )
}

#[test]
fn function_def() {
    let code0 = "
    func main() {
        return 0
    }
    ";
    let code1 = "
    func main(...args) {
        return args
    }
    ";

    assert_eq!(
        body(code0),
        vec![Statement::Function {
            name: "main",
            params: vec![],
            varargs: None,
            body: vec![Statement::Return(literal(0).value).node()]
        }
        .node()]
    );

    assert_eq!(
        body(code1),
        vec![Statement::Function {
            name: "main",
            params: vec![],
            varargs: Some("args"),
            body: vec![Statement::Return(Expression::Id("args")).node()]
        }
        .node()]
    );
}

#[test]
fn if_chain() {
    let code0 = "
    a = 1
    if a == 1 {
        print('nice')
    } else {
        print('impossible')
    }
    ";

    let code1 = "
    if true {
        print('indeed')
    }
    ";

    assert_eq!(
        body(code0),
        vec![
            Statement::from(Assign {
                targets: vec!["a"],
                expr: literal(1)
            })
            .node(),
            Statement::from(IfElse {
                chain: vec![If {
                    cond: Expression::Binop {
                        op: Op::Eq,
                        lhs: Box::new(Expression::Id("a").node()),
                        rhs: Box::new(literal(1)),
                    }
                    .node(),
                    body: vec![Expression::Call {
                        name: "print",
                        args: vec![literal("nice")]
                    }
                    .node()
                    .into()],
                }],
                else_: Some(vec![Expression::Call {
                    name: "print",
                    args: vec![literal("impossible")]
                }
                .node()
                .into()])
            })
            .node(),
        ]
    );

    assert_eq!(
        body(code1),
        vec![Statement::from(IfElse {
            chain: vec![If {
                cond: literal(true),
                body: vec![Expression::Call {
                    name: "print",
                    args: vec![literal("indeed")]
                }
                .node()
                .into()],
            }],
            else_: None,
        })
        .node()]
    );
}

#[test]
fn for_loop() {
    let code = "
    for i = 0, i < 5, i += 1 {
        print(i)
    }
    ";

    assert_eq!(
        body(code),
        vec![Statement::Loop {
            init: Some(Assign {
                targets: vec!["i"],
                expr: literal(0),
            }),
            cond: Expression::Binop {
                op: Op::Lt,
                lhs: Box::new(Expression::Id("i").node()),
                rhs: Box::new(literal(5)),
            }
            .node(),
            next: Some(AugAssign {
                target: "i",
                op: Op::Add,
                expr: literal(1)
            }),
            body: vec![Expression::Call {
                name: "print",
                args: vec![Expression::Id("i").node()]
            }
            .node()
            .into()]
        }
        .node()]
    )
}

#[test]
fn while_loop() {
    let code = "
    while true {
        print('lol')
        break
        continue
    }
    ";

    assert_eq!(
        body(code),
        vec![Statement::Loop {
            init: None,
            cond: literal(true),
            next: None,
            body: vec![
                Expression::Call {
                    name: "print",
                    args: vec![literal("lol")]
                }
                .node()
                .into(),
                Statement::Break.node(),
                Statement::Continue.node()
            ]
        }
        .node()]
    )
}

#[test]
fn import() {
    assert_eq!(
        body("import math"),
        vec![Statement::Import {
            package: "math",
            members: None,
        }
        .node()]
    )
}

#[test]
fn import_from() {
    assert_eq!(
        body("import ceil, floor from math"),
        vec![Statement::Import {
            package: "math",
            members: Some(vec!["ceil", "floor"])
        }
        .node()]
    );
    assert_eq!(
        body("import local_module from ."),
        vec![Statement::Import {
            package: ".",
            members: Some(vec!["local_module"])
        }
        .node()]
    );
    assert_eq!(
        body("import foo, bar from .local_module"),
        vec![Statement::Import {
            package: ".local_module",
            members: Some(vec!["foo", "bar"])
        }
        .node()]
    );
    assert_eq!(
        body("import * from ."),
        vec![Statement::Import {
            package: ".",
            members: Some(vec!["*"])
        }
        .node()]
    );
}
