use crate::ast::node::{Body, Node};
use crate::ast::token::Token;
use crate::ast::Ast;

fn body(ast: Ast) -> Body {
    match ast.err {
        Some(e) => {
            println!("{:?}", ast.tokens);
            panic!("{} (token: {:?})", e, ast.tokens[e.location]);
        }
        None => ast.root.expect("ast error"),
    }
}

fn expr(ast: Ast) -> Node {
    match ast.err {
        Some(e) => {
            println!("{:?}", ast.tokens);
            panic!("{} (token: {:?})", e, ast.tokens[e.location]);
        }
        None => {
            let mut root = ast.root.expect("ast error");
            root.swap_remove(0)
        }
    }
}

#[test]
fn array_literal() {
    assert_eq!(
        expr(Ast::new("[1, 2, 3]")),
        Node::Array(vec![
            Node::Value(Token::Integer(1)),
            Node::Value(Token::Integer(2)),
            Node::Value(Token::Integer(3)),
        ])
    )
}

#[test]
fn add() {
    assert_eq!(
        expr(Ast::new("1 + 1")),
        Node::Binop {
            op: Token::Add,
            lhs: Box::new(Node::Value(Token::Integer(1))),
            rhs: Box::new(Node::Value(Token::Integer(1))),
        }
    )
}

#[test]
fn nested_add() {
    assert_eq!(
        expr(Ast::new("(1 + (2 + (3)))")),
        Node::Binop {
            op: Token::Add,
            lhs: Box::new(Node::Value(Token::Integer(1))),
            rhs: Box::new(Node::Binop {
                op: Token::Add,
                lhs: Box::new(Node::Value(Token::Integer(2))),
                rhs: Box::new(Node::Value(Token::Integer(3))),
            }),
        }
    );
}

#[test]
fn assign() {
    assert_eq!(
        expr(Ast::new("a = b")),
        Node::Assign {
            targets: vec![Token::ID("a")],
            expr: Box::new(Node::Value(Token::ID("b")))
        }
    )
}

#[test]
fn multiple_assign() {
    assert_eq!(
        expr(Ast::new("a, b = [1, 2]")),
        Node::Assign {
            targets: vec![Token::ID("a"), Token::ID("b")],
            expr: Box::new(Node::Array(vec![
                Node::Value(Token::Integer(1)),
                Node::Value(Token::Integer(2))
            ]))
        }
    )
}

#[test]
fn function_call() {
    assert_eq!(
        body(Ast::new("print('Hello World!')")),
        vec![Node::Call {
            name: Token::ID("print"),
            args: vec![Node::Value(Token::Str("Hello World!"))]
        }]
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
        body(Ast::new(code)),
        vec![Node::Function {
            name: Token::ID("main"),
            params: vec![],
            body: vec![Node::Return(Box::new(Node::Value(Token::Integer(0))))]
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
        body(Ast::new(code)),
        vec![
            Node::Assign {
                targets: vec![Token::ID("a")],
                expr: Box::new(Node::Value(Token::Integer(1)))
            },
            Node::If {
                if_: vec![(
                    Box::new(Node::Binop {
                        op: Token::Eq,
                        lhs: Box::new(Node::Value(Token::ID("a"))),
                        rhs: Box::new(Node::Value(Token::Integer(1))),
                    }),
                    vec![Node::Call {
                        name: Token::ID("print"),
                        args: vec![Node::Value(Token::Str("nice"))]
                    }]
                )],
                else_: Some(vec![Node::Call {
                    name: Token::ID("print"),
                    args: vec![Node::Value(Token::Str("impossible"))]
                }])
            }
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
        body(Ast::new(code)),
        vec![Node::Loop {
            init: Some(Box::new(Node::Assign {
                targets: vec![Token::ID("i")],
                expr: Box::new(Node::Value(Token::Integer(0)))
            })),
            cond: Box::new(Node::Binop {
                op: Token::Lt,
                lhs: Box::new(Node::Value(Token::ID("i"))),
                rhs: Box::new(Node::Value(Token::Integer(5)))
            }),
            next: Some(Box::new(Node::AugAssign {
                target: Token::ID("i"),
                op: Token::Add,
                expr: Box::new(Node::Value(Token::Integer(1)))
            })),
            body: vec![Node::Call {
                name: Token::ID("print"),
                args: vec![Node::Value(Token::ID("i"))]
            }]
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
        body(Ast::new(code)),
        vec![Node::Loop {
            init: None,
            cond: Box::new(Node::Value(Token::True)),
            next: None,
            body: vec![
                Node::Call {
                    name: Token::ID("print"),
                    args: vec![Node::Value(Token::Str("lol"))]
                },
                Node::Keyword(Token::Break),
                Node::Keyword(Token::Continue),
            ]
        }]
    )
}
