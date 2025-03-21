use std::fmt;

use crate::loc::Loc;

use super::{Binary, Unary, Num};

#[derive(Clone)]
pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Num(Num),
}

impl Expr {
    pub fn loc(&self) -> Loc {
        match self {
            Expr::Binary(binary) => binary.loc,
            Expr::Unary(unary) => unary.loc,
            Expr::Num(num) => num.loc,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(binary) => write!(f, "{}", binary),
            Expr::Unary(unary) => write!(f, "{}", unary),
            Expr::Num(num) => write!(f, "{}", **num),
        }
    }
}
