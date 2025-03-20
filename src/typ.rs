#[derive(Clone, Copy, Debug)]
pub enum Type {
    Int,
    Bool,
}

impl Default for Type {
    fn default() -> Self {
        Type::Int
    }
}
