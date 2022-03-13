use std::{env, fs, process};

use oxide::ast::Ast;

fn main() {
    process::exit(_main())
}

fn _main() -> i32 {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: oxide <file>");
        return 1;
    }

    // open file and parse code
    let code = fs::read_to_string(&args[1]).expect("couldn't read file");

    let mut ast = Ast::new(code.as_str());

    if let Err(e) = ast.parse() {
        println!("{}", ast.format(e));
        return 1;
    }

    println!("{:?}", ast.root);
    0
}
