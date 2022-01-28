use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    Atom(Token<'a>),
    Binop(Token<'a>, Box<Node<'a>>, Box<Node<'a>>),
    Unop(Token<'a>, Box<Node<'a>>),
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
    pub grammar oxide_parser<'a>() for [Token<'a>] {

        rule literal() -> Token<'a> = t:[
            Token::True
            | Token::False
            | Token::Nil
            | Token::Integer(_)
            | Token::Float(_)
            | Token::Str(_)
        ] {t}

        rule value() -> Token<'a> = literal() / t:[Token::ID] {t}

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
            t:[Token::Sub] rhs:@ {unop(t, rhs)}
            --
            lhs:@ t:[Token::Pow] rhs:(@) { binop(t, lhs, rhs) }
            --
            v:value() { Node::Atom(v) }
            [Token::Lparen] e:expr() [Token::Rparen] {e}
        }
    }
}
