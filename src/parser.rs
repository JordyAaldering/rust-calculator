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
    UnexpectedToken(Token, Loc),
    Unbalanced(Token, Loc, Loc),
    NonAssoc,
    UnexpectedEof,
}

impl<'source> Parser<'source> {
    pub fn new(lexer: Lexer<'source>) -> Self {
        Self { lexer: lexer.peekable() }
    }

    pub fn parse_expr(&mut self) -> Result<(Expr, Loc), ParseError> {
        self.parse_binary((0, Assoc::LeftToRight))
    }

    pub fn parse_binary(&mut self, prec_assoc: (usize, Assoc)) -> Result<(Expr, Loc), ParseError> {
        let (token, mut loc) = self.next()?;

        let mut left = match token {
            Token::Int(num) => {
                Expr::Num(Num { num, loc })
            },
            Token::LParen => {
                let (expr, _) = self.parse_expr()?;

                let (token, end) = self.next()?;
                if token != Token::RParen {
                    return Err(ParseError::Unbalanced(Token::LParen, loc, end));
                }

                loc += end;
                expr
            },
            _ => {
                let op = token.try_into_uop()
                    .ok_or(ParseError::UnexpectedToken(token, loc))?;

                let unary = self.parse_unary(op, loc)?;
                loc += unary.loc;
                Expr::Unary(unary)
            },
        };

        while let Some((op, _loc)) = self.parse_bop(prec_assoc)? {
            let (right, end) = self.parse_binary(op.group())?;
            loc += end;
            let binary = Binary { l: Box::new(left), r: Box::new(right), op, loc };
            left = Expr::Binary(binary);
        }

        Ok((left, loc))
    }

    fn parse_unary(&mut self, op: Uop, loc: Loc) -> Result<Unary, ParseError> {
        // Unary operators always have precedence
        let (r, end) = self.parse_binary((256, Assoc::LeftToRight))?;
        let unary = Unary { op, r: Box::new(r), loc: loc + end };
        Ok(unary)
    }

    fn parse_bop(&mut self, prec_assoc: (usize, Assoc)) -> Result<Option<(Bop, Loc)>, ParseError> {
        if let Some((token, _)) = self.lexer.peek() {
            if let Some(bop) = token.try_into_bop() {
                if precedes(prec_assoc, bop.group())? {
                    // Consume the token
                    let (_, loc) = self.lexer.next().unwrap();
                    return Ok(Some((bop, loc)));
                }
            }
        }

        Ok(None)
    }

    fn next(&mut self) -> Result<(Token, Loc), ParseError> {
        self.lexer.next().ok_or(ParseError::UnexpectedEof)
    }
}

#[derive(Clone, Copy)]
pub enum Assoc {
    LeftToRight,
    RightToLeft,
    NonAssoc,
}

fn precedes((p1, a1): (usize, Assoc), (p2, a2): (usize, Assoc)) -> Result<bool, ParseError> {
    match (a1, a2) {
        (Assoc::NonAssoc, Assoc::NonAssoc) => Err(ParseError::NonAssoc),
        (_, Assoc::RightToLeft) => Ok(p1 <= p2),
        _ => Ok(p1 < p2),
    }
}

impl Bop {
    fn group(&self) -> (usize, Assoc) {
        match self {
            Bop::Pow => (5, Assoc::RightToLeft),
            Bop::Mul | Bop::Div => (4, Assoc::LeftToRight),
            Bop::Add | Bop::Sub => (3, Assoc::LeftToRight),
            Bop::Eq | Bop::Ne => (2, Assoc::NonAssoc),
        }
    }
}

impl Token {
    fn try_into_bop(&self) -> Option<Bop> {
        match self {
            Token::Add => Some(Bop::Add),
            Token::Sub => Some(Bop::Sub),
            Token::Mul => Some(Bop::Mul),
            Token::Div => Some(Bop::Div),
            Token::Pow => Some(Bop::Pow),
            Token::Eq => Some(Bop::Eq),
            Token::Ne => Some(Bop::Ne),
            _ => None,
        }
    }

    fn try_into_uop(&self) -> Option<Uop> {
        match self {
            Token::Sub => Some(Uop::Neg),
            Token::Not => Some(Uop::Not),
            _ => None,
        }
    }
}
