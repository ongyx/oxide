use std::{env, fs};

use std::iter::zip;

use oxide::parser::oxide_parser;
use oxide::token::tokenise;

fn main() {
    let args: Vec<String> = env::args().collect();

    // open file and parse code
    let code = fs::read_to_string(&args[1]).expect("couldn't read file");
    let (tokens, spans) = tokenise(code.as_str());

    for (t, s) in zip(&tokens, &spans) {
        println!("{:?} {:?}", t, s);
    }

    println!("{:?}", oxide_parser::expr(&tokens));
}
