
# Parsing

The parser is then responsible for converting this stream of tokens into an AST.
Conceptually this task is simple; given a stream of tokens like `INT(3)`, `ADD`,
and `INT(4)`, we as humans can look at the entire thing at once, and realize it
is a binary expression `Binary { l: Int(3), op: Bop::Add, r: Int(4) }`. However,
a computer only sees the tokens one-by-one. Given the first token, `INT(3)`, it
is unclear what the result is going to be. For this reason, we need to look
ahead one or more tokens.

One token lookahead might not be enough, consider the following C code:

```C
int foo;

int bar() { return 1; }
```

Both the global variable and the function start with a type name, followed by an
identifier (`foo` and `bar`). Only when a `;` or `(` is encountered, is it clear
whether we are parsing a global variable, or a function definition.

Luckily, for our calculator we only need one token of lookahead. If you are
developing a language, and find that multiple tokens of lookahead are needed, it
might be beneficial to go back to the grammar, and change it in such a way that
only a single token of lookahead is needed. In the case of the C code, for
example, you might add an `fn` before functions, just like in Rust.

We can use `std::iter::Peekable` to look ahead one token without consuming in.

```rust
struct Parser<'source> {
    lexer: Peekable<Lexer<'source>>,
}
```

Anything in our calculator language is an expression, which always result in a
value and do not incur any side effects. So we start with the function
`parse_expr`, which returns an `Expr` node we defined in the AST. We might
encounter errors, such as unclosed parentheses, so we wrap this node in a
`Result`.

Whenever we encounter an integer token there is nothing more to do, and we
create a `Num` node. Whenever we encounter an opening parenthesis we try to
parse the expression inside it. As we see in the `Num` case, we only consume
exactly those tokens that we need, and return whenever an AST node was
constructed. So after parsing the expression we expect the next token to be a
closing parenthesis, which should not have been consumed yet. If not, we raise
an error.

```rust
#[derive(Debug)]
enum ParseError {
    Unbalanced(Token, Token),
}

fn parse_expr(&mut self) -> Result<Expr, ParseError> {
    let token = self.lexer.next()
        .ok_or(ParseError::UnexpectedEof)?;

    let left = match token {
        Token::Int(num) => {
            Expr::Num(Num { num })
        },
        Token::LParen => {
            let expr = self.parse_expr()?;

            let token = self.next()?;
            if self.next()? != Token::RParen {
                // Unbalanced parenthesis; expected a `)` got `token`
                return Err(ParseError::Unbalanced(Token::RParen, token));
            }

            expr
        },
        _ => {
            todo!()
        }
    }

    Ok(left)
}
```

The next problem is dealing with operator precedence and associativity.
Precedence tells us which operator goes before which one. For example,
multiplication and division have a stronger precedence than addition and
subtraction. E.g., for some expression `1 + 2 * 3`, we must place the
parenthesis as `1 + (2 * 3)`. Note however that addition and subtraction share
the same precedence.

When two operators share the same precedence, their order is determined by their
associativity. Given some expression `1 + 2 + 3`, the associativity tells us
whether the parenthesis are placed as `(1 + 2) + 3` or as `1 + (2 + 3)`. If an
operator is left-associative, the first case happens. If it is right-associative
the second case occurs.

In some cases we don't want operators to be chained at all, which is why `==`
was introduced. Consider `1 == 2 == 2` if equality were left-associative, 1 is
not equal to 2, so then we get something like `false == 2`, which means we are
comparing two different types. To restrict this behaviour, equality is made
non-associative; meaning that we can never chain it with other non-associative
operators. Of course, something like `1 + 2 == 4 - 1` should be allowed, or even
something like `(1 == 2) == (2 == 3)`.

Handling precedence and associativity in a parser is a non-trivial task.
However, the Pratt parsing algorithm provides an elegant and relatively simple
solution to this problem.
