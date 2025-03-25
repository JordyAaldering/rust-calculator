use std::iter::Peekable;

use crate::ast::*;
use crate::lexer::{Lexer, Token};
use crate::loc::Loc;

pub struct Parser<'source> {
    lexer: Peekable<Lexer<'source>>,
}

#[derive(Debug)]
#[allow(unused)]
pub enum ParseError {
    NotAnExpr(Token, Loc),
    Unbalanced(Loc, Loc),
    NonAssoc,
    UnexpectedEof,
}

impl<'source> Parser<'source> {
    pub fn new(lexer: Lexer<'source>) -> Self {
        Self { lexer: lexer.peekable() }
    }

    pub fn parse_expr(&mut self) -> Result<(Expr, Loc), ParseError> {
        self.parse_binary(DefaultOperator())
    }

    pub fn parse_binary(&mut self, previous: impl Operator) -> Result<(Expr, Loc), ParseError> {
        let (token, mut loc) = self.next()?;

        let mut left = match token {
            Token::Int(num) => {
                Expr::Num(Num { num, loc })
            },
            Token::LParen => {
                let (expr, _) = self.parse_expr()?;

                let (token, rloc) = self.next()?;
                if token != Token::RParen {
                    // Unbalanced parenthesis; expected a `)` got `token`
                    return Err(ParseError::Unbalanced(loc, rloc));
                }

                loc += rloc;
                expr
            },
            _ => {
                let op = Uop::from_token(token)
                    .ok_or(ParseError::NotAnExpr(token, loc))?;

                let (unary, rloc) = self.parse_unary(op, loc)?;
                loc += rloc;
                Expr::Unary(unary)
            },
        };

        while let Some((op, _loc)) = self.parse_bop(previous)? {
            let (right, rloc) = self.parse_binary(op)?;
            loc += rloc;
            let binary = Binary { l: Box::new(left), r: Box::new(right), op, loc };
            left = Expr::Binary(binary);
        }

        Ok((left, loc))
    }

    fn parse_unary(&mut self, op: Uop, mut loc: Loc) -> Result<(Unary, Loc), ParseError> {
        let (r, rloc) = self.parse_binary(op)?;

        loc += rloc;
        let unary = Unary { op, r: Box::new(r), loc };
        Ok((unary, loc))
    }

    fn parse_bop(&mut self, previous: impl Operator) -> Result<Option<(Bop, Loc)>, ParseError> {
        if let Some((token, _)) = self.lexer.peek() {
            if let Some(op) = Bop::try_from(*token) {
                if precedes(previous, op)? {
                    // Consume the token
                    let (_, loc) = self.lexer.next().unwrap();
                    return Ok(Some((op, loc)));
                }
            }
        }

        Ok(None)
    }

    fn next(&mut self) -> Result<(Token, Loc), ParseError> {
        self.lexer.next().ok_or(ParseError::UnexpectedEof)
    }
}

fn precedes(l: impl Operator, r: impl Operator) -> Result<bool, ParseError> {
    match (l.associativity(), r.associativity()) {
        (Assoc::NonAssoc, Assoc::NonAssoc) => Err(ParseError::NonAssoc),
        (_, Assoc::RightToLeft) => Ok(l.precedence() <= r.precedence()),
        _ => Ok(l.precedence() < r.precedence()),
    }
}
