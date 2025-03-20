use crate::trav::Traversal;

pub struct TypeCheck { }

#[derive(Debug)]
pub enum TypeError {
    Error
}

impl Traversal<TypeError> for TypeCheck {

}
