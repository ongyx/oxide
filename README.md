# oxide

Interpreted programming language with a runtime written in rust.

oxide is the successor of [ox], a previous effort to create a programming language in Python.
Both oxide and ox share a syntax that is inspired by [Cub].

Like CPython and Cub, oxide compiles down into bytecode before being executed in a virtual machine.
The code pipeline looks like this:

1. Tokenise (`&str` -> `Vec<Token>`)
2. Parse (`Vec<Token>` -> `AST`)
3. Compile (`AST` -> `Vec<Bytecode>`)
4. Execute (inside the VM runtime)

oxide is my first foray into Rust (and I'm still learning),
so feel free to open PRs or issues if you have suggestions/improvements to contribute.

## Setup

oxide requires a nightly version of rust, so make sure to run `rustup toolchain install nightly` if you haven't done so.

To test, just run `cargo test`.

## Todo

- [x] AST parser
- [x] Type system
- [ ] Bytecode compile (from AST)
- [ ] Bytecode execution
- [x] Unit tests
- [ ] Integration tests
- [ ] Interactive interpreter?

## License

MIT.

[Cub]: https://github.com/louisdh/cub
[ox]: https://github.com/ongyx/ox
