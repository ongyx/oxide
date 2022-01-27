use crate::token::Token;

peg::parser! {
    grammar parser<'a>() for [Token<'a>] {
        rule literal() = [
            Token::True
            | Token::False
            | Token::Nil
            | Token::Integer(_)
            | Token::Float(_)
            | Token::Str(_)
        ]
        rule value() = literal() / [Token::ID]
        rule binop() = [
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
        ]
        rule unop() = [Token::Not]
    }
}
