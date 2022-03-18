mod setup;

use std::{fs, path};

use oxide::ast::Ast;

use setup::{find_ext, relpath};

fn astify(file: path::PathBuf) -> Result<(), String> {
    println!("{:?}", file.file_name().unwrap());

    let code = fs::read_to_string(&file).expect("couldn't read file");

    let mut ast = Ast::new(&code);

    match ast.parse() {
        Err(e) => Err(ast.format(e)),
        Ok(()) => Ok(()),
    }
}

fn must_astify(file: path::PathBuf) {
    if let Err(e) = astify(file) {
        panic!("{}", e);
    }
}

#[test]
fn source() {
    let files = find_ext(relpath("tests/source"), "oxide").unwrap();
    assert!(!files.is_empty());

    for file in files {
        must_astify(file);
    }
}

#[test]
fn cub_source_pass() {
    let files = find_ext(relpath("tests/cub/source_pass"), "cub").unwrap();
    assert!(!files.is_empty());

    for file in files {
        must_astify(file);
    }
}

#[test]
fn cub_source_fail() {
    let files = find_ext(relpath("tests/cub/source_fail"), "cub").unwrap();
    assert!(!files.is_empty());

    for file in files {
        if let Ok(()) = astify(file) {
            panic!("expected failure to parse")
        }
    }
}

#[test]
fn cub_misc() {
    let files = find_ext(relpath("tests/cub/misc"), "cub").unwrap();
    assert!(!files.is_empty());

    for file in files {
        must_astify(file);
    }
}
