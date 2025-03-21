use std::ops;

use crate::loc::Loc;

#[derive(Clone)]
pub struct Num {
    pub num: u32,
    // Metadata
    pub loc: Loc,
}

impl ops::Deref for Num {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.num
    }
}
