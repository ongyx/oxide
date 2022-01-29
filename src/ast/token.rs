use logos::{Logos, Span};

fn clean_str<'a>(s: &'a str) -> &'a str {
    &s[1..s.len() - 1]
}

pub fn tokenise(code: &str) -> (Vec<Token>, Vec<Span>) {
    Token::lexer(code).spanned().unzip()
}

#[derive(Logos, Copy, Clone, Debug, PartialEq)]
pub enum Token<'a> {
    // skip whitespace and comments
    #[error]
    #[regex(r"//.*", logos::skip)]
    #[regex(r"[ \t\f]+", logos::skip)]
    Error,

    // identifier
    #[regex(r"[^\W\d][\w]*", |lex| lex.slice())]
    ID(&'a str),

    // literals
    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("nil")]
    Nil,

    // literals
    #[regex(r"\d+", |lex| lex.slice().parse())]
    Integer(i64),

    #[regex(r"\d+\.(\d+)?", |lex| lex.slice().parse())]
    Float(f64),

    #[regex(r#""[^"]*""#, |lex| clean_str(lex.slice()))]
    #[regex(r"'[^']*'", |lex| clean_str(lex.slice()))]
    Str(&'a str),

    // keywords
    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("return")]
    Return,

    #[token("for")]
    For,

    #[token("in")]
    In,

    #[token("while")]
    While,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("func")]
    Func,

    #[token("struct")]
    Struct,

    #[token("import")]
    Import,

    // operators
    #[token("=")]
    Assign,

    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("^")]
    Pow,

    // comparisons
    #[token("==")]
    Eq,

    #[token("<=")]
    Le,

    #[token("<")]
    Lt,

    #[token(">=")]
    Ge,

    #[token(">")]
    Gt,

    #[token("!=")]
    Ne,

    // boolean operators
    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("!")]
    Not,

    // delimiters
    #[token("(")]
    Lparen,

    #[token(")")]
    Rparen,

    #[token("[")]
    Lbrack,

    #[token("]")]
    Rbrack,

    #[token("{")]
    Lbrace,

    #[token("}")]
    Rbrace,

    #[token(",")]
    Comma,

    #[token("...")]
    Ellipsis,

    #[token(".")]
    Dot,

    #[token("\n")]
    Newline,
}
