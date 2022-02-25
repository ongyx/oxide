use crate::ast::node::{Body, BoxedNode, Exprs, Node};
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
        rhs: Box::new(rhs),
    }
}

peg::parser! {
    pub grammar oxide_parser<'a>() for [Token<'a>] {

        pub rule body() -> Body<'a>
            = [Token::Lbrace] _ s:statements() _ [Token::Rbrace] {s}
            / _ s:statements() _ ![_] {s}

        rule statements() -> Body<'a>
            = s:(statement() ++ [Token::Newline]) {s}

        rule statement() -> Node<'a>
            = s:simple_statement() {s}
            / c:compound_statement() {c}

        rule simple_statement() -> Node<'a>
            = a:assignment() {a}
            / e:expr() {e}
            / k:keyword() {k}
            / r:return_() {r}

        rule compound_statement() -> Node<'a>
            = i:if_chain() {i}
            / l:for_loop() {l}
            / l:while_loop() {l}
            / f:function() {f}

        rule for_loop() -> Node<'a>
            = [Token::For] init:assignment() [Token::Comma] cond:expr() [Token::Comma] next:assignment() body:body() {
                Node::Loop{
                    init: Some(Box::new(init)),
                    cond: Box::new(cond),
                    next: Some(Box::new(next)),
                    body
                }
            }

        rule while_loop() -> Node<'a>
            = [Token::While] cond:expr() body:body() {
                Node::Loop{
                    init: None,
                    cond: Box::new(cond),
                    next: None,
                    body
                }
            }

        rule function() -> Node<'a>
            = [Token::Func] name:[Token::ID(_)] [Token::Lparen] params:targets()? [Token::Rparen] body:body() {
                Node::Function{
                    name,
                    params: params.unwrap_or(vec![]),
                    body
                }
            }

        rule if_chain() -> Node<'a>
        = if_:if_() else_if:(if_() ** [Token::Else]) else_:else_()? {

            let mut chain = vec![if_];
            chain.extend(else_if);

            Node::If{
                if_: chain,
                else_
            }
        }

        rule if_() -> (BoxedNode<'a>, Body<'a>)
            = [Token::If] cond:expr() body:body() {
                (Box::new(cond), body)
        }

        rule else_() -> Body<'a>
            = [Token::Else] body:body() {body}

        rule keyword() -> Node<'a>
            = t:[Token::Break | Token::Continue] {
            Node::Keyword(t)
        }

        rule return_() -> Node<'a>
            = [Token::Return] e:expr() {Node::Return(Box::new(e))}

        rule assignment() -> Node<'a>
            = target:target() op:arithop() [Token::Assign] e:expr() {
            Node::AugAssign{target, op, expr: Box::new(e)}
        }
            / targets:targets() [Token::Assign] e:expr() {
            Node::Assign{targets, expr: Box::new(e)}
        }

        pub rule expr() -> Node<'a> = precedence!{
            lhs:(@) t:[Token::Or] rhs:@ { binop(t, lhs, rhs) }
            --
            lhs:(@) t:[Token::And] rhs:@ { binop(t, lhs, rhs) }
            --
            t:[Token::Not] rhs:@ { unop(t, rhs) }
            --
            lhs:(@) t:[Token::Eq] rhs:@ { binop(t, lhs, rhs) }
            lhs:(@) t:[Token::Le] rhs:@ { binop(t, lhs, rhs) }
            lhs:(@) t:[Token::Lt] rhs:@ { binop(t, lhs, rhs) }
            lhs:(@) t:[Token::Ge] rhs:@ { binop(t, lhs, rhs) }
            lhs:(@) t:[Token::Gt] rhs:@ { binop(t, lhs, rhs) }
            lhs:(@) t:[Token::Ne] rhs:@ { binop(t, lhs, rhs) }
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
            // function call
            name:target() [Token::Lparen] args:args() [Token::Rparen] { Node::Call {name, args} }
            --
            // literal value
            v:value() { Node::Value(v) }
            // expr wrapped in parentheses
            [Token::Lparen] e:expr() [Token::Rparen] {e}
            // array literal
            [Token::Lbrack] a:args() [Token::Rbrack] { Node::Array(a) }
        }

        rule args() -> Exprs<'a>
            = a:(expr() ** [Token::Comma]) {a}

        rule binop() -> Token<'a>
            = a:arithop() {a}
            / t:[
                Token::Eq
                | Token::Le
                | Token::Lt
                | Token::Ge
                | Token::Gt
                | Token::Ne
                | Token::And
                | Token::Or
            ] {t}

        rule arithop() -> Token<'a> = t:[
            Token::Add
            | Token::Sub
            | Token::Mul
            | Token::Div
            | Token::Pow
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

        rule targets() -> Vec<Token<'a>>
            = ids:(target() ++ [Token::Comma]) {ids}
            / id:target() {vec![id]}

        rule target() -> Token<'a>
            = id:[Token::ID(_)] {id}

        rule _ = quiet!{[Token::Newline]*}
    }
}
