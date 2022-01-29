use crate::ast::node::Node;
use crate::ast::parser::Ast;
use crate::ast::token::Token;

fn expr(ast: Ast) -> Node {
    let root = ast.root.expect("ast error");
    match root {
        Node::Body(mut children) => children.swap_remove(0),
        _ => root,
    }
}

#[test]
fn array_literal() {
    assert_eq!(
        expr(Ast::new("[1, 2, 3]")),
        Node::Array(vec![
            Node::Atom(Token::Integer(1)),
            Node::Atom(Token::Integer(2)),
            Node::Atom(Token::Integer(3)),
        ])
    )
}

#[test]
fn add() {
    assert_eq!(
        expr(Ast::new("1 + 1")),
        Node::Binop(
            Token::Add,
            Box::new(Node::Atom(Token::Integer(1))),
            Box::new(Node::Atom(Token::Integer(1))),
        )
    )
}

#[test]
fn nested_add() {
    assert_eq!(
        expr(Ast::new("(1 + (2 + (3)))")),
        Node::Binop(
            Token::Add,
            Box::new(Node::Atom(Token::Integer(1))),
            Box::new(Node::Binop(
                Token::Add,
                Box::new(Node::Atom(Token::Integer(2))),
                Box::new(Node::Atom(Token::Integer(3))),
            )),
        )
    );
}

#[test]
fn assign() {
    assert_eq!(
        expr(Ast::new("a = b")),
        Node::Assign(vec![Token::ID("a")], Box::new(Node::Atom(Token::ID("b"))))
    )
}

#[test]
fn multiple_assign() {
    assert_eq!(
        expr(Ast::new("a, b = [1, 2]")),
        Node::Assign(
            vec![Token::ID("a"), Token::ID("b")],
            Box::new(Node::Array(vec![
                Node::Atom(Token::Integer(1)),
                Node::Atom(Token::Integer(2))
            ]))
        )
    )
}
