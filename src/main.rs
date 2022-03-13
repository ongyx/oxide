use std::{env, fs};

use oxide::ast::Ast;

fn main() {
    let args: Vec<String> = env::args().collect();

    // open file and parse code
    let code = fs::read_to_string(&args[1]).expect("couldn't read file");

    let ast = Ast::new(code.as_str());

    println!("{:?}", ast.root);
}
