use std::{fmt, ops};

#[derive(Clone, Copy, Debug)]
#[allow(unused)]
pub struct Loc {
    pub line_start: usize,
    pub line_end: usize,
    pub col_start: usize,
    pub col_end: usize,
}

impl Loc {
    pub fn new(line: usize, col_start: usize, col_end: usize) -> Self {
        Self {
            line_start: line,
            line_end: line,
            col_start,
            col_end
        }
    }
}

impl ops::Add for Loc {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            line_start: self.line_start,
            line_end: rhs.line_end,
            col_start: self.col_start,
            col_end: rhs.col_end,
        }
    }
}

impl ops::AddAssign for Loc {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{} to {}:{}", self.line_start, self.col_start, self.line_end, self.col_end)
    }
}
