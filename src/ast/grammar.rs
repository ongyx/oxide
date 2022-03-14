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
            = __ s:statements() __ {s}

        rule body() -> Body<'input>
            = "{" __ s:statements() __ "}" {s}

        rule statements() -> Body<'input>
            // Statements must end with a newline
            // (optionally followed by any more newlines and indentation)
            = s:(statement() ** __) {s}

        pub rule statement() -> Statement<'input>
            = compound_statement()
            / simple_statement()

        rule simple_statement() -> Statement<'input>
            = assignment()
            / e:expr() {Statement::Expr(e)}
            / break_()
            / continue_()
            / import()
            / return_()

        rule compound_statement() -> Statement<'input>
            = function()
            / struct_()
            / if_else()
            / for_loop()
            / for_in_loop()
            / while_loop()

        rule function() -> Statement<'input>
            = "func" _ name:target() _ "(" _ params:targets()? _ varargs:varargs()? _ ")" _ body:body() {
                Statement::Function {
                    name,
                    params: params.unwrap_or(vec![]),
                    varargs,
                    body
                }
            }

        rule varargs() -> Id<'input> = "..." t:target() {t}

        rule struct_() -> Statement<'input>
            = "struct" _ name:target() _ "{" __ fields:targets() __ "}" {
                Statement::Struct { name, fields }
            }

        rule if_else() -> Statement<'input>
            = if_:if_() __ else_if:(if_() ** ("else" _)) __ else_:else_()? {
                let mut chain = Vec::new();
                chain.push(if_);
                chain.extend(else_if);

                IfElse { chain, else_ }.into()
            }

        rule if_() -> If<'input>
            = "if" _ cond:expr() _ body:body() {
                If {
                    cond,
                    body,
                }
            }

        rule else_() -> Body<'input>
            = "else" _ body:body() { body }

        rule for_loop() -> Statement<'input>
            = "for" _ init:assign() _ "," _ cond:expr() _ "," _ next:augassign() _ body:body() {
                Statement::Loop{
                    init: Some(init),
                    cond,
                    next: Some(next),
                    body
                }
            }

        rule for_in_loop() -> Statement<'input>
            = "for" _ targets:targets() _ "in" _ value:expr() _ body:body() {
                Statement::Iter {
                    targets, value, body
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
            = "import" _ "*" _ "from" _ package:path() {
                Statement::Import {package, members: Some(vec!["*"])}
            }
            / "import" _ members:targets() _ "from" _ package:path() {
                Statement::Import {package, members: Some(members)}
            }
            / "import" _ package:path() {
                Statement::Import {package, members: None}
            }

        rule path() -> Id<'input> = $($("."*) identifier()) / $("."+)

        rule return_() -> Statement<'input>
            = "return" _ expr:expr() {Statement::Return(expr)}

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
            name:identifier() _ "(" __ args:exprs()? __ ")" { Expression::Call {name, args: args.unwrap_or(vec![])} }
            // subscript
            value:(@) __ "[" by:expr() "]" __ { Expression::Subscript {value: Box::new(value), by: Box::new(by)} }
            --
            // literal value
            l:literal() { Expression::Literal(l) }
            // identifier
            id:identifier() { Expression::Id(id) }
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

        rule __ = quiet!{([' ' | '\t' | '\n'] / comment())*}
        rule _ = quiet!{[' ' | '\t']*}

        rule comment() = quiet!{ "//" $([^'\n']*) }
    }
}
