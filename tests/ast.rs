mod setup;

use std::{fs, path};

use oxide::ast::Ast;

use setup::{find_ext, relpath};

fn astify(file: path::PathBuf) {
    println!("{:?}", file.file_name().unwrap());

    let code = fs::read_to_string(&file).expect("couldn't read file");

    let mut ast = Ast::new(&code);

    if let Err(e) = ast.parse() {
        panic!("{}", ast.format(e));
    }
}

#[test]
fn parse_source() {
    let files = find_ext(relpath("tests/source"), "oxide").unwrap();
    assert!(!files.is_empty());

    for file in files {
        astify(file);
    }
}

#[test]
fn parse_cub_source() {
    let files = find_ext(relpath("tests/cub/source"), "cub").unwrap();
    assert!(!files.is_empty());

    for file in files {
        astify(file);
    }
}

#[test]
fn parse_cub_misc() {
    let files = find_ext(relpath("tests/cub/misc"), "cub").unwrap();
    assert!(!files.is_empty());

    for file in files {
        astify(file);
    }
}
