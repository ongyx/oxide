use std::{env, fs};

mod parser;
mod token;

use crate::token::Token;
use logos::Logos;

fn main() {
    let args: Vec<String> = env::args().collect();

    // open file and parse code
    let code = fs::read_to_string(&args[1]).expect("couldn't read file");

    let mut lex = Token::lexer(code.as_str());

    while let Some(t) = lex.next() {
        println!("{:?} {:?} {:?}", t, lex.slice(), lex.span());
    }
}
