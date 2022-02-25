use crate::ast::node::Node;
use crate::ast::token::Token;
use crate::ast::Ast;

fn body(ast: Ast) -> Node {
    match ast.err {
        Some(e) => {
            panic!("{}", e)
        }
        None => ast.root.expect("ast error"),
    }
}

fn expr(ast: Ast) -> Node {
    match ast.err {
        Some(e) => {
            panic!("{}", e)
        }
        None => {
            let root = ast.root.expect("ast error");
            match root {
                Node::Body(mut stmts) => stmts.swap_remove(0),
                _ => root,
            }
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
fn function_def() {
    let code = "1
    func main() {
        return 0
    }
    ";

    assert_eq!(
        body(Ast::new(code)),
        Node::Body(vec![Node::Function {
            name: Token::ID("main"),
            params: vec![],
            body: Box::new(Node::Body(vec![Node::Return(Box::new(Node::Value(
                Token::Integer(0)
            )))]))
        }])
    )
}
