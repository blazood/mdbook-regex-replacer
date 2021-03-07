# mdbook-regex-replacer
> this is a preprocessor which  invoke regex.replace_all() to replcae all chapter content

## quick start

### install mdbook-regex-replacer

```shell script
cargo install mdbook-regex-replacer
```

### configure book.toml

```toml
[book]
authors = ["blazh"]
language = "en"
multilingual = false
src = "src"
title = "test"

# add into your book.toml
[preprocessor.regex-replacer]
command="mdbook-regex-replacer"

# add your regex rule
# use crates regex 
# https://crates.io/crates/regex
# https://docs.rs/regex/1.4.3/regex/#example-replacement-with-named-capture-groups
[[preprocessor.regex-replacer.items]]
regex="==(?P<c>.+?)=="
rep="<mark>$c</mark>"

```