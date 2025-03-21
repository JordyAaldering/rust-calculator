#[derive(Clone, Copy)]
pub enum Assoc {
    LeftToRight,
    RightToLeft,
    NonAssoc,
}

#[derive(Clone, Copy)]
pub struct DefaultOperator();

impl Operator for DefaultOperator {
    fn precedence(self) -> usize {
        0
    }

    fn associativity(self) -> Assoc {
        Assoc::LeftToRight
    }
}

pub trait Operator: Clone + Copy {
    fn precedence(self) -> usize;

    fn associativity(self) -> Assoc;
}
