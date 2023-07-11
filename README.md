# mdbook-hide

A preprocessor for [mdbook](https://rust-lang.github.io/mdBook) that adds
support for hidden chapters. The hidden chapters can be marked by adding a
special Markdown comment. Based on the config, hidden chapters can be added or
removed in the build.

## Installation

Currently, this preprocessor is only available as a crate. In future, I'll add
the binary releases as well. To install, run the command:

```sh
cargo install mdbook-hide
```

## Usage

To use the preprocessor in your mdbook, add the following section in the
`book.toml`. If the `hide` is set to true, then the hidden chapters will be
removed.

```toml
[preprocessor.hide]
hide = true
```

To mark a chapter as hidden, add this special Comment anywhere in the Markdown
file. It is better to have it at the top of the file for clarity.

```markdown
<!--hidden-->
```
