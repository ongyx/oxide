use std::{env, fs};

use std::iter::zip;

use oxide::ast::Ast;

fn main() {
    let args: Vec<String> = env::args().collect();

    // open file and parse code
    let code = fs::read_to_string(&args[1]).expect("couldn't read file");

    let ast = Ast::new(code.as_str());

    for (t, s) in zip(&ast.tokens, &ast.spans) {
        println!("{:?} {:?}", t, s);
    }

    println!("{:?}", ast.root);
}
