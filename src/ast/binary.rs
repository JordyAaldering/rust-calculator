use std::fmt;

use crate::{lexer::Token, loc::Loc};

use super::{Assoc, Expr, Operator};

#[derive(Clone)]
pub struct Binary {
    pub l: Box<Expr>,
    pub r: Box<Expr>,
    pub op: Bop,
    pub loc: Loc,
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.l, self.op, self.r)
    }
}

#[derive(Clone, Copy)]
pub enum Bop {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    Ne,
}

impl Bop {
    pub fn try_from(token: Token) -> Option<Self> {
        match token {
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
}

impl Operator for Bop {
    fn precedence(self) -> usize {
        match self {
            Bop::Eq | Bop::Ne => 2,
            Bop::Add | Bop::Sub => 3,
            Bop::Mul | Bop::Div => 4,
            Bop::Pow => 5,
        }
    }

    fn associativity(self) -> Assoc {
        match self {
            Bop::Eq | Bop::Ne => Assoc::NonAssoc,
            Bop::Add | Bop::Sub | Bop::Mul | Bop::Div => Assoc::LeftToRight,
            Bop::Pow => Assoc::RightToLeft,
        }
    }
}

impl fmt::Display for Bop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bop::Add => write!(f, "+"),
            Bop::Sub => write!(f, "-"),
            Bop::Mul => write!(f, "*"),
            Bop::Div => write!(f, "/"),
            Bop::Pow => write!(f, "^"),
            Bop::Eq => write!(f, "=="),
            Bop::Ne => write!(f, "!="),
        }
    }
}
