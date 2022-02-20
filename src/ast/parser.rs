use logos::Span;
use peg::error::ParseError;

use crate::ast::node::Node;
use crate::ast::token::{tokenise, Token};

pub struct Ast<'a> {
    pub tokens: Vec<Token<'a>>,
    pub spans: Vec<Span>,
    pub root: Option<Node<'a>>,
    pub err: Option<ParseError<usize>>,
}

impl<'a> Ast<'a> {
    pub fn new(code: &str) -> Ast {
        let (tokens, spans) = tokenise(code);

        match oxide_parser::body(&tokens) {
            Ok(node) => Ast {
                tokens,
                spans,
                root: Some(node),
                err: None,
            },
            Err(e) => Ast {
                tokens,
                spans,
                root: None,
                err: Some(e),
            },
        }
    }
}

#[inline(always)]
fn binop<'a>(op: Token<'a>, lhs: Node<'a>, rhs: Node<'a>) -> Node<'a> {
    Node::Binop(op, Box::new(lhs), Box::new(rhs))
}

#[inline(always)]
fn unop<'a>(op: Token<'a>, rhs: Node<'a>) -> Node<'a> {
    Node::Unop(op, Box::new(rhs))
}

peg::parser! {
    grammar oxide_parser<'a>() for [Token<'a>] {

        pub rule body() -> Node<'a>
            = s:statement()* {Node::Body(s)}

        pub rule statement() -> Node<'a>
            = a:assignment() [Token::Newline]* {a}
            / e:expr() [Token::Newline]* {e}

        rule assignment() -> Node<'a>
            = t:target() [Token::Assign] e:expr() {
            Node::Assign(t, Box::new(e))
        }

        rule target() -> Vec<Token<'a>>
            = ids:([Token::ID(_)] ++ [Token::Comma]) {ids}
            / id:[Token::ID(_)] {vec![id]}

        pub rule expr() -> Node<'a> = precedence!{
            lhs:(@) t:[Token::Or] rhs:@ { binop(t, lhs, rhs) }
            --
            lhs:(@) t:[Token::And] rhs:@ { binop(t, lhs, rhs) }
            --
            t:[Token::Not] rhs:@ { unop(t, rhs) }
            --
            lhs:(@) t:[Token::Add] rhs:@ { binop(t, lhs, rhs) }
            lhs:(@) t:[Token::Sub] rhs:@ { binop(t, lhs, rhs) }
            --
            lhs:(@) t:[Token::Mul] rhs:@ { binop(t, lhs, rhs) }
            lhs:(@) t:[Token::Div] rhs:@ { binop(t, lhs, rhs) }
            --
            t:[Token::Sub] rhs:@ { unop(t, rhs) }
            --
            lhs:@ t:[Token::Pow] rhs:(@) { binop(t, lhs, rhs) }
            --
            // literal value
            v:value() { Node::Value(v) }
            // expr wrapped in parentheses
            [Token::Lparen] e:expr() [Token::Rparen] {e}
            // array literal
            [Token::Lbrack] a:(expr() ** [Token::Comma]) [Token::Rbrack] { Node::Array(a) }
        }

        rule binop() -> Token<'a> = t:[
            Token::Add
            | Token::Sub
            | Token::Mul
            | Token::Div
            | Token::Eq
            | Token::Le
            | Token::Lt
            | Token::Ge
            | Token::Gt
            | Token::Ne
            | Token::And
            | Token::Or
        ] {t}

        rule unop() -> Token<'a> = t:[Token::Not] {t}

        rule value() -> Token<'a> = literal() / t:[Token::ID(_)] {t}

        rule literal() -> Token<'a> = t:[
            Token::True
            | Token::False
            | Token::Nil
            | Token::Integer(_)
            | Token::Float(_)
            | Token::Str(_)
        ] {t}

    }
}
