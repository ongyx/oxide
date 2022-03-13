mod setup;

use std::fs;

use oxide::ast::Ast;

#[test]
fn parse_examples() {
    let files = setup::examples().unwrap();
    assert!(!files.is_empty());

    for file in files {
        println!("{:?}", file.file_name().unwrap());

        let code = fs::read_to_string(&file).expect("couldn't read file");

        let mut ast = Ast::new(code.as_str());

        if let Err(e) = ast.parse() {
            panic!("{}", ast.format(e));
        }

        println!("{:?}", ast.root);
    }
}
