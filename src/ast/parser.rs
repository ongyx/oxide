use crate::ast::node::Node;
use crate::ast::token::Token;

#[inline(always)]
fn binop<'a>(op: Token<'a>, lhs: Node<'a>, rhs: Node<'a>) -> Node<'a> {
    Node::Binop {
        op,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

#[inline(always)]
fn unop<'a>(op: Token<'a>, rhs: Node<'a>) -> Node<'a> {
    Node::Unop {
        op,
        lhs: Box::new(rhs),
    }
}

peg::parser! {
    pub grammar oxide_parser<'a>() for [Token<'a>] {

        pub rule body() -> Node<'a>
            = [Token::Lbrace] s:statements() [Token::Rbrace] {s}
            / s:statements() ![_] {s}

        rule statements() -> Node<'a>
            = s:(statement() ++ [Token::Newline]) {Node::Body(s)}

        rule statement() -> Node<'a>
            = s:simple_statement() {s}
            / c:compound_statement() {c}

        rule simple_statement() -> Node<'a>
            = a:assignment() {a}
            / e:expr() {e}
            / r:return_() {r}

        rule compound_statement() -> Node<'a>
            = l:for_loop() {l}
            / l:while_loop() {l}
            / f:function() {f}

        rule for_loop() -> Node<'a>
            = [Token::For] init:assignment() [Token::Comma] cond:expr() [Token::Comma] next:assignment() body:body() {
                Node::Loop{
                    init: Some(Box::new(init)),
                    cond: Box::new(cond),
                    next: Some(Box::new(next)),
                    body: Box::new(body)
                }
            }

        rule while_loop() -> Node<'a>
            = [Token::While] cond:expr() body:body() {
                Node::Loop{
                    init: None,
                    cond: Box::new(cond),
                    next: None,
                    body: Box::new(body)
                }
            }

        rule function() -> Node<'a>
            = [Token::Func] name:[Token::ID(_)] [Token::Lparen] params:target()? [Token::Rparen] body:body() {
                Node::Function{
                    name,
                    params: params.unwrap_or(vec![]),
                    body: Box::new(body)
                }
            }

        rule return_() -> Node<'a>
            = [Token::Return] e:expr() {Node::Return(Box::new(e))}

        rule assignment() -> Node<'a>
            = t:target() [Token::Assign] e:expr() {
            Node::Assign{targets: t, expr: Box::new(e)}
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
