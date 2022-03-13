use peg::error::ParseError;
use peg::str::LineCol;

use crate::ast::node::*;
use crate::ast::Ast;

fn error_msg(code: &str, e: ParseError<LineCol>) -> String {
    let code_line = code.lines().nth(e.location.line - 1).unwrap();

    format!(
        "
    error at line {line}:{column}
    {code}
    {ptr:>hint$}
    expected one of {tokens}
    ",
        line = e.location.line,
        column = e.location.column,
        code = code_line,
        ptr = "^",
        hint = e.location.column,
        tokens = e.expected
    )
}

fn body(code: &str) -> Body {
    let mut ast = Ast::new(code);
    match ast.parse() {
        Err(e) => {
            panic!("{}", error_msg(code, e));
        }
        _ => ast.root.expect("ast error"),
    }
}

fn expr(code: &str) -> Statement {
    let mut ast = Ast::new(code);
    match ast.parse_expr() {
        Err(e) => {
            panic!("{}", error_msg(code, e));
        }
        _ => {
            let mut root = ast.root.expect("ast error");
            root.swap_remove(0)
        }
    }
}

#[test]
fn array_literal() {
    assert_eq!(
        expr("[1, 2, 3]"),
        Statement::Expr(Expression::Array(vec![
            Literal::Integer(1).into(),
            Literal::Integer(2).into(),
            Literal::Integer(3).into(),
        ]))
    )
}

#[test]
fn add() {
    assert_eq!(
        expr("1 + 1"),
        Expression::Binop {
            op: Op::Add,
            lhs: Box::new(Literal::Integer(1).into()),
            rhs: Box::new(Literal::Integer(1).into()),
        }
        .into()
    )
}

#[test]
fn nested_add() {
    assert_eq!(
        expr("(1 + (2 + (3)))"),
        Expression::Binop {
            op: Op::Add,
            lhs: Box::new(Literal::Integer(1).into()),
            rhs: Box::new(Expression::Binop {
                op: Op::Add,
                lhs: Box::new(Literal::Integer(2).into()),
                rhs: Box::new(Literal::Integer(3).into()),
            }),
        }
        .into()
    );
}

#[test]
fn assign() {
    assert_eq!(
        body("a = b"),
        vec![Assign {
            targets: vec!["a"],
            expr: Expression::Id("b")
        }
        .into()]
    )
}

#[test]
fn multiple_assign() {
    assert_eq!(
        body("a, b = [1, 2]"),
        vec![Assign {
            targets: vec!["a", "b"],
            expr: Expression::Array(vec![Literal::Integer(1).into(), Literal::Integer(2).into()])
        }
        .into()]
    )
}

#[test]
fn function_call() {
    assert_eq!(
        body("print('Hello World!')"),
        vec![Expression::Call {
            name: "print",
            args: vec![Literal::String("Hello World!").into()]
        }
        .into()]
    )
}

#[test]
fn function_def() {
    let code = "
    func main() {
        return 0
    }
    ";

    assert_eq!(
        body(code),
        vec![Statement::Function {
            name: "main",
            params: vec![],
            body: vec![Statement::Return(vec![Literal::Integer(0).into()])]
        }]
    )
}

#[test]
fn if_chain() {
    let code = "
    a = 1
    if a == 1 {
        print('nice')
    } else {
        print('impossible')
    }
    ";

    assert_eq!(
        body(code),
        vec![
            Assign {
                targets: vec!["a"],
                expr: Literal::Integer(1).into()
            }
            .into(),
            IfElse {
                if_: vec![If {
                    cond: Expression::Binop {
                        op: Op::Eq,
                        lhs: Box::new(Expression::Id("a")),
                        rhs: Box::new(Literal::Integer(1).into()),
                    },
                    body: vec![Expression::Call {
                        name: "print",
                        args: vec![Literal::String("nice").into()]
                    }
                    .into()]
                }],
                else_: Some(vec![Expression::Call {
                    name: "print",
                    args: vec![Literal::String("impossible").into()]
                }
                .into()])
            }
            .into(),
        ]
    )
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
            init: Some(
                Assign {
                    targets: vec!["i"],
                    expr: Literal::Integer(0).into(),
                }
                .into()
            ),
            cond: Expression::Binop {
                op: Op::Lt,
                lhs: Box::new(Expression::Id("i")),
                rhs: Box::new(Literal::Integer(5).into()),
            },
            next: Some(
                AugAssign {
                    target: "i",
                    op: Op::Add,
                    expr: Literal::Integer(1).into()
                }
                .into()
            ),
            body: vec![Expression::Call {
                name: "print",
                args: vec![Expression::Id("i")]
            }
            .into()]
        }]
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
            cond: Literal::Boolean(true).into(),
            next: None,
            body: vec![
                Expression::Call {
                    name: "print",
                    args: vec![Literal::String("lol").into()]
                }
                .into(),
                Statement::Break,
                Statement::Continue
            ]
        }]
    )
}
