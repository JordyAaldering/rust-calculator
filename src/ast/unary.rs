use std::fmt;

use crate::{lexer::Token, loc::Loc};

use super::{Assoc, Expr, Operator};

#[derive(Clone)]
pub struct Unary {
    pub r: Box<Expr>,
    pub op: Uop,
    pub loc: Loc,
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.op, self.r)
    }
}

#[derive(Clone, Copy)]
pub enum Uop {
    Neg,
    Not,
}

impl Uop {
    pub fn from_token(token: Token) -> Option<Self> {
        match token {
            Token::Sub => Some(Uop::Neg),
            Token::Not => Some(Uop::Not),
            _ => None,
        }
    }
}

impl Operator for Uop {
    /// Unary operators always have precedence
    fn precedence(self) -> usize {
        256
    }

    fn associativity(self) -> Assoc {
        Assoc::LeftToRight
    }
}

impl fmt::Display for Uop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Uop::Neg => write!(f, "-"),
            Uop::Not => write!(f, "!"),
        }
    }
}
