use crate::ast::node::*;

fn binop<'a>(op: Op, lhs: Expression<'a>, rhs: Expression<'a>) -> Expression<'a> {
    Expression::Binop {
        op,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

fn unop<'a>(op: Op, rhs: Expression<'a>) -> Expression<'a> {
    Expression::Unop {
        op,
        rhs: Box::new(rhs),
    }
}

peg::parser! {
    pub grammar parser() for str {

        pub rule body() -> Body<'input>
            = "{" s:statements() "}" {s}
            / statements()

        rule statements() -> Body<'input>
            // Statements must end with a newline
            // (optionally followed by any more newlines and indentation)
            = __ s:(statement() ** ("\n" __)) __ {s}

        rule statement() -> Statement<'input>
            = c:compound_statement() {c}
            / s:simple_statement() {s}

        rule simple_statement() -> Statement<'input>
            = assignment()
            / e:expr() {Statement::Expr(e)}
            / break_()
            / continue_()
            / import()
            / return_()

        rule compound_statement() -> Statement<'input>
            = for_loop()
            / while_loop()
            / function()
            / if_chain()

        rule for_loop() -> Statement<'input>
            = "for" _ init:assign() _ "," _ cond:expr() _ "," _ next:augassign() _ body:body() {
                Statement::Loop{
                    init: Some(init),
                    cond,
                    next: Some(next),
                    body
                }
            }

        rule while_loop() -> Statement<'input>
            = "while" _ cond:expr() _ body:body() {
                Statement::Loop {
                    init: None,
                    cond,
                    next: None,
                    body
                }
            }

        rule function() -> Statement<'input>
            = "func" _ name:target() _ "(" _ params:targets()? _ ")" _ body:body() {
                Statement::Function {
                    name,
                    params: params.unwrap_or(vec![]),
                    body
                }
            }

        rule if_chain() -> Statement<'input>
        = if_:if_() else_if:(if_() ** (__ "else")) __ else_:else_()? {
            let mut chain = vec![if_];
            chain.extend(else_if);

            IfElse {
                if_: chain,
                else_
            }.into()
        }

        rule if_() -> If<'input>
            = "if" _ cond:expr() _ body:body() {
                If {
                    cond, body
                }
        }

        rule else_() -> Body<'input>
            = "else" _ body:body() {body}

        rule assignment() -> Statement<'input>
            = a:assign() {a.into()}
            / aa:augassign() {aa.into()}

        rule assign() -> Assign<'input>
            = targets:targets() _ "=" _ expr:expr() {
                Assign {targets, expr}
            }

        rule augassign() -> AugAssign<'input>
            = target:target() _ op:augop() "=" _ expr:expr() {
                AugAssign {target, op, expr}
            }

        rule augop() -> Op
            = "+" {Op::Add}
            / "-" {Op::Sub}
            / "*" {Op::Mul}
            / "/" {Op::Div}
            / "^" {Op::Pow}

        rule break_() -> Statement<'input>
            = "break" {Statement::Break}

        rule continue_() -> Statement<'input>
            = "continue" {Statement::Continue}

        rule import() -> Statement<'input>
            = "import" _ target:target() {Statement::Import(target)}

        rule return_() -> Statement<'input>
            = "return" _ exprs:exprs() {Statement::Return(exprs)}

        rule exprs() -> Exprs<'input>
            = expr() ++ (_ "," __)

        pub rule expr() -> Expression<'input> = precedence!{
            lhs:(@) _ "||" _ rhs:@ { binop(Op::Or, lhs, rhs) }
            --
            lhs:(@) _ "&&" _ rhs:@ { binop(Op::And, lhs, rhs) }
            --
            "!" _ rhs:@ { unop(Op::Not, rhs) }
            --
            lhs:(@) _ "==" _ rhs:@ { binop(Op::Eq, lhs, rhs) }
            lhs:(@) _ "<=" _ rhs:@ { binop(Op::Le, lhs, rhs) }
            lhs:(@) _ "<" _ rhs:@ { binop(Op::Lt, lhs, rhs) }
            lhs:(@) _ ">=" _ rhs:@ { binop(Op::Ge, lhs, rhs) }
            lhs:(@) _ ">" _ rhs:@ { binop(Op::Gt, lhs, rhs) }
            lhs:(@) _ "!=" _ rhs:@ { binop(Op::Ne, lhs, rhs) }
            --
            lhs:(@) _ "+" _ rhs:@ { binop(Op::Add, lhs, rhs) }
            lhs:(@) _ "-" _ rhs:@ { binop(Op::Sub, lhs, rhs) }
            --
            lhs:(@) _ "*" _ rhs:@ { binop(Op::Mul, lhs, rhs) }
            lhs:(@) _ "/" _ rhs:@ { binop(Op::Div, lhs, rhs) }
            --
            "-" _ rhs:@ { unop(Op::Sub, rhs) }
            --
            lhs:@ _ "^" _ rhs:(@) { binop(Op::Pow, lhs, rhs) }
            --
            // function call
            name:target() _ "(" __ args:exprs()? __ ")" { Expression::Call {name, args: args.unwrap_or(vec![])} }
            --
            // literal value
            l:literal() { Expression::Literal(l) }
            // identifier
            id:target() { Expression::Id(id) }
            // expr wrapped in parentheses
            "(" __ e:expr() __ ")" {e}
            // array literal
            "[" __ e:exprs()? __ "]" { Expression::Array(e.unwrap_or(vec![])) }
        }

        rule literal() -> Literal<'input>
            = "true" {Literal::Boolean(true)}
            / "false" {Literal::Boolean(false)}
            / "nil" {Literal::Nil}
            / i:$(numeric()+) {Literal::Integer(i.parse().unwrap())}
            / f:$(numeric()+ "." numeric()*) {Literal::Float(f.parse().unwrap())}
            / s:string_literal() {Literal::String(s)}

        rule string_literal() -> &'input str
            = "'" s:$([^'\'']*) "'" {s}
            / "\"" s:$([^'"']*) "\"" {s}

        rule targets() -> Vec<Id<'input>>
            = target() ++ (_ "," _)

        rule target() -> Id<'input>
            = !keyword() id:$(alpha() alphanumeric()*) {id}

        rule alphanumeric() = ['a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9']

        rule numeric() = ['0' ..= '9']

        rule alpha() = ['a' ..= 'z' | 'A' ..= 'Z']

        // Keywords cannot be used as identifiers.
        rule keyword() = "true" / "false" / "nil" / "for" / "while" / "func" / "if" / "else" / "break" / "continue" / "import" / "return"

        rule __ = quiet!{[' ' | '\t' | '\n']*}
        rule _ = quiet!{[' ' | '\t']*}
    }
}
