use crate::{ast::*, trav::Traversal, typ::Type};

pub struct TypeCheck { }

#[derive(Debug)]
#[allow(unused)]
pub enum TypeError {
    UnificationError(Type, Type),
}

impl Traversal<Type, TypeError> for TypeCheck {
    fn trav_binary(&mut self, binary: &mut Binary) -> Result<Type, TypeError> {
        let l = self.trav_expr(&mut binary.l)?;
        let r = self.trav_expr(&mut binary.r)?;
        let typ = unify(l, r)?;

        match binary.op {
            Bop::Add | Bop::Sub | Bop::Mul | Bop::Div | Bop::Pow =>
                unify(typ, Type::Int),
            Bop::Eq | Bop::Ne =>
                unify(typ, Type::Bool),
        }
    }

    fn trav_unary(&mut self, unary: &mut Unary) -> Result<Type, TypeError> {
        let typ = self.trav_expr(&mut unary.r)?;

        match unary.op {
            Uop::Neg => unify(typ, Type::Int),
            Uop::Not => unify(typ, Type::Bool),
        }
    }

    fn trav_num(&mut self, _num: &mut Num) -> Result<Type, TypeError> {
        Ok(Type::Int)
    }
}

fn unify(a: Type, b: Type) -> Result<Type, TypeError> {
    match (a, b) {
        (Type::Int, Type::Int) => Ok(Type::Int),
        (Type::Bool, Type::Bool) => Ok(Type::Bool),
        _ => Err(TypeError::UnificationError(a, b)),
    }
}
