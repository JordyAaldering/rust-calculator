
# Lexing

The first step of the compilation process is lexing. The job of the lexer is to
convert the input string into a list of 'tokens' that our computer understands.
For example, we map the character '+' to the token `PLUS`, an opening
parenthesis '(' becomes `LPAREN`, and the string of numbers "234" is mapped into
an integer `INT(234)`.

Looking back at the grammar definition of our calculator to figure out which
tokens we need, we end up with the following enum type for tokens:

```rust
enum Token {
    LParen, RParen,
    Add, Sub, Mul, Div, Pow,
    Eq, Ne, Not,
    Int(u32),
}
```

To construct a stream of tokens, we define a `Lexer` struct, which needs a
lifetime `'source` for the string.

```rust
struct Lexer<'source> {
    /// The input program as a string.
    source: &'source str,
    /// Index of the current character in the source string.
    current: usize,
}
```

We implement the `Iterator` trait to on-demand generate tokens.

```rust
impl<'source> Iterator for Lexer<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
```

Before trying to lex any token, we first skip any potential whitespace.
It uses a helper function `peek_char`, which returns the current character
without consuming it, if any.

```rust
fn peek_char(&self) -> Option<char> {
    self.source.chars().nth(self.current)
}

fn skip_whitespace(&mut self) {
    while let Some(c) = self.peek_char() {
        match c {
            // Whitespace
            ' ' | '\t' | '\r' => {
                self.current += 1;
            },
            // Newline
            '\n' => {
                self.current += 1;
            }
            // Done
            _ => break,
        }
    }
}
```

Now we can start lexing tokens, for which we need another helper function:
`next_char`. It is similar to `peek_char`, however it consumes the character (if
it exists) by incrementing `current`. Using the try-operator `?`, we can return
early if we are at the end of the stream. Otherwise, we simply map the current
character to a token.

```rust
fn next(&mut self) -> Option<Self::Item> {
    self.skip_whitespace();

    let token = match self.next_char()? {
        '(' => Token::LParen,
        ')' => Token::RParen,
        '+' => Token::Add,
        '-' => Token::Sub,
        '*' => Token::Mul,
        '/' => Token::Div,
        '^' => Token::Pow,
        ...
    };

    Some(token)
}
```

The cases for `==`, `!=`, and `!` are a bit more complicated, since they exist
of multiple tokens, and have some overlap. Namely, whenever we encounter a `!`
we first need to check whether the next token is a `=`, in which case we should
consume that character and return a `Ne` token.

For this conditional consuming of tokens, we define the `match_char` function:

```rust
fn match_char(&mut self, expected: char) -> bool {
    if self.peek_char().is_some_and(|c| c == expected) {
        self.current += 1;
        true
    } else {
        false
    }
}
```

Using this, in combination with match guards, the cases for `==`, `!=`, and `!`
are then defined as follows:

```rust
'=' if self.match_char('=') => Token::Eq,
'!' if self.match_char('=') => Token::Ne,
'!' => Token::Not,
```

Finally, we have the case for integers. This is the most complicated case, as we
do not know how many characters we need to match.
...

```rust
'0'..='9' => {
    // Include this initial character as well.
    let start = self.current - 1;

    while self.peek_char().is_some_and(|c| c.is_digit(10)) {
        self.current += 1;
    }

    let end = self.current;
    Token::Int(self.source[start..end].parse().unwrap())
}
```

However, we need one final case for when an invalid token is supplied.
- We could decide to return `None` in that case, however then in the parser we
  will not be able to distinguish between the end of the stream and invalid
  tokens, so that is not a valid solution.
- We could change the type of the iterator from `Option<Token>` to
  `Option<Result<Token, ErrorType>>`, however we then need to pack and unpack
  tokens every time, which is cumbersome and makes the code harder to read.
- Instead, my preferred solution is to add one more case to the token enum for
  unexpected characters. It allows us to distinguish errors, without being
  overly cumbersome.

```rust
enum Token {
    ...
    Unexpected(char),
}
```

## Location Information

Whenever an error occurs, we want to be able to give the corresponding line
and column numbers from the source code file.
We represent this in a struct that tracks the starting line and column, and the
ending line and column.
Although a token can only span a single line, we will also be using this
location information in the parser, where nodes can span multiple lines.

```rust
struct Loc {
    line_start: usize,
    line_end: usize,
    col_start: usize,
    col_end: usize,
}

impl Loc {
    fn new(line: usize, col_start: usize, col_end: usize) -> Self {
        Self {
            line_start: line,
            line_end: line,
            col_start,
            col_end
        }
    }
}
```

We then extend the `Lexer` struct to keep track of the current line number and
column number in the source file.

```rust
struct Lexer<'source> {
    /// The input program as a string.
    source: &'source str,
    /// Index of the current character in the source string.
    current: usize,
    /// Line number of the current character.
    line: usize,
    /// Column number of the current character.
    col: usize,
}
```

Whenever we consume a character, we also increment the current column number.
Expect when we encounter a newline character, in which case we reset the column
number to one, and increment the current line number instead.

We change the type of the iterator from `Token` to a token-location tuple
`(Token, Loc)`, and create and return the location information.

```rust
impl<'source> Iterator for Lexer<'source> {
    type Item = (Token, Loc);

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let col_start = self.col;

        let token = match self.next_char()? {
            ...
        };

        let col_end = self.col;
        let loc = Loc::new(self.line, col_start, col_end);
        Some((token, loc))
    }
}
```
