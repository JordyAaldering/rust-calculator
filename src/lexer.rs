use crate::loc::Loc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    Ne,
    Not,
    Int(u32),
    Unexpected(char),
}

pub struct Lexer<'source> {
    /// The input program as a string.
    source: &'source str,
    /// Index of the current character in the source string.
    current: usize,
    /// Line number of the current character.
    line: usize,
    /// Column number of the current character.
    col: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self { source, current: 0, line: 1, col: 1 }
    }

    /// Return the next character without consuming it.
    fn peek_char(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    /// Return the next character and consume it.
    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.source.chars().nth(self.current) {
            self.current += 1;
            self.col += 1;
            Some(c)
        } else {
            None
        }
    }

    /// Check whether the next character is equal to the expected character.
    /// Consumes the next character if it matches.
    fn match_char(&mut self, expected: char) -> bool {
        if self.peek_char().is_some_and(|c| c == expected) {
            self.current += 1;
            self.col += 1;
            true
        } else {
            false
        }
    }

    /// Skip over any whitespace and newlines until we reach the next character.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            match c {
                // Whitespace
                ' ' | '\t' | '\r' => {
                    self.current += 1;
                    self.col += 1;
                },
                // Newline
                '\n' => {
                    self.current += 1;
                    self.line += 1;
                    self.col = 1;
                }
                // Done
                _ => break,
            }
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = (Token, Loc);

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let col_start = self.col;

        let token = match self.next_char()? {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '/' => Token::Div,
            '^' => Token::Pow,
            '=' if self.match_char('=') => Token::Eq,
            '!' if self.match_char('=') => Token::Ne,
            '!' => Token::Not,
            '0'..='9' => {
                // Include this initial character as well.
                let start = self.current - 1;

                while self.peek_char().is_some_and(|c| c.is_digit(10)) {
                    self.current += 1;
                    self.col += 1;
                }

                let end = self.current;
                Token::Int(self.source[start..end].parse().unwrap())
            }
            c => Token::Unexpected(c),
        };

        let col_end = self.col;
        let loc = Loc::new(self.line, col_start, col_end);
        Some((token, loc))
    }
}
