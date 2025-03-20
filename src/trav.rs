use std::fmt;

use crate::ast::*;

pub trait Traversal<T: Default, E: fmt::Debug> {
    fn trav_expr(&mut self, expr: &mut Expr) -> Result<T, E> {
        match expr {
            Expr::Binary(binary) => self.trav_binary(binary),
            Expr::Unary(unary) => self.trav_unary(unary),
            Expr::Num(num) => self.trav_num(num),
        }
    }

    fn trav_binary(&mut self, binary: &mut Binary) -> Result<T, E> {
        self.trav_expr(&mut binary.l)?;
        self.trav_expr(&mut binary.r)
    }

    fn trav_unary(&mut self, unary: &mut Unary) -> Result<T, E> {
        self.trav_expr(&mut unary.r)
    }

    fn trav_num(&mut self, _num: &mut Num) -> Result<T, E> {
        Ok(T::default())
    }
}
