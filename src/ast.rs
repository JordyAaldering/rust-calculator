use std::fmt;

use crate::loc::Loc;

pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    U32(U32),
}

impl Expr {
    pub fn loc(&self) -> Loc {
        match self {
            Expr::Binary(binary) => binary.loc,
            Expr::Unary(unary) => unary.loc,
            Expr::U32(u32) => u32.loc,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(binary) => write!(f, "{}", binary),
            Expr::Unary(unary) => write!(f, "{}", unary),
            Expr::U32(u32) => write!(f, "{}", **u32),
        }
    }
}

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

pub struct U32 {
    pub i: u32,
    pub loc: Loc,
}

impl std::ops::Deref for U32 {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.i
    }
}

pub enum Bop {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    Ne,
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

pub enum Uop {
    Neg,
    Not,
}

impl fmt::Display for Uop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Uop::Neg => write!(f, "-"),
            Uop::Not => write!(f, "!"),
        }
    }
}
