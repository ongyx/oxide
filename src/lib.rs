pub mod parser;
pub mod token;

#[cfg(test)]
mod test {
    use crate::parser::{oxide_parser, Node};
    use crate::token::{tokenise, Token};

    #[test]
    fn add() {
        let (tokens, _) = tokenise("1 + 1");
        assert_eq!(
            oxide_parser::expr(&tokens),
            Ok(Node::Binop(
                Token::Add,
                Box::new(Node::Atom(Token::Integer(1))),
                Box::new(Node::Atom(Token::Integer(1))),
            ))
        )
    }

    #[test]
    fn nested_add() {
        let (tokens, _) = tokenise("(1 + (2 + (3)))");
        assert_eq!(
            oxide_parser::expr(&tokens),
            Ok(Node::Binop(
                Token::Add,
                Box::new(Node::Atom(Token::Integer(1))),
                Box::new(Node::Binop(
                    Token::Add,
                    Box::new(Node::Atom(Token::Integer(2))),
                    Box::new(Node::Atom(Token::Integer(3))),
                )),
            ))
        );
    }
}
