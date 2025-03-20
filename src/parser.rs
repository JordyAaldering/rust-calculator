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

    pub fn parse_expr(&mut self, precedence: OperatorGroup) -> Result<(Expr, Loc), ParseError> {
        let (token, mut loc) = self.next()?;

        let mut left = match token {
            Token::Int(num) => {
                Expr::Num(Num { num, loc })
            },
            Token::LParen => {
                let (expr, _) = self.parse_expr(OperatorGroup::LeftToRight(0))?;

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

        while let Some((op, _loc)) = self.parse_bop(precedence)? {
            let (right, end) = self.parse_expr(op.group())?;
            loc += end;
            let binary = Binary { l: Box::new(left), r: Box::new(right), op, loc };
            left = Expr::Binary(binary);
        }

        Ok((left, loc))
    }

    fn parse_unary(&mut self, op: Uop, loc: Loc) -> Result<Unary, ParseError> {
        // Unary operators always have precedence
        let (r, end) = self.parse_expr(OperatorGroup::LeftToRight(256))?;
        let unary = Unary { op, r: Box::new(r), loc: loc + end };
        Ok(unary)
    }

    fn parse_bop(&mut self, precedence: OperatorGroup) -> Result<Option<(Bop, Loc)>, ParseError> {
        if let Some((token, _)) = self.lexer.peek() {
            if let Some(bop) = token.try_into_bop() {
                if precedence.precedes(bop.group())? {
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
pub enum OperatorGroup {
    LeftToRight(usize),
    RightToLeft(usize),
    NonAssoc(usize),
}

impl OperatorGroup {
    fn precedes(self, other: OperatorGroup) -> Result<bool, ParseError> {
        match (self, other) {
            (OperatorGroup::LeftToRight(l), OperatorGroup::RightToLeft(r)) |
            (OperatorGroup::RightToLeft(l), OperatorGroup::RightToLeft(r)) |
            (OperatorGroup::NonAssoc(l), OperatorGroup::RightToLeft(r)) => {
                Ok(l <= r)
            },
            (OperatorGroup::LeftToRight(l), OperatorGroup::LeftToRight(r)) |
            (OperatorGroup::LeftToRight(l), OperatorGroup::NonAssoc(r)) |
            (OperatorGroup::RightToLeft(l), OperatorGroup::LeftToRight(r)) |
            (OperatorGroup::RightToLeft(l), OperatorGroup::NonAssoc(r)) |
            (OperatorGroup::NonAssoc(l), OperatorGroup::LeftToRight(r)) => {
                Ok(l < r)
            },
            (OperatorGroup::NonAssoc(_), OperatorGroup::NonAssoc(_)) => {
                Err(ParseError::NonAssoc)
            }
        }
    }
}

impl Bop {
    fn group(&self) -> OperatorGroup {
        match self {
            Bop::Pow => OperatorGroup::RightToLeft(5),
            Bop::Mul | Bop::Div => OperatorGroup::LeftToRight(4),
            Bop::Add | Bop::Sub => OperatorGroup::LeftToRight(3),
            Bop::Eq | Bop::Ne => OperatorGroup::NonAssoc(2),
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
