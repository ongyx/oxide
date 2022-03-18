# cub

This folder contains source code files for testing, copied over from the [Cub] repo.

These files have been modified to remove Cub syntax not supported in oxide, namely:

- Identifiers: Strings recognised as [Unicode identifers](http://www.unicode.org/reports/tr31/) are allowed as identifers, but other codepoints (i.e emojis) are not supported.
- `do <expr> times`: Removed in favour of `for _ in range(<expr>)`.
- `returns`: Functions need not be marked as returning.

The following folders were copied over:

* `examples`: `Source examples/`
* `source`: `Tests/CubTests/Test source files/`
* `misc`: `Sources/Cub/Standard Library/Sources/`, `macOS Example/macOS Example/`

[Cub]: github.com/louisdh/cub
