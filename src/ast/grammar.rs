use std::fmt;

use crate::ast::*;

fn binop<'a>(op: Op, lhs: ExprNode<'a>, rhs: ExprNode<'a>) -> Expression<'a> {
    Expression::Binop {
        op,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

fn unop<'a>(op: Op, rhs: ExprNode<'a>) -> Expression<'a> {
    Expression::Unop {
        op,
        rhs: Box::new(rhs),
    }
}

peg::parser! {
    pub grammar parser() for str {
        pub rule file() -> Body<'input>
            = traced(<_file()>)

        // pegviz
        rule traced<T>(e: rule<T>) -> T =
            &(input:$([_]*) {
                #[cfg(feature = "trace")]
                println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]", input);
            })
            e:e()? {?
                #[cfg(feature = "trace")]
                println!("[PEG_TRACE_STOP]");
                e.ok_or("")
            }

        rule _file() -> Body<'input>
            = ___ s:statements() ___ {s}

        rule body() -> Body<'input>
            = "{" ___ s:statements() ___ "}" {s}

        rule statements() -> Body<'input>
            = l:line() ++ (_ "\n") {l.into_iter().flatten().collect()}
            // a single statement by itself
            / l:line() ![_] {
                if let Some(stmt) = l {
                    vec![stmt]
                } else {
                    Vec::new()
                }
            }

        // A line that might have a statement and/or a comment.
        rule line() -> Option<StmtNode<'input>>
            = _ s:statement()? _ comment()? {s}

        pub rule statement() -> StmtNode<'input>
            = compound_statement() / simple_statement()

        rule simple_statement() -> StmtNode<'input>
            = node(<assignment()>)
            / e:expr() {e.into()}
            / node(<break_()>)
            / node(<continue_()>)
            / node(<import()>)
            / node(<return_()>)

        rule compound_statement() -> StmtNode<'input>
            = node(<function()>)
            / node(<struct_()>)
            / node(<if_else()>)
            / node(<for_loop()>)
            / node(<for_in_loop()>)
            / node(<while_loop()>)

        rule function() -> Statement<'input>
            = "func" __ name:target() _ "(" _ params:targets()? _ varargs:varargs()? _ ")" _ body:body() {
                Statement::Function {
                    name,
                    params: params.unwrap_or(vec![]),
                    varargs,
                    body
                }
            }

        rule varargs() -> Id<'input> = "..." t:target() {t}

        rule struct_() -> Statement<'input>
            = "struct" __ name:target() _ "{" ___ fields:targets() ___ "}" {
                Statement::Struct { name, fields }
            }

        rule if_else() -> Statement<'input>
            = if_:if_() _ else_if:(else_if() ** _) else_:else_()? {
                let mut chain = vec![if_];
                chain.extend(else_if);

                IfElse { chain, else_ }.into()
            }

        rule else_if() -> If<'input>
            = "else" __ i:if_() {i}

        rule if_() -> If<'input>
            = "if" __ cond:expr() _ body:body() {
                If {
                    cond,
                    body,
                }
            }

        rule else_() -> Body<'input>
            = "else" __ body:body() { body }

        rule for_loop() -> Statement<'input>
            = "for" __ init:assign() _ "," _ cond:expr() _ "," _ next:augassign() _ body:body() {
                Statement::Loop{
                    init: Some(init),
                    cond,
                    next: Some(next),
                    body
                }
            }

        rule for_in_loop() -> Statement<'input>
            = "for" __ targets:targets() __ "in" __ value:expr() _ body:body() {
                Statement::Iter {
                    targets, value, body
                }
            }

        rule while_loop() -> Statement<'input>
            = "while" __ cond:expr() _ body:body() {
                Statement::Loop {
                    init: None,
                    cond,
                    next: None,
                    body
                }
            }

        rule assignment() -> Statement<'input>
            = a:assign() {a.into()}
            / aa:augassign() {aa.into()}

        rule assign() -> Assign<'input>
            = targets:identifiers() _ "=" _ expr:expr() {
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
            = "import" __ "*" __ "from" __ package:path() {
                Statement::Import {package, members: Some(vec!["*"])}
            }
            / "import" __ members:targets() __ "from" __ package:path() {
                Statement::Import {package, members: Some(members)}
            }
            / "import" __ package:path() {
                Statement::Import {package, members: None}
            }

        rule path() -> Id<'input> = $($("."*) identifier()) / $("."+)

        rule return_() -> Statement<'input>
            = "return" __ expr:expr() {Statement::Return(expr.value)}

        rule exprs() -> Exprs<'input>
            = expr() ++ (_ "," ___)

        pub rule expr() -> ExprNode<'input> = precedence!{
            start:position!() e:@ end:position!() {
                Node::new(e, Span(start, end))
            }
            --
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
            name:identifier() "(" ___ args:exprs()? ___ ")" { Expression::Call {name, args: args.unwrap_or(vec![])} }
            // subscript
            value:(@) "[" ___ by:expr() ___ "]" { Expression::Subscript {value: Box::new(value), by: Box::new(by)} }
            --
            // literal value
            l:literal() { Expression::Literal(l) }
            // identifier
            id:identifier() { Expression::Id(id) }
            // expr wrapped in parentheses
            "(" ___ e:expr() ___ ")" {e.value}
            // array literal
            "[" ___ e:exprs()? ___ "]" { Expression::Array(e.unwrap_or(vec![])) }
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

        rule identifiers() -> Vec<Id<'input>>
            = identifier() ++ (_ "," _)

        rule identifier() -> Id<'input>
            = $(target() ++ ".")

        rule targets() -> Vec<Id<'input>>
            = target() ++ (_ "," _)

        rule target() -> Id<'input>
            = quiet!{ !keyword() id:$(alpha() alphanumeric()*) {id} } / expected!("identifier")

        rule alphanumeric() = ['a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_']

        rule numeric() = ['0' ..= '9']

        rule alpha() = ['a' ..= 'z' | 'A' ..= 'Z' | '_']

        // Keywords cannot be used as identifiers.
        rule keyword() = "true" / "false" / "nil" / "for" / "while" / "break" / "continue" / "func" / "return" / "if" / "else" / "import" / "from"

        rule comment() = quiet!{ "//" $([^'\n']*) }

        rule node<T: fmt::Debug + PartialEq>(r: rule<T>) -> Node<T>
            = start:position!() r:r() end:position!() {
                Node::new(r, Span(start, end))
            }

        rule ___ = quiet!{[' ' | '\t' | '\n']*}
        rule __ = quiet!{[' ' | '\t']+}
        rule _ = quiet!{[' ' | '\t']*}

    }
}
