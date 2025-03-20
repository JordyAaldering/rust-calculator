
# Parsing

The parser is then responsible for converting this stream of tokens into an AST.
To determine the case we are in, we sometimes need to look ahead one token
without consuming it, for which we can use `std::iter::Peekable`.
It allows us to peek at the next token without consuming it, similarly to
what we did in the lexer, when peeking at the next character.

```rust
struct Parser<'source> {
    lexer: Peekable<Lexer<'source>>,
}
```
